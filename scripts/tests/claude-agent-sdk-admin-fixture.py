"""Rehearse proposed administrator-fixture ownership under owned scratch only."""

import ast
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import stat
import re
import shutil
import subprocess
import tempfile
import unittest

SPEC = importlib.util.spec_from_file_location(
    "admin_fixture",
    Path(__file__).resolve().parents[1] / "claude-agent-sdk-admin-fixture.py",
)
FIXTURE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(FIXTURE)


class FixtureTests(unittest.TestCase):
    def setUp(self):
        self.scratch = tempfile.TemporaryDirectory(
            prefix="sdk234-admin-rehearsal-", dir=os.environ["TMPDIR"]
        )
        self.root = Path(self.scratch.name).resolve(strict=True)
        self.owner = os.getuid()

    def tearDown(self):
        # This test created the entire root; no canonical host path is used.
        self.scratch.cleanup()

    def prepare(self, state="managed-deny", name="claude-code"):
        return FIXTURE.prepare(self.root, name, state, self.owner, "owned-test-run")

    def cleanup(self, state="managed-deny", name="claude-code", run="owned-test-run"):
        FIXTURE.cleanup(self.root, name, state, self.owner, run)

    def test_all_states_on_both_directory_spellings_leave_only_owned_sibling(self):
        sibling = self.root / "owned-sibling"
        sibling.write_bytes(b"preserve")
        for name in ("claude-code", "ClaudeCode"):
            for state, setting in FIXTURE.STATES.items():
                with self.subTest(name=name, state=state):
                    lease = self.prepare(state, name)
                    folder = self.root / name
                    self.assertEqual(folder.stat().st_uid, self.owner)
                    self.assertEqual(stat.S_IMODE(folder.stat().st_mode), 0o755)
                    self.assertEqual(lease["inode"], folder.stat().st_ino)
                    policy = folder / FIXTURE.POLICY
                    if state == "absent":
                        self.assertFalse(policy.exists())
                    else:
                        self.assertEqual(policy.stat().st_uid, self.owner)
                        self.assertEqual(stat.S_IMODE(policy.stat().st_mode), 0o444)
                        self.assertIs(
                            json.loads(policy.read_bytes())["permissions"][
                                "denyInsteadOfPrompting"
                            ],
                            setting,
                        )
                    self.cleanup(state, name)
                    self.assertFalse(folder.exists())
                    self.assertEqual(sibling.read_bytes(), b"preserve")

    def test_existing_directory_is_never_adopted(self):
        folder = self.root / "claude-code"
        folder.mkdir()
        (folder / "owned-sentinel").write_bytes(b"preserve")
        with self.assertRaises(FileExistsError):
            self.prepare()
        self.assertEqual((folder / "owned-sentinel").read_bytes(), b"preserve")

    def test_symlink_parent_and_leaf_refuse(self):
        alias = self.root / "alias"
        os.symlink(self.root, alias)
        with self.assertRaisesRegex(FIXTURE.Refused, "symlink_parent"):
            FIXTURE.prepare(
                alias, "claude-code", "absent", self.owner, "owned-test-run"
            )
        os.symlink(self.root, self.root / "claude-code")
        with self.assertRaises(FileExistsError):
            self.prepare()

    def test_wrong_owner_and_writable_parent_refuse_before_creation(self):
        with self.assertRaisesRegex(FIXTURE.Refused, "parent_owner_or_mode"):
            FIXTURE.prepare(self.root, "claude-code", "absent", self.owner + 1, "run")
        self.root.chmod(0o777)
        try:
            with self.assertRaisesRegex(FIXTURE.Refused, "parent_owner_or_mode"):
                self.prepare()
        finally:
            self.root.chmod(0o700)
        self.assertFalse((self.root / "claude-code").exists())

    def test_unknown_state_and_path_escape_refuse_before_creation(self):
        for name, state in (("../escape", "absent"), ("claude-code", "unknown")):
            with self.assertRaises(FIXTURE.Refused):
                FIXTURE.prepare(self.root, name, state, self.owner, "run")
        self.assertEqual(list(self.root.iterdir()), [])

    def test_changed_policy_and_permissions_refuse_without_deleting_lease(self):
        self.prepare()
        policy = self.root / "claude-code" / FIXTURE.POLICY
        original = policy.read_bytes()
        policy.chmod(0o644)
        with self.assertRaisesRegex(FIXTURE.Refused, "file_owner_type_mode_or_links"):
            self.cleanup()
        policy.write_bytes(b"{}\n")
        policy.chmod(0o444)
        with self.assertRaisesRegex(FIXTURE.Refused, "policy_substitution"):
            self.cleanup()
        self.assertTrue(policy.with_name(FIXTURE.LEASE).exists())
        policy.chmod(0o644)
        policy.write_bytes(original)
        policy.chmod(0o444)
        self.cleanup()

    def test_extra_entries_and_wrong_run_refuse_without_any_deletion(self):
        self.prepare()
        folder = self.root / "claude-code"
        with self.assertRaisesRegex(FIXTURE.Refused, "lease_mismatch"):
            self.cleanup(run="foreign-run")
        extra = folder / "unexpected"
        extra.write_bytes(b"preserve")
        before = set(folder.iterdir())
        with self.assertRaisesRegex(FIXTURE.Refused, "unexpected_entries"):
            self.cleanup()
        self.assertEqual(set(folder.iterdir()), before)
        extra.unlink()
        self.cleanup()

    def test_policy_symlink_and_hardlink_refuse(self):
        self.prepare()
        policy = self.root / "claude-code" / FIXTURE.POLICY
        owned_copy = self.root / "owned-copy"
        owned_copy.write_bytes(policy.read_bytes())
        owned_copy.chmod(0o444)
        policy.unlink()
        os.symlink(owned_copy, policy)
        with self.assertRaises(OSError):
            self.cleanup()
        policy.unlink()
        os.link(owned_copy, policy)
        with self.assertRaisesRegex(FIXTURE.Refused, "file_owner_type_mode_or_links"):
            self.cleanup()
        owned_copy.unlink()
        self.cleanup()

    def test_directory_replacement_cannot_reuse_a_lease(self):
        self.prepare()
        folder = self.root / "claude-code"
        held = self.root / "owned-held"
        folder.rename(held)
        folder.mkdir(mode=0o755)
        folder.chmod(0o755)
        (folder / FIXTURE.LEASE).write_bytes((held / FIXTURE.LEASE).read_bytes())
        (folder / FIXTURE.LEASE).chmod(0o600)
        with self.assertRaisesRegex(FIXTURE.Refused, "lease_mismatch"):
            self.cleanup()
        (folder / FIXTURE.LEASE).unlink()
        folder.rmdir()
        held.rename(folder)
        self.cleanup()

    def test_proposed_job_commands_compile_without_activation(self):
        root = Path(__file__).resolve().parents[2]
        proposal = (
            root / "docs/knowledge/operations/claude-agent-sdk-ephemeral-ci.patch"
        )
        subprocess.run(
            ["git", "apply", "--check", str(proposal)],
            cwd=root,
            capture_output=True,
            text=True,
            check=True,
            timeout=5,
        )
        lines = [
            line[1:]
            for line in proposal.read_text().splitlines()
            if line.startswith("+") and not line.startswith("+++")
        ]
        commands = []
        for index, line in enumerate(lines):
            if line == "        run: |":
                block = []
                for nested in lines[index + 1 :]:
                    if not nested.startswith("          "):
                        break
                    block.append(nested[10:])
                commands.append("\n".join(block) + "\n")
        self.assertEqual(len(commands), 3)
        python_blocks = 0
        javascript_blocks = 0
        for index, command in enumerate(commands):
            script = self.root / f"proposed-{index}.sh"
            script.write_text(command)
            subprocess.run(["bash", "-n", str(script)], check=True, timeout=5)
            for block in re.findall(r"<<'PY'\n(.*?)\nPY\n", command, re.S):
                ast.parse(block)
                python_blocks += 1
            for block in re.findall(r"<<'JS'\n(.*?)\nJS\n", command, re.S):
                script = self.root / "proposed-reader.mjs"
                script.write_text(block)
                subprocess.run(
                    [shutil.which("node"), "--check", str(script)],
                    check=True,
                    timeout=5,
                )
                javascript_blocks += 1
        self.assertEqual(python_blocks, 3)
        self.assertEqual(javascript_blocks, 1)

    def test_manifest_binds_proposal_without_changing_confinement_record(self):
        root = Path(__file__).resolve().parents[2]
        inputs = (
            "scripts/claude-agent-sdk-admin-fixture.py",
            "scripts/tests/claude-agent-sdk-admin-fixture.py",
            "effigy.toml",
            "docs/knowledge/operations/claude-agent-sdk-ephemeral-ci-proposal.md",
            "docs/knowledge/operations/claude-agent-sdk-ephemeral-ci.patch",
            "crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-agent-sdk-native-proof/confined-execution.sha256",
        )
        expected = "".join(
            f"{hashlib.sha256((root / name).read_bytes()).hexdigest()}  {name}\n"
            for name in inputs
        )
        record = root / "docs/knowledge/operations/claude-agent-sdk-ephemeral-ci.sha256"
        self.assertEqual(record.read_text(), expected)
        self.assertEqual(
            hashlib.sha256((root / inputs[-1]).read_bytes()).hexdigest(),
            "aa24aac1b8d8a223d83982103c2e18823877822911215eb3c3c30dcf695d08cb",
        )


if __name__ == "__main__":
    unittest.main()
