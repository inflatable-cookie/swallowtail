"""Owned-fork transaction tests; no candidate runtime or host activation."""

import hashlib
import importlib.util
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import types
import unittest
from unittest import mock

ROOT = Path(__file__).resolve().parents[2]


def load(name, file):
    spec = importlib.util.spec_from_file_location(name, file)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


S = load(
    "sdk_bootstrap_session", ROOT / "scripts/claude-agent-sdk-bootstrap-session.py"
)
OLD = load("sdk_owned_fork", ROOT / "scripts/tests/claude-agent-sdk-probe-custody.py")
ELF_TEST = load(
    "sdk_synthetic_elf", ROOT / "scripts/tests/claude-agent-sdk-bootstrap-elf.py"
)
C, E = OLD.C, ELF_TEST.E


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


class FixtureHost(S.HostInputs):
    """Model manifest/ELF admission only; retain real descriptor drift checks."""

    def __init__(self, custody, elf, manifest, *, recovery=False):
        self.custody, self.manifest, self.aliases = custody, manifest, {}
        self.capture = custody.Inputs(reserve_pipes=False)
        self.lookup = {"directories": {}, "absent": []}
        try:
            for role in (
                "libraries",
                "loaderInputs",
                "interpreters",
                "supervisorInputs",
            ):
                for name, digest in manifest["bootstrapInputs"][role].items():
                    self.capture.bind(name, digest)
            self.capture.bind_ancestors(Path(manifest["path"]))
            self.node_fd = os.open(manifest["path"], os.O_RDONLY | os.O_NOFOLLOW)
            self.node_identity = custody.identity(os.fstat(self.node_fd))
            self.check()
        except BaseException:
            self.close()
            raise


class SessionTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(
            prefix="sdk-session-", dir=os.environ["TMPDIR"]
        )
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name).resolve()
        self.environment = mock.patch.dict(
            os.environ, {key: str(self.root) for key in ("TMPDIR", "TMP", "TEMP")}
        )
        self.environment.start()
        self.addCleanup(self.environment.stop)
        self.node, self.library = self.root / "observer", self.root / "library"
        self.supervisor, self.interpreter = (
            self.root / "supervisor",
            self.root / "interpreter",
        )
        for path in (self.library, self.supervisor, self.interpreter):
            path.write_bytes(b"owned fixture input; never executed")
        self.interpreter.chmod(0o755)
        report = {
            "node": str(self.node),
            "version": "v0.0.0",
            "libraries": [str(self.library)],
        }
        self.node.write_text(
            "#!" + str(self.interpreter) + "\nprint(" + repr(json.dumps(report)) + ")\n"
        )
        self.node.chmod(0o777)
        loader = {}
        if sys.platform == "linux" and Path("/etc/ld.so.cache").exists():
            loader["/etc/ld.so.cache"] = sha("/etc/ld.so.cache")
        self.manifest = {
            "path": str(self.node),
            "sha256": sha(self.node),
            "bytes": self.node.stat().st_size,
            "version": "v0.0.0",
            "originalActivation": False,
            "bootstrapInputs": {
                "libraries": {str(self.library): sha(self.library)},
                "loaderInputs": loader,
                "interpreters": {str(self.interpreter): sha(self.interpreter)},
                "supervisorInputs": {str(self.supervisor): sha(self.supervisor)},
            },
        }
        self.journal = self.root / "receipt.json"
        self.session = S.Session(C, E, self.manifest, self.journal)
        self.children, self.captures, self.callbacks = [], [], []
        self.real_runtime = C.runtime_inventory
        self.host_patch = mock.patch.object(S, "HostInputs", FixtureHost)
        self.host_patch.start()
        self.addCleanup(self.host_patch.stop)
        self.python_patch = mock.patch.object(
            C, "python_inputs", return_value={self.supervisor}
        )
        self.python_patch.start()
        self.addCleanup(self.python_patch.stop)
        self.addCleanup(self.release)

    def release(self):
        for child in self.children:
            if child.returncode is None:
                child.kill()
            OLD.OwnedForkObserver.wait(child, timeout=2)
            child.stdout.close()
            child.stderr.close()
        for capture in self.captures:
            capture.active_child = None
            capture.close()
        preparation = self.session.preparation
        preparation.close()
        if self.session.host is not None and hasattr(self.session.host, "node_fd"):
            self.session.host.close()

    def observer(self, command, **kwargs):
        state = json.loads(self.journal.read_text())
        self.assertEqual(state["phase"], "observer_entry_intent")
        self.assertEqual(state["children"], [-1])
        self.assertEqual(self.node.stat().st_mode & 0o777, 0o755)
        self.assertEqual(command[:3], [str(self.node), *C.STARTUP])
        self.assertEqual(kwargs["cwd"], str(self.root))
        child = OLD.OwnedForkObserver(self.node)
        self.children.append(child)
        return child

    def inventory(self, node, digest, **kwargs):
        self.captures.append(kwargs["bootstrap"])
        return self.real_runtime(node, digest, **kwargs)

    def rehearsal(self, record, node, digest, **kwargs):
        observed = kwargs["runtime_factory"](C, node, digest)
        try:
            self.assertEqual(self.node.stat().st_mode & 0o777, 0o755)
            self.assertEqual(
                json.loads(self.journal.read_text())["phase"], "fake_entry_intent"
            )
            self.assertTrue(all(child.returncode == 0 for child in self.children))
            self.callbacks.append((kwargs["mode"], kwargs["state"]))
            return {"failure": None, "driverWait": 0, "nativeWaits": [{"code": 0}]}
        finally:
            observed.close()

    def run_session(self, callback=None):
        with (
            mock.patch.object(S.subprocess, "Popen", side_effect=self.observer),
            mock.patch.object(C, "runtime_inventory", side_effect=self.inventory),
        ):
            return self.session.run(
                types.SimpleNamespace(rehearsal=callback or self.rehearsal),
                "owned-record",
            )

    def test_same_invocation_observes_and_joins_before_checked_restore(self):
        original = self.session.helper.os.fchmod

        def chmod(descriptor, mode):
            state = json.loads(self.journal.read_text())
            if mode == 0o755:
                self.assertEqual(state["phase"], "intent")
                self.assertEqual(len(self.children), 0)
            else:
                self.assertEqual(mode, 0o777)
                self.assertEqual(len(state["observerWaits"]), 3)
                self.assertEqual(len(self.callbacks), 3)
                self.assertTrue(all(child.returncode == 0 for child in self.children))
            original(descriptor, mode)

        with mock.patch.object(self.session.helper.os, "fchmod", side_effect=chmod):
            result = self.run_session()
        self.assertEqual(len(result), 3)
        self.assertEqual(
            self.callbacks,
            [("probe", "absent"), ("sidecar", "absent"), ("sidecar", "managed-deny")],
        )
        self.assertEqual(self.node.stat().st_mode & 0o777, 0o777)
        state = json.loads(self.journal.read_text())
        self.assertTrue(state["fixtureCleanup"])
        self.assertTrue(state["runtimeEntryAttempted"])
        self.assertTrue(state["runtimeExecuted"])
        self.assertEqual(len(state["runtimeObservations"]), 3)
        self.assertEqual(state["phase"], "restored")
        self.assertEqual(state["children"], [])
        self.assertTrue(self.session.host.capture.closed)

    def test_unsettled_fake_result_retains_receipt_and_prepared_mode(self):
        def failed(*args, **kwargs):
            result = self.rehearsal(*args, **kwargs)
            result["driverWait"] = None
            return result

        with self.assertRaisesRegex(OSError, "child_settlement_unknown"):
            self.run_session(failed)
        state = json.loads(self.journal.read_text())
        self.assertEqual(state["phase"], "fake_entry_intent")
        self.assertFalse(state["fixtureCleanup"])
        self.assertEqual(state["children"], [-1])
        self.assertEqual(self.node.stat().st_mode & 0o777, 0o755)
        self.assertFalse(self.session.host.capture.closed)

    def test_unknown_rehearsal_retains_live_owner_and_outer_capture(self):
        owner = object()

        def failed(*args, **kwargs):
            error = RuntimeError("owned rehearsal unsettled")
            error.rehearsal = owner
            raise error

        with self.assertRaisesRegex(OSError, "child_settlement_unknown"):
            self.run_session(failed)
        self.assertIs(self.session.retained_rehearsal, owner)
        self.assertFalse(self.session.host.capture.closed)
        self.assertEqual(self.node.stat().st_mode & 0o777, 0o755)
        self.assertFalse(json.loads(self.journal.read_text())["fixtureCleanup"])

    def test_missing_native_wait_retains_real_inner_and_outer_custody(self):
        self.inner_wait_retention({}, admitted=True)

    def test_cancel_before_admission_retains_inner_and_outer_custody(self):
        self.inner_wait_retention(
            {"cancel": lambda phase: phase == "native-entry"}, admitted=False
        )

    def test_revocation_before_admission_retains_inner_and_outer_custody(self):
        def revoke(phase, lease, root):
            if phase == "native-entry":
                lease.revoke()

        self.inner_wait_retention({"custody_hook": revoke}, admitted=False)

    def test_pre_notification_refusal_retains_inner_and_outer_custody(self):
        self.inner_wait_retention(
            {"fault": "native-pre-notification", "mode": "probe"}, admitted=False
        )

    def inner_wait_retention(self, options, *, admitted):
        core = load(
            "sdk_inner_rehearsal", ROOT / "scripts/claude-agent-sdk-spawn-proposal.py"
        )
        model = load("sdk_inner_model", ROOT / "scripts/tests/sdk-runtime-model.py")
        original_module, parse = core.module, core.parse_events
        popen = subprocess.Popen
        node = Path(shutil.which("node")).resolve(strict=True)

        def modeled_module(name, data):
            selected = original_module(name, data)
            if name == "scripts/claude-agent-sdk-probe-custody.py":
                selected.runtime_inventory = lambda runtime, digest: model.observation(
                    selected, runtime, digest
                )
            return selected

        def withheld(data):
            return [
                row for row in parse(data) if row.get("probeEvent") != "native_wait"
            ]

        def inner(*args, **kwargs):
            # Compose lifetime boundaries only: the outer owned-fork host fixture
            # and the inner ordinary source-modeled Node peer are distinct.
            # This is neither candidate execution nor same-host qualification.
            self.rehearsal(*args, **kwargs)
            # run_session models the outer observer through the shared subprocess
            # module; restore real Popen for the inner owned source/fake peer.
            with mock.patch.object(core.subprocess, "Popen", popen):
                return core.rehearsal(
                    sha(ROOT / core.RECORD), node, sha(node), **options
                )

        owner = None
        try:
            with (
                mock.patch.object(core, "module", side_effect=modeled_module),
                mock.patch.object(core, "parse_events", side_effect=withheld),
            ):
                with self.assertRaisesRegex(OSError, "child_settlement_unknown"):
                    self.run_session(inner)
                owner = self.session.retained_rehearsal
                self.assertIsNotNone(owner)
                self.assertEqual(len(owner.registered), 1 if admitted else 0)
                self.assertIsNotNone(owner.driver.returncode)
                self.assertEqual(owner.launch["admitted"], admitted)
                self.assertNotEqual(owner.launch["state"], "settled")
                if not admitted:
                    self.assertFalse((owner.root / "native-payload-effect").exists())
                self.assertTrue(owner.retained)
                self.assertFalse(owner.lease.closed)
                self.assertTrue(owner.lease.revoked)
                self.assertTrue((owner.root / "native.mjs").is_file())
                for descriptor in owner.lease.descriptors:
                    os.fstat(descriptor)
                if owner.pidfd is not None:
                    os.fstat(owner.pidfd)
                self.assertFalse(self.session.host.capture.closed)
                self.assertEqual(self.node.stat().st_mode & 0o777, 0o755)
                self.assertFalse(json.loads(self.journal.read_text())["fixtureCleanup"])
        finally:
            if owner is not None:
                owner.settle()
        self.assertTrue(owner.lease.closed)
        self.assertFalse(owner.root.exists())
        # Settling the inner owner alone cannot clear an outer unknown receipt.
        self.assertFalse(self.session.host.capture.closed)
        self.assertEqual(self.node.stat().st_mode & 0o777, 0o755)
        self.assertFalse(json.loads(self.journal.read_text())["fixtureCleanup"])

    def test_unknown_actual_observer_wait_retains_custody_and_mode(self):
        original = self.observer

        def uncertain(*args, **kwargs):
            child = original(*args, **kwargs)
            child.wait = mock.Mock(
                side_effect=subprocess.TimeoutExpired("owned-fork", 2)
            )
            return child

        with mock.patch.object(self, "observer", side_effect=uncertain):
            with self.assertRaises((OSError, subprocess.TimeoutExpired)):
                self.run_session()
        self.assertEqual(self.node.stat().st_mode & 0o777, 0o755)
        self.assertFalse(json.loads(self.journal.read_text())["fixtureCleanup"])
        self.assertIsNotNone(self.session.child)
        self.assertFalse(self.session.host.capture.closed)

    def test_changed_path_never_restores_unbound_input(self):
        def drift(*args, **kwargs):
            result = self.rehearsal(*args, **kwargs)
            replacement = self.root / "replacement"
            replacement.write_bytes(b"unbound replacement")
            replacement.chmod(0o755)
            os.replace(replacement, self.node)
            return result

        with self.assertRaisesRegex(OSError, "candidate_descriptor_or_path_changed"):
            self.run_session(drift)
        self.assertEqual(self.node.stat().st_mode & 0o777, 0o755)
        self.assertFalse(json.loads(self.journal.read_text())["fixtureCleanup"])

    def test_changed_candidate_bytes_refuse_restore(self):
        def drift(*args, **kwargs):
            result = self.rehearsal(*args, **kwargs)
            self.node.write_bytes(b"changed owned input")
            return result

        with self.assertRaisesRegex(OSError, "candidate_descriptor_or_path_changed"):
            self.run_session(drift)
        self.assertEqual(self.node.stat().st_mode & 0o777, 0o755)
        self.assertFalse(json.loads(self.journal.read_text())["fixtureCleanup"])

    def test_changed_library_refuses_restore(self):
        def drift(*args, **kwargs):
            result = self.rehearsal(*args, **kwargs)
            self.library.write_bytes(b"changed owned library")
            return result

        with self.assertRaisesRegex(OSError, "custody_input_identity_changed"):
            self.run_session(drift)
        self.assertEqual(self.node.stat().st_mode & 0o777, 0o755)
        self.assertFalse(json.loads(self.journal.read_text())["fixtureCleanup"])

    def test_failed_durable_intent_never_mutates(self):
        with (
            mock.patch.object(
                self.session.helper,
                "persist",
                side_effect=OSError("intent unavailable"),
            ),
            mock.patch.object(self.session.helper.os, "fchmod") as mutate,
        ):
            with self.assertRaisesRegex(OSError, "intent unavailable"):
                self.run_session()
        mutate.assert_not_called()
        self.assertEqual(self.node.stat().st_mode & 0o777, 0o777)
        self.assertEqual(self.children, [])

    def test_null_manifest_refuses_before_mutation_or_observation(self):
        self.manifest["bootstrapInputs"] = None
        with mock.patch.object(self.session.helper.os, "fchmod") as mutate:
            with self.assertRaises((OSError, TypeError)):
                self.run_session()
        mutate.assert_not_called()
        self.assertEqual(self.node.stat().st_mode & 0o777, 0o777)
        self.assertFalse(self.journal.exists())

    def test_fresh_recovery_restores_clean_prepared_intent(self):
        self.session.preparation.prepare()
        self.assertEqual(self.node.stat().st_mode & 0o777, 0o755)
        self.session.preparation.close()
        recovered = S.Session(C, E, self.manifest, self.journal)
        self.addCleanup(recovered.preparation.close)
        result = recovered.recover()
        self.assertTrue(result["fixtureCleanup"])
        self.assertEqual(self.node.stat().st_mode & 0o777, 0o777)
        self.assertEqual(json.loads(self.journal.read_text())["phase"], "restored")

    def test_fresh_recovery_refuses_orphan_entry_without_touching_mode(self):
        self.session.preparation.prepare()
        self.session.preparation.state.update(
            phase="observer_entry_intent", children=[-1]
        )
        self.session.helper.persist(self.journal, self.session.preparation.state)
        self.session.preparation.close()
        recovered = S.Session(C, E, self.manifest, self.journal)
        self.addCleanup(recovered.preparation.close)
        with self.assertRaisesRegex(OSError, "child_settlement_unknown"):
            recovered.recover()
        self.assertEqual(self.node.stat().st_mode & 0o777, 0o755)
        state = json.loads(self.journal.read_text())
        self.assertEqual(state["phase"], "observer_entry_intent")
        self.assertFalse(state["fixtureCleanup"])


class HostInputTests(unittest.TestCase):
    def test_real_reader_accepts_exact_synthetic_graph_and_detects_alias_drift(self):
        with tempfile.TemporaryDirectory(
            prefix="sdk-host-input-", dir=os.environ["TMPDIR"]
        ) as root:
            root = Path(root).resolve()
            node, library, loader = (
                root / name for name in ("node", "liba.so", "loader")
            )
            alias = root / "loader-alias"
            alias.symlink_to(loader)
            node.write_bytes(ELF_TEST.elf(needed=("liba.so",), interpreter=str(alias)))
            library.write_bytes(ELF_TEST.elf(soname="liba.so"))
            loader.write_bytes(ELF_TEST.elf())
            node.chmod(0o777)
            supervisor = root / "supervisor"
            supervisor.write_bytes(b"owned supervisor binding")
            loader_inputs = {}
            if sys.platform == "linux" and Path("/etc/ld.so.cache").exists():
                loader_inputs["/etc/ld.so.cache"] = sha("/etc/ld.so.cache")
            manifest = {
                "path": str(node),
                "sha256": sha(node),
                "bytes": node.stat().st_size,
                "host": S.host_identity(),
                "bootstrapInputs": {
                    "libraries": {str(library): sha(library)},
                    "loaderInputs": loader_inputs,
                    "interpreters": {str(loader): sha(loader)},
                    "supervisorInputs": {str(supervisor): sha(supervisor)},
                },
                "resolutions": {
                    str(node): {"liba.so": str(library)},
                    str(library): {},
                    str(loader): {},
                },
                "aliases": {str(alias): str(loader)},
                "interpreterAliases": {str(alias): str(loader)},
                "lookup": {
                    "directories": {str(root): sorted(os.listdir(root))},
                    "absent": [str(root / "not-present")],
                    "environment": {
                        "PATH": "/usr/bin:/bin",
                        "LANG": "C",
                        "LC_ALL": "C",
                    },
                },
            }
            with mock.patch.object(C, "python_inputs", return_value={supervisor}):
                host = S.HostInputs(C, E, manifest)
            try:
                host.check()
                changed_host = {**manifest["host"], "bootId": "different-host"}
                with mock.patch.object(S, "host_identity", return_value=changed_host):
                    with self.assertRaisesRegex(OSError, "fresh_host_or_loader"):
                        host.check()
                alias.unlink()
                alias.symlink_to(library)
                with self.assertRaisesRegex(OSError, "loader_alias_changed"):
                    host.check()
            finally:
                host.close()


class SourcePacketTests(unittest.TestCase):
    def test_source_binding_and_drift_refuse_before_loading(self):
        with tempfile.TemporaryDirectory(
            prefix="sdk-source-packet-", dir=os.environ["TMPDIR"]
        ) as temporary:
            root = Path(temporary).resolve()
            source, record = root / "owned.py", root / "record.sha256"
            source.write_bytes(b"owned source bytes; never executed")
            record.write_text(sha(source) + "  owned.py\n")
            expected = sha(record)
            self.assertEqual(
                S.source_packet(record, expected, root=root),
                {"owned.py": source.read_bytes()},
            )
            with self.assertRaisesRegex(OSError, "source_record_changed"):
                S.source_packet(record, "0" * 64, root=root)
            source.write_bytes(b"changed source bytes")
            with self.assertRaisesRegex(OSError, "source_input_changed"):
                S.source_packet(record, expected, root=root)
            source.unlink()
            target = root / "target.py"
            target.write_bytes(b"owned source bytes; never executed")
            source.symlink_to(target)
            with self.assertRaisesRegex(OSError, "source_input_symlink"):
                S.source_packet(record, expected, root=root)
            record.write_text(sha(target) + "  ../outside.py\n")
            with self.assertRaisesRegex(OSError, "source_record_path"):
                S.source_packet(record, sha(record), root=root)


if __name__ == "__main__":
    unittest.main()
