"""Unprivileged tests only: namespace/kernel actions are models, never admission."""

import ctypes
import ast
import errno
import importlib.util
import json
import os
from pathlib import Path
import shutil
import re
import socket
import subprocess
import sys
import tempfile
import time
import unittest
from unittest import mock

ROOT = Path(__file__).resolve().parents[2]


def load(name):
    spec = importlib.util.spec_from_file_location(name, ROOT / "scripts" / name)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


CORE = load("claude-agent-sdk-netns-proposal.py")
KERNEL = load("claude-agent-sdk-netns-kernel.py")
CUSTODY = load("claude-agent-sdk-probe-custody.py")
ENTRY = load("claude-agent-sdk-netns-entry.py")


class NamespaceProposalTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.node = Path(shutil.which("node")).resolve(strict=True)
        cls.node_sha = CORE.sha(cls.node.read_bytes())
        cls.record_sha = CORE.sha((ROOT / CORE.RECORD).read_bytes())

    def assert_no_children(self, result):
        self.assertFalse(result["kernelEnforced"])
        self.assertFalse(result["originalActivation"])
        if result["protocol"]:
            for row in result["protocol"]["exits"]:
                with self.assertRaises(ProcessLookupError):
                    os.kill(row["pid"], 0)

    def run_protocol(self, ops=None, **kwargs):
        result = CORE.rehearse(self.record_sha, self.node, self.node_sha, ops, **kwargs)
        self.assert_no_children(result)
        return result

    def test_real_ordinary_tcp_selected_driver_and_sidecar(self):
        for mode in ("probe", "sidecar"):
            with self.subTest(mode=mode):
                result = self.run_protocol(
                    mode=mode,
                    scenario="managed-deny" if mode == "sidecar" else "permission",
                )
                self.assertIsNone(
                    result["failure"],
                    {
                        "failure": result["failure"],
                        "hostExitEvents": (result["protocol"] or {}).get(
                            "hostExitEvents"
                        ),
                        "driver": (result["protocol"] or {}).get("driver"),
                    },
                )
                self.assertTrue(result["protocol"]["ordinaryTcp"])
                self.assertEqual(len(result["protocol"]["requests"]), 1)
                self.assertEqual(
                    [row["returncode"] for row in result["protocol"]["exits"]], [0, 0]
                )
                self.assertIn(
                    '"event":"permission"'
                    if mode == "probe"
                    else '"event":"turn_ended"',
                    result["protocol"]["driver"],
                )

    def test_callback_and_managed_denial_remain_distinct(self):
        callback = self.run_protocol()
        managed = self.run_protocol(scenario="managed-deny")
        self.assertIn('"event":"permission"', callback["protocol"]["driver"])
        self.assertNotIn('"event":"permission"', managed["protocol"]["driver"])
        self.assertIsNone(managed["failure"], managed)

    def test_actual_cancellation_after_owned_children_start_joins_both(self):
        ops = CORE.Model(cancel_after_children=True)
        result = self.run_protocol(ops, scenario="never-exit")
        self.assertEqual(result["failure"], "protocol_supervisor_cancelled", result)
        self.assertEqual(ops.started, 2)
        self.assertTrue(ops.cleaned)

    def test_actual_hanging_child_deadline_joins_both(self):
        result = self.run_protocol(scenario="never-exit", bound=1)
        self.assertEqual(
            result["failure"], "protocol_deadline_or_hanging_native", result
        )

    def test_cancel_and_controlled_clock_expiry_at_every_phase(self):
        def no_processes(ops):
            return {"failure": None, "exits": [{}, {}]}

        for phase in CORE.PHASES:
            for kind in ("cancel", "expire"):
                with self.subTest(phase=phase, kind=kind):
                    ops = CORE.Model(**{kind: phase})
                    result = CORE.execute(ops, no_processes)
                    self.assertIn(
                        ("cancelled" if kind == "cancel" else "deadline")
                        + "_at_"
                        + phase,
                        result["failure"],
                    )
                    self.assertTrue(ops.cleaned)
                    self.assertFalse(result["kernelEnforced"])

    def test_setup_refusal_partial_setup_and_cleanup_failure(self):
        for phase in CORE.PHASES[:-1]:
            ops = CORE.Model(fail=phase)
            result = CORE.execute(
                ops, lambda selected: self.fail("must refuse before protocol")
            )
            self.assertIn("simulated_" + phase, result["failure"])
            self.assertTrue(ops.cleaned)
            self.assertEqual(ops.created, CORE.PHASES.index(phase) > 1)
        ops = CORE.Model(fail="cleanup_observation")
        result = CORE.execute(
            ops, lambda selected: {"failure": None, "exits": [{}, {}]}
        )
        self.assertIn("simulated_cleanup_failure", result["failure"])
        self.assertIsNone(result["cleanup"])

    def test_missing_join_does_not_credit_sdk_completion(self):
        result = self.run_protocol(CORE.Model(missing_join=True))
        self.assertEqual(result["failure"], "missing_join_observation")

    def test_wrong_namespace_interfaces_and_routes_refuse_before_protocol(self):
        for change in (
            {"namespace": {"device": 0, "inode": 1}},
            {"interfaces": ["lo", "eth0"]},
            {"externalRoutes": ["default via modeled-host"]},
        ):
            ops = CORE.Model()
            ops.network.update(change)
            result = CORE.execute(
                ops, lambda selected: self.fail("wrong namespace must refuse")
            )
            self.assertIsNotNone(result["failure"])
            self.assertTrue(ops.cleaned)

    def test_inherited_real_socket_is_refused_even_in_model(self):
        first, second = socket.socketpair()
        try:
            with self.assertRaisesRegex(CORE.Refused, "inherited_egress_socket"):
                CORE.reject_inherited_sockets([first.fileno(), second.fileno()])
        finally:
            first.close()
            second.close()

    def test_source_and_admission_drift_before_import_or_child(self):
        with tempfile.TemporaryDirectory(
            prefix="sdk-netns-source-", dir=os.environ["TMPDIR"]
        ) as directory:
            root = Path(directory).resolve(strict=True)
            original = (ROOT / CORE.RECORD).read_text()
            for row in original.splitlines():
                _, name = row.split("  ", 1)
                (root / name).parent.mkdir(parents=True, exist_ok=True)
                shutil.copyfile(ROOT / name, root / name)
            (root / CORE.RECORD).parent.mkdir(parents=True, exist_ok=True)
            (root / CORE.RECORD).write_text(original)
            target = root / "scripts/claude-agent-sdk-netns-native.mjs"
            target.write_bytes(target.read_bytes() + b"\n")
            with self.assertRaisesRegex(CORE.Refused, "source_mismatch_before_import"):
                CORE.snapshots(root, self.record_sha)
            shutil.copyfile(ROOT / "scripts/claude-agent-sdk-netns-native.mjs", target)
            admission = root / CORE.ADMISSION
            value = json.loads(admission.read_text())
            value["originalActivation"] = True
            admission.write_text(json.dumps(value))
            record = "".join(
                f"{CORE.sha((root / row.split('  ', 1)[1]).read_bytes())}  {row.split('  ', 1)[1]}\n"
                for row in original.splitlines()
            )
            (root / CORE.RECORD).write_text(record)
            with self.assertRaisesRegex(CORE.Refused, "fake_admission_scope_mismatch"):
                CORE.snapshots(root, CORE.sha(record.encode()))

    def test_partial_child_start_refusal_still_observes_real_wait(self):
        class RefuseSecond(CORE.Model):
            def __init__(self):
                super().__init__()
                self.pids = []

            def confirm_child(self, child):
                self.pids.append(child.pid)
                if len(self.pids) == 2:
                    raise CORE.Refused("second_child_admission_refused")

        ops = RefuseSecond()
        result = self.run_protocol(ops)
        self.assertEqual(result["failure"], "second_child_admission_refused")
        self.assertEqual(len(ops.pids), 2)
        for pid in ops.pids:
            with self.assertRaises(ProcessLookupError):
                os.kill(pid, 0)

    def test_control_pipe_partial_message_deadline_and_cancellation(self):
        read, write = os.pipe()
        try:
            os.write(write, b'{"phase":')
            with self.assertRaisesRegex(OSError, "monitor_deadline"):
                KERNEL.read_line(read, time.monotonic() - 1, bytearray())
            with self.assertRaisesRegex(OSError, "monitor_cancelled"):
                KERNEL.read_line(read, time.monotonic() + 5, bytearray(), lambda: True)
        finally:
            os.close(read)
            os.close(write)

    def test_nonroot_identity_requires_all_observations(self):
        good = {
            "uids": [1001] * 4,
            "gids": [1001] * 4,
            "groups": [],
            "caps": {
                key: "0" for key in ("CapEff", "CapPrm", "CapInh", "CapAmb", "CapBnd")
            },
        }
        KERNEL.require_nonroot(good, 1001, 1001)
        for change in (
            {"uids": [0] * 4},
            {"groups": ["10"]},
            {"caps": {}},
            {"gids": [0] * 4},
        ):
            with self.assertRaises(OSError):
                KERNEL.require_nonroot({**good, **change}, 1001, 1001)

    def test_original_mode_is_not_an_admitted_target(self):
        with self.assertRaisesRegex(CORE.Refused, "original_or_unknown_mode"):
            CORE.rehearse(self.record_sha, self.node, self.node_sha, mode="original")

    def test_socket_and_process_filter_programs_without_loading_a_kernel_filter(self):
        captured = []

        def fake_prctl(option, mode, pointer, *rest):
            program = ctypes.cast(pointer, ctypes.POINTER(ENTRY.Program)).contents
            captured.append(
                [
                    (row.code, row.jt, row.jf, row.k)
                    for row in program.filter[: program.length]
                ]
            )
            return 0

        with mock.patch.object(
            ENTRY, "LIBC", type("FakeLibc", (), {"prctl": staticmethod(fake_prctl)})()
        ):
            ENTRY.restrict_syscalls()
            ENTRY.restrict_tcp_sockets()

        def interpret(rows, number, args):
            memory = {0: number, 4: 0xC000003E, 16: args[0], 24: args[1], 32: args[2]}
            pc = 0
            value = 0
            for _ in range(1000):
                code, yes, no, key = rows[pc]
                if code == 0x20:
                    value = memory[key]
                elif code == 0x15:
                    pc += yes if value == key else no
                elif code == 0x45:
                    pc += yes if value & key else no
                elif code == 0x54:
                    value &= key
                elif code == 0x06:
                    return key
                else:
                    self.fail("unknown BPF instruction")
                pc += 1
            self.fail("filter did not terminate")

        allow = 0x7FFF0000
        for family, kind, protocol, allowed in [
            (socket.AF_INET, socket.SOCK_STREAM, 0, True),
            (socket.AF_INET6, socket.SOCK_STREAM, 6, True),
            (socket.AF_UNIX, socket.SOCK_STREAM, 0, False),
            (socket.AF_INET, socket.SOCK_DGRAM, 0, False),
            (socket.AF_INET, socket.SOCK_RAW, 0, False),
            (socket.AF_INET, socket.SOCK_STREAM, 132, False),
        ]:
            self.assertEqual(
                all(
                    interpret(rows, 41, [family, kind, protocol]) == allow
                    for rows in captured
                ),
                allowed,
            )
        for syscall in (57, 58, 49, 50, 272, 308, 425, 438, 109, 112):
            self.assertNotEqual(interpret(captured[0], syscall, [0, 0, 0]), allow)
        self.assertEqual(interpret(captured[0], 42, [0, 0, 0]), allow)

    def test_applied_job_lint_and_commands_without_dispatch_or_privilege(self):
        proposal = ROOT / "docs/knowledge/operations/claude-agent-sdk-netns-ci.patch"
        subprocess.run(
            [
                "git",
                "apply",
                "--reverse",
                "--check",
                "--include=.github/workflows/sdk234-netns-proof.yml",
                str(proposal),
            ],
            cwd=ROOT,
            check=True,
            capture_output=True,
            timeout=5,
        )
        lines = [
            row[1:]
            for row in proposal.read_text()
            .split("diff --git a/scripts/tests/", 1)[0]
            .splitlines()
            if row.startswith("+") and not row.startswith("+++")
        ]
        with tempfile.TemporaryDirectory(
            prefix="sdk-netns-job-", dir=os.environ["TMPDIR"]
        ) as directory:
            root = Path(directory)
            workflow = root / "proposal.yml"
            workflow.write_text("\n".join(lines) + "\n")
            subprocess.run(
                ["actionlint", str(workflow)], cwd=ROOT, check=True, timeout=10
            )
            commands = []
            for index, line in enumerate(lines):
                if line == "        run: |":
                    block = []
                    for nested in lines[index + 1 :]:
                        if not nested.startswith("          "):
                            break
                        block.append(nested[10:])
                    commands.append("\n".join(block) + "\n")
            self.assertEqual(len(commands), 3)
            python_blocks = 0
            for index, command in enumerate(commands):
                script = root / f"phase-{index}.sh"
                script.write_text(command)
                subprocess.run(["bash", "-n", str(script)], check=True, timeout=5)
                for block in re.findall(r"<<'PYCODE'\n(.*?)\nPYCODE\n", command, re.S):
                    ast.parse(block)
                    python_blocks += 1
            self.assertEqual(python_blocks, 2)

    def test_proposed_routing_exception_is_exact_and_unapplied(self):
        proposal = ROOT / "docs/knowledge/operations/claude-agent-sdk-netns-ci.patch"
        with tempfile.TemporaryDirectory(
            prefix="sdk-netns-routing-", dir=os.environ["TMPDIR"]
        ) as directory:
            root = Path(directory)
            (root / ".github/workflows").mkdir(parents=True)
            guard = root / "scripts/tests/oxide-selector-routing.py"
            guard.parent.mkdir(parents=True)
            guard.write_bytes(
                subprocess.check_output(
                    [
                        "git",
                        "show",
                        "36fc153aca5c89028681601b04bdf9f62bbcfb7b:scripts/tests/oxide-selector-routing.py",
                    ],
                    cwd=ROOT,
                    timeout=5,
                )
            )
            subprocess.run(
                ["patch", "--batch", "--forward", "-p1", "-i", str(proposal)],
                cwd=root,
                check=True,
                capture_output=True,
                timeout=5,
            )
            spec = importlib.util.spec_from_file_location("proposed_routing", guard)
            module = importlib.util.module_from_spec(spec)
            spec.loader.exec_module(module)
            module.check_workflows()
            workflow = root / ".github/workflows/sdk234-netns-proof.yml"
            workflow.write_text(
                workflow.read_text().replace("contents: read", "contents: write")
            )
            with self.assertRaises(module.RoutingDefect):
                module.check_workflows()
            workflow.unlink()
            (workflow.parent / "unrelated.yml").write_text(
                "name: forbidden non-Cargo workflow\n"
            )
            with self.assertRaises(module.RoutingDefect):
                module.check_workflows()

    def test_custody_is_live_exact_and_closed_after_real_orchestration(self):
        result = self.run_protocol()
        admitted = result["protocol"]["custody"]
        self.assertEqual(admitted["decision"], "89db0f25-dc94-4feb-b447-8b9fabe52561")
        self.assertEqual(admitted["inputs"][str(self.node)], self.node_sha)
        self.assertEqual(admitted["launches"], 2)
        self.assertLess(max(result["protocol"]["stdioDescriptors"]), 64)
        self.assertFalse(admitted["verifiedExecutionPrimitive"])
        self.assertFalse(admitted["hostileSameUserReplacementCovered"])

    def test_custody_revocation_before_each_launch_stops_and_joins_owned_peers(self):
        for boundary in (0, 1):

            class Revoke(CORE.Model):
                def __init__(self):
                    super().__init__()
                    self.pids = []

                def confirm_child(self, child):
                    self.pids.append(child.pid)
                    super().confirm_child(child)

                def custody_event(self, number, lease, command):
                    if number == boundary:
                        lease.revoke()

            ops = Revoke()
            result = self.run_protocol(ops)
            self.assertIn("custody_absent_revoked", result["failure"])
            self.assertEqual(len(ops.pids), boundary)
            for pid in ops.pids:
                with self.assertRaises(ProcessLookupError):
                    os.kill(pid, 0)

    def test_custody_command_substitution_before_payload(self):
        class Substitute(CORE.Model):
            def custody_event(self, number, lease, command):
                command.append("--require=unadmitted-loader")

        result = self.run_protocol(Substitute())
        self.assertIn("custody_launch_mismatch", result["failure"])

    def test_host_owned_file_and_library_custody_refuses_drift_and_closes_descriptors(
        self,
    ):
        runtime = CUSTODY.runtime_inventory(self.node, self.node_sha)
        for mode in ("replace", "bytes", "remove", "revoke"):
            with tempfile.TemporaryDirectory(
                prefix="sdk-custody-", dir=os.environ["TMPDIR"]
            ) as directory:
                target = Path(directory).resolve(strict=True) / "owned-library"
                target.write_bytes(b"owned library bytes")
                lease = CUSTODY.Lease(
                    runtime, self.node, self.node_sha, {"fakeOnly": True}
                )
                try:
                    lease.bind(target, CORE.sha(target.read_bytes()))
                    handles = lease.descriptors.copy()
                    if mode == "replace":
                        target.unlink()
                        target.write_bytes(b"owned library bytes")
                    elif mode == "bytes":
                        target.write_bytes(b"changed library")
                    elif mode == "remove":
                        target.unlink()
                    else:
                        lease.revoke()
                    with self.assertRaises(OSError):
                        lease.check()
                finally:
                    lease.close()
                for fd in handles:
                    with self.assertRaises(OSError):
                        os.fstat(fd)

    def test_missing_or_wrong_runtime_custody_refuses(self):
        runtime = CUSTODY.runtime_inventory(self.node, self.node_sha)
        for change in (
            {"decision": "caller-success"},
            {"nodeSha256": "0" * 64},
            {"startup": ["--require=foreign"]},
        ):
            with self.assertRaises(OSError):
                CUSTODY.Lease({**runtime, **change}, self.node, self.node_sha, {})
        files = CORE.snapshots(ROOT, self.record_sha)
        with tempfile.TemporaryDirectory(
            prefix="sdk-no-custody-", dir=os.environ["TMPDIR"]
        ) as directory:
            with self.assertRaisesRegex(CORE.Refused, "host_custody_missing"):
                CORE.protocol(
                    self.node, self.node_sha, 1, files, CORE.Model(), directory
                )

    def test_monitor_death_recheck_and_unknown_cleanup_are_failures(self):
        with mock.patch.object(
            KERNEL.LIBC, "prctl", return_value=0, create=True
        ) as arm:
            with mock.patch.object(KERNEL.os, "getppid", return_value=1):
                with self.assertRaisesRegex(
                    OSError, "monitor_died_after_credential_drop"
                ):
                    KERNEL.arm_parent(123)
            self.assertEqual(arm.call_args.args[0], 1)

        class Unknown(CORE.Model):
            def cleanup(self):
                return {"observed": False, "kernelEnforced": False}

        result = CORE.execute(
            Unknown(), lambda ops: {"failure": None, "exits": [{}, {}]}
        )
        self.assertEqual(result["failure"], "cleanup_join_unknown")
        self.assertFalse(result["kernelEnforced"])

    def test_linux_inspect_rejects_unpinned_node_before_running_it(self):
        with mock.patch.dict(os.environ, {"NETNS_RECORD": self.record_sha}):
            with mock.patch.object(CORE.sys, "platform", "linux"):
                with mock.patch.object(CORE.subprocess, "run") as run:
                    with self.assertRaisesRegex(
                        CORE.Refused, "preflight_Node22_identity_before_execution"
                    ):
                        CORE.inspect_host(self.node)
                    run.assert_not_called()

    def test_source_record_has_exact_acyclic_input_set(self):
        expected = "".join(
            f"{CORE.sha((ROOT / name).read_bytes())}  {name}\n" for name in CORE.INPUTS
        )
        self.assertEqual((ROOT / CORE.RECORD).read_text(), expected)
        self.assertNotIn(str(CORE.RECORD), CORE.INPUTS)
        self.assertEqual(
            CORE.sha(
                (
                    ROOT
                    / "crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-agent-sdk-native-proof/confined-execution.sha256"
                ).read_bytes()
            ),
            CORE.OLD_RECORD,
        )


if __name__ == "__main__":
    unittest.main()
