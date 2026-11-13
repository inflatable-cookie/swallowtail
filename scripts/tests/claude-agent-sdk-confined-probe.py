#!/usr/bin/env python3
"""Owned fake tests for the private SDK confinement boundary."""

import importlib.util
import os
from pathlib import Path
import shutil
import json
import socket
import subprocess
import sys
import tempfile
import threading
import unittest
from http.server import BaseHTTPRequestHandler, HTTPServer
from unittest import mock

sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location(
    "confined_probe", ROOT / "scripts/claude-agent-sdk-confined-probe.py"
)
PROBE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(PROBE)


class ConfinedProbeTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.node = Path(shutil.which("node")).resolve()
        cls.node_digest = PROBE.digest(PROBE.read_regular(cls.node))
        cls.execution_digest = PROBE.digest((ROOT / PROBE.RECORD).read_bytes())

    def run_probe(self, **kwargs):
        result = PROBE.run_fake(
            self.node, self.node_digest, self.execution_digest, **kwargs
        )
        result.pop("runtimeFiles", None)
        self.assertEqual(len(result["exits"]), 2, result)
        for row in result["exits"]:
            with self.assertRaises(ProcessLookupError):
                os.kill(row["pid"], 0)
        return result

    def test_real_boundary_smoke(self):
        result = self.run_probe()
        self.assertIsNone(result["failure"], result)
        self.assertEqual([row["returncode"] for row in result["exits"]], [0, 0], result)
        self.assertIn('"event":"initialized"', result["driver"])

    def test_kernel_filesystem_constraints(self):
        result = self.run_probe(attack="filesystem")
        rows = [json.loads(line) for line in result["native"].splitlines()]
        self.assertEqual(
            {row["name"] for row in rows},
            {
                "read_outside",
                "stat_outside",
                "write_outside",
                "symlink_escape",
                "source_write",
                "extra_file",
                "new_directory",
                "hardlink",
            },
            result,
        )
        self.assertTrue(all(row["denied"] for row in rows), result)
        self.assertTrue(result["outsideUnchanged"])

    def test_kernel_fork_denial(self):
        result = self.run_probe(attack="fork")
        row = json.loads(result["native"].strip())
        self.assertEqual(row["code"], "EPERM", result)
        self.assertIsNone(row["status"], result)

    def test_selected_permission_deny_and_managed_deny_remain_distinct(self):
        callback = self.run_probe(scenario="permission")
        managed = self.run_probe(scenario="managed-deny")
        self.assertIn('"event":"permission"', callback["driver"], callback)
        self.assertNotIn('"event":"permission"', managed["driver"], managed)
        self.assertIn('"subtype":"can_use_tool"', callback["native"], callback)
        self.assertNotIn('"subtype":"can_use_tool"', managed["native"], managed)
        for result in (callback, managed):
            self.assertIn('"type":"result"', result["driver"], result)
            self.assertEqual(
                [row["returncode"] for row in result["exits"]], [0, 0], result
            )

    def test_shipped_sidecar_runs_under_real_boundary(self):
        result = self.run_probe(driver_mode="sidecar")
        self.assertIn(
            '"id":"open","command":"open","success":true', result["driver"], result
        )
        self.assertIn('"event":"turn_ended"', result["driver"], result)
        self.assertIn(
            '"id":"close","command":"close","success":true', result["driver"], result
        )
        self.assertEqual([row["returncode"] for row in result["exits"]], [0, 0], result)

    def test_error_usage_interrupt_and_native_failure(self):
        error = self.run_probe(scenario="error-result")
        self.assertIn('"subtype":"error_during_execution"', error["driver"], error)
        self.assertIn('"usage":', error["driver"], error)
        interrupted = self.run_probe(scenario="abortable-turn")
        self.assertIn('"type":"result"', interrupted["driver"], interrupted)
        crash = self.run_probe(scenario="crash-after-turn")
        self.assertEqual(crash["exits"][0]["returncode"], 3, crash)

    def test_iterator_completion_does_not_hide_a_hanging_native(self):
        result = self.run_probe(
            scenario="never-exit",
            sdk_config={"endIterationAfterMessages": 0},
            bound=0.5,
        )
        self.assertIn('"event":"iterator_done"', result["driver"], result)
        self.assertEqual(result["failure"], "deadline_or_hanging_native", result)

    def test_missing_sdk_join_uses_only_supervisor_observation(self):
        result = self.run_probe(sdk_config={"withoutSdkJoin": True})
        self.assertIn('"event":"sdk_join_unavailable"', result["driver"], result)
        self.assertIsNone(result["failure"], result)
        self.assertEqual([row["returncode"] for row in result["exits"]], [0, 0], result)

    def test_sigterm_cancels_actual_supervisor_at_endpoint_barrier(self):
        ready = threading.Event()

        class Endpoint(BaseHTTPRequestHandler):
            def do_POST(self):
                self.rfile.read(int(self.headers["Content-Length"]))
                self.send_response(200)
                self.end_headers()
                self.wfile.write(b"{}")
                ready.set()

            def log_message(self, *args):
                pass

        with HTTPServer(("127.0.0.1", 0), Endpoint) as server:
            thread = threading.Thread(target=server.serve_forever)
            thread.start()
            command = [
                sys.executable,
                "-B",
                str(ROOT / PROBE.SOURCES[0]),
                "--node",
                str(self.node),
                "--node-sha256",
                self.node_digest,
                "--execution-sha256",
                self.execution_digest,
                "--endpoint",
                f"127.0.0.1:{server.server_port}",
                "--scenario",
                "never-exit",
            ]
            child = subprocess.Popen(
                command,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                env={
                    name: os.environ[name] for name in ("PATH", "TMPDIR", "TMP", "TEMP")
                },
            )
            try:
                reached = ready.wait(timeout=10)
                child.terminate()
                output, errors = child.communicate(timeout=15)
                self.assertTrue(reached, errors.decode())
                result = json.loads(output)
                self.assertEqual(result["failure"], "supervisor_cancelled", result)
                self.assertEqual(len(result["exits"]), 2, result)
                for row in result["exits"]:
                    with self.assertRaises(ProcessLookupError):
                        os.kill(row["pid"], 0)
            finally:
                if child.poll() is None:
                    child.kill()
                child.communicate(timeout=15)
                server.shutdown()
                thread.join(timeout=2)

    def test_missing_capability_and_wrong_node_refuse(self):
        with mock.patch.object(PROBE.platform, "system", return_value="Linux"):
            with self.assertRaisesRegex(PROBE.Refused, "boundary_unavailable"):
                self.run_probe()
        with self.assertRaisesRegex(PROBE.Refused, "node_substitution"):
            PROBE.run_fake(self.node, "0" * 64, self.execution_digest)
        with mock.patch.object(PROBE.Path, "is_file", return_value=False):
            with self.assertRaisesRegex(PROBE.Refused, "capability_missing_or_revoked"):
                self.run_probe()
        with self.assertRaisesRegex(PROBE.Refused, "unadmitted_tuple"):
            self.run_probe(sdk_version="0.3.323")

    def test_original_cli_refuses_before_node_or_artifact_selection(self):
        result = subprocess.run(
            [sys.executable, "-B", str(ROOT / PROBE.SOURCES[0]), "--mode", "original"],
            capture_output=True,
            text=True,
            timeout=5,
            env={"PATH": "/usr/bin:/bin"},
        )
        self.assertEqual(result.returncode, 2, result)
        self.assertEqual(
            json.loads(result.stdout)["refused"],
            "original_mode_unavailable: custody, Linux x64, authority and Nucleus proof gate",
        )

    def test_kernel_network_denial_has_independent_listener(self):
        with socket.socket() as forbidden:
            forbidden.bind(("127.0.0.1", 0))
            forbidden.listen()
            forbidden.settimeout(0.1)
            result = self.run_probe(
                attack="network", forbidden_port=forbidden.getsockname()[1]
            )
            self.assertIn('"code":"EPERM"', result["native"], result)
            with self.assertRaises(socket.timeout):
                forbidden.accept()

    def test_only_declared_endpoint_is_reached(self):
        requests = []

        class Endpoint(BaseHTTPRequestHandler):
            def do_POST(self):
                body = self.rfile.read(int(self.headers["Content-Length"]))
                requests.append((self.path, json.loads(body)))
                self.send_response(200)
                self.end_headers()
                self.wfile.write(b"{}")

            def log_message(self, *args):
                pass

        with HTTPServer(("127.0.0.1", 0), Endpoint) as server:
            thread = threading.Thread(target=server.serve_forever)
            thread.start()
            try:
                result = self.run_probe(endpoint=f"127.0.0.1:{server.server_port}")
            finally:
                server.shutdown()
                thread.join(timeout=2)
            self.assertFalse(thread.is_alive())
        self.assertIsNone(result["failure"], result)
        self.assertEqual(len(requests), 1, result)
        self.assertEqual(requests[0][0], "/v1/messages")

    def test_kernel_storage_bound(self):
        result = self.run_probe(attack="storage")
        self.assertLessEqual(result["storageBytes"], PROBE.MAX_FILE, result)
        self.assertTrue(
            result["exits"][0]["returncode"] != 0
            or '"denied":true' in result["native"],
            result,
        )

    def test_live_output_bound(self):
        result = self.run_probe(attack="output", output_limit=1024)
        self.assertEqual(result["failure"], "live_output_limit", result)

    def test_cancel_at_controlled_barrier_and_full_tree_join(self):
        result = self.run_probe(attack="hang", cancel_on_barrier=True)
        self.assertEqual(result["failure"], "cancelled_at_owned_barrier", result)

    def test_deadline_joins_both_owned_processes(self):
        result = self.run_probe(attack="hang", bound=0.25)
        self.assertEqual(result["failure"], "deadline_or_hanging_native", result)

    def test_source_substitution_and_symlinks_refuse_before_launch(self):
        with tempfile.TemporaryDirectory(
            prefix="sdk-confined-substitute-", dir=os.environ["TMPDIR"]
        ) as temporary:
            root = Path(temporary)
            for name in (*PROBE.SOURCES, PROBE.RECORD):
                (root / name).parent.mkdir(parents=True, exist_ok=True)
                shutil.copyfile(ROOT / name, root / name)
            for name in (
                PROBE.SUPPORT / "confined-runner.mjs",
                PROBE.FIXTURE / "probe-admission.json",
                PROBE.FIXTURE / "probe-options.json",
                PROBE.SUPPORT / "fake-sdk-native-wire.mjs",
            ):
                original = (root / name).read_bytes()
                (root / name).write_bytes(original + b"\n")
                with self.assertRaisesRegex(PROBE.Refused, "source_substitution"):
                    PROBE.verified_sources(root, self.execution_digest)
                (root / name).write_bytes(original)
            candidate = root / PROBE.SUPPORT / "confined-runner.mjs"
            candidate.unlink()
            os.symlink(root / PROBE.SUPPORT / "probe-runner.mjs", candidate)
            with self.assertRaisesRegex(PROBE.Refused, "symlink_refused"):
                PROBE.verified_sources(root, self.execution_digest)


if __name__ == "__main__":
    unittest.main()
