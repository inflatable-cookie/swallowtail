"""Synthetic image/ELF/cache/import inputs; no candidate or root tooling runs."""

import importlib.util
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
            (root / "opt").mkdir(mode=0o755)
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
            (root / "opt").mkdir(mode=0o755)
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


if __name__ == "__main__":
    unittest.main()
