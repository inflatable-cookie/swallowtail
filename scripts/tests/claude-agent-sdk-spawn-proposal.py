"""Owned fake child-spawn tests; no privileged/kernel/canonical activation."""

import hashlib
import importlib.util
import json
import os
from pathlib import Path
import shutil
import tempfile
import unittest
from unittest import mock
import threading

ROOT = Path(__file__).resolve().parents[2]
NAME = "scripts/claude-agent-sdk-spawn-proposal.py"
spec = importlib.util.spec_from_file_location(NAME, ROOT / NAME)
CORE = importlib.util.module_from_spec(spec)
spec.loader.exec_module(CORE)


class SpawnTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.node = Path(shutil.which("node")).resolve(strict=True)
        cls.node_sha = hashlib.sha256(cls.node.read_bytes()).hexdigest()
        cls.record = hashlib.sha256((ROOT / CORE.RECORD).read_bytes()).hexdigest()
        cls.evidence = []

    @classmethod
    def tearDownClass(cls):
        (Path(os.environ["TMPDIR"]) / "spawn-software-observations.json").write_text(
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

    def run_peer(self, **kwargs):
        result = CORE.rehearsal(self.record, self.node, self.node_sha, **kwargs)
        self.assertFalse(result["kernelEnforced"])
        self.assertFalse(result["originalActivation"])
        self.assertFalse(result["fullKernelTreeSettlement"])
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

    def test_second_start_and_wrong_parent_refuse(self):
        for fault in ("double-start", "wrong-parent"):
            with self.subTest(fault=fault):
                result = self.run_peer(fault=fault)
                self.assertIsNotNone(result["failure"])

    def test_missing_wait_and_failed_cleanup_refuse(self):
        for fault in ("missing-wait", "cleanup"):
            with self.subTest(fault=fault):
                result = self.run_peer(fault=fault)
                self.assertIsNotNone(result["failure"])

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
            {"storage_limit": 1},
            {"scenario": "never-exit", "bound": 1},
        ):
            with self.subTest(kwargs=kwargs):
                result = self.run_peer(**kwargs)
                self.assertIsNotNone(result["failure"])

    def test_cleanup_signal_failure_still_releases_endpoint_and_custody(self):
        before = set(threading.enumerate())
        with mock.patch.object(
            CORE, "stop_owned", side_effect=PermissionError("owned signal refused")
        ):
            with self.assertRaises(PermissionError):
                self.run_peer()
        self.assertEqual(set(threading.enumerate()), before)

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
        self.assertFalse(any(plan["authority"].values()))
        self.assertEqual(plan["limits"]["proposedCgroupTasksIncludingThreads"], 32)
        self.assertEqual(
            plan["fixture"]["states"], ["absent", "disabled", "managed-deny"]
        )
        self.assertTrue(plan["retainedTransportEvidence"]["grantConsumed"])


if __name__ == "__main__":
    unittest.main()
