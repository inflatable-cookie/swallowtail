"""Strict pre-entry tests with owned observer sentinels; no candidate activation."""

import hashlib
import importlib.util
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import time
import unittest
from unittest import mock

ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location(
    "strict_custody", ROOT / "scripts/claude-agent-sdk-probe-custody.py"
)
C = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(C)


def sha(file):
    return hashlib.sha256(Path(file).read_bytes()).hexdigest()


class OwnedForkObserver:
    """Actual owned child/wait, with first-party observer code and no exec.

    This tests entry ordering/cleanup without using an unadmitted interpreter.
    """

    def __init__(self, file):
        out_r, out_w = os.pipe()
        err_r, err_w = os.pipe()
        self.returncode = None
        sys.stdout.flush()
        sys.stderr.flush()
        self.pid = os.fork()
        if self.pid == 0:
            try:
                os.close(out_r)
                os.close(err_r)
                os.dup2(out_w, 1)
                os.dup2(err_w, 2)
                os.close(out_w)
                os.close(err_w)
                exec(
                    compile(file.read_text(), str(file), "exec"),
                    {"__name__": "__main__"},
                )
                sys.stdout.flush()
                sys.stderr.flush()
                os._exit(0)
            except BaseException:
                os._exit(1)
        os.close(out_w)
        os.close(err_w)
        self.stdout = os.fdopen(out_r, "rb", buffering=0)
        self.stderr = os.fdopen(err_r, "rb", buffering=0)

    def poll(self):
        if self.returncode is None:
            pid, status = os.waitpid(self.pid, os.WNOHANG)
            if pid:
                self.returncode = os.waitstatus_to_exitcode(status)
        return self.returncode

    def wait(self, timeout):
        deadline = time.monotonic() + timeout
        while self.poll() is None:
            if time.monotonic() > deadline:
                raise subprocess.TimeoutExpired("owned-fork", timeout)
            time.sleep(0.005)
        return self.returncode

    def kill(self):
        os.kill(self.pid, 9)


class BootstrapTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix="sdk-bootstrap-")
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name).resolve()
        self.node = self.root / "owned-observer"
        self.marker = self.root / "effect"
        self.library = self.root / "owned-library"
        self.library.write_bytes(b"owned source-model library")
        self.python = self.root / "owned-interpreter-input"
        self.python.write_bytes(b"owned interpreter binding; never exec")
        self.python.chmod(0o755)
        self.supervisor = self.root / "owned-supervisor-input"
        self.supervisor.write_bytes(b"owned supervisor model binding")
        patch = mock.patch.object(C, "python_inputs", return_value={self.supervisor})
        patch.start()
        self.addCleanup(patch.stop)
        patch = mock.patch.object(
            C.subprocess,
            "Popen",
            side_effect=lambda *a, **kw: OwnedForkObserver(self.node),
        )
        patch.start()
        self.addCleanup(patch.stop)
        self.body = ""
        self.write_observer()

    def write_observer(self):
        report = {
            "node": str(self.node),
            "version": "v0.0.0",
            "libraries": getattr(self, "reported_libraries", [str(self.library)]),
        }
        self.node.write_text(
            "#!"
            + str(self.python)
            + "\nimport json,os,time\n"
            + "with open("
            + repr(str(self.marker))
            + ",'w') as marker: marker.write(str(os.getpid()))\n"
            + self.body
            + "print("
            + repr(json.dumps(report))
            + ")\n"
        )
        self.node.chmod(0o755)

    def bootstrap(self, **changes):
        loader = {}
        if sys.platform == "linux" and Path("/etc/ld.so.cache").exists():
            loader["/etc/ld.so.cache"] = sha("/etc/ld.so.cache")
        values = dict(
            version="v0.0.0",
            libraries={str(self.library): sha(self.library)},
            loader_inputs=loader,
            interpreters={str(self.python): sha(self.python)},
            supervisor_inputs={str(p): sha(p) for p in C.python_inputs()},
        )
        values.update(changes)
        return C.Bootstrap(self.node, sha(self.node), **values)

    def observe(self, capture):
        return C.runtime_inventory(self.node, sha(self.node), bootstrap=capture)

    def test_writable_exact_byte_sentinel_never_executes(self):
        self.node.chmod(0o777)
        with self.assertRaisesRegex(OSError, "writable_input"):
            C.runtime_inventory(self.node, sha(self.node))
        self.assertFalse(self.marker.exists())

    def test_absent_bootstrap_closure_never_executes(self):
        with self.assertRaisesRegex(OSError, "manifest_required_before_execution"):
            C.runtime_inventory(self.node, sha(self.node))
        self.assertFalse(self.marker.exists())

    def test_symlink_and_byte_substitution_never_execute(self):
        expected = sha(self.node)
        alias = self.root / "alias"
        alias.symlink_to(self.node)
        for node, digest in [(alias, expected), (self.node, "0" * 64)]:
            with self.subTest(node=node):
                with self.assertRaises(OSError):
                    C.runtime_inventory(node, digest)
        self.assertFalse(self.marker.exists())

    def test_unsafe_ancestor_refuses(self):
        self.root.chmod(0o777)
        try:
            with self.assertRaisesRegex(OSError, "unsafe_ancestor"):
                C.runtime_inventory(self.node, sha(self.node))
        finally:
            self.root.chmod(0o700)
        self.assertFalse(self.marker.exists())

    def test_unsafe_interpreter_and_library_refuse(self):
        for target in (self.library, self.python):
            target.chmod(0o777)
            try:
                with self.assertRaisesRegex(OSError, "writable_input"):
                    self.bootstrap()
            finally:
                target.chmod(0o755 if target == self.python else 0o644)
        self.assertFalse(self.marker.exists())

    def test_missing_interpreter_and_supervisor_bindings_refuse(self):
        for changes in [{"interpreters": {}}, {"supervisor_inputs": {}}]:
            with self.subTest(changes=changes):
                with self.assertRaises(OSError):
                    self.bootstrap(**changes)
        self.assertFalse(self.marker.exists())

    def test_foreign_owner_refuses(self):
        original = C.os.fstat

        def changed(fd):
            info = original(fd)
            if info.st_ino != self.node.stat().st_ino:
                return info

            class Foreign:
                st_uid = 987654

                def __getattr__(self, name):
                    return getattr(info, name)

            return Foreign()

        with mock.patch.object(C.os, "fstat", side_effect=changed):
            with self.assertRaisesRegex(OSError, "unowned"):
                C.runtime_inventory(self.node, sha(self.node))
        self.assertFalse(self.marker.exists())

    def test_file_mode_bytes_path_and_library_drift_before_observation(self):
        for change in ["mode", "bytes", "replace", "library", "ancestor", "revoke"]:
            with self.subTest(change=change):
                self.write_observer()
                self.library.write_bytes(b"owned source-model library")
                capture = self.bootstrap()
                try:
                    if change == "mode":
                        self.node.chmod(0o777)
                    elif change == "bytes":
                        self.node.write_text("changed")
                    elif change == "replace":
                        data = self.node.read_bytes()
                        self.node.unlink()
                        self.node.write_bytes(data)
                        self.node.chmod(0o755)
                    elif change == "library":
                        self.library.write_bytes(b"changed")
                    elif change == "ancestor":
                        self.root.chmod(0o777)
                    else:
                        capture.revoke()
                    with self.assertRaises(OSError):
                        self.observe(capture)
                    self.assertFalse(self.marker.exists())
                finally:
                    self.root.chmod(0o700)
                    capture.close()

    def test_positive_owned_observation_retains_capture_to_launch(self):
        capture = self.bootstrap()
        runtime = self.observe(capture)
        self.assertTrue(self.marker.exists())
        pid = int(self.marker.read_text())
        with self.assertRaises(ProcessLookupError):
            os.kill(pid, 0)
        lease = C.Lease(runtime, self.node, sha(self.node), {"ownedFixtureOnly": True})
        handles = lease.descriptors.copy()
        capture.revoke()
        with self.assertRaisesRegex(OSError, "revoked"):
            lease.check()
        lease.close()
        for fd in handles:
            with self.assertRaises(OSError):
                os.fstat(fd)

    def test_observation_serialization_and_drift_cannot_supply_live_custody(self):
        capture = self.bootstrap()
        runtime = self.observe(capture)
        try:
            with self.assertRaisesRegex(OSError, "live_preentry"):
                C.Lease(dict(runtime), self.node, sha(self.node), {})
            self.node.chmod(0o777)
            with self.assertRaises(OSError):
                C.Lease(runtime, self.node, sha(self.node), {})
        finally:
            capture.close()

    def test_failed_report_has_actual_owned_child_wait_and_closed_custody(self):
        self.body = "print('not-json')\n"
        self.write_observer()
        capture = self.bootstrap()
        with self.assertRaises(ValueError):
            self.observe(capture)
        with self.assertRaises(ProcessLookupError):
            os.kill(int(self.marker.read_text()), 0)
        self.assertTrue(capture.closed)

    def test_live_output_excess_stops_and_joins_observer(self):
        self.body = "os.write(1,b'x'*70000)\ntime.sleep(10)\n"
        self.write_observer()
        capture = self.bootstrap()
        with self.assertRaisesRegex(OSError, "observation_limit"):
            self.observe(capture)
        with self.assertRaises(ProcessLookupError):
            os.kill(int(self.marker.read_text()), 0)
        self.assertTrue(capture.closed)

    def test_changed_version_or_startup_binding_refuses_before_child_entry(self):
        capture = self.bootstrap()
        capture.version = "changed"
        with self.assertRaisesRegex(OSError, "launch_inputs_changed"):
            self.observe(capture)
        self.assertFalse(self.marker.exists())
        capture = self.bootstrap()
        with mock.patch.object(C, "STARTUP", ("--require=unadmitted",)):
            with self.assertRaisesRegex(OSError, "launch_inputs_changed"):
                self.observe(capture)
        self.assertFalse(self.marker.exists())

    def test_unknown_observer_wait_retains_capture_until_actual_join(self):
        self.body = "os.write(1,b'x'*70000)\ntime.sleep(10)\n"
        self.write_observer()
        capture = self.bootstrap()
        children = []

        def child_factory(*args, **kwargs):
            child = OwnedForkObserver(self.node)
            children.append((child, child.wait))
            child.wait = mock.Mock(
                side_effect=subprocess.TimeoutExpired("owned-fork", 2)
            )
            return child

        with mock.patch.object(C.subprocess, "Popen", side_effect=child_factory):
            with self.assertRaisesRegex(OSError, "join_unknown"):
                self.observe(capture)
        self.assertFalse(capture.closed)
        with self.assertRaisesRegex(OSError, "join_unknown"):
            capture.close()
        child, wait = children[0]
        if child.poll() is None:
            child.kill()
        wait(timeout=2)
        capture.active_child = None
        capture.close()
        self.assertTrue(capture.closed)

    def test_unexpected_library_report_refuses_and_joins(self):
        unexpected = self.root / "unexpected"
        unexpected.write_bytes(b"owned unexpected library")
        self.reported_libraries = [str(unexpected)]
        self.write_observer()
        capture = self.bootstrap()
        with self.assertRaisesRegex(OSError, "library_manifest_mismatch"):
            self.observe(capture)
        with self.assertRaises(ProcessLookupError):
            os.kill(int(self.marker.read_text()), 0)
        self.assertTrue(capture.closed)


if __name__ == "__main__":
    unittest.main()
