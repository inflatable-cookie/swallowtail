"""Owned refusal/callsite fixtures; vendor text is parsed only by the build selector."""

import base64
import copy
import importlib.util
import io
import json
from pathlib import Path
import subprocess
import sys
import tarfile
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[2]
spec = importlib.util.spec_from_file_location(
    "flow", ROOT / "scripts/claude-acp-source-flow.py"
)
flow = importlib.util.module_from_spec(spec)
exec(
    compile(
        (ROOT / "scripts/claude-acp-source-flow.py").read_bytes(),
        str(spec.origin),
        "exec",
    ),
    flow.__dict__,
)


def fixture(members):
    data = io.BytesIO()
    with tarfile.open(fileobj=data, mode="w:gz") as archive:
        for name, content, kind in members:
            item = tarfile.TarInfo(name)
            item.type = kind
            item.size = len(content) if kind == tarfile.REGTYPE else 0
            item.linkname = "outside"
            archive.addfile(item, io.BytesIO(content) if item.isfile() else None)
    raw = data.getvalue()
    return raw, {
        "sha256": flow.sha(raw),
        "integrity": "sha512-"
        + base64.b64encode(flow.hashlib.sha512(raw).digest()).decode(),
        "files": {"source.js": flow.sha(b"owned source")},
    }


class SourceFlow(unittest.TestCase):
    def test_archive_identity_and_selected_set(self):
        raw, expected = fixture(
            [("package/source.js", b"owned source", tarfile.REGTYPE)]
        )
        self.assertEqual(
            flow.archive_sources(raw, expected), {"source.js": b"owned source"}
        )
        with self.assertRaisesRegex(ValueError, "identity"):
            flow.archive_sources(raw + b"drift", expected)
        wrong = copy.deepcopy(expected)
        wrong["files"]["source.js"] = "00" * 32
        with self.assertRaisesRegex(ValueError, "regular-file identity"):
            flow.archive_sources(raw, wrong)
        missing = copy.deepcopy(expected)
        missing["files"]["missing.js"] = "00" * 32
        with self.assertRaisesRegex(ValueError, "incomplete"):
            flow.archive_sources(raw, missing)
        with self.assertRaisesRegex(ValueError, "excessive"):
            flow.archive_sources(raw, {**expected, "memberLimit": 0})

    def test_archive_link_traversal_duplicate_and_special_refuse(self):
        for name, kind in [
            ("../escape", tarfile.REGTYPE),
            ("/absolute", tarfile.REGTYPE),
            ("package/../escape", tarfile.REGTYPE),
            ("package/link", tarfile.SYMTYPE),
            ("package/hard", tarfile.LNKTYPE),
            ("package/fifo", tarfile.FIFOTYPE),
            ("package\\escape", tarfile.REGTYPE),
        ]:
            raw, expected = fixture(
                [
                    ("package/source.js", b"owned source", tarfile.REGTYPE),
                    (name, b"owned", kind),
                ]
            )
            with (
                self.subTest(name=name, kind=kind),
                self.assertRaisesRegex(ValueError, "unsafe"),
            ):
                flow.archive_sources(raw, expected)
        raw, expected = fixture(
            [("package/source.js", b"owned source", tarfile.REGTYPE)] * 2
        )
        with self.assertRaisesRegex(ValueError, "duplicate"):
            flow.archive_sources(raw, expected)

    def test_ast_call_aliases_do_not_execute_decoy_source(self):
        source = """
import { execFile as launch } from "node:child_process";
import { promisify as promise } from "node:util";
const run = promise(launch);
const decoy = "process.execve('not a call'); Function('not code')";
// launch('comment', []); process.binding('not an edge');
function actual(input) { return run("owned-command", [input], {cwd:"owned-cwd"}); }
const prototype = Function.prototype;
function doc() { const F = Function; return new F("owned text"); }
process["binding"]("owned binding name");
throw Error("this vendor-shaped fixture must never execute");
"""
        record = flow.inspect({"owned.js": source})["files"]["owned.js"]
        self.assertEqual(len(record["calls"]), 1)
        self.assertEqual(record["calls"][0]["alias"], "run")
        self.assertEqual(record["calls"][0]["arguments"][0]["text"], '"owned-command"')
        self.assertEqual(record["calls"][0]["owner"]["name"], "actual")
        self.assertEqual(
            [d["classification"] for d in record["dynamic"]],
            ["prototype-access-not-code-generation", "constructor-alias"],
        )
        self.assertEqual(
            record["dynamic"][1]["aliasCalls"][0]["text"], 'new F("owned text")'
        )
        self.assertEqual(
            [d["expression"]["text"] for d in record["raw"]], ['process["binding"]']
        )
        raw = source.encode()
        call = record["calls"][0]["expression"]
        self.assertEqual(
            flow.sha(raw[call["startByte"] : call["endByte"]]), call["sha256"]
        )

    def test_namespace_and_invalid_syntax_refuse(self):
        for source in [
            'import * as cp from "child_process"; cp.spawn("x");',
            "function {",
        ]:
            with (
                self.subTest(source=source),
                self.assertRaises(subprocess.CalledProcessError),
            ):
                flow.inspect({"owned.js": source})

    def test_input_bundle_roundtrip_and_missing_or_foreign_entries_refuse(self):
        # The bundle contains frozen archives as data, never extracted source.
        with tempfile.TemporaryDirectory(dir=flow.os.environ["TMPDIR"]) as directory:
            file = Path(directory) / "inputs.tgz"
            raw, _ = fixture([("owned.tgz", b"owned data", tarfile.REGTYPE)])
            file.write_bytes(raw)
            self.assertEqual(
                flow.bundle_archives([file], {"owned": {}}), {"owned": b"owned data"}
            )
            for packages in ({"missing": {}}, {"owned": {}, "missing": {}}):
                with self.assertRaises(ValueError):
                    flow.bundle_archives([file], packages)
            with self.assertRaises(ValueError):
                flow.bundle_archives([file, file], {"owned": {}})

    def test_create_require_alias_calls_are_bound_as_data(self):
        record = flow.inspect(
            {
                "owned.js": 'import {createRequire as make} from "node:module"; const req=make(import.meta.url); req("fs"); req.resolve("owned-native");'
            }
        )["files"]["owned.js"]
        selected = [
            d
            for d in record["dynamic"]
            if d["classification"]
            in ("caller-bound-createRequire", "captured-require-or-resolve")
        ]
        self.assertEqual(
            [d["expression"]["text"] for d in selected],
            ["make(import.meta.url)", 'req("fs")', 'req.resolve("owned-native")'],
        )

    def test_owned_startup_optional_call_and_function_fallback(self):
        # Owned miniature semantics, not an SDK/Zod import or original attempt.
        source = """
const assert = (ok) => { if (!ok) throw Error("owned fixture failed"); };
const globalThis = { process: { getBuiltinModule() { throw Error("refused"); } } };
let fallback = false, refused = false;
try { const module = globalThis.process?.getBuiltinModule?.("owned-name"); fallback = !module; }
catch (e) { refused = e.message === "refused"; }
assert(refused && !fallback);
let allowsEval;
try { new Function(""); allowsEval = true; } catch { allowsEval = false; }
assert(allowsEval === false);
const shape = allowsEval ? "generated" : "interpreter";
assert(shape === "interpreter");
console.log("owned optional-call refusal and codegen fallback passed");
"""
        result = subprocess.run(
            [
                "node",
                "--disallow-code-generation-from-strings",
                "--no-addons",
                "--input-type=module",
                "-e",
                source,
            ],
            text=True,
            capture_output=True,
            check=True,
            timeout=10,
        )
        self.assertIn("owned optional-call refusal", result.stdout)

    def test_record_removal_policy_and_admission_mutations_refuse(self):
        expected = {
            "calls": [
                {"argv": ["auth", "status", "--json"], "sourceSha256": "aa" * 32}
            ],
            "policies": flow.POLICIES,
            "originalCommand": None,
            "admission": "closed",
        }
        flow.verify(copy.deepcopy(expected), expected)
        for change in [
            lambda r: r["calls"].clear(),
            lambda r: r["calls"][0].update(argv=["--cli"]),
            lambda r: r["calls"][0].update(sourceSha256="00" * 32),
            lambda r: r["policies"]["sdk-other-exec"].update(classification="allowed"),
            lambda r: r["policies"]["Function"].update(startup="relaxed"),
            lambda r: r.update(originalCommand=["vendor"]),
            lambda r: r.update(admission="open"),
        ]:
            mutated = copy.deepcopy(expected)
            change(mutated)
            with self.assertRaises(ValueError):
                flow.verify(mutated, expected)


if __name__ == "__main__":
    if "--compile-only" in sys.argv:
        for name in (
            "scripts/claude-acp-source-flow.py",
            "scripts/tests/claude-acp-source-flow.py",
        ):
            compile((ROOT / name).read_bytes(), name, "exec")
        subprocess.run(
            ["node", "--check", str(ROOT / "scripts/claude-acp-call-sites.cjs")],
            check=True,
        )
        print("Python compile and static Node syntax checks passed")
    else:
        unittest.main(verbosity=2)
