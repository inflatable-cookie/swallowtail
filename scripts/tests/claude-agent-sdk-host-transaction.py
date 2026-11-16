"""Owned fixture mutation/real-child tests. No live root/host/candidate operation."""

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

ROOT = Path(__file__).resolve().parents[2]


def load(name):
    spec = importlib.util.spec_from_file_location(
        name, ROOT / ("scripts/" + name + ".py")
    )
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


A = load("claude-agent-sdk-host-transaction")


class OwnedDirectScope:
    """Software fixture for a no-descendant owned Python command; not kernel proof."""

    def before_entry(self, owner):
        for row in owner.rows:
            assert not os.get_inheritable(row["fd"])

    def attach(self, child):
        self.child = child

    def wait(self, child, timeout):
        assert child is self.child
        return child.wait(timeout=timeout)

    def settled(self, child):
        assert child is self.child and child.returncode is not None


class AdminTests(unittest.TestCase):
    def fixture(self):
        root = Path(
            tempfile.mkdtemp(prefix="admin-owned-", dir=os.environ["TMPDIR"])
        ).resolve()
        (root / "opt").mkdir(mode=0o755)
        for name in A.DIRECTORIES:
            (root / name.lstrip("/")).mkdir(mode=0o777)
        # mkdir respects parent umask; set only owned test paths explicitly.
        for name in A.DIRECTORIES:
            (root / name.lstrip("/")).chmod(0o777)
        binding = {"fixture": str(root), "source": "owned", "boot": "owned-test-only"}
        owner = A.AdministrativeOwner(
            root / "admin.json", binding, root=root, uid=os.getuid()
        )
        return root, owner

    def dispose(self, root, owner):
        owner.restore()
        self.assertEqual(
            json.loads((root / "admin.json").read_text())["phase"], "restored"
        )
        shutil.rmtree(root)

    def test_exact_chain_is_safe_before_candidate_and_reverse_restore(self):
        root, owner = self.fixture()
        events = []
        owner.fault = lambda phase, name: events.append((phase, name))
        owner.prepare()
        self.assertFalse((root / A.CANDIDATE.lstrip("/")).exists())
        for name in A.DIRECTORIES:
            self.assertEqual((root / name.lstrip("/")).stat().st_mode & 0o777, 0o755)
        for row in owner.rows:
            self.assertFalse(os.get_inheritable(row["fd"]))
        owner.restore()
        effects = [name for phase, name in events if phase == "restore_after_effect"]
        self.assertEqual(effects, list(reversed(A.DIRECTORIES)))
        shutil.rmtree(root)

    def test_every_prepare_mutation_boundary_keeps_receipts_and_known_or_unknown_state(
        self,
    ):
        for phase in ("before_intent", "after_intent", "after_effect", "after_receipt"):
            for target in A.DIRECTORIES:
                with self.subTest(phase=phase, target=target):
                    root, owner = self.fixture()

                    def fault(point, name):
                        if point == "prepare_" + phase and name == target:
                            raise OSError("owned injected boundary")

                    owner.fault = fault
                    with self.assertRaises(OSError):
                        owner.prepare()
                    self.assertTrue((root / "admin.json").is_file())
                    owner.fault = lambda phase, name: None
                    if phase == "after_effect":
                        with self.assertRaisesRegex(A.Refused, "ambiguous"):
                            owner.restore()
                        self.assertTrue(owner.rows)
                        # Same live owner may finish its known post-effect receipt;
                        # a new recovery process may never infer it from the path.
                        owner.finish_known_effect()
                    else:
                        owner.cancel_known_no_effect()
                    self.dispose(root, owner)

    def test_every_restore_boundary_retains_owner_until_exact_retry(self):
        for phase in ("before_intent", "after_intent", "after_effect", "after_receipt"):
            for target in A.DIRECTORIES:
                with self.subTest(phase=phase, target=target):
                    root, owner = self.fixture()
                    owner.prepare()

                    def fault(point, name):
                        if point == "restore_" + phase and name == target:
                            raise OSError("owned restore fault")

                    owner.fault = fault
                    with self.assertRaises(OSError):
                        owner.restore()
                    self.assertTrue(owner.rows)
                    owner.fault = lambda phase, name: None
                    if phase == "after_effect":
                        with self.assertRaisesRegex(A.Refused, "ambiguous"):
                            owner.restore()
                        owner.finish_known_effect()
                    else:
                        owner.cancel_known_no_effect()
                    self.dispose(root, owner)

    def test_mode_alias_owner_and_identity_drift_refuse(self):
        for failure in ("mode", "alias", "outer", "replacement"):
            with self.subTest(failure=failure):
                root, owner = self.fixture()
                target = root / A.DIRECTORIES[-1].lstrip("/")
                if failure == "mode":
                    target.chmod(0o775)
                elif failure == "alias":
                    target.rmdir()
                    target.symlink_to(root / "opt", target_is_directory=True)
                elif failure == "outer":
                    (root / "opt").chmod(0o777)
                if failure == "replacement":
                    owner.prepare()
                    target.rename(target.with_name("prior"))
                    target.mkdir(mode=0o755)
                    with self.assertRaisesRegex(A.Refused, "drift"):
                        owner.restore()
                    target.rmdir()
                    target.with_name("prior").rename(target)
                    # Rename changes ctime: retained receipt is not silently adopted.
                    with self.assertRaisesRegex(A.Refused, "drift"):
                        owner.restore()
                    # Retain the owned unknown fixture and journal; no cleanup credit.
                    continue
                with self.assertRaises(OSError):
                    owner.prepare()
                # Restore only known exact captured objects; never repair the refused path.
                if failure == "outer":
                    self.assertEqual((root / "opt").stat().st_mode & 0o777, 0o777)
                self.dispose(root, owner)

    def test_ordinary_guard_remains_strict_before_and_after_administrative_scope(self):
        guard = load("claude-agent-sdk-host-admission")
        root, owner = self.fixture()
        with self.assertRaisesRegex(guard.Refused, "unsafe_ancestor"):
            guard.Ancestry(guard.IMAGE, root=root, uid=os.getuid())
        owner.prepare()
        held = guard.Ancestry(guard.IMAGE, root=root, uid=os.getuid())
        held.close()
        owner.restore()
        with self.assertRaisesRegex(guard.Refused, "unsafe_ancestor"):
            guard.Ancestry(guard.IMAGE, root=root, uid=os.getuid())
        shutil.rmtree(root)

    def test_real_wait_and_noninheritance_gate_restore(self):
        root, owner = self.fixture()
        owner.prepare()
        fds = [row["fd"] for row in owner.rows] + [owner.journal_fd, owner.parent_fd]
        identities = [(os.fstat(fd).st_dev, os.fstat(fd).st_ino) for fd in fds]
        code = (
            "import os\nexpected="
            + repr(identities)
            + "\nfor fd in range(3,64):\n try: st=os.fstat(fd)\n except OSError: continue\n assert (st.st_dev,st.st_ino) not in expected\n"
        )
        child = owner.entry(
            lambda: subprocess.Popen(
                [sys.executable, "-I", "-B", "-S", "-c", code],
                close_fds=True,
                env={"PATH": "/usr/bin:/bin", "TMPDIR": str(root)},
            ),
            OwnedDirectScope(),
        )
        with self.assertRaisesRegex(A.Refused, "unknown_child"):
            owner.restore()
        self.assertEqual(owner.wait(), 0)
        self.dispose(root, owner)

    def test_missing_wait_retains_and_later_actual_wait_releases(self):
        root, owner = self.fixture()
        owner.prepare()
        child = owner.entry(
            lambda: subprocess.Popen(
                [sys.executable, "-I", "-B", "-S", "-c", "pass"],
                close_fds=True,
                env={"PATH": "/usr/bin:/bin", "HOME": str(root), "TMPDIR": str(root)},
            ),
            OwnedDirectScope(),
        )
        with mock.patch.object(
            child, "wait", side_effect=subprocess.TimeoutExpired("owned", 1)
        ):
            with self.assertRaises(subprocess.TimeoutExpired):
                owner.wait()
        with self.assertRaisesRegex(A.Refused, "unknown_child"):
            owner.restore()
        owner.wait()
        self.dispose(root, owner)

    def test_unknown_launch_and_recovery_never_accept_serialized_wait(self):
        root, owner = self.fixture()
        owner.prepare()
        with self.assertRaises(OSError):
            owner.entry(
                lambda: (_ for _ in ()).throw(OSError("unknown Popen return")),
                OwnedDirectScope(),
            )
        with self.assertRaisesRegex(A.Refused, "unknown_child"):
            owner.restore()
        owner.state["wait"] = {"pid": 1, "code": 0}
        owner.store()
        with self.assertRaisesRegex(A.Refused, "unknown_child"):
            owner.restore()
        with self.assertRaisesRegex(A.Refused, "unknown_intent"):
            A.AdministrativeOwner.recover(
                owner.journal,
                owner.binding,
                inspect_inputs=lambda: None,
                root=root,
                uid=os.getuid(),
            )
        # Deliberately retain fixture, handles and journal: no actual launch/wait authority.

    def test_known_no_entry_recovery_rebinds_and_restores(self):
        root, owner = self.fixture()
        owner.prepare()
        recovered = A.AdministrativeOwner.recover(
            owner.journal,
            owner.binding,
            inspect_inputs=lambda: None,
            root=root,
            uid=os.getuid(),
        )
        recovered.restore()
        # Original owner is now revoked by changed identities; it cannot authorize entry.
        with self.assertRaisesRegex(A.Refused, "drift"):
            owner.check()
        for row in owner.rows:
            os.close(row["fd"])
        os.close(owner.journal_fd)
        os.close(owner.parent_fd)
        shutil.rmtree(root)

    def test_source_drift_and_tool_failure_known_wait_are_not_cleanup_success(self):
        root, owner = self.fixture()
        source = root / "source"
        source.write_bytes(b"owned source")
        owner.prepare()
        digest = hashlib.sha256(source.read_bytes()).hexdigest()

        def check():
            if hashlib.sha256(source.read_bytes()).hexdigest() != digest:
                raise A.Refused("source_drift")

        owner.check_inputs = check
        code, data = owner.tool(
            [sys.executable, "-I", "-B", "-S", "-c", "raise SystemExit(7)"],
            environment={"PATH": "/usr/bin:/bin"},
        )
        self.assertEqual(code, 7)
        self.assertTrue(owner.state["toolWaits"])
        source.write_bytes(b"drift")
        with self.assertRaisesRegex(A.Refused, "source_drift"):
            owner.restore()
        source.write_bytes(b"owned source")
        self.dispose(root, owner)


if __name__ == "__main__":
    unittest.main()
