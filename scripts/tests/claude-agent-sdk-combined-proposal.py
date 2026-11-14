"""Owned event/filesystem models; no root adapter or kernel policy is activated."""

import copy
from contextlib import contextmanager
import errno
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import signal
import tempfile
import time
import unittest
from unittest import mock

ROOT = Path(__file__).resolve().parents[2]


def load(name):
    file = ROOT / f"scripts/claude-agent-sdk-spawn-{name}.py"
    spec = importlib.util.spec_from_file_location(name, file)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


TRACE, CG, POLICY, COMBINED = (
    load(name) for name in ("trace", "cgroup", "policy", "combined")
)


def stopped(event, sig=signal.SIGTRAP):
    return (event << 16) | (sig << 8) | 0x7F


class TraceModel:
    kernel_enforced = False

    def __init__(self):
        self.calls, self.messages, self.images = [], {}, {}

    def seize(self, pid):
        self.calls.append(("seize", pid))

    def resume(self, pid, sig=0):
        self.calls.append(("resume", pid, sig))

    def event_message(self, pid):
        return self.messages[pid]

    def image(self, pid):
        return self.images[pid]


class GateFixture:
    def __init__(self):
        self.io = TraceModel()
        self.rows = {100: self.row(100, 1), 101: self.row(101, 100)}
        self.images = {
            key: {"sha256": key, "argv": [key], "environment": {}, "cwd": "owned"}
            for key in ("driver", "entry", "native")
        }
        self.revoked = False
        self.gate = TRACE.ExecGate(
            self.io,
            self.rows.__getitem__,
            self.member,
            self.images,
            lambda pid: self.io.calls.append(("pin", pid)),
            self.custody,
        )
        self.gate.driver(100)

    @staticmethod
    def row(pid, parent, tgid=None):
        return {
            "pid": pid,
            "parent": parent,
            "tgid": pid if tgid is None else tgid,
            "namespace": "owned-net",
            "pidNamespace": "owned-pid",
            "membership": "owned-cgroup",
            "uids": [1001] * 4,
            "gids": [1001] * 4,
            "start": str(pid),
        }

    def member(self, row):
        if (
            row["namespace"] != "owned-net"
            or row["membership"] != "owned-cgroup"
            or row["uids"] != [1001] * 4
        ):
            raise OSError(errno.EPERM, "modeled_membership_changed")

    def custody(self):
        if self.revoked:
            raise OSError(errno.EPERM, "modeled_custody_revoked")

    def event(self, pid, event, message=None, image=None):
        if message is not None:
            self.io.messages[pid] = message
        if image is not None:
            self.io.images[pid] = image
        return self.gate.process(
            {"actor": 1, "pid": pid, "status": stopped(event), "source": "owned_model"}
        )

    def driver(self):
        self.event(100, TRACE.STOP)
        self.event(100, TRACE.EXEC, 100, self.images["driver"])

    def native(self, release=True):
        self.driver()
        self.event(100, TRACE.FORK, 101)
        self.event(101, TRACE.STOP)
        self.event(101, TRACE.EXEC, 101, self.images["entry"])
        if release:
            self.gate.arm_native(101)


class TraceTests(unittest.TestCase):
    def test_real_state_machine_releases_only_exact_ordered_images(self):
        f = GateFixture()
        f.native()
        f.event(101, TRACE.EXEC, 101, f.images["native"])
        self.assertFalse(f.io.kernel_enforced)
        self.assertEqual(
            [
                row["state"]
                for row in f.gate.events
                if row["kind"] == "exec_image_released"
            ],
            ["driver", "native-entry", "native"],
        )
        pinned = f.io.calls.index(("pin", 101))
        self.assertLess(pinned, f.io.calls.index(("resume", 100, 0), pinned))
        for pid in (101, 100):
            f.event(pid, TRACE.EXIT)
            self.assertFalse(f.gate.complete())
            f.gate.process(
                {"pid": pid, "status": 0, "actor": 1, "source": "owned_model"}
            )
        self.assertTrue(f.gate.complete())

    def test_committed_image_argv_env_cwd_and_alternate_interpreter_refuse(self):
        for changed in (
            {"sha256": "other"},
            {"argv": ["node", "--eval", "effect"]},
            {"environment": {"NODE_OPTIONS": "effect"}},
            {"cwd": "elsewhere"},
        ):
            f = GateFixture()
            f.driver()
            before = len(f.io.calls)
            with self.assertRaisesRegex(OSError, "unapproved_committed_exec"):
                f.event(100, TRACE.EXEC, 100, {**f.images["native"], **changed})
            self.assertEqual(len(f.io.calls), before)

    def test_native_requires_live_one_shot_approval(self):
        f = GateFixture()
        f.native(release=False)
        with self.assertRaisesRegex(OSError, "unapproved_committed_exec"):
            f.event(101, TRACE.EXEC, 101, f.images["native"])
        f.gate.arm_native(101)
        with self.assertRaisesRegex(OSError, "duplicate_or_early"):
            f.gate.arm_native(101)
        f.event(101, TRACE.EXEC, 101, f.images["native"])
        with self.assertRaisesRegex(OSError, "duplicate_or_early"):
            f.gate.arm_native(101)
        with self.assertRaises(OSError):
            f.gate.arm_native(999)

    def test_revocation_is_checked_at_committed_exec_stop(self):
        f = GateFixture()
        f.native()
        f.revoked = True
        before = len(f.io.calls)
        with self.assertRaisesRegex(OSError, "custody_revoked"):
            f.event(101, TRACE.EXEC, 101, f.images["native"])
        self.assertEqual(len(f.io.calls), before)

    def test_membership_change_refuses_before_resume(self):
        for changed in (
            {"membership": "outside"},
            {"namespace": "host"},
            {"uids": [0] * 4},
        ):
            f = GateFixture()
            f.driver()
            f.rows[100].update(changed)
            before = len(f.io.calls)
            with self.assertRaisesRegex(OSError, "membership_changed"):
                f.event(100, TRACE.FORK, 101)
            self.assertEqual(len(f.io.calls), before)

    def test_extra_fork_or_orphan_and_native_double_fork_refuse_before_child_release(
        self,
    ):
        for parent in (100, 101):
            f = GateFixture()
            f.native()
            f.event(101, TRACE.EXEC, 101, f.images["native"])
            f.rows[102] = f.row(102, parent)
            before = len(f.io.calls)
            with self.assertRaisesRegex(OSError, "extra_descendant_before_entry"):
                f.event(parent, TRACE.FORK, 102)
            self.assertFalse(
                any(row[:2] == ("resume", 102) for row in f.io.calls[before:])
            )
        f = GateFixture()
        f.driver()
        f.rows[101]["parent"] = 999
        with self.assertRaisesRegex(OSError, "native_kernel_parent"):
            f.event(100, TRACE.FORK, 101)

    def test_threads_are_counted_with_processes_and_cannot_exec(self):
        f = GateFixture()
        f.driver()
        for pid in range(200, 231):
            f.rows[pid] = f.row(pid, 1, 100)
            f.event(100, TRACE.CLONE, pid)
            f.event(pid, TRACE.STOP)
        self.assertEqual(len(f.gate.tasks), 32)
        with self.assertRaisesRegex(OSError, "nonleader_exec"):
            f.event(200, TRACE.EXEC, 100, f.images["native"])
        f.rows[231] = f.row(231, 1, 100)
        with self.assertRaisesRegex(OSError, "kernel_task_budget"):
            f.event(100, TRACE.CLONE, 231)

    def test_child_stop_is_held_until_real_parent_event_and_pin(self):
        f = GateFixture()
        f.driver()
        before = len(f.io.calls)
        f.event(101, TRACE.STOP)
        self.assertEqual(len(f.io.calls), before)
        self.assertFalse(f.gate.complete())
        f.event(100, TRACE.FORK, 101)
        self.assertLess(
            f.io.calls.index(("pin", 101)), f.io.calls.index(("resume", 101, 0))
        )
        self.assertFalse(f.gate.pending_stops)

    def test_exit_stop_and_echild_do_not_substitute_for_terminal_wait(self):
        f = GateFixture()
        f.driver()
        f.event(100, TRACE.EXIT)
        self.assertFalse(f.gate.complete())
        self.assertNotIn("terminal", f.gate.tasks[100])
        f.gate.process({"pid": 100, "status": signal.SIGKILL, "source": "owned_model"})
        self.assertTrue(f.gate.complete())


def interpret(code, nr, args=(), arch=0xC000003E):
    words = {0: nr, 4: arch, **{16 + 8 * i: x for i, x in enumerate(args)}}
    index = 0
    acc = 0
    for _ in range(1000):
        op, jt, jf, k = code[index]
        if op == 0x20:
            acc = words.get(k, 0) & 0xFFFFFFFF
            index += 1
        elif op == 0x15:
            index += 1 + (jt if acc == k else jf)
        elif op == 0x45:
            index += 1 + (jt if acc & k else jf)
        elif op == 0x06:
            return k
        else:
            raise AssertionError(op)
    raise AssertionError("nonterminating BPF")


class PolicyTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        source = ROOT / "scripts/claude-agent-sdk-netns-entry.py"
        _, cls.blocked = POLICY.load_entry(
            source, hashlib.sha256(source.read_bytes()).hexdigest()
        )

    def test_process_thread_and_namespace_policy_without_loading_it(self):
        driver = POLICY.instructions("driver", 100, self.blocked)
        native = POLICY.instructions("native", 101, self.blocked)
        for nr in (57, 58):
            self.assertEqual(interpret(driver, nr), POLICY.ALLOW)
            self.assertEqual(interpret(native, nr), POLICY.DENY)
        self.assertEqual(interpret(native, 56, [0x10000]), POLICY.ALLOW)
        self.assertEqual(interpret(native, 56, [0]), POLICY.DENY)
        for flag in (0x80, 0x20000, 0x800000, 0x20000000, 0x40000000):
            for code in (driver, native):
                self.assertEqual(interpret(code, 56, [flag | 0x10000]), POLICY.DENY)
        self.assertEqual(interpret(driver, 435), 0x50000 | errno.ENOSYS)
        for nr in (101, 272, 308, 321, 424, 425):
            self.assertEqual(interpret(driver, nr), POLICY.DENY)
        self.assertEqual(interpret(driver, 1, arch=0), POLICY.DENY)
        self.assertEqual(interpret(driver, 0x40000001), POLICY.DENY)

    def test_signal_policy_cannot_reach_outer_host_or_broadcast_groups(self):
        for role in ("driver", "native"):
            code = POLICY.instructions(role, 100, self.blocked)
            for target in (0, 0xFFFFFFFF):
                self.assertEqual(interpret(code, 62, [target]), POLICY.DENY)
        code = POLICY.instructions("native", 100, self.blocked)
        self.assertEqual(interpret(code, 234, [100]), POLICY.ALLOW)
        self.assertEqual(interpret(code, 234, [101]), POLICY.DENY)


class CgroupTests(unittest.TestCase):
    def test_missing_delegation_controller_or_host_custody_refuses(self):
        CG.admit_delegation(0, 0o755, CG.CGROUP2_MAGIC, CG.REQUIRED, CG.REQUIRED)
        for owner, mode, fs, controllers, enabled in (
            (1001, 0o755, CG.CGROUP2_MAGIC, CG.REQUIRED, CG.REQUIRED),
            (0, 0o777, CG.CGROUP2_MAGIC, CG.REQUIRED, CG.REQUIRED),
            (0, 0o755, 0, CG.REQUIRED, CG.REQUIRED),
            (0, 0o755, CG.CGROUP2_MAGIC, {"memory", "cpu"}, CG.REQUIRED),
            (0, 0o755, CG.CGROUP2_MAGIC, CG.REQUIRED, set()),
        ):
            with self.assertRaises(OSError):
                CG.admit_delegation(owner, mode, fs, controllers, enabled)

    def fixture(self, root):
        group = CG.LinuxCgroup.__new__(CG.LinuxCgroup)
        group.parent = Path(root)
        group.name = "sdk234-" + "1" * 24
        group.parent_fd = os.open(root, os.O_RDONLY | os.O_DIRECTORY)
        group.fd = None
        group.created = False
        st = os.fstat(group.parent_fd)
        group.parent_identity = {"device": st.st_dev, "inode": st.st_ino}
        mkdir = os.mkdir

        def create(name, mode=0o777, **kwargs):
            mkdir(name, mode, **kwargs)
            for file, data in {
                **{k: "0" for k in CG.LIMITS},
                "cgroup.procs": "",
                "cgroup.threads": "",
                "cgroup.events": "populated 0\nfrozen 0",
                "cgroup.freeze": "0",
                "cgroup.kill": "0",
                "pids.events": "max 0",
                "memory.events": "oom_kill 0",
            }.items():
                (group.path / file).write_text(data)

        with mock.patch.object(CG.os, "mkdir", side_effect=create):
            group.create()
        return group

    def test_limits_precede_initial_membership_and_changed_limits_refuse(self):
        with tempfile.TemporaryDirectory(dir=os.environ["TMPDIR"]) as root:
            group = self.fixture(root)
            try:
                self.assertEqual(
                    {k: CG.read_at(group.fd, k) for k in CG.LIMITS}, CG.LIMITS
                )
                group.attach(100)
                self.assertEqual(group.members(), [100])
                with self.assertRaises(OSError):
                    group.attach(101)
                (group.path / "pids.max").write_text("max")
                with self.assertRaisesRegex(OSError, "limit_changed"):
                    group.assert_limits()
            finally:
                group.close()

    def test_thread_exhaustion_unknown_freeze_and_cleanup_identity_refuse(self):
        with tempfile.TemporaryDirectory(dir=os.environ["TMPDIR"]) as root:
            group = self.fixture(root)
            try:
                (group.path / "cgroup.threads").write_text(
                    "\n".join(str(x) for x in range(33))
                )
                with self.assertRaisesRegex(OSError, "task_limit"):
                    group.tasks()
                with self.assertRaisesRegex(OSError, "freeze_unknown"):
                    group.stop(time.monotonic() - 1)
                self.assertEqual(CG.read_at(group.fd, "cgroup.kill"), "1")
                with self.assertRaisesRegex(OSError, "cleanup_identity"):
                    group.remove({"identity": {"device": -1, "inode": -1}})
                self.assertTrue(group.path.is_dir())
            finally:
                group.close()


class CombinedTests(unittest.TestCase):
    creation_observations = []

    @contextmanager
    def fixture_transaction(self, root, fault=None):
        file = ROOT / "scripts/claude-agent-sdk-admin-fixture.py"
        spec = importlib.util.spec_from_file_location("fixture", file)
        fixture = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(fixture)
        parent = root / "parent"
        parent.mkdir(mode=0o755)
        journal_path = root / "journal.json"
        fd = os.open(journal_path, os.O_CREAT | os.O_EXCL | os.O_RDWR, 0o600)
        journal = {
            "fixture": None,
            "fixtureStage": "not-started",
            "cgroup": None,
            "cgroupStage": "not-started",
            "state": "disabled",
            "run": "owned-model",
        }
        COMBINED.persist_journal(fd, journal, root)

        def store(stage, receipt):
            journal.update(fixtureStage=stage, fixture=receipt)
            if stage == "creating" and fault == "intent-persistence":
                raise OSError("owned intent persistence failure")
            if stage == "created" and fault in {
                "receipt-persistence",
                "interrupt-receipt",
            }:
                if fault == "interrupt-receipt":
                    raise KeyboardInterrupt()
                raise OSError("owned identity persistence failure")
            COMBINED.persist_journal(fd, journal, root)
            if stage == "creating" and fault == "interrupt-before-mkdir":
                raise KeyboardInterrupt()

        try:
            yield fixture, parent, journal_path, store
        finally:
            os.close(fd)

    def observed_cleanup(self, fixture, parent, journal_path, *, tree=True):
        journal = json.loads(journal_path.read_bytes())
        resources = COMBINED.cleanup_resources(
            journal, parent, os.getuid(), fixture, mock.Mock()
        )
        return COMBINED.finish_cleanup(
            journal_path, journal_path.lstat(), journal, resources, tree
        )

    def test_creation_faults_preserve_unknown_identity_and_reported_journal(self):
        cases = (
            "after-mkdir",
            "before-open",
            "before-stat",
            "before-chmod",
            "before-chmod-private-mode",
            "receipt-persistence",
            "interrupt-after-mkdir",
            "interrupt-receipt",
            "interrupt-before-mkdir",
        )
        for fault in cases:
            with (
                self.subTest(fault=fault),
                tempfile.TemporaryDirectory(dir=os.environ["TMPDIR"]) as temporary,
            ):
                root = Path(temporary)
                with self.fixture_transaction(root, fault) as (
                    fixture,
                    parent,
                    journal_path,
                    store,
                ):
                    original_open, original_mkdir, original_fstat, original_fchmod = (
                        os.open,
                        os.mkdir,
                        os.fstat,
                        os.fchmod,
                    )
                    child = []

                    def mkdir(name, *args, **kwargs):
                        if (
                            name == "claude-code"
                            and fault == "before-chmod-private-mode"
                        ):
                            result = original_mkdir(name, 0o700, **kwargs)
                        else:
                            result = original_mkdir(name, *args, **kwargs)
                        if name == "claude-code" and fault in {
                            "after-mkdir",
                            "interrupt-after-mkdir",
                        }:
                            if fault.startswith("interrupt"):
                                raise KeyboardInterrupt()
                            raise OSError("owned interruption after mkdir")
                        return result

                    def open_file(name, *args, **kwargs):
                        if name == "claude-code" and fault == "before-open":
                            raise OSError("owned open refusal")
                        result = original_open(name, *args, **kwargs)
                        if name == "claude-code":
                            child.append(result)
                        return result

                    def fstat(descriptor):
                        if descriptor in child and fault == "before-stat":
                            raise OSError("owned stat refusal")
                        return original_fstat(descriptor)

                    def fchmod(descriptor, mode):
                        if descriptor in child and fault in {
                            "before-chmod",
                            "before-chmod-private-mode",
                        }:
                            raise OSError("owned chmod refusal")
                        return original_fchmod(descriptor, mode)

                    with (
                        mock.patch.object(COMBINED.os, "mkdir", side_effect=mkdir),
                        mock.patch.object(COMBINED.os, "open", side_effect=open_file),
                        mock.patch.object(COMBINED.os, "fstat", side_effect=fstat),
                        mock.patch.object(COMBINED.os, "fchmod", side_effect=fchmod),
                    ):
                        with self.assertRaises((OSError, KeyboardInterrupt)):
                            COMBINED.prepare_fixture(
                                parent,
                                os.getuid(),
                                "disabled",
                                "owned-model",
                                fixture,
                                store,
                            )
                    persisted = json.loads(journal_path.read_bytes())
                    known = fault in {"before-chmod", "before-chmod-private-mode"}
                    self.assertEqual(
                        persisted["fixtureStage"], "created" if known else "creating"
                    )
                    self.assertEqual(persisted["fixture"] is not None, known)
                    result = self.observed_cleanup(fixture, parent, journal_path)
                    self.assertEqual(result["fixtureCleanup"], known)
                    self.assertEqual(result["journalRetained"], not known)
                    self.assertEqual(journal_path.exists(), not known)
                    self.assertEqual(
                        (parent / "claude-code").exists(),
                        not known and fault != "interrupt-before-mkdir",
                    )
                    if not known:
                        self.assertIn(
                            "fixture_creation_identity_unresolved", result["failure"]
                        )
                    self.creation_observations.append(
                        {
                            "fault": fault,
                            "persistedStage": persisted["fixtureStage"],
                            "identityKnown": known,
                            "fixtureCleanup": result["fixtureCleanup"],
                            "journalRetained": result["journalRetained"],
                            "kernelEnforced": False,
                        }
                    )

    def test_intent_must_persist_before_mkdir(self):
        with tempfile.TemporaryDirectory(dir=os.environ["TMPDIR"]) as temporary:
            with self.fixture_transaction(Path(temporary), "intent-persistence") as (
                fixture,
                parent,
                journal_path,
                store,
            ):
                with mock.patch.object(COMBINED.os, "mkdir") as mkdir:
                    with self.assertRaisesRegex(OSError, "intent persistence"):
                        COMBINED.prepare_fixture(
                            parent,
                            os.getuid(),
                            "disabled",
                            "owned-model",
                            fixture,
                            store,
                        )
                mkdir.assert_not_called()
                self.assertEqual(
                    json.loads(journal_path.read_bytes())["fixtureStage"], "not-started"
                )
                result = self.observed_cleanup(
                    fixture, parent, journal_path, tree=False
                )
                self.assertTrue(result["fixtureCleanup"])
                self.assertTrue(result["journalRetained"])
                self.assertTrue(journal_path.exists())

    def test_absence_or_legacy_null_receipt_never_adopts_creation(self):
        for stage in (None, "creating", "created", "ready"):
            with (
                self.subTest(stage=stage),
                tempfile.TemporaryDirectory(dir=os.environ["TMPDIR"]) as temporary,
            ):
                with self.fixture_transaction(Path(temporary)) as (
                    fixture,
                    parent,
                    journal_path,
                    store,
                ):
                    journal = json.loads(journal_path.read_bytes())
                    if stage is None:
                        journal.pop("fixtureStage")
                    else:
                        journal["fixtureStage"] = stage
                    journal_path.write_text(json.dumps(journal))
                    result = self.observed_cleanup(fixture, parent, journal_path)
                    self.assertFalse(result["fixtureCleanup"])
                    self.assertTrue(result["journalRetained"])
                    self.assertTrue(journal_path.exists())
                    self.assertFalse((parent / "claude-code").exists())

    def test_existing_object_is_not_adopted_after_exclusive_creation_refusal(self):
        with tempfile.TemporaryDirectory(dir=os.environ["TMPDIR"]) as temporary:
            with self.fixture_transaction(Path(temporary)) as (
                fixture,
                parent,
                journal_path,
                store,
            ):
                existing = parent / "claude-code"
                existing.mkdir()
                (existing / "unowned-by-this-transaction").write_text("preserve")
                with self.assertRaises(FileExistsError):
                    COMBINED.prepare_fixture(
                        parent, os.getuid(), "disabled", "owned-model", fixture, store
                    )
                result = self.observed_cleanup(fixture, parent, journal_path)
                self.assertFalse(result["fixtureCleanup"])
                self.assertTrue(result["journalRetained"])
                self.assertTrue((existing / "unowned-by-this-transaction").is_file())

    def test_short_intent_write_refuses_before_creation_and_preserves_corrupt_journal(
        self,
    ):
        with tempfile.TemporaryDirectory(dir=os.environ["TMPDIR"]) as temporary:
            with self.fixture_transaction(Path(temporary)) as (
                fixture,
                parent,
                journal_path,
                store,
            ):
                original = os.write
                with mock.patch.object(
                    COMBINED.os,
                    "write",
                    side_effect=lambda fd, data: original(fd, data[:1]),
                ):
                    with self.assertRaisesRegex(OSError, "journal_short_write"):
                        COMBINED.prepare_fixture(
                            parent,
                            os.getuid(),
                            "disabled",
                            "owned-model",
                            fixture,
                            store,
                        )
                self.assertFalse((parent / "claude-code").exists())
                self.assertTrue(journal_path.exists())
                with self.assertRaises(json.JSONDecodeError):
                    self.observed_cleanup(fixture, parent, journal_path)
                self.assertTrue(journal_path.exists())

    def test_unknown_cgroup_creation_also_prevents_journal_disposal(self):
        with tempfile.TemporaryDirectory(dir=os.environ["TMPDIR"]) as temporary:
            with self.fixture_transaction(Path(temporary)) as (
                fixture,
                parent,
                journal_path,
                store,
            ):
                journal = json.loads(journal_path.read_bytes())
                journal["cgroupStage"] = "creating"
                journal_path.write_text(json.dumps(journal))
                result = self.observed_cleanup(fixture, parent, journal_path)
                self.assertFalse(result["cgroupCleanup"])
                self.assertTrue(result["journalRetained"])
                self.assertTrue(journal_path.exists())

    def test_known_identity_cleanup_failure_and_interruption_preserve_journal(self):
        for interrupt in (False, True):
            with tempfile.TemporaryDirectory(dir=os.environ["TMPDIR"]) as temporary:
                with self.fixture_transaction(Path(temporary)) as (
                    fixture,
                    parent,
                    journal_path,
                    store,
                ):
                    COMBINED.prepare_fixture(
                        parent, os.getuid(), "disabled", "owned-model", fixture, store
                    )
                    unlink = os.unlink
                    removed = []

                    def partial_unlink(name, *args, **kwargs):
                        unlink(name, *args, **kwargs)
                        removed.append(name)
                        if len(removed) == 1:
                            if interrupt:
                                raise KeyboardInterrupt()
                            raise OSError("owned partial cleanup failure")

                    reported = None
                    with mock.patch.object(
                        COMBINED.os, "unlink", side_effect=partial_unlink
                    ):
                        if interrupt:
                            with self.assertRaises(KeyboardInterrupt):
                                reported = self.observed_cleanup(
                                    fixture, parent, journal_path
                                )
                        else:
                            reported = self.observed_cleanup(
                                fixture, parent, journal_path
                            )
                    self.assertTrue(journal_path.exists())
                    self.assertEqual(len(os.listdir(parent / "claude-code")), 1)
                    if interrupt:
                        self.assertIsNone(reported)
                    else:
                        self.assertFalse(reported["fixtureCleanup"])
                        self.assertTrue(reported["journalRetained"])
                        self.assertIn(
                            "owned partial cleanup failure", reported["failure"][0]
                        )

    def test_known_receipt_with_missing_object_preserves_unresolved_evidence(self):
        with tempfile.TemporaryDirectory(dir=os.environ["TMPDIR"]) as temporary:
            with self.fixture_transaction(Path(temporary)) as (
                fixture,
                parent,
                journal_path,
                store,
            ):
                COMBINED.prepare_fixture(
                    parent, os.getuid(), "disabled", "owned-model", fixture, store
                )
                fixture.cleanup(
                    parent, "claude-code", "disabled", os.getuid(), "owned-model"
                )
                result = self.observed_cleanup(fixture, parent, journal_path)
                self.assertFalse(result["fixtureCleanup"])
                self.assertTrue(result["journalRetained"])
                self.assertTrue(journal_path.exists())

    def test_proved_fixture_disposal_cannot_discard_unknown_tree_evidence(self):
        with tempfile.TemporaryDirectory(dir=os.environ["TMPDIR"]) as temporary:
            with self.fixture_transaction(Path(temporary)) as (
                fixture,
                parent,
                journal_path,
                store,
            ):
                COMBINED.prepare_fixture(
                    parent, os.getuid(), "disabled", "owned-model", fixture, store
                )
                result = self.observed_cleanup(
                    fixture, parent, journal_path, tree=False
                )
                self.assertTrue(result["fixtureCleanup"])
                self.assertFalse((parent / "claude-code").exists())
                self.assertFalse(result["independentTreeSettlement"])
                self.assertEqual(result["failure"], "tree_settlement_unproved")
                self.assertTrue(result["journalRetained"])
                self.assertTrue(journal_path.exists())
                self.creation_observations.append(
                    {
                        "fault": "tree-settlement-unproved",
                        "identityKnown": True,
                        "fixtureCleanup": True,
                        "journalRetained": True,
                        "kernelEnforced": False,
                    }
                )

    def test_fixture_partial_setup_cleans_only_its_exclusive_exact_inode(self):
        file = ROOT / "scripts/claude-agent-sdk-admin-fixture.py"
        spec = importlib.util.spec_from_file_location("fixture", file)
        fixture = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(fixture)
        for fault in ("after_identity", "after_policy", "unknown_file"):
            with tempfile.TemporaryDirectory(dir=os.environ["TMPDIR"]) as temporary:
                root = Path(temporary)
                receipt = []
                original = fixture.write_file

                def store(stage, value):
                    if stage == "created":
                        receipt.append(value)
                    if stage == "created" and fault == "after_identity":
                        raise OSError("modeled write refusal")

                def write(fd, name, data, mode):
                    if name == fixture.LEASE:
                        raise OSError("modeled receipt write refusal")
                    return original(fd, name, data, mode)

                with mock.patch.object(fixture, "write_file", side_effect=write):
                    with self.assertRaises(OSError):
                        COMBINED.prepare_fixture(
                            root, os.getuid(), "disabled", "owned-model", fixture, store
                        )
                if fault == "unknown_file":
                    (root / "claude-code" / "unexpected").write_text("preserve")
                    with self.assertRaisesRegex(OSError, "unknown_file"):
                        COMBINED.cleanup_partial_fixture(
                            root, os.getuid(), receipt[0], fixture
                        )
                    self.assertTrue((root / "claude-code" / "unexpected").exists())
                else:
                    COMBINED.cleanup_partial_fixture(
                        root, os.getuid(), receipt[0], fixture
                    )
                    self.assertFalse((root / "claude-code").exists())

    def test_combined_fixture_uses_existing_exact_bytes_and_cleanup(self):
        file = ROOT / "scripts/claude-agent-sdk-admin-fixture.py"
        spec = importlib.util.spec_from_file_location("fixture", file)
        fixture = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(fixture)
        for state in ("absent", "disabled", "managed-deny"):
            with tempfile.TemporaryDirectory(dir=os.environ["TMPDIR"]) as temporary:
                root = Path(temporary)
                receipt = []
                COMBINED.prepare_fixture(
                    root,
                    os.getuid(),
                    state,
                    "owned-model",
                    fixture,
                    lambda stage, value: receipt.append(value)
                    if stage == "created"
                    else None,
                )
                policy = root / "claude-code" / fixture.POLICY
                if state == "absent":
                    self.assertFalse(policy.exists())
                else:
                    self.assertEqual(policy.read_bytes(), fixture.payload(state))
                fixture.cleanup(root, "claude-code", state, os.getuid(), "owned-model")
                self.assertFalse(policy.parent.exists())

    def test_root_socket_owner_gate_refuses_forged_approval_before_release(self):
        m = COMBINED.Monitor.__new__(COMBINED.Monitor)
        m.lease = mock.Mock()
        m.check_network = mock.Mock()
        m.group = mock.Mock()
        m.group.assert_limits = mock.Mock()
        m.init = 100
        m.find = mock.Mock(return_value={"pid": 101})
        m.pin = mock.Mock()
        m.gate = mock.Mock()
        with mock.patch.object(COMBINED, "socket_peer", return_value=999):
            with self.assertRaisesRegex(OSError, "forged_entry_peer"):
                m.handle(
                    {
                        "kind": "native_gate",
                        "namespacePid": 2,
                        "clientPort": 123,
                        "serverPort": 456,
                    }
                )
        m.pin.assert_not_called()
        m.gate.arm_native.assert_not_called()

    def test_actual_membership_predicate_rejects_migration_credentials_and_namespace_escape(
        self,
    ):
        m = COMBINED.Monitor.__new__(COMBINED.Monitor)
        m.group = mock.Mock()
        m.group.assert_limits = mock.Mock()
        m.group.members.return_value = [101]
        m.init = 100
        m.journal = {"membership": "owned", "namespace": "net", "pidNamespace": "pid"}
        m.host = {"pidNamespace": "host"}
        m.args = mock.Mock(uid=1001, gid=1001)
        row = {
            "tgid": 101,
            "membership": "owned",
            "namespace": "net",
            "pidNamespace": "pid",
            "uids": [1001] * 4,
            "gids": [1001] * 4,
            "groups": [],
            "caps": {"CapEff": "0"},
        }
        m.member(row)
        for change in (
            {"tgid": 999},
            {"membership": "outside"},
            {"namespace": "host"},
            {"pidNamespace": "host"},
            {"pidNamespace": "other"},
            {"uids": [0] * 4},
            {"gids": [0] * 4},
            {"groups": [1001]},
            {"caps": {"CapEff": "1"}},
        ):
            with self.assertRaises(OSError):
                m.member({**row, **change})

    def test_large_layout_crosses_only_bounded_acknowledged_frames(self):
        m = COMBINED.Monitor.__new__(COMBINED.Monitor)
        m.layout_header = None
        m.layout_data = bytearray()
        received = []

        def handle(request):
            if request["kind"] == "layout":
                received.append(request["packet"])
                return {}
            return COMBINED.Monitor.handle(m, request)

        m.handle = handle
        guest = COMBINED.GuestContext.__new__(COMBINED.GuestContext)
        guest.layout = {"root": "owned", "readFiles": []}
        frames = []

        def request(kind, **values):
            frame = {"kind": kind, **values}
            self.assertLessEqual(len(json.dumps(frame).encode()) + 1, COMBINED.FRAME)
            frames.append(frame)
            return m.handle(frame)

        guest.request = request
        lease = mock.Mock()
        lease.files = {"/owned/" + str(i) + "/library": None for i in range(900)}
        guest.ready(lease)
        self.assertEqual(received, [guest.layout])
        self.assertGreater(len(m.layout_data), COMBINED.FRAME)
        self.assertGreater(len(frames), 3)
        m.layout_header["sha256"] = "0" * 64
        with self.assertRaisesRegex(OSError, "incomplete_layout"):
            m.handle({"kind": "layout_end"})

    def test_applied_workflow_retains_fixed_scope_and_prior_workflow_bytes(self):
        patch = (
            ROOT / "docs/knowledge/operations/claude-agent-sdk-combined-workflow.patch"
        ).read_bytes()
        body = COMBINED.proposed_workflow(patch)
        self.assertIn("--kernel-combined", body)
        self.assertIn("--cleanup-combined", body)
        for before, after in (
            (b"contents: read", b"contents: write"),
            (b"timeout-minutes: 5", b"timeout-minutes: 6"),
            (b"if: always()", b"if: success()"),
            (b"workflow_dispatch:", b"push:"),
        ):
            with self.assertRaises(OSError):
                COMBINED.proposed_workflow(patch.replace(before, after))
        current = ROOT / ".github/workflows/sdk234-netns-proof.yml"
        self.assertEqual(
            hashlib.sha256(current.read_bytes()).hexdigest(),
            "07a24539f219a477886cae19f9de5d738cc021d998469401eefb3faa2c2c1d19",
        )
        self.assertEqual(
            (ROOT / ".github/workflows/sdk234-combined-fake.yml").read_bytes(),
            body.encode(),
        )

    def test_routing_patch_applies_only_in_owned_fixture_and_rejects_scope_mutations(
        self,
    ):
        names = [
            "scripts/tests/oxide-selector-routing.py",
            "docs/knowledge/operations/claude-agent-sdk-spawn-plan.json",
            "docs/knowledge/operations/claude-agent-sdk-combined-routing.patch",
        ]
        files = {name: (ROOT / name).read_bytes() for name in names}
        patch = (
            ROOT / "docs/knowledge/operations/claude-agent-sdk-combined-workflow.patch"
        ).read_bytes()
        body = COMBINED.proposed_workflow(patch)
        before = files[names[0]]
        COMBINED.check_routing_packet(files, body)
        self.assertEqual((ROOT / names[0]).read_bytes(), before)
        with self.assertRaises(OSError):
            COMBINED.check_routing_packet(
                {**files, names[0]: before + b"# changed base\n"}, body
            )
        changed = files[names[2]].replace(b"sdk234-combined-fake.yml", b"other.yml")
        with self.assertRaises(OSError):
            COMBINED.check_routing_packet({**files, names[2]: changed}, body)

    def test_sequence_length_digest_and_missing_result_are_checked_by_actual_monitor(
        self,
    ):
        import base64

        m = COMBINED.Monitor.__new__(COMBINED.Monitor)
        m.result_header = None
        m.result_data = bytearray()
        m.result = None
        data = b'{"failure":null}'
        m.handle(
            {"kind": "result_begin", "length": len(data), "sha256": COMBINED.sha(data)}
        )
        for changed in (
            {"index": 1, "data": base64.b64encode(data).decode()},
            {"index": 0, "data": base64.b64encode(data[:-1]).decode()},
        ):
            with self.assertRaises(OSError):
                m.handle({"kind": "result_chunk", **changed})
        with self.assertRaises(OSError):
            m.handle({"kind": "done"})
        m.handle(
            {
                "kind": "result_chunk",
                "index": 0,
                "data": base64.b64encode(data).decode(),
            }
        )
        m.handle({"kind": "done"})
        self.assertIsNone(m.result["failure"])
        m.result_header["sha256"] = "0" * 64
        with self.assertRaises(OSError):
            m.handle({"kind": "done"})

    def test_empty_cgroup_and_summary_flag_cannot_attest_reaping(self):
        good = {
            "treeSettled": True,
            "failure": None,
            "init": {"pid": 100},
            "processes": [{"pid": 100}, {"pid": 101}, {"pid": 102}],
            "pidfdReaped": [100, 101, 102],
            "waits": [{"pid": 100, "status": 0}],
            "traceWaits": [{"pid": 101, "status": 0}, {"pid": 102, "status": 0}],
            "softwareResult": {
                "failure": None,
                "driverWait": 0,
                "actorGraph": {
                    "monitor": {"pid": 1, "hostPid": 100},
                    "driver": {"pid": 2, "hostPid": 102},
                    "native": {"pid": 3, "hostPid": 101},
                },
                "nativeWaits": [{"code": 0, "pid": 3, "actor": 2}],
                "events": [{"kind": "driver_wait", "pid": 2, "actor": 1, "code": 0}],
            },
            "resourceEvents": {"empty": True, "tasks": []},
            "outsideCanaryUnreached": True,
        }
        self.assertTrue(COMBINED.verify_settlement(good))
        for key, value in (
            ("pidfdReaped", [100]),
            ("waits", []),
            ("traceWaits", []),
            ("softwareResult", {"failure": None, "driverWait": 0}),
            ("failure", "cancelled"),
            ("resourceEvents", {"empty": True, "tasks": [101]}),
        ):
            self.assertFalse(COMBINED.verify_settlement({**good, key: value}))
        self.assertFalse(COMBINED.verify_settlement({"treeSettled": True}))

    def test_partial_setup_and_cancel_use_owned_pidfd_and_real_wait_vocabulary(self):
        for message in ("attach_refused_before_release", "combined_cancelled"):
            with tempfile.TemporaryDirectory(dir=os.environ["TMPDIR"]) as root:
                m = COMBINED.Monitor.__new__(COMBINED.Monitor)
                read, write = os.pipe()
                m.setup_fds = [write]
                m.parent_fd = os.dup(read)
                m.init = 500
                m.handles = {500: read}
                m.waits = []
                m.gate = None
                m.previous_signals = {}
                m.canary = mock.Mock()
                m.journal = {"treeSettled": False}
                m.lease = mock.Mock()
                m.lease.observation.return_value = {"modeled": True}
                m.trace = mock.Mock()
                m.trace.waits = []
                m.trace.wait.return_value = {
                    "pid": 500,
                    "status": 78 << 8,
                    "actor": 1,
                    "source": "owned_model",
                }
                m.journal_fd = os.open(
                    Path(root) / "receipt", os.O_CREAT | os.O_WRONLY, 0o600
                )
                m.journal_parent = Path(root)
                m.run_owned = mock.Mock(side_effect=OSError(errno.EPERM, message))
                try:
                    with mock.patch.object(
                        COMBINED.signal, "pidfd_send_signal", create=True
                    ) as kill:
                        with self.assertRaisesRegex(OSError, message):
                            m.run()
                    kill.assert_called_once_with(read, signal.SIGKILL)
                    self.assertEqual(
                        m.journal["waits"][0]["source"], "parent_or_adoptee_waitpid"
                    )
                    self.assertFalse(m.journal["treeSettled"])
                finally:
                    os.close(m.journal_fd)

    def test_root_entrypoints_refuse_this_unprivileged_host_without_mutation(self):
        if os.geteuid() == 0:
            self.skipTest("requires ordinary uid")
        args = mock.Mock()
        with self.assertRaisesRegex(OSError, "separate_root_Linux"):
            COMBINED.activate(args, None, None)
        with self.assertRaisesRegex(OSError, "root_cleanup_authority"):
            COMBINED.cleanup(args, None, None)

    @classmethod
    def tearDownClass(cls):
        (Path(os.environ["TMPDIR"]) / "combined-model-observations.json").write_text(
            json.dumps(
                {
                    "kernelEnforced": False,
                    "creationCounterexamples": cls.creation_observations,
                    "originalActivation": False,
                    "sourceRecord": hashlib.sha256(
                        (ROOT / COMBINED.RECORD).read_bytes()
                    ).hexdigest(),
                    "cases": "actual source state machines with owned event/filesystem models; no activated host",
                }
            )
            + "\n"
        )


if __name__ == "__main__":
    unittest.main()
