"""Synthetic ELF graph tests. No candidate or host loader is invoked."""

import importlib.util
import os
from pathlib import Path
import struct
import tempfile
import unittest
from unittest import mock

ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location(
    "sdk_bootstrap_elf", ROOT / "scripts/claude-agent-sdk-bootstrap-elf.py"
)
E = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(E)


def elf(*, needed=(), soname=None, interpreter=None, extra=()):
    """Build bytes sufficient for the private reader; these are never executed."""
    data, strings, entries = bytearray(4096), bytearray(b"\x00"), []

    def intern(text):
        index = len(strings)
        strings.extend(text.encode("ascii") + b"\x00")
        return index

    for name in needed:
        entries.append((1, intern(name)))
    if soname is not None:
        entries.append((14, intern(soname)))
    for tag, value in extra:
        entries.append((tag, intern(value) if isinstance(value, str) else value))
    entries.extend(((5, 0x400000 + 2048), (10, len(strings)), (0, 0)))
    headers = [(1, 5, 0, 0x400000, 0, len(data), len(data), 4096)]
    if interpreter is not None:
        raw = interpreter.encode("ascii") + b"\x00"
        data[256 : 256 + len(raw)] = raw
        headers.append((3, 4, 256, 0x400100, 0, len(raw), len(raw), 1))
    headers.append((2, 6, 512, 0x400200, 0, 16 * len(entries), 16 * len(entries), 8))
    struct.pack_into(
        "<16sHHIQQQIHHHHHH",
        data,
        0,
        b"\x7fELF\x02\x01\x01" + b"\x00" * 9,
        3,
        62,
        1,
        0,
        64,
        0,
        0,
        64,
        56,
        len(headers),
        0,
        0,
        0,
    )
    for index, header in enumerate(headers):
        struct.pack_into("<IIQQQQQQ", data, 64 + index * 56, *header)
    for index, entry in enumerate(entries):
        struct.pack_into("<qQ", data, 512 + index * 16, *entry)
    data[2048 : 2048 + len(strings)] = strings
    return bytes(data)


class ElfTests(unittest.TestCase):
    def test_extracts_exact_interpreter_and_transitive_inputs(self):
        value = E.parse_elf(
            elf(
                needed=("liba.so.1", "libb.so.2"),
                soname="libroot.so",
                interpreter="/usr/lib/ld-linux-x86-64.so.2",
            )
        )
        self.assertEqual(value["interpreter"], "/usr/lib/ld-linux-x86-64.so.2")
        self.assertEqual(value["needed"], ["liba.so.1", "libb.so.2"])
        self.assertEqual(value["soname"], "libroot.so")
        self.assertEqual(value["machine"], 62)
        self.assertIsNone(value["rpath"])
        self.assertIsNone(value["runpath"])

    def test_wrong_architecture_encoding_and_extended_headers_refuse(self):
        cases = ((4, b"\x01"), (5, b"\x02"), (18, b"\xb7\x00"), (56, b"\xff\xff"))
        for offset, replacement in cases:
            with self.subTest(offset=offset):
                data = bytearray(elf())
                data[offset : offset + len(replacement)] = replacement
                with self.assertRaisesRegex(E.Refused, "unsupported_header"):
                    E.parse_elf(data)

    def test_file_header_and_segment_bounds_refuse(self):
        for data in (b"", elf()[:63], elf()[:1024]):
            with self.subTest(size=len(data)), self.assertRaises(E.Refused):
                E.parse_elf(data)
        data = bytearray(elf())
        struct.pack_into("<Q", data, 32, 32)
        with self.assertRaisesRegex(E.Refused, "header_overlap"):
            E.parse_elf(data)
        with mock.patch.object(E, "MAX_FILE", 100):
            with self.assertRaisesRegex(E.Refused, "file_bound"):
                E.parse_elf(elf())

    def test_overlapping_virtual_loads_are_not_resolved_by_first_match(self):
        data = bytearray(elf())
        struct.pack_into("<H", data, 56, 3)
        struct.pack_into("<IIQQQQQQ", data, 176, 1, 4, 0, 0x400000, 0, 4096, 4096, 4096)
        with self.assertRaisesRegex(E.Refused, "ambiguous_load_mapping"):
            E.parse_elf(data)

    def test_dynamic_mapping_and_termination_are_checked(self):
        data = bytearray(elf())
        struct.pack_into("<Q", data, 120 + 16, 0x500000)
        with self.assertRaisesRegex(E.Refused, "unmapped_or_ambiguous"):
            E.parse_elf(data)
        data = bytearray(elf())
        struct.pack_into("<qQ", data, 544, 21, 0)
        with self.assertRaisesRegex(E.Refused, "dynamic_unterminated"):
            E.parse_elf(data)
        data = bytearray(elf(extra=((0, 0),)))
        with self.assertRaisesRegex(E.Refused, "dynamic_after_null"):
            E.parse_elf(data)

    def test_string_bounds_and_duplicate_needed_refuse(self):
        with self.assertRaisesRegex(E.Refused, "needed_name_or_duplicate"):
            E.parse_elf(elf(needed=("liba.so", "liba.so")))
        with self.assertRaisesRegex(E.Refused, "needed_name_or_duplicate"):
            E.parse_elf(elf(needed=("../liba.so",)))
        data = bytearray(elf(needed=("liba.so",)))
        struct.pack_into("<Q", data, 520, 100000)
        with self.assertRaisesRegex(E.Refused, "string_offset"):
            E.parse_elf(data)
        data = bytearray(elf(needed=("liba.so",)))
        data[2056] = 65
        with self.assertRaisesRegex(E.Refused, "string_unterminated"):
            E.parse_elf(data)
        with self.assertRaisesRegex(E.Refused, "duplicate_dynamic_input"):
            E.parse_elf(elf(extra=((5, 1),)))

    def test_search_paths_and_extra_loader_behavior_refuse(self):
        for tag, value in (
            (15, "/other"),
            (29, "$ORIGIN"),
            (22, 0),
            (0x6FFFFEFA, 1),
            (0x6FFFFEFB, 1),
            (0x6FFFFEFC, 1),
            (0x7FFFFFFD, 1),
            (0x7FFFFFFF, 1),
            (30, 4),
            (0x6FFFFFFB, 0x800),
        ):
            with self.subTest(tag=tag), self.assertRaises(E.Refused):
                E.parse_elf(elf(extra=((tag, value),)))
        self.assertEqual(E.parse_elf(elf(extra=((29, ""),)))["runpath"], "")

    def test_interpreter_requires_one_absolute_terminated_string(self):
        for name in ("relative", "/lib/../loader", "/lib//loader"):
            with self.subTest(name=name), self.assertRaises(E.Refused):
                E.parse_elf(elf(interpreter=name))
        data = bytearray(elf(interpreter="/loader"))
        data[263] = 65
        with self.assertRaisesRegex(E.Refused, "interpreter_string"):
            E.parse_elf(data)

    def test_fd_read_preserves_position_and_refuses_changed_or_nonregular_file(self):
        with tempfile.TemporaryDirectory(
            prefix="sdk-elf-", dir=os.environ["TMPDIR"]
        ) as root:
            path = Path(root) / "owned-elf"
            path.write_bytes(elf(soname="libowned.so"))
            with path.open("rb") as source:
                source.seek(17)
                self.assertEqual(E.parse_fd(source.fileno())["soname"], "libowned.so")
                self.assertEqual(source.tell(), 17)
                real = E.os.pread

                def mutate(descriptor, count, offset):
                    value = real(descriptor, count, offset)
                    path.write_bytes(b"changed owned fixture")
                    return value

                with mock.patch.object(E.os, "pread", side_effect=mutate):
                    with self.assertRaisesRegex(E.Refused, "changed_during_read"):
                        E.parse_fd(source.fileno())
            descriptor = os.open(root, os.O_RDONLY | os.O_DIRECTORY)
            try:
                with self.assertRaisesRegex(E.Refused, "regular_file_bound"):
                    E.parse_fd(descriptor)
            finally:
                os.close(descriptor)


class GraphTests(unittest.TestCase):
    def setUp(self):
        self.root, self.loader, self.library = (
            "/candidate",
            "/usr/lib/loader",
            "/usr/lib/liba.so",
        )
        self.objects = {
            self.root: elf(needed=("liba.so",), interpreter="/lib64/loader"),
            self.loader: elf(soname="loader"),
            self.library: elf(needed=("liba.so",), soname="liba.so"),
        }
        self.edges = {
            self.root: {"liba.so": self.library},
            self.loader: {},
            self.library: {"liba.so": self.library},
        }
        self.aliases = {"/lib64/loader": self.loader}

    def verify(self):
        return E.verify_graph(
            self.root, self.objects, self.edges, interpreter_paths=self.aliases
        )

    def test_exact_transitive_graph_accepts_cycles_and_bound_interpreter_alias(self):
        result = self.verify()
        self.assertEqual(set(result), set(self.objects))
        self.assertEqual(result[self.library]["needed"], ["liba.so"])

    def test_missing_and_extra_needed_edges_refuse(self):
        for replacement in ({}, {"liba.so": self.library, "extra.so": self.library}):
            with self.subTest(replacement=replacement):
                self.edges[self.root] = replacement
                with self.assertRaisesRegex(E.Refused, "needed_edge_set"):
                    self.verify()

    def test_missing_library_loader_and_unbound_alias_refuse(self):
        del self.objects[self.library]
        del self.edges[self.library]
        with self.assertRaisesRegex(E.Refused, "dependency_binding_missing"):
            self.verify()
        self.setUp()
        self.aliases.clear()
        with self.assertRaisesRegex(E.Refused, "interpreter_binding_missing"):
            self.verify()

    def test_dependency_soname_and_architecture_are_checked(self):
        self.objects[self.library] = elf(soname="other.so")
        with self.assertRaisesRegex(E.Refused, "soname_mismatch"):
            self.verify()
        data = bytearray(elf(soname="liba.so"))
        struct.pack_into("<H", data, 18, 183)
        self.objects[self.library] = bytes(data)
        with self.assertRaisesRegex(E.Refused, "unsupported_header"):
            self.verify()

    def test_unused_objects_aliases_and_missing_edge_maps_refuse(self):
        self.objects["/unused"] = elf()
        self.edges["/unused"] = {}
        with self.assertRaisesRegex(E.Refused, "unused_manifest_input"):
            self.verify()
        self.setUp()
        self.aliases["/unused-alias"] = self.loader
        with self.assertRaisesRegex(E.Refused, "unused_manifest_input"):
            self.verify()
        self.setUp()
        del self.edges[self.loader]
        with self.assertRaisesRegex(E.Refused, "graph_shape"):
            self.verify()

    def test_no_interpreter_and_noncanonical_manifest_paths_refuse(self):
        with self.assertRaisesRegex(E.Refused, "root_interpreter_required"):
            E.verify_graph("/static", {"/static": elf()}, {"/static": {}})
        with self.assertRaisesRegex(E.Refused, "absolute_path_required"):
            E.verify_graph("/lib/../candidate", self.objects, self.edges)


if __name__ == "__main__":
    unittest.main()
