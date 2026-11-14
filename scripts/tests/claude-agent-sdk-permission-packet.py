"""Apply inactive permission helper only in fresh owned fixtures."""

import hashlib
import importlib.util
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest
from unittest import mock

ROOT = Path(__file__).resolve().parents[2]
PATCH = ROOT / "docs/knowledge/operations/claude-agent-sdk-runtime-permissions.patch"
WORKFLOW_PATCH = (
    ROOT
    / "docs/knowledge/operations/claude-agent-sdk-runtime-permissions-workflow.patch"
)


def extract(patch):
    return (
        "\n".join(
            line[1:]
            for line in patch.read_text().splitlines()
            if line.startswith("+") and not line.startswith("+++")
        )
        + "\n"
    )


def rewrite(patch, name, content):
    patch.write_text(
        "diff --git a/"
        + name
        + " b/"
        + name
        + "\nnew file mode 100644\n--- /dev/null\n+++ b/"
        + name
        + "\n@@ -0,0 +1,"
        + str(len(content.splitlines()))
        + " @@\n"
        + "".join("+" + line + "\n" for line in content.splitlines())
    )


def format_packet(write=False):
    with tempfile.TemporaryDirectory(prefix="sdk-permission-format-") as temporary:
        file = Path(temporary) / "helper.py"
        file.write_text(extract(PATCH))
        subprocess.run(
            [
                "ruff",
                "format",
                "--no-cache",
                *([] if write else ["--check"]),
                str(file),
            ],
            check=True,
        )
        if write:
            rewrite(
                PATCH,
                "scripts/claude-agent-sdk-runtime-permissions.py",
                file.read_text(),
            )
        workflow = Path(temporary) / "workflow.yml"
        workflow.write_text(extract(WORKFLOW_PATCH))
        subprocess.run(["actionlint", str(workflow)], check=True)


class PermissionTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix="sdk-permission-")
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name).resolve()
        spec_data = json.loads(
            (
                ROOT
                / "docs/knowledge/operations/claude-agent-sdk-runtime-permissions.json"
            ).read_text()
        )
        self.assertFalse(spec_data["applied"])
        self.assertFalse(spec_data["activationAuthorized"])
        self.assertEqual(
            hashlib.sha256(PATCH.read_bytes()).hexdigest(),
            spec_data["helperPatchSha256"],
        )
        self.assertEqual(
            hashlib.sha256(extract(PATCH).encode()).hexdigest(),
            spec_data["helperSha256"],
        )
        self.assertEqual(
            hashlib.sha256(WORKFLOW_PATCH.read_bytes()).hexdigest(),
            spec_data["workflowPatchSha256"],
        )
        self.assertEqual(
            hashlib.sha256(extract(WORKFLOW_PATCH).encode()).hexdigest(),
            spec_data["workflowSha256"],
        )
        subprocess.run(
            ["git", "apply", str(PATCH)], cwd=self.root, check=True, capture_output=True
        )
        spec = importlib.util.spec_from_file_location(
            "owned_permission",
            self.root / "scripts/claude-agent-sdk-runtime-permissions.py",
        )
        self.helper = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(self.helper)
        spec = importlib.util.spec_from_file_location(
            "permission_custody", ROOT / "scripts/claude-agent-sdk-probe-custody.py"
        )
        self.custody = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(self.custody)
        self.node = self.root / "owned-node"
        self.node.write_bytes(b"owned fake, never executable runtime")
        self.node.chmod(0o777)
        self.library = self.root / "library"
        self.library.write_bytes(b"owned fake library")
        sha = lambda p: hashlib.sha256(Path(p).read_bytes()).hexdigest()
        loader = {}
        if sys.platform == "linux" and Path("/etc/ld.so.cache").exists():
            loader["/etc/ld.so.cache"] = sha("/etc/ld.so.cache")
        supervisor = self.root / "owned-supervisor-input"
        supervisor.write_bytes(b"owned supervisor fixture")
        patch = mock.patch.object(
            self.custody, "python_inputs", return_value={supervisor}
        )
        patch.start()
        self.addCleanup(patch.stop)
        self.manifest = {
            "path": str(self.node),
            "sha256": sha(self.node),
            "bytes": self.node.stat().st_size,
            "originalActivation": False,
            "bootstrapInputs": {
                "libraries": {str(self.library): sha(self.library)},
                "loaderInputs": loader,
                "interpreters": {},
                "supervisorInputs": {
                    str(p): sha(p) for p in self.custody.python_inputs()
                },
            },
        }
        self.journal = self.root / "receipt.json"
        self.prep = self.helper.Preparation(self.custody, self.manifest, self.journal)
        self.addCleanup(self.release)

    def release(self):
        for child in self.prep.children:
            if child.poll() is None:
                child.kill()
            child.wait(timeout=2)
        if self.prep.fd is not None:
            os.close(self.prep.fd)
            self.prep.fd = None
        if self.prep.capture is not None:
            self.prep.capture.close()

    def test_intent_precedes_mode_change_and_rollback_is_checked(self):
        original = self.helper.os.fchmod

        def check(fd, mode):
            state = json.loads(self.journal.read_text())
            if mode == 0o755:
                self.assertEqual(state["phase"], "intent")
            return original(fd, mode)

        with mock.patch.object(self.helper.os, "fchmod", side_effect=check):
            self.prep.prepare()
            self.assertEqual(self.node.stat().st_mode & 0o777, 0o755)
            result = self.prep.cleanup()
        self.assertTrue(result["fixtureCleanup"])
        self.assertEqual(self.node.stat().st_mode & 0o777, 0o777)

    def test_missing_library_prerequisite_and_unsafe_ancestor_never_mutate(self):
        self.manifest["bootstrapInputs"] = None
        with self.assertRaises(OSError):
            self.prep.prepare()
        self.assertEqual(self.node.stat().st_mode & 0o777, 0o777)
        self.assertFalse(self.journal.exists())

    def test_changed_bytes_or_unexpected_mode_never_mutate(self):
        self.node.write_bytes(b"changed")
        with self.assertRaises(OSError):
            self.prep.prepare()
        self.assertFalse(self.journal.exists())

    def test_failed_durable_intent_never_calls_fchmod(self):
        with (
            mock.patch.object(
                self.helper, "persist", side_effect=OSError("owned intent failure")
            ),
            mock.patch.object(self.helper.os, "fchmod") as change,
        ):
            with self.assertRaises(OSError):
                self.prep.prepare()
        change.assert_not_called()
        self.assertEqual(self.node.stat().st_mode & 0o777, 0o777)
        self.assertFalse(self.journal.exists())

    def test_fault_after_fchmod_preserves_intent_and_fresh_recovery(self):
        original = self.helper.persist

        def fail(file, value):
            if value["phase"] == "prepared":
                raise OSError("owned persistence failure")
            return original(file, value)

        with mock.patch.object(self.helper, "persist", side_effect=fail):
            with self.assertRaises(OSError):
                self.prep.prepare()
        self.assertEqual(json.loads(self.journal.read_text())["phase"], "intent")
        self.release()
        self.prep.capture = None
        recovered = self.helper.Preparation(self.custody, self.manifest, self.journal)
        try:
            self.assertTrue(recovered.cleanup()["fixtureCleanup"])
        finally:
            recovered.close()
        self.assertEqual(self.node.stat().st_mode & 0o777, 0o777)

    def test_fault_before_fchmod_and_unknown_replacement_never_touch_unbound_leaf(self):
        with mock.patch.object(
            self.helper.os, "fchmod", side_effect=OSError("owned fault")
        ):
            with self.assertRaises(OSError):
                self.prep.prepare()
        self.node.unlink()
        self.node.write_bytes(b"replacement")
        self.node.chmod(0o777)
        with self.assertRaises(OSError):
            self.prep.cleanup()
        self.assertEqual(
            json.loads(self.journal.read_text())["phase"], "cleanup_unknown"
        )
        self.assertFalse(json.loads(self.journal.read_text())["fixtureCleanup"])
        self.assertEqual(self.node.read_bytes(), b"replacement")

    def test_actual_owned_child_is_waited_before_permission_release(self):
        self.prep.prepare()
        child = subprocess.Popen(
            [sys.executable, "-I", "-B", "-S", "-c", "import time;time.sleep(30)"],
            env={"PATH": "/usr/bin:/bin", "TMPDIR": os.environ["TMPDIR"]},
        )
        self.prep.attach_child(child)
        result = self.prep.cleanup()
        self.assertEqual(result["ownedWaits"][0]["pid"], child.pid)
        with self.assertRaises(ProcessLookupError):
            os.kill(child.pid, 0)
        self.assertTrue(result["fixtureCleanup"])

    def test_library_and_ancestor_drift_refuse_before_mode_mutation(self):
        self.library.chmod(0o777)
        with self.assertRaises(OSError):
            self.prep.prepare()
        self.assertFalse(self.journal.exists())
        self.assertEqual(self.node.stat().st_mode & 0o777, 0o777)

    def test_unexpected_mode_retains_unknown_receipt_without_rollback(self):
        self.prep.prepare()
        self.node.chmod(0o600)
        with self.assertRaises(OSError):
            self.prep.cleanup()
        state = json.loads(self.journal.read_text())
        self.assertEqual(state["phase"], "cleanup_unknown")
        self.assertFalse(state["fixtureCleanup"])
        self.assertEqual(self.node.stat().st_mode & 0o777, 0o600)

    def test_orphan_child_receipt_and_unknown_cleanup_preserve_failure(self):
        self.prep.prepare()
        state = json.loads(self.journal.read_text())
        state["children"] = [12345]
        self.helper.persist(self.journal, state)
        recovered = self.helper.Preparation(self.custody, self.manifest, self.journal)
        with self.assertRaisesRegex(OSError, "child_settlement_unknown"):
            recovered.cleanup()
        self.assertEqual(self.node.stat().st_mode & 0o777, 0o755)


if __name__ == "__main__":
    if sys.argv[1:] == ["--format-packet"]:
        format_packet(True)
    elif sys.argv[1:] == ["--check-format"]:
        format_packet()
    elif sys.argv[1:] == ["--check-compile"]:
        compile(extract(PATCH), "inactive-permission-helper.py", "exec")
    else:
        unittest.main()
