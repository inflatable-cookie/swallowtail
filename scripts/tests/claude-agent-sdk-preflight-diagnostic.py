"""Owned read-only diagnostic fixtures; no candidate runtime is executed."""

import hashlib
import importlib.util
import io
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
SPEC = importlib.util.spec_from_file_location(
    "readonly_diagnostic", ROOT / "scripts/claude-agent-sdk-preflight-diagnostic.py"
)
D = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(D)


def packet_check():
    proposal = json.loads(
        (
            ROOT
            / "docs/knowledge/operations/claude-agent-sdk-preflight-diagnostic.json"
        ).read_text()
    )
    with tempfile.TemporaryDirectory(prefix="sdk234-packet-") as temporary:
        fixture = Path(temporary)
        for relative in (
            ".github/workflows",
            "scripts/tests",
        ):
            (fixture / relative).mkdir(parents=True)
        shutil.copytree(
            ROOT / ".github/workflows",
            fixture / ".github/workflows",
            dirs_exist_ok=True,
        )
        helper = fixture / "scripts/tests/oxide-selector-routing.py"
        helper.write_bytes(
            (ROOT / "scripts/tests/oxide-selector-routing.py").read_bytes()
        )
        assert (
            hashlib.sha256(helper.read_bytes()).hexdigest()
            == proposal["routingBaseSha256"]
        )
        for kind in ("workflow", "routing"):
            patch = ROOT / proposal[kind + "Patch"]
            assert (
                hashlib.sha256(patch.read_bytes()).hexdigest()
                == proposal[kind + "PatchSha256"]
            )
            subprocess.run(
                ["git", "apply", str(patch)],
                cwd=fixture,
                check=True,
                capture_output=True,
            )
        workflow = fixture / proposal["workflowPath"]
        assert (
            hashlib.sha256(workflow.read_bytes()).hexdigest()
            == proposal["workflowSha256"]
        )
        assert (
            hashlib.sha256(helper.read_bytes()).hexdigest()
            == proposal["routingPreparedSha256"]
        )
        subprocess.run(["actionlint", str(workflow)], check=True)
        subprocess.run(
            ["ruff", "format", "--check", "--no-cache", str(helper)], check=True
        )
        namespace = {
            "__name__": "owned_packet",
            "__file__": str(ROOT / "scripts/tests/oxide-selector-routing.py"),
        }
        exec(compile(helper.read_bytes(), str(helper), "exec"), namespace)
        proposal_path = (
            fixture / "docs/knowledge/operations/claude-agent-sdk-ephemeral-ci.patch"
        )
        proposal_path.parent.mkdir(parents=True)
        proposal_path.write_bytes(
            (ROOT / proposal_path.relative_to(fixture)).read_bytes()
        )
        namespace["ROOT"] = fixture
        namespace["WORKFLOWS"] = fixture / ".github/workflows"
        namespace["check_workflows"]()
        workflow.write_text(workflow.read_text() + "# changed\n")
        try:
            namespace["check_workflows"]()
        except namespace["RoutingDefect"]:
            pass
        else:
            raise AssertionError("changed diagnostic workflow admitted")
        workflow.unlink()
        (fixture / ".github/workflows/unrelated.yml").write_text("name: unrelated\n")
        try:
            namespace["check_workflows"]()
        except namespace["RoutingDefect"]:
            pass
        else:
            raise AssertionError("unrelated workflow exception admitted")
    print("unapplied read-only workflow/routing packet and changed-byte refusal passed")


class DiagnosticTests(unittest.TestCase):
    def setUp(self):
        ordinary_uid = mock.patch.object(D.os, "geteuid", return_value=1001)
        ordinary_uid.start()
        self.addCleanup(ordinary_uid.stop)
        self.temporary = tempfile.TemporaryDirectory(prefix="sdk234-diagnostic-test-")
        self.addCleanup(self.temporary.cleanup)
        self.base = Path(self.temporary.name)
        self.repo = self.base / "repo"
        self.repo.mkdir()
        self.scratch = self.base / "scratch"
        self.scratch.mkdir()
        self.node = self.base / "owned-candidate"
        self.marker = self.base / "must-not-exist"
        self.node.write_text("#!/bin/sh\ntouch '" + str(self.marker) + "'\n")
        self.node.chmod(0o755)
        for name in D.INPUTS:
            target = self.repo / name
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes((ROOT / name).read_bytes())
        record = self.repo / D.RECORD
        record.write_text(
            "".join(
                hashlib.sha256((self.repo / name).read_bytes()).hexdigest()
                + "  "
                + name
                + "\n"
                for name in D.INPUTS
            )
        )
        self.record = hashlib.sha256(record.read_bytes()).hexdigest()
        self.git("init", "--quiet")
        self.git("add", ".")
        self.git(
            "-c",
            "user.name=Owned fixture",
            "-c",
            "user.email=fixture@example.invalid",
            "commit",
            "--quiet",
            "-m",
            "owned",
        )
        self.source = self.git("rev-parse", "HEAD").decode().strip()
        self.cgroup = self.base / "cgroup-model"
        self.cgroup.mkdir()
        (self.cgroup / "cgroup.controllers").write_text("pids memory cpu\n")
        (self.cgroup / "cgroup.subtree_control").write_text("pids memory cpu\n")
        self.output = io.StringIO()

    def git(self, *args):
        return subprocess.check_output(
            ["/usr/bin/git", *args],
            cwd=self.repo,
            env={
                "PATH": "/usr/bin:/bin",
                "GIT_CONFIG_NOSYSTEM": "1",
                "GIT_CONFIG_GLOBAL": "/dev/null",
                "TMPDIR": os.environ["TMPDIR"],
            },
            stderr=subprocess.DEVNULL,
        )

    def diagnostic(self):
        return D.Diagnostic(
            self.repo,
            self.source,
            self.record,
            self.scratch,
            self.output,
            self.node,
            self.cgroup,
        )

    def run_diagnostic(self, diagnostic=None):
        code = (diagnostic or self.diagnostic()).run()
        rows = [json.loads(line) for line in self.output.getvalue().splitlines()]
        self.assertFalse(self.marker.exists())
        self.assertFalse(rows[-1]["hostAdmission"])
        self.assertFalse(rows[-1]["kernelEnforced"])
        self.assertFalse(rows[-1]["originalActivation"])
        return code, rows

    def refuses_at(self, phase, diagnostic=None):
        code, rows = self.run_diagnostic(diagnostic)
        self.assertEqual(code, 1)
        self.assertEqual(rows[-1]["failedPhase"], phase)
        self.assertEqual([row["phase"] for row in rows[:-1]], list(D.PHASES))
        index = D.PHASES.index(phase)
        self.assertTrue(all(row["status"] == "observed" for row in rows[:index]))
        self.assertEqual(rows[index]["status"], "refused")
        self.assertTrue(all(row["status"] == "skipped" for row in rows[index + 1 : -1]))
        return rows

    def test_missing_cached_executable_preserves_earlier_observations(self):
        self.node.unlink()
        rows = self.refuses_at("cached_executable")
        self.assertEqual(rows[3]["reason"], "cached_executable_unavailable")
        self.assertFalse(rows[-1]["reportProduced"])
        self.assertEqual(list(self.scratch.iterdir()), [])

    def test_nonexecutable_candidate_refuses_without_execution(self):
        self.node.chmod(0o644)
        self.refuses_at("cached_executable")

    def test_wrong_source_is_labelled(self):
        diagnostic = self.diagnostic()
        diagnostic.source = "0" * 40
        self.refuses_at("source_identity", diagnostic)

    def test_changed_source_refuses_before_later_phases(self):
        (self.repo / D.INPUTS[0]).write_text("changed")
        self.refuses_at("source_identity")

    def test_source_symlink_refuses(self):
        file = self.repo / D.INPUTS[0]
        data = file.read_bytes()
        file.unlink()
        target = self.base / "owned-source"
        target.write_bytes(data)
        file.symlink_to(target)
        self.refuses_at("source_identity")

    def test_fifo_record_refuses_without_blocking(self):
        file = self.repo / D.RECORD
        file.unlink()
        os.mkfifo(file)
        self.refuses_at("source_identity")

    def test_dirty_worktree_does_not_print_filename(self):
        (self.repo / "SECRET-DO-NOT-PRINT").write_text("owned")
        self.refuses_at("clean_worktree")
        self.assertNotIn("SECRET", self.output.getvalue())

    def test_scratch_failure_is_sanitized(self):
        with mock.patch.object(D.tempfile, "mkdtemp", side_effect=OSError("SECRET")):
            self.refuses_at("scratch_creation")
        self.assertNotIn("SECRET", self.output.getvalue())

    def test_missing_temporary_root_does_not_write_in_checkout(self):
        diagnostic = self.diagnostic()
        diagnostic.temporary = None
        self.refuses_at("scratch_creation", diagnostic)

    def test_dirty_git_output_bound_refuses_without_printing_paths(self):
        for index in range(350):
            (self.repo / ("SECRET-" + str(index) + "x" * 30)).touch()
        rows = self.refuses_at("clean_worktree")
        self.assertEqual(rows[1]["reason"], "git_output_bound")
        self.assertNotIn("SECRET", self.output.getvalue())

    def test_inventory_failure_retains_prior_phase_records(self):
        diagnostic = self.diagnostic()
        original = diagnostic.file_identity

        def fail_candidate(file, maximum=268435456):
            if file == self.node:
                raise OSError("SECRET")
            return original(file, maximum)

        with mock.patch.object(diagnostic, "file_identity", side_effect=fail_candidate):
            rows = self.refuses_at("inventory_production", diagnostic)
        self.assertFalse(rows[-1]["reportProduced"])
        self.assertNotIn("SECRET", self.output.getvalue())

    def test_report_write_failure_is_not_report_production(self):
        original = self.output.write

        def failed_write(raw):
            if '"phase": "inventory_production"' in raw and '"facts"' in raw:
                raise OSError("SECRET")
            return original(raw)

        with mock.patch.object(self.output, "write", side_effect=failed_write):
            rows = self.refuses_at("inventory_production")
        self.assertFalse(rows[-1]["reportProduced"])
        self.assertNotIn("SECRET", self.output.getvalue())

    def test_produced_inventory_is_not_successful_host_admission(self):
        rows = self.refuses_at("host_capabilities")
        self.assertTrue(rows[-1]["reportProduced"])
        self.assertFalse(rows[-1]["readOnlyChecksPassed"])
        self.assertFalse(rows[5]["facts"]["checks"]["nodeBytesMatch"])
        self.assertEqual(
            rows[5]["facts"]["cgroupParent"]["inode"], self.cgroup.stat().st_ino
        )
        self.assertEqual(
            rows[5]["facts"]["requiredControllersEnabled"], ["cpu", "memory", "pids"]
        )

    def test_missing_controller_record_is_a_host_phase_refusal(self):
        (self.cgroup / "cgroup.controllers").unlink()
        self.refuses_at("host_capabilities")

    def test_software_model_success_still_never_admits(self):
        diagnostic = self.diagnostic()
        with mock.patch.object(
            diagnostic,
            "host_capabilities",
            return_value={"readOnlyChecksPassed": True, "scope": "owned model only"},
        ):
            code, rows = self.run_diagnostic(diagnostic)
        self.assertEqual(code, 0)
        self.assertTrue(rows[-1]["reportProduced"])
        self.assertTrue(rows[-1]["readOnlyChecksPassed"])

    def test_oversized_report_refuses_and_keeps_previous_records(self):
        diagnostic = self.diagnostic()
        with mock.patch.object(
            diagnostic, "inventory_production", return_value={"owned": "x" * 5000}
        ):
            self.refuses_at("inventory_production", diagnostic)

    def test_deadline_refuses_without_running_later_phases(self):
        diagnostic = self.diagnostic()
        diagnostic.deadline = 0
        self.refuses_at("source_identity", diagnostic)

    def test_actual_cli_source_refusal_is_nonzero_sanitized_json(self):
        result = subprocess.run(
            [
                sys.executable,
                "-I",
                "-B",
                "-S",
                str(self.repo / D.INPUTS[0]),
                "--source-sha",
                "0" * 40,
                "--record-sha256",
                self.record,
            ],
            env={
                "PATH": "/usr/bin:/bin",
                "TMPDIR": str(self.scratch),
                "SECRET": "DO-NOT-PRINT",
            },
            capture_output=True,
            text=True,
            timeout=15,
        )
        self.assertEqual(result.returncode, 1)
        self.assertEqual(result.stderr, "")
        self.assertEqual(
            json.loads(result.stdout.splitlines()[0])["phase"], "source_identity"
        )
        self.assertNotIn("DO-NOT-PRINT", result.stdout)


if __name__ == "__main__":
    if sys.argv[1:] == ["--check-packet"]:
        packet_check()
    else:
        unittest.main()
