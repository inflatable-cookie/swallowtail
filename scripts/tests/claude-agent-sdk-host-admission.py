"""Owned image-chain fixtures only; no host admission, candidate or repair."""

import importlib.util
import io
import json
import hashlib
import os
from pathlib import Path
import tempfile
import unittest
from unittest import mock

ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location(
    "sdk_host_admission", ROOT / "scripts/claude-agent-sdk-host-admission.py"
)
A = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(A)


class AdmissionTests(unittest.TestCase):
    def fixture(self, root):
        current = root
        for part in Path(A.CANDIDATE).parent.parts[1:]:
            current = current / part
            current.mkdir(mode=0o700)
        return root / "opt/hostedtoolcache"

    def test_safe_fixture_holds_parents_without_opening_candidate(self):
        with tempfile.TemporaryDirectory(
            dir=os.environ["TMPDIR"], prefix="host-ancestry-"
        ) as temporary:
            root = Path(temporary).resolve()
            self.fixture(root)
            # No candidate exists: no bytes are opened by this prerequisite gate.
            held = A.Ancestry(A.IMAGE, root=root, uid=os.getuid())
            fds = [row[2] for row in held.rows]
            self.assertEqual(len(fds), 7)
            held.check()
            held.close()
            held.close()
            for fd in fds:
                with self.assertRaises(OSError):
                    os.fstat(fd)

    def test_published_0777_cache_refuses_before_candidate_and_closes_partial_capture(
        self,
    ):
        with tempfile.TemporaryDirectory(
            dir=os.environ["TMPDIR"], prefix="host-unsafe-"
        ) as temporary:
            root = Path(temporary).resolve()
            cache = self.fixture(root)
            cache.chmod(0o777)
            opened, original = [], A.os.open

            def capture(*args, **kwargs):
                fd = original(*args, **kwargs)
                opened.append(fd)
                return fd

            with mock.patch.object(A.os, "open", side_effect=capture):
                with self.assertRaises(A.Refused) as caught:
                    A.Ancestry(A.IMAGE, root=root, uid=os.getuid())
            self.assertEqual(
                (caught.exception.code, caught.exception.name, caught.exception.mode),
                ("unsafe_ancestor", "/opt/hostedtoolcache", "0777"),
            )
            self.assertEqual(len(opened), 3)
            self.assertEqual(cache.stat().st_mode & 0o777, 0o777)
            for fd in opened:
                with self.assertRaises(OSError):
                    os.fstat(fd)

    def test_image_drift_refuses_before_open(self):
        for key in A.IMAGE:
            with self.subTest(key=key), mock.patch.object(A.os, "open") as opened:
                with self.assertRaisesRegex(A.Refused, "selected_image_mismatch"):
                    A.Ancestry({**A.IMAGE, key: "different"})
                opened.assert_not_called()

    def test_alias_and_directory_replacement_refuse(self):
        with tempfile.TemporaryDirectory(
            dir=os.environ["TMPDIR"], prefix="host-alias-"
        ) as temporary:
            root = Path(temporary).resolve()
            cache = self.fixture(root)
            held = A.Ancestry(A.IMAGE, root=root, uid=os.getuid())
            try:
                cache.rename(cache.with_name("old-cache"))
                cache.mkdir(mode=0o700)
                with self.assertRaisesRegex(A.Refused, "ancestor_identity_changed"):
                    held.check()
            finally:
                held.close()
            cache.rmdir()
            cache.symlink_to(cache.with_name("old-cache"), target_is_directory=True)
            with self.assertRaisesRegex(A.Refused, "ancestor_alias_or_open_refused"):
                A.Ancestry(A.IMAGE, root=root, uid=os.getuid())

    def test_owner_and_metadata_failure_release_exact_owned_descriptors(self):
        for failure in ("owner", "fstat"):
            with self.subTest(failure=failure):
                with tempfile.TemporaryDirectory(
                    dir=os.environ["TMPDIR"], prefix="host-partial-"
                ) as temporary:
                    root = Path(temporary).resolve()
                    self.fixture(root)
                    opened, original_open, original_stat = [], A.os.open, A.os.fstat

                    def capture(*args, **kwargs):
                        fd = original_open(*args, **kwargs)
                        opened.append(fd)
                        return fd

                    def observe(fd):
                        if failure == "fstat":
                            raise OSError("owned fixture metadata failure")
                        values = list(original_stat(fd))
                        values[4] = os.getuid() + 999
                        return os.stat_result(values)

                    with (
                        mock.patch.object(A.os, "open", side_effect=capture),
                        mock.patch.object(A.os, "fstat", side_effect=observe),
                    ):
                        with self.assertRaises(OSError):
                            A.Ancestry(A.IMAGE, root=root, uid=os.getuid())
                    self.assertEqual(len(opened), 1)
                    for fd in opened:
                        with self.assertRaises(OSError):
                            original_stat(fd)

    def test_profile_and_unapplied_alternative_keep_exact_scope_and_authority(self):
        profile = json.loads(
            (
                ROOT / "docs/knowledge/operations/claude-agent-sdk-host-package.json"
            ).read_text()
        )
        self.assertEqual(profile["image"], A.IMAGE)
        self.assertEqual(
            profile["sourceBasis"]["commit"], "93a4e4e98715026b60fdf86893cf193b4be3bd47"
        )
        self.assertEqual(
            profile["activationPacket"],
            "docs/knowledge/operations/claude-agent-sdk-host-setup.json",
        )
        self.assertFalse(profile["activationAuthorized"])
        self.assertFalse(profile["hostPackageProduced"])
        packet = json.loads((ROOT / profile["activationPacket"]).read_text())
        self.assertFalse(packet["activationAuthorized"])
        self.assertFalse(packet["qualified"])
        self.assertIsNone(packet["originalCommand"])
        self.assertEqual(packet["originalAdmission"], "closed")
        self.assertEqual(packet["providerAdmission"], "closed")
        self.assertEqual(packet["qualificationMergeHolds"], "both retained")
        self.assertEqual(
            [effect["phase"] for effect in packet["reservedEffects"]],
            ["application", "registration", "one_attempt"],
        )
        for effect in packet["reservedEffects"]:
            self.assertTrue(effect["authorityRequired"])
        alternative = profile["unappliedAlternative"]
        patch = (ROOT / alternative["path"]).read_bytes()
        self.assertEqual(hashlib.sha256(patch).hexdigest(), alternative["sha256"])
        contract = json.loads(
            "\n".join(
                row[1:]
                for row in patch.decode().splitlines()
                if row.startswith("+") and not row.startswith("+++")
            )
        )
        expected = [
            str(p)
            for p in reversed(Path(A.CANDIDATE).parent.parents)
            if str(p).startswith("/opt/hostedtoolcache")
        ]
        expected.append(str(Path(A.CANDIDATE).parent))
        self.assertEqual(contract["ancestorObjects"], expected)
        self.assertEqual(len(expected), 5)
        self.assertFalse(contract["activationAuthorized"])
        self.assertFalse(contract["originalAdmission"])
        self.assertTrue(contract["qualificationMergeHold"])

    def test_report_is_bounded_and_grants_no_package_or_runtime_credit(self):
        output = io.StringIO()
        A.emit(output, {"state": "unsupported", "impediment": "unsafe_ancestor"})
        self.assertIn('"hostPackageProduced": false', output.getvalue())
        self.assertIn('"candidateOpened": false', output.getvalue())
        before = output.getvalue()
        with self.assertRaisesRegex(A.Refused, "admission_output_bound"):
            A.emit(output, {"unexpected": "x" * A.MAX_OUTPUT})
        self.assertEqual(output.getvalue(), before)


if __name__ == "__main__":
    unittest.main()
