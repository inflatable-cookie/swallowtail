"""Owned Python barrier and source-shaped tracer models; never kernel proof."""

import hashlib
import importlib.util
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
from types import SimpleNamespace
import unittest
from unittest import mock

ROOT = Path(__file__).resolve().parents[2]


def load(name):
    spec = importlib.util.spec_from_file_location(
        name, ROOT / ("scripts/" + name + ".py")
    )
    result = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(result)
    return result


B = load("claude-agent-sdk-host-observer")
E = load("claude-agent-sdk-observer-entry")
C = load("claude-agent-sdk-probe-custody")
T = load("claude-agent-sdk-spawn-trace")


class Capture:
    def __init__(self):
        self.files = {}

    def bind(self, name, digest):
        self.files[str(name)] = digest

    def check(self):
        for name, digest in self.files.items():
            if B.sha(Path(name).read_bytes()) != digest:
                raise OSError("owned_capture_drift")


class ObserverTests(unittest.TestCase):
    def setUp(self):
        self.root = Path(
            tempfile.mkdtemp(prefix="observer-owned-", dir=os.environ["TMPDIR"])
        ).resolve()
        self.saved_tmp = os.environ["TMPDIR"]
        os.environ["TMPDIR"] = str(self.root)
        self.files = {
            name: (ROOT / name).read_bytes() for name in (B.ENTRY, B.POLICY, B.KERNEL)
        }
        (self.root / "cwd").mkdir(mode=0o700)
        self.capture = Capture()
        self.requests = []
        self.context = SimpleNamespace(
            observer_command=C.observer_command,
            loader="/owned/loader",
            request=self.request,
        )
        self.guest = B.GuestObserver(self.context, self.files)
        self.child = None

    def request(self, kind, **values):
        self.requests.append((kind, values))
        return {"policy": {"fixture": "not activated"}}

    def prepare(self):
        command = C.observer_command("/owned/never-executed-node")
        self.kwargs = {
            "env": {
                "PATH": "/usr/bin:/bin",
                "HOME": str(self.root),
                "TMPDIR": str(self.root),
                "LANG": "C",
                "LC_ALL": "C",
            },
            "cwd": self.root / "cwd",
            "stdout": subprocess.PIPE,
            "stderr": subprocess.PIPE,
        }
        return self.guest.prepare(command, self.kwargs, self.capture)

    def tearDown(self):
        if self.child is not None:
            if self.child.poll() is None:
                self.child.kill()
            self.child.wait(timeout=2)
            self.child.stdout.close()
            self.child.stderr.close()
        for key in ("read", "write"):
            fd = getattr(self.guest, key)
            if fd is not None:
                os.close(fd)
                setattr(self.guest, key, None)
        os.environ["TMPDIR"] = self.saved_tmp
        shutil.rmtree(self.root)

    def test_owned_python_waits_behind_barrier_and_eof_refuses_before_policy(self):
        command, kwargs = self.prepare()
        self.child = subprocess.Popen(command, **kwargs)
        # No grant is written. EOF is an explicit refusal, followed by an actual
        # parent wait of this exact owned Python child; no Node is executed.
        os.close(self.guest.read)
        self.guest.read = None
        os.close(self.guest.write)
        self.guest.write = None
        stdout, stderr = self.child.communicate(timeout=2)
        self.assertEqual(self.child.returncode, 78)
        self.assertEqual(stdout, b"")
        self.assertIn(b"observer_not_admitted", stderr)
        self.assertTrue(self.guest.pending)
        self.assertFalse(
            self.guest.settled
        )  # A parent wait alone is not root settlement.

    def test_root_rpc_refusal_keeps_actual_child_and_captured_inputs(self):
        command, kwargs = self.prepare()
        self.child = subprocess.Popen(command, **kwargs)
        self.context.request = mock.Mock(side_effect=OSError("root admission refused"))
        with self.assertRaisesRegex(OSError, "root admission"):
            self.guest.started(self.child)
        self.assertIs(self.guest.child, self.child)
        self.assertTrue(self.guest.pending)
        self.assertIsNotNone(self.guest.write)
        self.capture.check()
        self.child.kill()
        self.child.wait(timeout=2)
        with self.assertRaisesRegex(OSError, "root admission"):
            self.guest.joined(self.child)
        self.assertFalse(self.guest.settled)

    def test_exact_parent_wait_required_and_root_ack_is_last(self):
        self.prepare()
        child = SimpleNamespace(pid=42, returncode=None)
        self.guest.child = child
        with self.assertRaisesRegex(OSError, "actual_owned"):
            self.guest.joined(child)
        child.returncode = -15
        self.guest.joined(child)
        self.assertEqual(
            self.requests[-1], ("observer_joined", {"namespacePid": 42, "code": -15})
        )
        self.assertTrue(self.guest.settled)
        self.assertTrue(self.guest.root.exists())
        self.capture.check()

    def test_changed_command_or_options_refuse_without_spawn_or_intent(self):
        with self.assertRaisesRegex(OSError, "command_tail"):
            self.guest.prepare(["/owned/never", "--version"], {}, self.capture)
        self.prepare()
        with self.assertRaisesRegex(OSError, "already_pending"):
            self.prepare()
        self.assertEqual(len(self.requests), 1)

    def test_changed_config_after_grant_refuses_before_policy_effect(self):
        command, kwargs = self.prepare()
        config = Path(command[5])
        source = config.read_bytes()

        def grant(fd):
            config.write_bytes(source + b" ")
            return {"fixture": True}

        with (
            mock.patch.object(E, "read_gate", side_effect=grant),
            mock.patch.object(E.os, "execve") as execute,
        ):
            with self.assertRaisesRegex(OSError, "config_changed_before"):
                E.run(config, B.sha(source), 91)
            execute.assert_not_called()

    def test_gate_is_bounded_and_only_the_owned_descriptor_is_closed(self):
        read, write = os.pipe()
        os.write(write, b"x" * 4097)
        os.close(write)
        with self.assertRaisesRegex(OSError, "gate_bound"):
            E.read_gate(read)
        with self.assertRaises(OSError):
            os.fstat(read)

    def root_phase(self):
        self.prepare()
        packet = self.requests[-1][1]["packet"]
        self.trace = SimpleNamespace(waits=[], seize=mock.Mock())
        self.info = {
            "pid": 123,
            "parent": 100,
            "namespacePids": [123, 3],
            "command": packet["entry"],
        }
        monitor = SimpleNamespace(
            args=SimpleNamespace(uid=os.getuid(), node="/owned/never-executed-node"),
            files=self.files,
            loader=self.context.loader,
            lease=SimpleNamespace(bind=mock.Mock()),
            trace=self.trace,
            init=100,
            gate=None,
            modules={"trace": T},
            check_custody=mock.Mock(),
            member=mock.Mock(),
            pin=lambda pid: self.info,
            find=lambda pid: self.info,
            policy=lambda info: {"fixture": True},
            drain=mock.Mock(),
        )
        phase = B.RootObserver(monitor, C.observer_command, lambda pid: self.info)
        with mock.patch.object(T, "image_binding", return_value={"fixture": "image"}):
            phase.intent(packet)
        phase.started(3)
        return phase

    def test_root_requires_live_tracer_and_parent_waits_agree(self):
        phase = self.root_phase()
        with self.assertRaisesRegex(OSError, "trace_wait_missing"):
            phase.joined(3, 0)
        row = {"actor": os.getpid(), "pid": 123, "status": 0}
        phase.gate.tasks[123]["terminal"] = row
        with self.assertRaisesRegex(OSError, "live_trace_wait_missing"):
            phase.joined(3, 0)
        self.trace.waits.append(row)
        with self.assertRaisesRegex(OSError, "wait_disagree"):
            phase.joined(3, -15)
        phase.joined(3, 0)
        self.assertTrue(phase.settled)

    def test_missing_start_notification_and_failed_identity_retain_obligation(self):
        phase = self.root_phase()
        phase.actor = None
        with self.assertRaisesRegex(OSError, "wait_identity"):
            phase.joined(3, 0)
        self.assertTrue(phase.attempted)
        self.assertFalse(phase.settled)
        self.info["parent"] = 99
        with self.assertRaisesRegex(OSError, "parent_changed"):
            phase.started(3)
        self.assertIs(phase.actor, self.info)
        self.assertFalse(phase.settled)

    def test_observer_tracer_refuses_even_one_process_child(self):
        phase = self.root_phase()
        phase.gate.tasks[123]["state"] = "driver"
        phase.monitor.trace.event_message = lambda pid: 124
        child = {"tgid": 124, "parent": 123}
        phase.identity_reader = lambda pid: self.info if pid == 123 else child
        phase.gate.snapshot = phase.identity_reader
        with self.assertRaisesRegex(OSError, "extra_descendant"):
            phase.gate.process({"pid": 123, "status": (T.FORK << 16) | (5 << 8) | 0x7F})

    def test_fixture_ack_requires_exact_live_driver_native_parent_waits(self):
        file = self.root / "spawn-config.json"
        file.write_text(json.dumps({"native": {"launchId": "owned-launch"}}))
        rows = [{"actor": os.getpid(), "pid": pid, "status": 0} for pid in (1001, 1002)]
        tasks = {
            pid: {"identity": {"namespacePids": [pid, namespace]}, "terminal": row}
            for pid, namespace, row in zip((1001, 1002), (3, 4), rows)
        }
        gate = SimpleNamespace(
            roles={"driver": 1001, "native": 1002}, tasks=tasks, complete=lambda: True
        )
        monitor = SimpleNamespace(
            packet={"root": str(self.root), "config": str(file)},
            gate=gate,
            trace=SimpleNamespace(waits=[]),
            check_custody=mock.Mock(),
            drain=mock.Mock(),
        )
        proof = {
            "driverIntent": True,
            "driver": {"pid": 3, "code": 0},
            "launch": {
                "state": "settled",
                "active": True,
                "settlement": "exact_parent_handle_wait",
                "parent": {"pid": 3},
                "wait": {
                    "pid": 4,
                    "actor": 3,
                    "launchId": "owned-launch",
                    "source": "ChildProcess.exit",
                    "code": 0,
                    "signal": None,
                },
            },
        }
        with self.assertRaisesRegex(OSError, "live_trace_wait_missing"):
            B.fixture_settlement(monitor, self.root, proof)
        monitor.trace.waits = rows
        proof["launch"]["wait"]["code"] = 78
        with self.assertRaisesRegex(OSError, "parent_wait_disagrees"):
            B.fixture_settlement(monitor, self.root, proof)
        proof["launch"]["wait"]["code"] = 0
        proof["driver"]["pid"] = 9
        with self.assertRaisesRegex(OSError, "driver_parent_wait_disagrees"):
            B.fixture_settlement(monitor, self.root, proof)
        proof["driver"]["pid"] = 3
        ack = B.fixture_settlement(monitor, self.root, proof)
        self.assertEqual((ack["driver"], ack["native"]), (3, 4))
        self.assertTrue(file.exists())  # Settlement never performs unlink itself.

    def test_root_fixture_ack_failure_retains_actual_child_fixture_and_capture(self):
        core = load("claude-agent-sdk-spawn-proposal")
        context = SimpleNamespace(
            observe=mock.Mock(),
            stop=mock.Mock(),
            before_fixture_cleanup=mock.Mock(side_effect=OSError("root wait missing")),
        )
        owner = core.RehearsalResources(
            [],
            context,
            lambda *args, **kwargs: None,
            {"stdout": bytearray(), "stderr": bytearray()},
        )
        child = subprocess.Popen(
            [sys.executable, "-I", "-S", "-B", "-c", "pass"],
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            close_fds=True,
        )
        child.wait(timeout=2)
        owner.driver, owner.driver_intent = child, True
        owner.lease = mock.Mock()
        try:
            with self.assertRaisesRegex(core.Refused, "cleanup_unknown") as caught:
                owner.settle()
            self.assertIn("root wait missing", str(caught.exception.__cause__))
            self.assertEqual(child.returncode, 0)
            self.assertTrue(owner.root.exists())
            self.assertTrue(owner.retained)
            self.assertFalse(child.stdout.closed)
            owner.lease.close.assert_not_called()
            context.before_fixture_cleanup.side_effect = None
            owner.settle()
            self.assertTrue(owner.released)
            self.assertFalse(owner.root.exists())
            self.assertTrue(child.stdout.closed)
            owner.lease.close.assert_called_once()
        finally:
            if not owner.released:
                context.before_fixture_cleanup.side_effect = None
                owner.settle()  # Same exact already-observed child wait, never a restart.

    def test_observer_fixture_release_requires_closed_capture_and_exact_members(self):
        self.prepare()
        self.capture.closed = False
        with self.assertRaisesRegex(OSError, "capture_or_wait_still_live"):
            self.guest.release()
        for name in ("read", "write"):
            os.close(getattr(self.guest, name))
            setattr(self.guest, name, None)
        self.guest.settled = (
            True  # Source model only: no child was launched in this test.
        )
        self.capture.closed = True
        file = self.guest.root / "entry.py"
        source = file.read_bytes()
        file.write_bytes(b"owned drift")
        with self.assertRaisesRegex(OSError, "cleanup_bytes_or_alias"):
            self.guest.release()
        file.write_bytes(source)
        self.guest.release()
        self.assertFalse(self.guest.root.exists())


if __name__ == "__main__":
    unittest.main()
