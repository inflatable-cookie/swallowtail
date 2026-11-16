"""Owned fake child-spawn tests; no privileged/kernel/canonical activation."""

import hashlib
import importlib.util
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import unittest
from unittest import mock
import threading

ROOT = Path(__file__).resolve().parents[2]
NAME = "scripts/claude-agent-sdk-spawn-proposal.py"
spec = importlib.util.spec_from_file_location(NAME, ROOT / NAME)
CORE = importlib.util.module_from_spec(spec)
spec.loader.exec_module(CORE)

MODEL_SPEC = importlib.util.spec_from_file_location(
    "runtime_model", ROOT / "scripts/tests/sdk-runtime-model.py"
)
MODEL = importlib.util.module_from_spec(MODEL_SPEC)
MODEL_SPEC.loader.exec_module(MODEL)


class SpawnTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.node = Path(shutil.which("node")).resolve(strict=True)
        cls.node_sha = hashlib.sha256(cls.node.read_bytes()).hexdigest()
        cls.record = hashlib.sha256((ROOT / CORE.RECORD).read_bytes()).hexdigest()
        cls.evidence = []
        cls.diagnostics = []

    @classmethod
    def tearDownClass(cls):
        evidence_root = Path(
            tempfile.mkdtemp(prefix="spawn-evidence-", dir=os.environ["TMPDIR"])
        )
        (evidence_root / "spawn-software-diagnostics.json").write_text(
            json.dumps(
                {
                    "sourceRecord": cls.record,
                    "kernelEnforced": False,
                    "originalActivation": False,
                    "cases": cls.diagnostics,
                },
                indent=2,
            )
            + "\n"
        )
        (evidence_root / "spawn-software-observations.json").write_text(
            json.dumps(
                {
                    "sourceRecord": cls.record,
                    "kernelEnforced": False,
                    "runs": cls.evidence,
                },
                indent=2,
            )
            + "\n"
        )

    def setUp(self):
        original = CORE.module

        def load(name, data):
            selected = original(name, data)
            if name == "scripts/claude-agent-sdk-probe-custody.py":
                selected.runtime_inventory = lambda node, expected: MODEL.observation(
                    selected, node, expected
                )
            return selected

        patch = mock.patch.object(CORE, "module", side_effect=load)
        patch.start()
        self.addCleanup(patch.stop)

    def test_exit_between_identity_and_signal_still_reaches_driver_cleanup(self):
        driver = mock.Mock()
        driver.poll.return_value = None
        owned = {"pid": 123, "start": 456, "group": 789}
        with mock.patch.object(CORE.os, "kill", side_effect=ProcessLookupError):
            CORE.stop_owned(driver, [owned], observer=lambda pid: owned)
        driver.kill.assert_called_once()

    def run_peer(self, **kwargs):
        result = CORE.rehearsal(self.record, self.node, self.node_sha, **kwargs)
        self.diagnostics.append(
            {
                "fault": kwargs.get("fault"),
                "failure": result.get("failure"),
                "timings": result.get("timings", []),
                "driverWait": result.get("driverWait"),
                "nativeWaits": result.get("nativeWaits", []),
                "guardRefusals": [
                    row
                    for row in result.get("peerEvents", [])
                    if row.get("probeEvent") == "guard_refused"
                ],
                "registeredCount": len(result.get("registered", [])),
                "requestCount": len(result.get("requests", [])),
                "bypassEffectObserved": result.get("bypassEffectObserved", False),
            }
        )
        self.assertFalse(result["kernelEnforced"])
        self.assertFalse(result["originalActivation"])
        self.assertFalse(result["fullKernelTreeSettlement"])
        if result.get("custody"):
            self.assertFalse(result["custody"]["runtimeCustodyComplete"])
        if result["failure"] is None:
            self.evidence.append(result)
        return result

    def test_shipped_sidecar_real_spawn_and_fixture_states(self):
        for state in ("absent", "disabled", "managed-deny"):
            with self.subTest(state=state):
                result = self.run_peer(state=state)
                self.assertIsNone(
                    result["failure"],
                    {
                        k: result.get(k)
                        for k in ("failure", "peerEvents", "stdout", "stderr", "events")
                    },
                )
                self.assertEqual(result["driverWait"], 0)
                self.assertEqual(
                    [row["seq"] for row in result["events"]],
                    list(range(len(result["events"]))),
                )
                self.assertTrue(
                    {"native_started", "native_entry", "native_wait", "driver_wait"}
                    <= {row["kind"] for row in result["events"]}
                )
                self.assertEqual(len(result["registered"]), 1)
                self.assertEqual(result["nativeWaits"][0]["code"], 0)
                self.assertEqual(
                    result["registered"][0]["parent"], result["nativeWaits"][0]["actor"]
                )
                self.assertEqual(len(result["requests"]), 1)
                events = CORE.parse_events(result["stdout"])
                callbacks = [x for x in events if x.get("type") == "callback"]
                self.assertEqual(len(callbacks), 0 if state == "managed-deny" else 1)
                entry = next(
                    x
                    for x in result["peerEvents"]
                    if x.get("probeEvent") == "native_entry"
                )
                self.assertEqual(entry["managed"], state == "managed-deny")
                close = next(
                    x
                    for x in events
                    if x.get("id") == "close" and x.get("type") == "response"
                )
                self.assertTrue(close["data"]["nativeExitObserved"])

    def test_prepared_probe_real_child(self):
        result = self.run_peer(mode="probe")
        self.assertIsNone(
            result["failure"],
            {
                k: result.get(k)
                for k in ("failure", "peerEvents", "stdout", "stderr", "events")
            },
        )
        self.assertEqual(len(result["nativeWaits"]), 1)
        self.assertIn('"initialized"', result["stdout"])
        self.assertIn('"permission"', result["stdout"])

    def test_source_bound_runtime_factory_reaches_real_prepared_and_sidecar_peers(self):
        captures = []

        def factory(custody, node, expected):
            observed = MODEL.observation(custody, node, expected)
            captures.append(observed.capture)
            return observed

        for mode in ("probe", "sidecar"):
            result = self.run_peer(mode=mode, runtime_factory=factory)
            self.assertIsNone(result["failure"])
            self.assertEqual(result["driverWait"], 0)
            self.assertEqual(len(result["nativeWaits"]), 1)
            self.assertFalse(result["custody"]["runtimeCustodyComplete"])
        self.assertEqual(len(captures), 2)
        self.assertTrue(all(capture.closed for capture in captures))

    def test_source_shaped_sdk_startup_keeps_permission_and_managed_denial(self):
        for mode, state in (
            ("probe", "absent"),
            ("sidecar", "absent"),
            ("sidecar", "managed-deny"),
        ):
            result = self.run_peer(mode=mode, state=state, fault="sdk-startup-probes")
            self.assertIsNone(result["failure"], result.get("stderr"))
            self.assertEqual(result["driverWait"], 0)
            self.assertEqual(result["nativeWaits"][0]["code"], 0)
            if mode == "probe":
                self.assertIn('"permission"', result["stdout"])
            else:
                callbacks = [
                    row
                    for row in CORE.parse_events(result["stdout"])
                    if row.get("type") == "callback"
                ]
                self.assertEqual(len(callbacks), 0 if state == "managed-deny" else 1)

    def test_second_start_and_wrong_parent_refuse(self):
        for fault in ("double-start", "wrong-parent"):
            with self.subTest(fault=fault):
                result = self.run_peer(fault=fault)
                self.assertIsNotNone(result["failure"])

    def test_failed_cleanup_refuses(self):
        result = self.run_peer(fault="cleanup")
        self.assertIsNotNone(result["failure"])

    def test_missing_native_parent_wait_retains_custody_until_observed(self):
        parse = CORE.parse_events
        owner = None

        def withheld(data):
            return [
                row for row in parse(data) if row.get("probeEvent") != "native_wait"
            ]

        try:
            with mock.patch.object(CORE, "parse_events", side_effect=withheld):
                with self.assertRaisesRegex(
                    CORE.Refused, "rehearsal_cleanup_unknown"
                ) as caught:
                    self.run_peer()
                owner = caught.exception.rehearsal
                self.assert_retained(owner)
                self.assertEqual(len(owner.registered), 1)
                self.assertEqual(owner.driver.returncode, 0)
                self.assertIn(
                    "native_parent_wait_missing", str(caught.exception.__cause__)
                )
                # The parent really waited. Only its observation was withheld;
                # exit/disappearance/driver wait must not authorize release.
                waits = [
                    row
                    for row in parse(owner.output["stderr"].decode())
                    if row.get("probeEvent") == "native_wait"
                ]
                self.assertEqual(len(waits), 1)
                self.assertEqual(waits[0]["pid"], owner.registered[0]["pid"])
                self.assertEqual(waits[0]["actor"], owner.registered[0]["parent"])
                with self.assertRaises(CORE.Refused):
                    owner.settle()
                self.assert_retained(owner)
        finally:
            if owner is not None:
                # Parser restored: retry reads the retained owned stream's real
                # parent exit observation, never a supplied receipt or PID probe.
                owner.settle()
        self.assertTrue(owner.released)
        self.assertTrue(owner.lease.closed)
        self.assertFalse(owner.root.exists())

    def test_unbound_or_nonterminal_native_wait_cannot_release_custody(self):
        parse = CORE.parse_events
        for changed in (
            {"pid": -1},
            {"actor": -1},
            {"source": "pidfd HUP"},
            {"code": None, "signal": None},
        ):
            with self.subTest(changed=changed):
                owner = None

                def unbound(data):
                    return [
                        {**row, **changed}
                        if row.get("probeEvent") == "native_wait"
                        else row
                        for row in parse(data)
                    ]

                try:
                    with mock.patch.object(CORE, "parse_events", side_effect=unbound):
                        with self.assertRaises(CORE.Refused) as caught:
                            self.run_peer()
                        owner = caught.exception.rehearsal
                        self.assert_retained(owner)
                finally:
                    if owner is not None:
                        owner.settle()
                self.assertTrue(owner.released)

    def test_revocation_before_driver_and_native_entry(self):
        for phase in ("driver-start", "native-entry"):
            with self.subTest(phase=phase):

                def revoke(actual, lease, root):
                    if actual == phase:
                        lease.revoke()

                result = self.run_peer(custody_hook=revoke)
                self.assertIsNotNone(result["failure"])
                self.assertFalse(result["registered"])

    def test_changed_staged_native_refuses_before_payload(self):
        def change(phase, lease, root):
            if phase == "native-entry":
                (root / "native.mjs").write_text('throw Error("changed")')

        result = self.run_peer(custody_hook=change)
        self.assertIsNotNone(result["failure"])
        self.assertFalse(result["registered"])

    def test_cancel_before_start_and_entry(self):
        for phase in ("driver-start", "native-entry"):
            result = self.run_peer(cancel=lambda actual: actual == phase)
            self.assertIsNotNone(result["failure"])
            self.assertFalse(result["registered"])

    def test_live_limits_and_hanging_native_refuse(self):
        for kwargs in (
            {"output_limit": 1},
            {"storage_limit": 1024, "fault": "storage-excess"},
            {"scenario": "never-exit", "bound": 1},
        ):
            with self.subTest(kwargs=kwargs):
                result = self.run_peer(**kwargs)
                self.assertIsNotNone(result["failure"])

    def assert_retained(self, owner):
        self.assertTrue(owner.retained)
        self.assertFalse(owner.released)
        self.assertTrue(owner.root.is_dir())
        self.assertTrue((owner.root / "native.mjs").is_file())
        self.assertTrue(owner.lease.revoked)
        self.assertFalse(owner.lease.closed)
        self.assertTrue(owner.lease.descriptors)
        for descriptor in owner.lease.descriptors:
            os.fstat(descriptor)
        self.assertFalse(owner.driver.stdout.closed)
        self.assertFalse(owner.driver.stderr.closed)
        if owner.pidfd is not None:
            os.fstat(owner.pidfd)

    def test_cleanup_signal_failure_retains_endpoint_and_custody(self):
        before = set(threading.enumerate())
        with mock.patch.object(
            CORE, "stop_owned", side_effect=PermissionError("owned signal refused")
        ):
            with self.assertRaisesRegex(
                CORE.Refused, "rehearsal_cleanup_unknown"
            ) as caught:
                self.run_peer()
        owner = caught.exception.rehearsal
        try:
            self.assert_retained(owner)
            self.assertTrue(owner.thread.is_alive())
        finally:
            owner.settle()
        self.assertTrue(owner.released)
        self.assertTrue(owner.lease.closed)
        self.assertFalse(owner.root.exists())
        self.assertEqual(set(threading.enumerate()), before)

    def test_unsettled_actual_child_retains_handles_and_fixture_until_wait(self):
        # Lifetime-only first-party sleeper, not SDK/native or topology proof.
        popen = subprocess.Popen
        child, owner, kill, wait = None, None, None, None

        def start(*args, **kwargs):
            nonlocal child, kill, wait
            if not kwargs.get("start_new_session"):
                return popen(*args, **kwargs)
            child = popen(
                [sys.executable, "-I", "-B", "-S", "-c", "import time; time.sleep(30)"],
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                env={"PATH": "/usr/bin:/bin"},
                cwd=os.environ["TMPDIR"],
            )
            kill, wait = child.kill, child.wait
            child.kill = child.terminate = lambda: None
            child.wait = mock.Mock(
                side_effect=subprocess.TimeoutExpired("owned sleeper", 1)
            )
            return child

        try:
            with mock.patch.object(CORE.subprocess, "Popen", side_effect=start):
                with self.assertRaisesRegex(
                    CORE.Refused, "rehearsal_cleanup_unknown"
                ) as caught:
                    self.run_peer(bound=0.1)
            owner = caught.exception.rehearsal
            self.assertIs(owner.driver, child)
            self.assertIsNone(child.poll())
            self.assert_retained(owner)
            self.assertTrue(owner.thread.is_alive())
        finally:
            if child is not None:
                kill()
                wait(timeout=2)
                child.wait, child.kill = wait, kill
            if owner is not None:
                owner.settle()
        self.assertTrue(owner.lease.closed)
        self.assertFalse(owner.root.exists())
        self.assertFalse(owner.thread.is_alive())

    def test_captured_graph_denies_launch_and_dynamic_edges_before_effects(self):
        for fault in (*CORE.GRAPH_ATTACKS, "alternate-flags", "forged-approval"):
            with self.subTest(fault=fault):
                result = self.run_peer(fault=fault)
                self.assertIsNotNone(result["failure"])
                self.assertNotIn("deadline", result["failure"])
                if fault != "forged-approval":
                    refusals = [
                        row
                        for row in result.get("peerEvents", [])
                        if row.get("probeEvent") == "guard_refused"
                    ]
                    self.assertEqual(len(refusals), 1, result)
                    expected = {
                        "direct-exec": "unapproved child edge",
                        "alternate-interpreter": "unapproved child edge",
                        "create-require": "unapproved module edge",
                        "worker": "unapproved module edge",
                        "dynamic-code": "code_generation_refused",
                        "vm": "unapproved module edge",
                        "addon": "unapproved process edge",
                        "process-binding": "unapproved process edge",
                        "data-import": "unapproved module edge",
                        "alternate-flags": "unapproved child edge",
                    }
                    self.assertEqual(refusals[0]["reason"], expected[fault])
                else:
                    self.assertEqual(result["failure"], "driver_registration")
                self.assertFalse(result.get("bypassEffectObserved", False))
                self.assertFalse(result.get("requests", []))
                self.assertFalse(result["registered"])

    def test_driver_actor_group_is_explicit_not_assumed_to_be_driver_pid(self):
        actor = {"pid": 101, "group": 100, "start": "owned"}
        observed = {"pid": 102, "parent": 101, "group": 100, "command": ["owned"]}
        CORE.validate_entry(observed, actor, ["owned"])
        with self.assertRaises(CORE.Refused):
            CORE.validate_entry({**observed, "group": 101}, actor, ["owned"])

    def test_real_nonleader_driver_actor_and_native_parent_group_chain(self):
        result = self.run_peer(composition="acp-actor-shape")
        self.assertIsNone(
            result["failure"],
            {k: result.get(k) for k in ("failure", "peerEvents", "stderr", "events")},
        )
        actors = result["actorGraph"]
        self.assertEqual(set(actors), {"monitor", "group-owner", "driver", "native"})
        self.assertNotEqual(actors["driver"]["pid"], actors["driver"]["group"])
        self.assertEqual(actors["driver"]["group"], actors["group-owner"]["pid"])
        self.assertEqual(actors["native"]["parent"], actors["driver"]["pid"])
        self.assertEqual(actors["native"]["group"], actors["group-owner"]["pid"])
        self.assertEqual(result["nativeWaits"][0]["actor"], actors["driver"]["pid"])
        self.assertEqual(result["actorWaits"][0]["actor"], actors["group-owner"]["pid"])
        self.assertEqual(result["actorWaits"][0]["code"], 0)

    def test_actor_registry_rejects_forged_or_duplicate_roles(self):
        registry = CORE.ActorRegistry("sdk")
        with self.assertRaises(CORE.Refused):
            registry.bind("driver", os.getpid(), ["fake"], "monitor")
        with self.assertRaises(CORE.Refused):
            registry.bind("native", os.getpid(), ["fake"], "monitor")

    def test_source_record_and_symlink_refuse(self):
        with self.assertRaises(CORE.Refused):
            CORE.captured("0" * 64)
        with tempfile.TemporaryDirectory(dir=os.environ["TMPDIR"]) as temporary:
            root = Path(temporary)
            (root / CORE.RECORD).parent.mkdir(parents=True)
            (root / CORE.RECORD).write_bytes((ROOT / CORE.RECORD).read_bytes())
            source = CORE.INPUTS[0]
            (root / source).parent.mkdir(parents=True)
            target = root / "owned-file"
            target.write_bytes((ROOT / source).read_bytes())
            (root / source).symlink_to(target)
            with self.assertRaises(CORE.Refused):
                CORE.captured(self.record, root)

    def test_nonregular_source_refuses_without_reading_fifo(self):
        with tempfile.TemporaryDirectory(dir=os.environ["TMPDIR"]) as temporary:
            root = Path(temporary)
            (root / CORE.RECORD).parent.mkdir(parents=True)
            (root / CORE.RECORD).write_bytes((ROOT / CORE.RECORD).read_bytes())
            source = root / CORE.INPUTS[0]
            source.parent.mkdir(parents=True)
            os.mkfifo(source)
            with self.assertRaisesRegex(CORE.Refused, "source_type"):
                CORE.captured(self.record, root)

    def test_plan_retains_closed_kernel_and_original_admission(self):
        plan = json.loads(
            (
                ROOT / "docs/knowledge/operations/claude-agent-sdk-spawn-plan.json"
            ).read_text()
        )
        self.assertFalse(plan["originalActivation"])
        self.assertFalse(plan["authority"]["kernelSetup"])
        self.assertFalse(plan["authority"]["canonicalSetup"])
        self.assertTrue(plan["authority"]["workflowApplied"])
        self.assertEqual(plan["limits"]["proposedCgroupTasksIncludingThreads"], 32)
        self.assertEqual(
            plan["fixture"]["states"], ["absent", "disabled", "managed-deny"]
        )
        self.assertTrue(plan["retainedTransportEvidence"]["grantConsumed"])


if __name__ == "__main__":
    unittest.main()
