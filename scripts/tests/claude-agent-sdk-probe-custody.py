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
import types
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
        self.temporary = tempfile.TemporaryDirectory(
            prefix="sdk-bootstrap-", dir=os.environ["TMPDIR"]
        )
        self.retain_fixture = False

        def cleanup():
            if self.retain_fixture:
                self.temporary._finalizer.detach()
            else:
                self.temporary.cleanup()

        self.addCleanup(cleanup)
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

    def test_owner_post_spawn_refusal_uses_actual_handle_and_wait_before_capture_close(
        self,
    ):
        capture = self.bootstrap()
        waits = []
        spec = importlib.util.spec_from_file_location(
            "owned_session_start",
            ROOT / "scripts/claude-agent-sdk-bootstrap-session.py",
        )
        session_module = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(session_module)
        owner = session_module.Session.__new__(session_module.Session)
        owner.host = mock.Mock()
        owner.preparation = types.SimpleNamespace(state={})
        owner.runtime_bootstrap = capture
        owner.store = mock.Mock()
        owner.helper = types.SimpleNamespace(
            persist=mock.Mock(
                side_effect=OSError("owned journal failed after Popen returned")
            )
        )

        def joined(child):
            self.assertIs(child, owner.child)
            self.assertIsNotNone(child.returncode)
            waits.append(child)

        owner.joined = joined
        with self.assertRaisesRegex(OSError, "journal failed"):
            C.runtime_inventory(
                self.node, sha(self.node), bootstrap=capture, observer_owner=owner
            )
        self.assertEqual(waits, [owner.child])
        self.assertTrue(capture.closed)
        self.assertFalse(capture.launch_pending)

    def test_unknown_constructor_return_keeps_capture_and_refuses_close(self):
        capture = self.bootstrap()
        with mock.patch.object(
            C.subprocess,
            "Popen",
            side_effect=OSError("owned constructor return unknown"),
        ):
            with self.assertRaisesRegex(OSError, "launch_unknown") as failure:
                self.observe(capture)
        self.assertIs(failure.exception.capture, capture)
        self.assertTrue(capture.launch_pending)
        self.assertFalse(capture.closed)
        with self.assertRaisesRegex(OSError, "launch_unknown"):
            capture.close()
        self.assertFalse(self.marker.exists())
        self.retain_fixture = (
            True  # No fabricated wait/cleanup credit for this unknown seam.
        )

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


class BatchTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(
            prefix="sdk-custody-batch-", dir=os.environ["TMPDIR"]
        )
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name).resolve()
        self.files = [self.root / name for name in ("held", "second", "third")]
        for index, file in enumerate(self.files):
            file.write_bytes(("owned input " + str(index)).encode())
        self.capture = C.Inputs(reserve_pipes=False)
        self.addCleanup(self.release)

    def release(self):
        self.capture.active_child = None
        self.capture.close()

    def test_partial_batch_cannot_be_checked_or_configured_during_a_read(self):
        real_read = C.os.read
        seen = []

        def attempt_entry(descriptor, count):
            if not seen:
                for action in (
                    self.capture.check,
                    lambda: self.capture.configure([], []),
                    lambda: self.capture.permit(["unadmitted"], {}),
                ):
                    with self.assertRaisesRegex(OSError, "custody_batch_in_progress"):
                        action()
                seen.append(True)
            return real_read(descriptor, count)

        with mock.patch.object(C.os, "read", side_effect=attempt_entry):
            self.capture.bind_many([(file, sha(file)) for file in self.files])
        self.assertTrue(seen)
        self.assertIsNone(self.capture.commands)
        self.assertEqual(self.capture.launched, 0)
        self.capture.check()

    def test_later_capture_detects_drift_of_previously_held_input(self):
        held, second, third = self.files
        self.capture.bind(held, sha(held))
        real_read = C.os.read
        changed = False

        def drift(descriptor, count):
            nonlocal changed
            data = real_read(descriptor, count)
            if not changed and os.fstat(descriptor).st_ino == second.stat().st_ino:
                held.write_bytes(b"changed earlier owned input")
                changed = True
            return data

        with (
            mock.patch.object(C.os, "read", side_effect=drift),
            mock.patch.object(
                self.capture, "check", wraps=self.capture.check
            ) as checks,
        ):
            with self.assertRaisesRegex(OSError, "custody_input_identity_changed"):
                self.capture.bind_many([(second, sha(second)), (third, sha(third))])
        self.assertTrue(changed)
        self.assertEqual(checks.call_count, 2)
        with self.assertRaises(OSError):
            self.capture.permit(["unadmitted"], {})

    def test_revocation_during_last_capture_cannot_admit_partial_batch(self):
        second, third = self.files[1:]
        real_read = C.os.read
        revoked = False

        def revoke(descriptor, count):
            nonlocal revoked
            data = real_read(descriptor, count)
            if not revoked and os.fstat(descriptor).st_ino == third.stat().st_ino:
                self.capture.revoke()
                revoked = True
            return data

        with mock.patch.object(C.os, "read", side_effect=revoke):
            with self.assertRaisesRegex(
                OSError, "custody_absent_revoked_or_wrong_host"
            ):
                self.capture.bind_many([(second, sha(second)), (third, sha(third))])
        self.assertTrue(revoked)
        with self.assertRaisesRegex(OSError, "custody_absent_revoked_or_wrong_host"):
            self.capture.permit(["unadmitted"], {})
        self.assertEqual(self.capture.launched, 0)

    def test_wrong_owner_closed_or_active_observer_refuses_before_any_capture(self):
        for state in ("wrong-owner", "closed", "active-observer", "configured"):
            with self.subTest(state=state):
                capture = C.Inputs(reserve_pipes=False)
                try:
                    if state == "wrong-owner":
                        capture.owner = -1
                    elif state == "closed":
                        capture.close()
                    elif state == "active-observer":
                        capture.active_child = object()
                    else:
                        capture.commands = []
                    with mock.patch.object(C.os, "open") as opened:
                        with self.assertRaises(OSError):
                            capture.bind_many([(self.files[0], sha(self.files[0]))])
                    opened.assert_not_called()
                    self.assertEqual(capture.files, {})
                    self.assertEqual(capture.launched, 0)
                finally:
                    capture.active_child = None
                    capture.close()

    def test_success_holds_exact_bytes_with_two_full_boundary_checks(self):
        pairs = [(file, sha(file)) for file in self.files]
        with mock.patch.object(
            self.capture, "check", wraps=self.capture.check
        ) as checks:
            self.capture.bind_many(pairs)
        self.assertEqual(checks.call_count, 2)
        self.assertEqual(set(self.capture.files), {str(file) for file in self.files})
        for file, expected in pairs:
            descriptor, identity, observed = self.capture.files[str(file)]
            self.assertEqual(os.pread(descriptor, identity[5], 0), file.read_bytes())
            self.assertEqual(observed, expected)
        self.files[0].write_bytes(b"changed after complete acquisition")
        with self.assertRaisesRegex(OSError, "custody_input_identity_changed"):
            self.capture.check()

    def test_deadline_and_batch_count_remain_finite(self):
        self.capture.deadline = -1  # already expired; never reset or extend a timer
        with self.assertRaisesRegex(OSError, "custody_capture_deadline"):
            self.capture.bind_many([(self.files[0], sha(self.files[0]))])
        self.assertEqual(self.capture.files, {})
        capture = C.Inputs(reserve_pipes=False)
        try:
            with mock.patch.object(C.os, "open") as opened:
                with self.assertRaises(OSError):
                    capture.bind_many([(self.files[0], sha(self.files[0]))] * 513)
            opened.assert_not_called()
            self.assertEqual(capture.files, {})
        finally:
            capture.close()

    def test_partial_batch_byte_failure_poison_prevents_later_configuration(self):
        first, second = self.files[:2]
        with self.assertRaisesRegex(OSError, "custody_bytes_changed"):
            self.capture.bind_many([(first, sha(first)), (second, "0" * 64)])
        self.assertTrue(self.capture.revoked)
        self.assertIn(str(first), self.capture.files)
        with mock.patch.object(C, "python_inputs", return_value=set()):
            with self.assertRaisesRegex(
                OSError, "custody_absent_revoked_or_wrong_host"
            ):
                self.capture.configure([["unadmitted"]], [{}])
        with self.assertRaisesRegex(OSError, "custody_absent_revoked_or_wrong_host"):
            self.capture.permit(["unadmitted"], {})
        self.assertEqual(self.capture.launched, 0)


if __name__ == "__main__":
    unittest.main()
