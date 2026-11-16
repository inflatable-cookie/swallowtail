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
RETAINED = []


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
        for name in A.DIRECTORIES:
            (root / name.lstrip("/")).mkdir(mode=0o777)
        # mkdir respects parent umask; set only owned test paths explicitly.
        for name in A.DIRECTORIES:
            (root / name.lstrip("/")).chmod(0o777)
        binding = {"fixture": str(root), "source": "owned", "boot": "owned-test-only"}
        owner = A.AdministrativeOwner(
            root / "admin.json",
            binding,
            inspect_inputs=lambda: None,
            root=root,
            uid=os.getuid(),
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

    def test_actual_writable_opt_counterexample_is_the_only_added_object(self):
        self.assertEqual(
            A.DIRECTORIES,
            (
                "/opt",
                "/opt/hostedtoolcache",
                "/opt/hostedtoolcache/node",
                "/opt/hostedtoolcache/node/22.23.4",
                "/opt/hostedtoolcache/node/22.23.4/x64",
                "/opt/hostedtoolcache/node/22.23.4/x64/bin",
            ),
        )
        root, owner = self.fixture()
        self.assertEqual((root / "opt").stat().st_mode & 0o7777, 0o777)
        owner.prepare()
        self.assertEqual((root / "opt").stat().st_mode & 0o7777, 0o755)
        self.assertFalse(owner.state["entryAttempted"])
        self.assertFalse((root / A.CANDIDATE.lstrip("/")).exists())
        self.dispose(root, owner)

    def test_opt_rejects_other_modes_alias_type_and_administrative_owner(self):
        for failure in ("mode", "alias", "type", "uid", "gid"):
            with self.subTest(failure=failure):
                root, owner = self.fixture()
                target = root / "opt"
                if failure == "mode":
                    target.chmod(0o775)
                elif failure in ("alias", "type"):
                    target.rename(root / "retained-opt")
                    if failure == "alias":
                        target.symlink_to(
                            root / "retained-opt", target_is_directory=True
                        )
                    else:
                        target.write_bytes(b"owned non-directory")
                original = A.os.fstat

                def observe(fd):
                    info = original(fd)
                    if (
                        failure in ("uid", "gid")
                        and info.st_ino == target.stat().st_ino
                    ):
                        values = list(info)
                        values[4 if failure == "uid" else 5] += 999
                        return os.stat_result(values)
                    return info

                with mock.patch.object(A.os, "fstat", side_effect=observe):
                    with self.assertRaises(OSError):
                        owner.prepare()
                self.assertFalse(owner.state["entryAttempted"])
                if failure in ("uid", "gid"):
                    # An inconsistent captured identity cannot authorize cleanup.
                    RETAINED.append(owner)
                else:
                    self.dispose(root, owner)

    def test_existing_safe_directory_is_never_changed_or_restored(self):
        root, owner = self.fixture()
        safe = root / A.DIRECTORIES[2].lstrip("/")
        safe.chmod(0o755)
        events = []
        owner.fault = lambda phase, name: events.append((phase, name))
        owner.prepare()
        row = next(row for row in owner.rows if row["name"] == A.DIRECTORIES[2])
        self.assertFalse(row["changed"])
        self.assertFalse(any(name == A.DIRECTORIES[2] for phase, name in events))
        owner.restore()
        self.assertEqual(safe.stat().st_mode & 0o777, 0o755)
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
                    root.chmod(0o777)
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
                    self.assertEqual(root.stat().st_mode & 0o777, 0o777)
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

    def test_recovery_refuses_forged_restore_mode_or_transition(self):
        for field, value in (
            ("original", 0o40700),
            ("current", 0o40777),
            ("changed", False),
        ):
            with self.subTest(field=field):
                root, owner = self.fixture()
                owner.prepare()
                saved = json.loads(json.dumps(owner.state))
                row = owner.state["rows"][-1]
                if field == "changed":
                    row[field] = value
                else:
                    row[field][4] = value
                owner.store()
                with self.assertRaisesRegex(A.Refused, "admin_recovery_exact"):
                    A.AdministrativeOwner.recover(
                        owner.journal,
                        owner.binding,
                        inspect_inputs=lambda: None,
                        root=root,
                        uid=os.getuid(),
                    )
                owner.state = saved
                # The journal normally shares identity lists with the live rows;
                # undo only this test's metadata mutation, not filesystem drift.
                for live, record in zip(owner.rows, saved["rows"]):
                    live["original"] = list(record["original"])
                    live["current"] = list(record["current"])
                owner.store()
                self.dispose(root, owner)

    def test_tool_output_bound_and_missing_wait_keep_live_owner(self):
        root, owner = self.fixture()
        owner.prepare()
        with self.assertRaisesRegex(A.Refused, "output_bound"):
            owner.tool(
                [
                    sys.executable,
                    "-I",
                    "-B",
                    "-S",
                    "-c",
                    "import os;os.write(1,b'x'*16385)",
                ],
                environment={"PATH": "/usr/bin:/bin"},
            )
        self.assertIsNone(owner.tool_child)
        self.assertEqual(len(owner.state["toolWaits"]), 1)
        real = subprocess.Popen
        children = []

        def spawn(*args, **kwargs):
            child = real(*args, **kwargs)
            children.append(child)
            child.wait = mock.Mock(
                side_effect=subprocess.TimeoutExpired("owned wait withheld", 1)
            )
            return child

        with mock.patch.object(A.subprocess, "Popen", side_effect=spawn):
            with self.assertRaises(subprocess.TimeoutExpired):
                owner.tool(
                    [sys.executable, "-I", "-B", "-S", "-c", "pass"],
                    environment={"PATH": "/usr/bin:/bin"},
                )
        with self.assertRaisesRegex(A.Refused, "unknown_administrative_tool"):
            owner.restore()
        self.assertIs(owner.tool_child, children[0])
        del children[0].wait
        owner.settle_tool()
        self.dispose(root, owner)

    def test_directory_sync_failure_keeps_ambiguous_effect_and_exact_owner(self):
        root, owner = self.fixture()
        real = os.fsync

        def sync(fd):
            if any(
                row["fd"] == fd and row["name"] == A.DIRECTORIES[0]
                for row in owner.rows
            ):
                raise OSError("owned directory fsync failed after chmod")
            return real(fd)

        with mock.patch.object(A.os, "fsync", side_effect=sync):
            with self.assertRaisesRegex(OSError, "fsync failed"):
                owner.prepare()
        self.assertIsNotNone(owner.state["operation"])
        with self.assertRaisesRegex(A.Refused, "ambiguous"):
            owner.restore()
        with self.assertRaisesRegex(A.Refused, "unobserved"):
            owner.finish_known_effect()
        # Intentionally retain this owned fixture: durability/receipt is unknown.

    def test_manifest_transfer_passes_only_readonly_regular_file_to_owned_child(self):
        root, owner = self.fixture()
        transfer = A.ManifestTransfer(owner, root / "manifest.json")
        owner.prepare()
        fd, digest = transfer.publish({"owned": True, "originalActivation": False})
        self.assertFalse(os.get_inheritable(fd))
        code = (
            "import runpy,os; s=runpy.run_path("
            + repr(str(ROOT / "scripts/claude-agent-sdk-bootstrap-session.py"))
            + "); s['manifest_bytes']("
            + repr(digest)
            + ",descriptor="
            + str(fd)
            + ",owner_uid=os.getuid());\ntry: os.fstat("
            + str(fd)
            + ")\nexcept OSError: pass\nelse: raise RuntimeError('manifest descriptor leaked')"
        )
        child = owner.entry(
            lambda: subprocess.Popen(
                [sys.executable, "-I", "-B", "-S", "-c", code],
                pass_fds=(fd,),
                close_fds=True,
                env={"PATH": "/usr/bin:/bin", "TMPDIR": str(root)},
            ),
            OwnedDirectScope(),
        )
        with self.assertRaisesRegex(A.Refused, "unknown_child"):
            transfer.close()
        self.assertEqual(owner.wait(), 0)
        transfer.check()
        transfer.close()
        self.dispose(root, owner)

    def test_manifest_transfer_refuses_replacement_and_oversize(self):
        root, owner = self.fixture()
        transfer = A.ManifestTransfer(owner, root / "manifest.json")
        owner.prepare()
        with self.assertRaisesRegex(A.Refused, "manifest_bound"):
            transfer.publish({"owned": "x" * 1048576})
        transfer.publish({"owned": True})
        transfer.path.chmod(0o644)
        with self.assertRaisesRegex(A.Refused, "transfer_drift"):
            transfer.check()
        # File-mode drift affects the retained output, not ancestor identities;
        # a known no-entry attempt may restore ancestors, retaining the output.
        transfer.close()
        self.dispose(root, owner)

    def test_monitor_entry_keeps_outer_owner_until_same_owned_child_actual_wait(self):
        class OwnedMonitor:
            # Direct owned Python fixture only, never a kernel/topology claim.
            fork_attempted = False
            init = None
            observed = None

            def run(self):
                self.fork_attempted = True
                self.child = subprocess.Popen(
                    [sys.executable, "-I", "-B", "-S", "-c", "pass"],
                    close_fds=True,
                    env={"PATH": "/usr/bin:/bin"},
                )
                self.init = self.child.pid
                raise OSError("owned monitor wait withheld")

            def settlement_observed(self):
                return self.observed is self.child and self.child.returncode is not None

            def actual_wait(self):
                self.child.wait(timeout=2)
                self.observed = self.child

        root, owner = self.fixture()
        owner.prepare()
        monitor = OwnedMonitor()
        with self.assertRaisesRegex(OSError, "wait withheld") as error:
            owner.monitor_entry(monitor)
        self.assertIs(error.exception.owner, owner)
        owner.state["wait"] = {"forged": True}
        with self.assertRaisesRegex(A.Refused, "actual_monitor_tree"):
            owner.finish_monitor()
        with self.assertRaisesRegex(A.Refused, "unknown_child"):
            owner.restore()
        monitor.actual_wait()
        owner.finish_monitor()
        self.dispose(root, owner)

    def test_monitor_known_no_fork_refusal_restores_but_unknown_fork_retains(self):
        for attempted in (False, True):
            root, owner = self.fixture()
            owner.prepare()
            monitor = mock.Mock()
            monitor.fork_attempted, monitor.init = attempted, None
            monitor.run.side_effect = OSError("owned monitor fork refused")
            monitor.settlement_observed.return_value = False
            with self.assertRaisesRegex(OSError, "fork refused"):
                owner.monitor_entry(monitor)
            if attempted:
                with self.assertRaisesRegex(A.Refused, "unknown_child"):
                    owner.restore()
                # Retain owned fixture/receipt: unknown fork supplies no wait.
            else:
                self.dispose(root, owner)

    def test_initial_binding_refusal_has_no_mutation_or_journal(self):
        root = Path(
            tempfile.mkdtemp(prefix="admin-binding-", dir=os.environ["TMPDIR"])
        ).resolve()

        def refuse():
            raise A.Refused("fresh image/source binding refused")

        with self.assertRaisesRegex(A.Refused, "fresh image"):
            A.AdministrativeOwner(
                root / "admin.json",
                {"source": "wrong"},
                inspect_inputs=refuse,
                root=root,
                uid=os.getuid(),
            )
        self.assertFalse((root / "admin.json").exists())
        self.assertEqual(list(root.iterdir()), [])
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
