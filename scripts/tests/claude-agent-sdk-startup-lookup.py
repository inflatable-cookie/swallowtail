"""First-party startup guard regressions. Vendor contexts remain JSON data only."""

import base64
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[2]
RECEIPT = ROOT / "docs/knowledge/operations/claude-agent-sdk-startup-source.json"
PATCH = ROOT / "docs/knowledge/operations/claude-agent-sdk-startup-migration.patch"
LOOKUP = ROOT / "scripts/claude-agent-sdk-startup-lookup.mjs"
OWNED = ROOT / "scripts/tests/fixtures/sdk-startup-probes.mjs"
FLOW = (
    ROOT
    / "crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-agent-sdk-native-proof/selected-flow.json"
)


def sha(data):
    return hashlib.sha256(data).hexdigest()


def data_url(source):
    return "data:text/javascript;base64," + base64.b64encode(source.encode()).decode()


def rust_preload(source, loader):
    """Expand only Rust doubled braces and the one declared loader placeholder."""
    result, index, replacements = [], 0, 0
    while index < len(source):
        if source.startswith("{{", index):
            result.append("{")
            index += 2
        elif source.startswith("}}", index):
            result.append("}")
            index += 2
        elif source.startswith("{loader_json}", index):
            result.append(json.dumps(loader))
            replacements += 1
            index += len("{loader_json}")
        else:
            if source[index] in "{}":
                raise ValueError("unsupported Rust format field")
            result.append(source[index])
            index += 1
    if replacements != 1:
        raise ValueError("one loader field required")
    return "".join(result)


def guard_replacements(patch):
    """Read the unapplied Rust hunk changes as text, without applying a patch."""
    selected = False
    removed, added, changes = [], [], []

    def flush():
        if removed or added:
            changes.append(("\n".join(removed), "\n".join(added)))
            removed.clear()
            added.clear()

    for line in patch.splitlines():
        if line.startswith("diff --git "):
            flush()
            selected = line.endswith(
                " b/crates/swallowtail-host-local/src/captured_source.rs"
            )
        elif not selected or line.startswith(("---", "+++")):
            continue
        elif line.startswith("-"):
            removed.append(line[1:])
        elif line.startswith("+"):
            added.append(line[1:])
        else:
            flush()
    flush()
    return changes


class SourceReceiptTests(unittest.TestCase):
    def setUp(self):
        self.receipt = json.loads(RECEIPT.read_bytes())

    def test_exact_four_sdk_identities_and_retained_module_scope_spans(self):
        flow = json.loads(FLOW.read_bytes())
        starts = {
            "0.3.318": [176698, 260441],
            "0.3.319": [177342, 262549],
            "0.3.321": [177972, 268137],
            "0.3.322": [177972, 268137],
        }
        self.assertEqual(set(self.receipt["sdk"]), set(starts))
        self.assertEqual(
            self.receipt["upstream"]["commit"],
            "81c27b9d2756842d9f1db3f3203c5f7b40474adb",
        )
        self.assertEqual(
            self.receipt["upstream"]["path"],
            "docs/knowledge/operations/claude-agent-acp-source-flow.json",
        )
        self.assertEqual(
            self.receipt["upstream"]["sha256"],
            "e57249e1b0a12a4d6785b7e4c27c5d0c5e4d8ba5164d21130f908182b4f0506d",
        )
        for version, offsets in starts.items():
            with self.subTest(version=version):
                retained, frozen = (
                    self.receipt["sdk"][version],
                    flow["identity"][version],
                )
                self.assertEqual(retained["sdkMjsSha256"], frozen["sdk_mjs_sha256"])
                self.assertEqual(
                    retained["tarballSha256"], frozen["tarball_sha256_frozen"]
                )
                self.assertEqual(len(retained["sites"]), 2)
                for site, offset in zip(retained["sites"], offsets):
                    expression = site["expression"]
                    self.assertEqual(site["owner"], {"name": "<module>"})
                    self.assertEqual(
                        expression["text"], "globalThis.process?.getBuiltinModule"
                    )
                    self.assertEqual(
                        expression["sha256"], sha(expression["text"].encode())
                    )
                    self.assertEqual(expression["startByte"], offset)
                    self.assertEqual(
                        expression["endByte"], offset + len(expression["text"].encode())
                    )
                    self.assertLess(expression["endByte"], frozen["sdk_mjs_bytes"])
                    # DATA ONLY: retained ownership/reachability came from SDK239.
                    self.assertEqual(site["context"].count(expression["text"]), 1)
                    self.assertIn('getBuiltinModule?.("async_hooks")', site["context"])
                self.assertIn("AsyncResource.bind", retained["sites"][0]["context"])
                self.assertIn("AsyncLocalStorage", retained["sites"][1]["context"])
                self.assertIn("getStore:()=>{return}", retained["sites"][1]["context"])

    def test_unapplied_patch_and_helper_are_exact_receipt_inputs(self):
        self.assertEqual(sha(PATCH.read_bytes()), self.receipt["migrationPatchSha256"])
        self.assertEqual(sha(LOOKUP.read_bytes()), self.receipt["lookupSha256"])
        self.assertTrue(self.receipt["independentReviewRequired"])
        self.assertFalse(self.receipt["consumable"])
        self.assertFalse(self.receipt["originalExecution"])
        self.assertEqual(
            self.receipt["guard"]["sha256"],
            "e75caa2819f12d2653a97153c276259c7ca3f0d961241007ec15372a57f50dbf",
        )
        self.assertEqual(self.receipt["guard"]["bytes"], 23089)
        migrated = self.receipt["guard"]["originalPreloadRust"]
        changes = guard_replacements(PATCH.read_text())
        self.assertEqual(len(changes), 2)
        for before, after in changes:
            self.assertTrue(before)
            self.assertEqual(migrated.count(before), 1)
            migrated = migrated.replace(before, after)
        self.assertEqual(migrated, self.receipt["guard"]["proposedPreloadRust"])
        loader_patch = PATCH.read_text().split(
            "diff --git a/crates/swallowtail-adapter-claude-agent/sidecar/claude-agent-acp-guard-loader.mjs",
            1,
        )[1]
        additions = "\n".join(
            line[1:]
            for line in loader_patch.splitlines()
            if line.startswith("+") and not line.startswith("+++")
        )
        self.assertEqual(additions.strip(), LOOKUP.read_text().strip())

    def test_rust_expansion_refuses_unexpected_fields_or_missing_loader(self):
        for source in ("{{}}", "{unknown}{loader_json}", "{loader_json}{loader_json}"):
            with self.subTest(source=source), self.assertRaises(ValueError):
                rust_preload(source, "data:owned")


class OwnedStartupTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.node = Path(shutil.which("node")).resolve(strict=True)
        cls.receipt = json.loads(RECEIPT.read_bytes())

    def execute(self, guard, *, helper=None):
        # Only this owned source-shaped fixture is executable. Receipt contexts
        # and all vendor files remain inert data throughout these tests.
        fixture = OWNED.read_text()
        ambient = """
globalThis.sdkOwnedAmbientLookupCalls = 0;
Object.defineProperty(process, 'getBuiltinModule', {
  value() { globalThis.sdkOwnedAmbientLookupCalls += 1; throw new Error('ambient lookup reached'); },
  writable: true, configurable: true
});
"""
        loader = (
            ambient
            + "\nexport function install(data) {\n"
            + "if(data.owned!==true) throw new Error('invalid capture');\n"
            + fixture
            + "\nconsole.log('owned_startup_ok');\n}\n"
            + (helper or "")
        )
        preload = rust_preload(self.receipt["guard"][guard], data_url(loader))
        with tempfile.TemporaryDirectory(
            prefix="sdk-startup-", dir=os.environ["TMPDIR"]
        ) as temporary:
            root = Path(temporary)
            entry = root / "owned-entry.mjs"
            entry.write_text(
                "// Owned empty entry; the first-party install stub runs the fixture.\n"
            )
            env = {key: str(root) for key in ("HOME", "TMPDIR", "TMP", "TEMP")}
            env.update(PATH="/usr/bin:/bin", LANG="C", LC_ALL="C")
            child = subprocess.Popen(
                [
                    str(self.node),
                    "--no-addons",
                    "--openssl-config=/dev/null",
                    "--disallow-code-generation-from-strings",
                    "--import",
                    data_url(preload),
                    str(entry),
                ],
                cwd=root,
                env=env,
                stdin=subprocess.PIPE,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
            )
            raw = json.dumps({"owned": True}, separators=(",", ":")).encode()
            try:
                stdout, stderr = child.communicate(
                    len(raw).to_bytes(4, "big") + raw + b"\xa5", timeout=5
                )
            finally:
                if child.poll() is None:
                    child.kill()
                child.wait(timeout=2)
                child.stdin.close()
                child.stdout.close()
                child.stderr.close()
            self.assertLessEqual(len(stdout) + len(stderr), 16384)
            return child.returncode, stdout, stderr

    def test_original_guard_refuses_first_optional_probe_before_effect(self):
        status, stdout, stderr = self.execute("originalPreloadRust")
        self.assertEqual(status, 78, stderr)
        self.assertEqual(stdout, b"")
        self.assertEqual(stderr, b"guard bootstrap refused\n")

    def test_migrated_generic_guard_without_opt_in_still_refuses(self):
        status, stdout, stderr = self.execute("proposedPreloadRust")
        self.assertEqual(status, 78, stderr)
        self.assertEqual(stdout, b"")
        self.assertEqual(stderr, b"guard bootstrap refused\n")

    def test_exact_opt_in_preserves_fallbacks_and_denies_every_other_lookup(self):
        status, stdout, stderr = self.execute(
            "proposedPreloadRust", helper=LOOKUP.read_text()
        )
        self.assertEqual(status, 0, stderr)
        self.assertEqual(stdout, b"owned_startup_ok\n")
        self.assertEqual(stderr, b"")

    def test_loader_export_returning_builtin_object_is_refused(self):
        status, stdout, stderr = self.execute(
            "proposedPreloadRust",
            helper="export function sdkStartupBuiltinLookup() { return process; }\n",
        )
        self.assertEqual(status, 78, stderr)
        self.assertEqual(stdout, b"")
        self.assertEqual(stderr, b"guard bootstrap refused\n")


if __name__ == "__main__":
    unittest.main()
