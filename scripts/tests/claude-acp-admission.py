"""Mutation oracles for a source-only proposal; no vendor/runtime execution."""

import copy
import importlib.util
import json
import os
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch
from types import SimpleNamespace

ROOT = Path(__file__).resolve().parents[2]
spec = importlib.util.spec_from_file_location(
    "admission", ROOT / "scripts/claude-acp-admission.py"
)
admission = importlib.util.module_from_spec(spec)
exec(
    compile(
        (ROOT / "scripts/claude-acp-admission.py").read_bytes(),
        str(ROOT / "scripts/claude-acp-admission.py"),
        "exec",
    ),
    admission.__dict__,
)


class AdmissionRecord(unittest.TestCase):
    def setUp(self):
        self.record = admission.build()

    def test_exact_record_is_source_only(self):
        admission.verify(self.record)
        self.assertEqual(len(self.record["hops"]), 8)
        self.assertIsNone(self.record["originalCommand"])
        self.assertFalse(self.record["pythonInputs"]["admitted"])
        # Even an exact preparation record supplies no execution/admission API.
        self.assertEqual(self.record["admission"], "closed")

    def test_independent_missing_extra_reclassified_and_digest_mutations_refuse(self):
        def corrupt(label, change):
            value = copy.deepcopy(self.record)
            change(value)
            with self.subTest(label=label), self.assertRaises(ValueError):
                admission.verify(value)

        corrupt("missing caller", lambda v: v["hops"][0]["executionImportEdges"].pop())
        corrupt(
            "extra caller",
            lambda v: v["hops"][1]["executionImportEdges"].append(
                {"parent": "unreviewed.js", "target": "node:child_process"}
            ),
        )
        corrupt(
            "reclassified ambient process",
            lambda v: v["hops"][2]["executionImportEdges"][-1].update(
                classification="prohibited"
            ),
        )
        corrupt("graph digest", lambda v: v["hops"][3].update(graphSha256="00" * 32))
        corrupt(
            "native digest",
            lambda v: v["hops"][4]["nativeImages"]["darwin-arm64"].update(
                sha256="00" * 32
            ),
        )
        corrupt(
            "dynamic caution removed", lambda v: v["hops"][5]["dynamicCautions"].pop()
        )
        corrupt(
            "raw process risk hidden",
            lambda v: v["edgeClasses"].update(rawProcess="sandboxed"),
        )
        corrupt(
            "fake recipe transferred",
            lambda v: v["edgeClasses"].update(
                childProcess=v["edgeClasses"]["fakeRecipe"]
            ),
        )
        corrupt(
            "incorrect original sibling graph",
            lambda v: v["compositionDelta"]["actors"][2].update(parent="group-owner"),
        )
        corrupt(
            "shared packet silently changed",
            lambda v: v["compositionDelta"].update(frozenPacketMutation=True),
        )
        corrupt("missing independent gate", lambda v: v["remainingGates"].pop())
        corrupt(
            "invented original command",
            lambda v: v.update(originalCommand=["node", "upstream.js"]),
        )
        corrupt("claim raised", lambda v: v.update(qualifiedCeiling="0.106.0"))
        corrupt(
            "Python source missing",
            lambda v: v["pythonInputs"]["machOFiles"].pop(
                next(iter(v["pythonInputs"]["machOFiles"]))
            ),
        )
        corrupt(
            "Python source substituted",
            lambda v: next(iter(v["pythonInputs"]["machOFiles"].values())).update(
                sha256="00" * 32
            ),
        )
        corrupt(
            "Python cache rule weakened",
            lambda v: v["pythonInputs"].update(bytecodeRule="-B prevents all reads"),
        )
        corrupt(
            "Python custody marker", lambda v: v["pythonInputs"].update(admitted=True)
        )
        corrupt(
            "source record changed",
            lambda v: v["inputs"].update(
                {"scripts/claude-acp-admission.py": "00" * 32}
            ),
        )

    def test_framework_self_identity_and_unknown_dependency_are_distinct(self):
        with tempfile.TemporaryDirectory(
            prefix="acp-python-layout.", dir=os.environ["TMPDIR"]
        ) as directory:
            framework = Path(directory) / "Python3.framework/Versions/3.9"
            exe = framework / "Resources/Python.app/Contents/MacOS/Python"
            exe.parent.mkdir(parents=True)
            exe.write_bytes(b"owned fake executable metadata")
            (framework / "Python3").write_bytes(b"owned fake framework metadata")
            (framework / "lib/python3.9/http").mkdir(parents=True)
            (framework / "lib/python3.9/http/__init__.py").write_bytes(b"owned package")
            (framework / "lib/python3.9/http/client.py").write_bytes(
                b"owned direct module"
            )
            own_id = "@rpath/Python3.framework/Versions/3.9/Python3"

            def run(command, **kwargs):
                flag, file = command[1:]
                if flag == "-D":
                    body = own_id if file.endswith("/Python3") else ""
                elif file.endswith("/Python3"):
                    body = "\t" + own_id + " (compatibility version 3.9.0)\n"
                else:
                    body = "\t@executable_path/../../../../Python3 (compatibility version 3.9.0)\n"
                return SimpleNamespace(stdout=body.encode(), stderr=b"")

            with (
                patch.object(admission.platform, "system", return_value="Darwin"),
                patch.object(admission.platform, "machine", return_value="arm64"),
                patch.object(admission.subprocess, "run", side_effect=run),
            ):
                value = admission.inspect_python(exe)
            self.assertEqual(len(value["machOFiles"]), 2)
            self.assertEqual(
                set(value["directImportCandidates"]["http.client"]["sources"]),
                {"lib/python3.9/http/__init__.py", "lib/python3.9/http/client.py"},
            )
            self.assertEqual(len(value["machOLinks"]), 1)
            self.assertFalse(any(isinstance(row, dict) for row in value["unresolved"]))
            self.assertFalse(value["admitted"])

            def unknown(command, **kwargs):
                result = run(command, **kwargs)
                if command[1] == "-L" and command[2].endswith("/Python3"):
                    result.stdout += (
                        b"\t@rpath/unreviewed.dylib (compatibility version 1.0.0)\n"
                    )
                return result

            with (
                patch.object(admission.platform, "system", return_value="Darwin"),
                patch.object(admission.platform, "machine", return_value="arm64"),
                patch.object(admission.subprocess, "run", side_effect=unknown),
            ):
                value = admission.inspect_python(exe)
            self.assertTrue(
                any(
                    isinstance(row, dict)
                    and row["gate"] == "UNRESOLVED_PYTHON_MACHO_SEARCH"
                    for row in value["unresolved"]
                )
            )
            self.assertFalse(value["admitted"])

    def test_duplicate_record_keys_refuse(self):
        with tempfile.TemporaryDirectory(
            prefix="acp-admission-mutation.", dir=os.environ["TMPDIR"]
        ) as directory:
            file = Path(directory) / "duplicate.json"
            file.write_text('{"admission":"closed","admission":"closed"}')
            with self.assertRaises(ValueError):
                admission.read_json(file)


if __name__ == "__main__":
    unittest.main(verbosity=2)
