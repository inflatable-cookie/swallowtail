"""Synthetic image/ELF/cache/import inputs; no candidate or root tooling runs."""

import importlib.util
import base64
import os
import hashlib
import json
import types
from unittest import mock
from pathlib import Path
import struct
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[2]


def load(name, area="scripts"):
    spec = importlib.util.spec_from_file_location(name, ROOT / area / (name + ".py"))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


P = load("claude-agent-sdk-host-package")
E = load("claude-agent-sdk-bootstrap-elf")
F = load("claude-agent-sdk-bootstrap-elf", "scripts/tests")
HELP = "Shared library search path:\n  (libraries located via /etc/ld.so.cache)\n  /lib (system search path)\n\nSubdirectories of glibc-hwcaps directories, in priority order:\n  x86-64-v4 (supported, searched)\n  x86-64-v3 (supported, searched)\n  x86-64-v2 (supported, searched)\n"


def cache(rows):
    data = bytearray(48 + 24 * len(rows))
    data[:20] = b"glibc-ld.so.cache1.1"
    data[28] = 2
    for i, (name, target, hwcap) in enumerate(rows):
        key = len(data)
        data.extend(name.encode() + b"\0")
        value = len(data)
        data.extend(target.encode() + b"\0")
        struct.pack_into("<iIIIQ", data, 48 + i * 24, 0x303, key, value, 0, hwcap)
    struct.pack_into("<II", data, 20, len(rows), len(data) - 48 - 24 * len(rows))
    return bytes(data)


class PackageTests(unittest.TestCase):
    def fixture(self, root):
        for name in ("lib", "etc", "app", "python"):
            (root / name).mkdir(mode=0o700)
        (root / "lib/loader.so").write_bytes(F.elf(soname="loader.so"))
        (root / "lib/liba.so").write_bytes(F.elf(soname="liba.so"))
        (root / "app/owned").write_bytes(
            F.elf(needed=("liba.so",), interpreter="/lib/loader.so")
        )
        return P.ImageFiles(root=root, uid=os.getuid())

    def test_loader_search_policy_and_cache_structure_are_source_bound(self):
        self.assertEqual(P.loader_directories(HELP), ["/lib"])
        result = P.cache_entries(cache([("liba.so", "/lib/liba.so", 0)]))
        self.assertEqual(result, {"liba.so": [("/lib/liba.so", 0)]})
        for text in (
            HELP.replace("system search path", "LD_LIBRARY_PATH"),
            HELP.replace("x86-64-v4", "x86-64-v5"),
            HELP + "Legacy HWCAP directories",
            HELP + "  x86-64-v5 (supported, searched)\n",
            HELP.replace("x86-64-v3", "x86-64-v4"),
            HELP.replace("supported, searched", "supported, masked"),
        ):
            with self.subTest(text=text), self.assertRaises(P.Refused):
                P.loader_directories(text)
        for data in (
            b"",
            b"oldcache" + b"x" * 100,
            cache([])[:20],
            cache([("a", "/lib/a", 0)])[:-1],
        ):
            with self.assertRaises(P.Refused):
                P.cache_entries(data)

    def test_exact_transitive_graph_and_aliases_are_derived_without_exec(self):
        with tempfile.TemporaryDirectory(
            dir=os.environ["TMPDIR"], prefix="package-elf-"
        ) as temporary:
            root = Path(temporary).resolve()
            files = self.fixture(root)
            try:
                (root / "lib/alias.so").symlink_to("liba.so")
                graph = P.LoaderGraph(
                    files, E, {"liba.so": [("/lib/alias.so", 0)]}, ["/lib"]
                )
                objects, edges, interpreters = graph.graph("/app/owned")
                E.verify_graph(
                    "/app/owned",
                    {name: files.read(name)[1] for name in objects},
                    edges,
                    interpreter_paths=interpreters,
                )
                self.assertEqual(
                    objects, {"/app/owned", "/lib/liba.so", "/lib/loader.so"}
                )
                self.assertEqual(graph.aliases["/lib/alias.so"], "/lib/liba.so")
                files.check()
            finally:
                files.close()

    def test_ambiguous_hwcaps_preload_and_changed_lookup_refuse(self):
        for failure in ("ambiguity", "hwcap", "preload", "search-drift", "alias-drift"):
            with (
                self.subTest(failure=failure),
                tempfile.TemporaryDirectory(
                    dir=os.environ["TMPDIR"], prefix="package-refusal-"
                ) as temporary,
            ):
                root = Path(temporary).resolve()
                files = self.fixture(root)
                try:
                    entries = {"liba.so": [("/lib/liba.so", int(failure == "hwcap"))]}
                    if failure == "ambiguity":
                        (root / "lib/other.so").write_bytes(F.elf(soname="liba.so"))
                        entries["liba.so"].append(("/lib/other.so", 0))
                    if failure == "preload":
                        (root / "etc/ld.so.preload").write_bytes(b"/owned")
                    if failure in ("ambiguity", "hwcap", "preload"):
                        with self.assertRaises(P.Refused):
                            P.LoaderGraph(files, E, entries, ["/lib"]).graph(
                                "/app/owned"
                            )
                    elif failure == "search-drift":
                        graph = P.LoaderGraph(files, E, entries, ["/lib"])
                        graph.graph("/app/owned")
                        (root / "lib/new-entry").write_bytes(b"changed")
                        with self.assertRaisesRegex(P.Refused, "search_drift"):
                            files.check()
                    else:
                        (root / "lib/alias").symlink_to("liba.so")
                        files.read("/lib/alias")
                        (root / "lib/alias").unlink()
                        (root / "lib/alias").symlink_to("loader.so")
                        with self.assertRaisesRegex(P.Refused, "alias_drift"):
                            files.check()
                finally:
                    files.close()

    def test_first_party_dynamic_import_refuses_and_relative_literals_close(self):
        names = P.source_imports(
            "from . import sibling\nfrom ..parent import member", "owned.child"
        )
        self.assertTrue(
            {
                "owned.child",
                "owned.child.sibling",
                "owned.parent",
                "owned.parent.member",
            }
            <= names
        )
        with self.assertRaisesRegex(P.Refused, "dynamic_supervisor"):
            P.source_imports("__import__(caller_value)", supervisor=True)
        self.assertEqual(
            P.source_imports("__import__('encodings.utf_8')", supervisor=True),
            {"encodings.utf_8"},
        )

    def test_full_synthetic_package_is_produced_after_real_owned_ancestor_prepare(self):
        A = load("claude-agent-sdk-host-transaction")
        with tempfile.TemporaryDirectory(
            dir=os.environ["TMPDIR"], prefix="package-complete-"
        ) as temporary:
            root = Path(temporary).resolve()
            for name in A.DIRECTORIES:
                actual = root / name.lstrip("/")
                actual.mkdir(mode=0o777)
                actual.chmod(0o777)
            owner = A.AdministrativeOwner(
                root / "admin.json",
                {"fixture": True},
                inspect_inputs=lambda: None,
                root=root,
                uid=os.getuid(),
            )
            data = F.elf(needed=("liba.so",), interpreter="/lib/loader.so")
            candidate = root / P.CANDIDATE.lstrip("/")
            candidate.write_bytes(data)
            candidate.chmod(0o777)
            for directory in (
                "lib",
                "etc",
                "repo",
                "usr/bin",
                "usr/lib/python3.14/lib-dynload",
                "usr/lib/python3.14/encodings",
                "var/lib/dpkg",
                "lib64",
            ):
                (root / directory).mkdir(parents=True, exist_ok=True, mode=0o700)
            (root / "lib/loader.so").write_bytes(F.elf(soname="loader.so"))
            (root / "lib/liba.so").write_bytes(F.elf(soname="liba.so"))
            (root / "lib64/ld-linux-x86-64.so.2").symlink_to("../lib/loader.so")
            (root / "etc/ld.so.cache").write_bytes(
                cache([("liba.so", "/lib/liba.so", 0)])
            )
            for tool in (
                "python3.14",
                "dpkg-query",
                "timeout",
                "prlimit",
                "bash",
                "wc",
            ):
                (root / ("usr/bin/" + tool)).write_bytes(data)
            for module in (
                "encodings/__init__",
                "encodings/utf_8",
                "encodings/ascii",
                "encodings/latin_1",
                "_strptime",
            ):
                (root / ("usr/lib/python3.14/" + module + ".py")).write_bytes(
                    b"# owned source profile\n"
                )
            (root / "var/lib/dpkg/status").write_bytes(b"owned package database\n")
            (root / "repo/source.py").write_bytes(b"import sys\n")
            context = {
                "version": (3, 14),
                "cacheTag": "cpython-314",
                "cachePrefix": None,
                "optimize": 0,
                "search": [
                    "/usr/lib/python314.zip",
                    "/usr/lib/python3.14",
                    "/usr/lib/python3.14/lib-dynload",
                ],
                "executable": "/usr/bin/python3.14",
                "loaded": [],
            }
            session = types.SimpleNamespace(
                ROOT=Path("/repo"),
                host_identity=lambda: {
                    "uid": 0,
                    "system": "Linux",
                    "machine": "x86_64",
                    "bootId": "owned-only",
                },
            )

            def tool(command, **kwargs):
                if command[-1] == "--help":
                    return 0, HELP.encode()
                names = command[3:]
                return 0, "".join(
                    name + "\t1-owned\towned-source\t1-owned\tinstall ok installed\n"
                    for name in names
                ).encode()

            owner.prepare()
            package = None
            try:
                with (
                    mock.patch.object(owner, "tool", side_effect=tool),
                    mock.patch.object(P, "python_context", return_value=context),
                    mock.patch.multiple(
                        P,
                        CANDIDATE_SHA=hashlib.sha256(data).hexdigest(),
                        CANDIDATE_BYTES=len(data),
                    ),
                ):
                    package = P.produce(
                        owner,
                        session,
                        E,
                        {"source.py": b"import sys\n"},
                        root=root,
                        uid=os.getuid(),
                    )
                self.assertEqual(package.manifest["bytes"], len(data))
                self.assertFalse(package.manifest["originalActivation"])
                self.assertEqual(
                    package.manifest["resolutions"][P.CANDIDATE],
                    {"liba.so": "/lib/liba.so"},
                )
                self.assertIn(
                    "/usr/bin/python3.14",
                    package.manifest["bootstrapInputs"]["supervisorInputs"],
                )
                self.assertIn(
                    "/etc/ld.so.preload", package.manifest["lookup"]["absent"]
                )
                owner.check_inputs = package.check
                owner.restore()
            finally:
                if package is not None:
                    package.close()
                if owner.state["phase"] != "restored":
                    owner.check_inputs = lambda: None
                    owner.cancel_known_no_effect()
                    owner.restore()

    def test_partial_package_retains_inputs_until_owned_tool_wait_is_observed(self):
        import subprocess
        import sys

        A = load("claude-agent-sdk-host-transaction")
        with tempfile.TemporaryDirectory(
            dir=os.environ["TMPDIR"], prefix="package-tool-wait-"
        ) as temporary:
            root = Path(temporary).resolve()
            initial = self.fixture(root)
            initial.close()
            (root / "lib64").mkdir(mode=0o700)
            (root / "lib64/ld-linux-x86-64.so.2").symlink_to("../lib/loader.so")
            for name in A.DIRECTORIES:
                actual = root / name.lstrip("/")
                actual.mkdir(mode=0o777)
                actual.chmod(0o777)
            owner = A.AdministrativeOwner(
                root / "admin.json",
                {"fixture": True},
                inspect_inputs=lambda: None,
                root=root,
                uid=os.getuid(),
            )
            owner.prepare()
            actual_tool, actual_spawn = owner.tool, subprocess.Popen
            children = []

            def spawn(*args, **kwargs):
                child = actual_spawn(*args, **kwargs)
                child.wait = mock.Mock(
                    side_effect=subprocess.TimeoutExpired("owned wait withheld", 1)
                )
                children.append(child)
                return child

            def owned_tool(*args, **kwargs):
                return actual_tool(
                    [sys.executable, "-I", "-B", "-S", "-c", "pass"], environment=P.ENV
                )

            session = types.SimpleNamespace(ROOT=Path("/repo"))
            with (
                mock.patch.object(owner, "tool", side_effect=owned_tool),
                mock.patch.object(A.subprocess, "Popen", side_effect=spawn),
            ):
                with self.assertRaises(subprocess.TimeoutExpired) as error:
                    P.produce(owner, session, E, {}, root=root, uid=os.getuid())
            self.assertIs(error.exception.owner, owner)
            self.assertIsNotNone(owner.retained_inputs)
            for fd, *_ in owner.retained_inputs.files.values():
                os.fstat(fd)
            with self.assertRaisesRegex(A.Refused, "unknown_administrative_tool"):
                owner.restore()
            del children[0].wait
            owner.settle_tool()
            owner.check_inputs = owner.retained_inputs.check
            owner.restore()
            owner.retained_inputs.close()

    def test_python_cache_lookup_stays_inside_owned_image_and_checks_source(self):
        import importlib.util
        import marshal

        with tempfile.TemporaryDirectory(
            dir=os.environ["TMPDIR"], prefix="package-cache-"
        ) as temporary:
            root = Path(temporary).resolve()
            directory = root / "usr/lib/python3.14"
            (directory / "encodings/__pycache__").mkdir(parents=True)
            source = b"# owned source\n"
            for module in (
                "encodings/__init__",
                "encodings/utf_8",
                "encodings/ascii",
                "encodings/latin_1",
                "_strptime",
            ):
                (directory / (module + ".py")).write_bytes(source)
            cached = directory / "encodings/__pycache__/utf_8.cpython-314.pyc"
            cached.write_bytes(
                importlib.util.MAGIC_NUMBER
                + b"\0" * 12
                + marshal.dumps(compile(source, "owned-only", "exec"))
            )
            context = {
                "version": (3, 14),
                "cacheTag": "cpython-314",
                "cachePrefix": None,
                "optimize": 0,
                "search": [
                    "/usr/lib/python314.zip",
                    "/usr/lib/python3.14",
                    "/usr/lib/python3.14/lib-dynload",
                ],
                "loaded": [],
            }
            files = P.ImageFiles(root=root, uid=os.getuid())
            try:
                selected = P.python_closure(files, {}, context)
                self.assertIn(
                    "/usr/lib/python3.14/encodings/__pycache__/utf_8.cpython-314.pyc",
                    selected,
                )
                self.assertTrue(
                    all(
                        name.startswith(str(root) + "/") or name == str(root)
                        for name in files.directories
                    )
                )
            finally:
                files.close()
            cached.write_bytes(
                importlib.util.MAGIC_NUMBER
                + b"\0" * 12
                + marshal.dumps(compile(b"effect=1", "owned-only", "exec"))
            )
            files = P.ImageFiles(root=root, uid=os.getuid())
            try:
                with self.assertRaisesRegex(P.Refused, "cache_source_mismatch"):
                    P.python_closure(files, {}, context)
            finally:
                files.close()

    def test_input_mode_bytes_identity_and_partial_failure_close_owned_fds(self):
        with tempfile.TemporaryDirectory(
            dir=os.environ["TMPDIR"], prefix="package-input-"
        ) as temporary:
            root = Path(temporary).resolve()
            files = self.fixture(root)
            try:
                files.read("/lib/liba.so")
                fd = files.files["/lib/liba.so"][0]
                (root / "lib/liba.so").write_bytes(b"drift")
                with self.assertRaisesRegex(P.Refused, "input_drift"):
                    files.check()
                (root / "lib/loader.so").chmod(0o777)
                with self.assertRaisesRegex(P.Refused, "writable_input"):
                    files.read("/lib/loader.so")
            finally:
                files.close()
            with self.assertRaises(OSError):
                os.fstat(fd)


class OpenSSLEvidenceTests(unittest.TestCase):
    """Public-source-shaped bytes, never evidence of the failed host's config."""

    CONFIG = b"""# Shape from OpenSSL3.5.5 apps/openssl.cnf; owned fixture only.
# .include /etc/ssl/private/never.cnf
openssl_conf = openssl_init
[openssl_init]
providers = provider_sect
[provider_sect]
default = default_sect
[default_sect]
# activate = 1
.include = /etc/ssl/owned.d
"""

    def fixture(self, root):
        for name in (
            "etc/ssl/owned.d",
            "usr/lib/ssl",
            *[p.lstrip("/") for p in P.OPENSSL_SEARCH],
            "lib",
            "lib64",
        ):
            (root / name).mkdir(parents=True, exist_ok=True, mode=0o700)
        (root / "etc/ssl/openssl.cnf").write_bytes(self.CONFIG)
        (root / "usr/lib/ssl/openssl.cnf").symlink_to("../../../etc/ssl/openssl.cnf")
        (root / "etc/ssl/owned.d/extra.cnf").write_bytes(
            b"[owned_engine]\ndynamic_path = /usr/lib/x86_64-linux-gnu/engines-3/owned.so\n"
        )
        (root / "etc/ssl/owned.d/ignored.txt").write_bytes(b"not a config input")
        for item in (root / "etc/ssl").rglob("*.cnf"):
            item.chmod(0o644)
        (root / "lib/liba.so").write_bytes(F.elf(soname="liba.so"))
        (root / "lib/loader.so").write_bytes(F.elf(soname="loader.so"))
        (root / "lib64/ld-linux-x86-64.so.2").symlink_to("../lib/loader.so")
        (root / "etc/ld.so.cache").write_bytes(cache([("liba.so", "/lib/liba.so", 0)]))
        for folder, name in zip(P.OPENSSL_SEARCH, ("legacy.so", "owned.so")):
            (root / folder.lstrip("/") / name).write_bytes(
                F.elf(needed=("liba.so",), soname=name)
            )
        return P.ImageFiles(root=root, uid=os.getuid())

    def test_full_raw_roots_include_alias_and_module_candidates_are_data_only(self):
        with tempfile.TemporaryDirectory(
            prefix="openssl-owned-", dir=os.environ["TMPDIR"]
        ) as temporary:
            root = Path(temporary).resolve()
            files = self.fixture(root)
            try:
                with mock.patch(
                    "subprocess.Popen", side_effect=AssertionError("no execution")
                ):
                    report = P.openssl_evidence(files, E, ["/lib"])
                config = report["configurations"]["/etc/ssl/openssl.cnf"]
                self.assertEqual(base64.b64decode(config["rawBase64"]), self.CONFIG)
                self.assertEqual(
                    config["sha256"], hashlib.sha256(self.CONFIG).hexdigest()
                )
                self.assertEqual(
                    report["configurationAliases"]["/usr/lib/ssl/openssl.cnf"],
                    "/etc/ssl/openssl.cnf",
                )
                self.assertIn("/usr/lib/ssl/openssl.cnf", report["configurationLinks"])
                self.assertIn("/etc/ssl/owned.d/extra.cnf", report["configurations"])
                self.assertNotIn("/etc/ssl/private/never.cnf", report["paths"])
                for row in report["modules"].values():
                    self.assertEqual(row["state"], "present")
                    self.assertIn("/lib/liba.so", row["closure"])
                    self.assertNotIn("rawBase64", row)
                self.assertTrue(report["activationUnknown"])
                self.assertFalse(report["completeForAdmission"])
                self.assertFalse(report["runtimeExecuted"])
                self.assertLessEqual(len(json.dumps(report).encode()), 262144)
                files.check()
                (root / "etc/ssl/openssl.cnf").write_bytes(self.CONFIG + b"# drift\n")
                with self.assertRaisesRegex(P.Refused, "drift"):
                    files.check()
            finally:
                files.close()

    def test_missing_unknown_cycles_alias_and_bounds_remain_explicit(self):
        for failure in (
            "missing",
            "permission",
            "cycle",
            "alias",
            "environment",
            "module",
            "bound",
        ):
            with (
                self.subTest(failure=failure),
                tempfile.TemporaryDirectory(
                    prefix="openssl-refusal-", dir=os.environ["TMPDIR"]
                ) as temporary,
            ):
                root = Path(temporary).resolve()
                files = self.fixture(root)
                extra = root / "etc/ssl/owned.d/extra.cnf"
                if failure == "missing":
                    extra.unlink()
                    (root / "etc/ssl/openssl.cnf").write_bytes(
                        self.CONFIG + b".include /etc/ssl/absent.cnf\n"
                    )
                elif failure == "cycle":
                    extra.write_bytes(b".include /etc/ssl/openssl.cnf\n")
                elif failure == "alias":
                    extra.unlink()
                    extra.symlink_to("../../../lib/liba.so")
                elif failure == "environment":
                    extra.write_bytes(b".include $ENV::UNKNOWN/config.cnf\n")
                elif failure == "module":
                    (root / P.OPENSSL_SEARCH[0].lstrip("/") / "legacy.so").write_bytes(
                        b"not ELF"
                    )
                elif failure == "bound":
                    extra.write_bytes(b"x" * 65537)
                original = P.os.open

                def opening(name, *args, **kwargs):
                    if failure == "permission" and Path(name) == extra:
                        raise PermissionError(13, "owned denial")
                    return original(name, *args, **kwargs)

                try:
                    with mock.patch.object(P.os, "open", side_effect=opening):
                        report = P.openssl_evidence(files, E, ["/lib"])
                    self.assertTrue(report["unknown"])
                    self.assertIn("/etc/ssl/openssl.cnf", report["configurations"])
                    self.assertFalse(report["completeForAdmission"])
                    if failure == "missing":
                        # Directory includes select existing names only; bind its
                        # inventory and a separate explicit missing include too.
                        self.assertNotIn(
                            "extra.cnf", report["paths"]["/etc/ssl/owned.d"]["entries"]
                        )
                finally:
                    files.close()

    def test_producer_publishes_evidence_then_refuses_before_candidate_open(self):
        with tempfile.TemporaryDirectory(
            prefix="openssl-producer-", dir=os.environ["TMPDIR"]
        ) as temporary:
            root = Path(temporary).resolve()
            seed = self.fixture(root)
            seed.close()
            published = []
            owner = types.SimpleNamespace(
                prepared=lambda: None,
                unsettled=lambda: False,
                check_ancestors=lambda: None,
                tool=mock.Mock(return_value=(0, HELP.encode())),
            )
            session = types.SimpleNamespace(ROOT=Path("/repo"))
            original = P.ImageFiles.read

            def read(files, name, **kwargs):
                self.assertFalse(
                    kwargs.get("candidate"), "candidate must stay unopened"
                )
                return original(files, name, **kwargs)

            with mock.patch.object(P.ImageFiles, "read", read):
                with self.assertRaisesRegex(
                    P.Refused, "package_unreviewed_openssl_configuration"
                ):
                    P.produce(
                        owner,
                        session,
                        E,
                        {},
                        root=root,
                        uid=os.getuid(),
                        configuration_evidence=lambda report: published.append(
                            json.loads(json.dumps(report))
                        ),
                    )
            self.assertEqual(len(published), 1)
            self.assertTrue(published[0]["diagnosticOnly"])
            self.assertFalse(published[0]["qualified"])

    def test_evidence_only_mode_refuses_even_without_any_guard_marker(self):
        with tempfile.TemporaryDirectory(
            prefix="openssl-only-", dir=os.environ["TMPDIR"]
        ) as temporary:
            root = Path(temporary).resolve()
            seed = self.fixture(root)
            seed.close()
            (root / "etc/ssl/openssl.cnf").write_bytes(
                b"# providers = commented_only\n"
            )
            owner = types.SimpleNamespace(
                prepared=lambda: None,
                unsettled=lambda: False,
                check_ancestors=lambda: None,
                tool=mock.Mock(return_value=(0, HELP.encode())),
            )
            published = []
            original = P.ImageFiles.read

            def read(files, name, **kwargs):
                self.assertFalse(
                    kwargs.get("candidate"), "evidence-only cannot open candidate"
                )
                return original(files, name, **kwargs)

            with mock.patch.object(P.ImageFiles, "read", read):
                with self.assertRaisesRegex(P.Refused, "package_openssl_evidence_only"):
                    P.produce(
                        owner,
                        types.SimpleNamespace(ROOT=Path("/repo")),
                        E,
                        {},
                        root=root,
                        uid=os.getuid(),
                        configuration_evidence_only=True,
                        configuration_evidence=published.append,
                    )
            self.assertEqual(published[0]["guardMatches"], [])
            self.assertIn("/etc/ssl/openssl.cnf", published[0]["configurations"])

    def test_module_resolution_cannot_open_candidate_as_a_library(self):
        with tempfile.TemporaryDirectory(
            prefix="openssl-no-node-", dir=os.environ["TMPDIR"]
        ) as temporary:
            root = Path(temporary).resolve()
            files = self.fixture(root)
            node = root / P.CANDIDATE.lstrip("/")
            node.parent.mkdir(parents=True, mode=0o700)
            node.write_bytes(b"owned forbidden bytes")
            (root / "etc/ld.so.cache").write_bytes(cache([("liba.so", P.CANDIDATE, 0)]))
            # Remove the competing system candidate so resolution reaches this
            # exact forbidden edge rather than refusing only for ambiguity.
            (root / "lib/liba.so").unlink()
            original = P.os.open

            def opening(name, *args, **kwargs):
                self.assertNotEqual(Path(name), node)
                return original(name, *args, **kwargs)

            try:
                with mock.patch.object(P.os, "open", side_effect=opening):
                    report = P.openssl_evidence(files, E, ["/lib"], force=True)
                self.assertTrue(
                    any(
                        "candidate_forbidden" in row["reason"]
                        for row in report["unknown"]
                    )
                )
                self.assertNotIn(P.CANDIDATE, files.files)
            finally:
                files.close()


if __name__ == "__main__":
    unittest.main()
