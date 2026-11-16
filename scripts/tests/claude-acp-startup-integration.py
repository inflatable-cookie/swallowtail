"""Owned receipt/input counterexamples; no vendor or runtime evaluation."""

import copy
import importlib.util
import json
from pathlib import Path
import shutil
import sys
import tempfile
import unittest
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[2]
SCRIPTS = (
    "scripts/claude-acp-startup-integration.py",
    "scripts/tests/claude-acp-startup-integration.py",
)
spec = importlib.util.spec_from_file_location("startup", ROOT / SCRIPTS[0])
startup = importlib.util.module_from_spec(spec)
exec(compile(Path(spec.origin).read_bytes(), spec.origin, "exec"), startup.__dict__)


class StartupReceipt(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.reference = startup.decode(startup.bound_reference())
        cls.blobs = {(startup.REVIEWED_HEAD, startup.RECORD): startup.bound_reference()}
        ref = cls.reference
        for head, files in [
            (ref["acceptedShared"]["head"], ref["acceptedShared"]["files"]),
            (ref["baseHead"], ref["before"]),
            *[
                (ref[key]["head"], [ref[key]["path"]])
                for key in ("unchangedInvocation", "sourceFlow")
            ],
        ]:
            for name in files:
                cls.blobs[head, name] = startup.git_blob(head, name)

    def setUp(self):
        self.fixture = tempfile.TemporaryDirectory(prefix="acp-startup-receipt-")
        self.addCleanup(self.fixture.cleanup)
        self.root = Path(self.fixture.name)
        for name in [
            *self.reference["currentSources"],
            self.reference["unchangedInvocation"]["path"],
            self.reference["sourceFlow"]["path"],
        ]:
            target = self.root / name
            target.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(ROOT / name, target)
        self.blobs = self.blobs.copy()
        self.reader = patch.object(startup, "git_blob", self.git_blob)
        self.reader.start()
        self.addCleanup(self.reader.stop)

    def git_blob(self, head, name):
        try:
            return self.blobs[head, name]
        except KeyError as error:
            raise startup.Refused(f"missing_git_binding:{head}:{name}") from error

    def verify(self, value=None):
        return startup.verify(
            copy.deepcopy(self.reference) if value is None else value, self.root
        )

    def test_reviewed_receipt_and_reordered_json_pass_without_authority(self):
        record = startup.decode(json.dumps(self.reference, sort_keys=True).encode())
        self.assertEqual(self.verify(record)["admission"], "closed")
        self.assertEqual(
            self.verify(record)["reviewedReceiptSha256"], startup.REVIEWED_SHA
        )

    def test_every_field_value_deletion_and_extra_field_refuses(self):
        def mutations(value):
            if isinstance(value, dict):
                extra = copy.deepcopy(value)
                extra["unreviewed"] = True
                yield extra
                for key, item in value.items():
                    removed = copy.deepcopy(value)
                    del removed[key]
                    yield removed
                    for changed in mutations(item):
                        altered = copy.deepcopy(value)
                        altered[key] = changed
                        yield altered
            else:
                yield "changed" if value != "changed" else None

        for index, value in enumerate(mutations(self.reference)):
            with (
                self.subTest(mutation=index),
                self.assertRaisesRegex(startup.Refused, "receipt_content_drift"),
            ):
                self.verify(value)

    def test_boolean_integer_and_null_coercions_refuse(self):
        for key, replacement in [("arity", True), ("ambientForward", 0)]:
            value = copy.deepcopy(self.reference)
            value["lookup"][key] = replacement
            with self.assertRaises(startup.Refused):
                self.verify(value)
        value = copy.deepcopy(self.reference)
        value["limits"]["originalCommand"] = []
        with self.assertRaises(startup.Refused):
            self.verify(value)

    def test_each_current_source_and_synchronized_receipt_drift_refuses(self):
        for name in self.reference["currentSources"]:
            file = self.root / name
            original = file.read_bytes()
            file.write_bytes(original + b"\nchanged\n")
            value = copy.deepcopy(self.reference)
            value["currentSources"][name] = startup.digest(file.read_bytes())
            with (
                self.subTest(source=name),
                self.assertRaisesRegex(
                    startup.Refused, "source_binding_drift:currentSources"
                ),
            ):
                self.verify(value)
            file.write_bytes(original)

    def test_each_git_binding_drift_or_missing_refuses(self):
        for key, original in list(self.blobs.items()):
            for replacement in (None, original + b"changed"):
                if replacement is None:
                    del self.blobs[key]
                else:
                    self.blobs[key] = replacement
                with self.subTest(binding=key, missing=replacement is None):
                    with self.assertRaises(startup.Refused):
                        self.verify()
                self.blobs[key] = original

    def test_current_invocation_and_source_flow_drift_refuse(self):
        for key in ("unchangedInvocation", "sourceFlow"):
            file = self.root / self.reference[key]["path"]
            original = file.read_bytes()
            file.write_bytes(original + b"changed")
            with (
                self.subTest(binding=key),
                self.assertRaisesRegex(
                    startup.Refused, f"source_binding_drift:{key}:current"
                ),
            ):
                self.verify()
            file.write_bytes(original)

    def test_duplicate_keys_invalid_json_numbers_and_oversize_refuse(self):
        for raw in (
            b'{"lookup":{},"lookup":{}}',
            b'{"x":NaN}',
            b'{"x":Infinity}',
            b"{",
            b"x" * (startup.MAX_BYTES + 1),
        ):
            with self.subTest(raw=raw[:40]), self.assertRaises(startup.Refused):
                startup.decode(raw)

    def test_missing_symlink_and_oversize_owned_source_refuse(self):
        file = self.root / next(iter(self.reference["currentSources"]))
        original = file.read_bytes()
        file.unlink()
        with self.assertRaises(startup.Refused):
            self.verify()
        target = self.root / "owned-source"
        target.write_bytes(original)
        file.symlink_to(target)
        with self.assertRaises(startup.Refused):
            self.verify()
        file.unlink()
        file.write_bytes(b"x" * (startup.MAX_BYTES + 1))
        with self.assertRaises(startup.Refused):
            self.verify()


if __name__ == "__main__":
    if "--compile-only" in sys.argv:
        for name in SCRIPTS:
            compile((ROOT / name).read_bytes(), name, "exec")
        print("ACP startup receipt Python compile passed")
    else:
        unittest.main(verbosity=2)
