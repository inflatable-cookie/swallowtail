"""Private SDK240 ELF reader; never loads a candidate or invokes host tools.

The independent host manifest still owns loader selection: DT_NEEDED edges,
PT_INTERP aliases, cache/search policy and complete supervisor inputs. This
reader checks the supplied exact ELF graph, not that glibc will select it.
Only little-endian ELF64 x86_64 without RPATH/RUNPATH is supported here.
"""

import os
from pathlib import PurePosixPath
import stat
import struct

MAX_FILE = 268435456
MAX_HEADERS = 128
MAX_DYNAMIC = 512
MAX_OBJECTS = 128
MAX_STRINGS = 1048576


class Refused(OSError):
    pass


def absolute(name):
    if (
        not isinstance(name, str)
        or not name.startswith("/")
        or name.startswith("//")
        or str(PurePosixPath(name)) != name
        or ".." in PurePosixPath(name).parts
        or "\x00" in name
        or len(name) > 4096
    ):
        raise Refused("elf_absolute_path_required")
    return name


def parse_elf(data):
    """Parse bounded bytes, refusing unsupported or ambiguous loader inputs."""
    if not isinstance(data, (bytes, bytearray, memoryview)):
        raise Refused("elf_bytes_required")
    if not 64 <= len(data) <= MAX_FILE:
        raise Refused("elf_file_bound")
    fields = struct.unpack_from("<16sHHIQQQIHHHHHH", data)
    ident, kind, machine, version = fields[:4]
    phoff, shoff, flags, ehsize, phsize, phcount, shsize, shcount, shstr = fields[5:]
    if (
        ident[:7] != b"\x7fELF\x02\x01\x01"
        or ident[7] not in (0, 3)
        or ident[8] != 0
        or any(ident[9:])
        or kind not in (2, 3)
        or machine != 62
        or version != 1
        or flags != 0
        or ehsize != 64
        or phsize != 56
        or not 1 <= phcount <= MAX_HEADERS
        or shstr == 65535
        or (shoff and (not shcount or shsize != 64))
        or (not shoff and (shcount or shstr))
        or (shcount and shstr >= shcount)
    ):
        raise Refused("elf_unsupported_header")

    def extent(offset, size):
        if offset < 0 or size < 0 or offset > len(data) or size > len(data) - offset:
            raise Refused("elf_offset_or_size")
        return offset, offset + size

    if phoff < 64:
        raise Refused("elf_program_header_overlap")
    extent(phoff, phcount * phsize)
    if shoff:
        extent(shoff, shcount * shsize)
    loads, dynamics, interpreters = [], [], []
    for index in range(phcount):
        row = struct.unpack_from("<IIQQQQQQ", data, phoff + index * phsize)
        tag, _, offset, address, _, filesz, memsz, align = row
        extent(offset, filesz)
        if tag == 1:
            if (
                filesz > memsz
                or address + memsz > 2**64
                or (
                    align > 1
                    and (align & (align - 1) or address % align != offset % align)
                )
            ):
                raise Refused("elf_load_shape")
            if any(address < old[1] and old[0] < address + memsz for old in loads):
                raise Refused("elf_ambiguous_load_mapping")
            loads.append((address, address + memsz, offset, filesz))
        elif tag == 2:
            dynamics.append((offset, address, filesz))
        elif tag == 3:
            interpreters.append((offset, filesz))
    if not loads or len(dynamics) > 1 or len(interpreters) > 1:
        raise Refused("elf_ambiguous_program_headers")

    def mapped(address, size):
        choices = [
            offset + address - start
            for start, _, offset, filesz in loads
            if start <= address and address + size <= start + filesz
        ]
        if len(choices) != 1:
            raise Refused("elf_unmapped_or_ambiguous_input")
        return choices[0]

    interpreter = None
    if interpreters:
        offset, size = interpreters[0]
        if not 2 <= size <= 4096:
            raise Refused("elf_interpreter_bound")
        raw = bytes(data[offset : offset + size])
        if raw[-1:] != b"\x00" or b"\x00" in raw[:-1]:
            raise Refused("elf_interpreter_string")
        try:
            interpreter = absolute(raw[:-1].decode("ascii"))
        except UnicodeError as error:
            raise Refused("elf_interpreter_string") from error

    tags, needed = {}, []
    if dynamics:
        offset, address, size = dynamics[0]
        if (
            not size
            or size % 16
            or size // 16 > MAX_DYNAMIC
            or mapped(address, size) != offset
        ):
            raise Refused("elf_dynamic_bound_or_mapping")
        ended = False
        for index in range(size // 16):
            tag, value = struct.unpack_from("<qQ", data, offset + index * 16)
            if ended:
                if tag or value:
                    raise Refused("elf_dynamic_after_null")
                continue
            if tag == 0:
                if value:
                    raise Refused("elf_dynamic_null")
                ended = True
                continue
            if tag in {22, 0x6FFFFEFA, 0x6FFFFEFB, 0x6FFFFEFC, 0x7FFFFFFD, 0x7FFFFFFF}:
                raise Refused("elf_unsupported_dynamic_input")
            if (tag == 30 and value & 4) or (tag == 0x6FFFFFFB and value & 0x800):
                raise Refused("elf_unsupported_dynamic_flags")
            if tag == 1:
                needed.append(value)
                if len(needed) > 128:
                    raise Refused("elf_needed_bound")
            elif tag in {5, 10, 14, 15, 29}:
                if tag in tags:
                    raise Refused("elf_duplicate_dynamic_input")
                tags[tag] = value
        if not ended:
            raise Refused("elf_dynamic_unterminated")
    strings = needed or any(tag in tags for tag in (14, 15, 29))
    if strings or 5 in tags or 10 in tags:
        if 5 not in tags or 10 not in tags or not 1 <= tags[10] <= MAX_STRINGS:
            raise Refused("elf_string_table_required")
        table = mapped(tags[5], tags[10])

        def string(index):
            if index >= tags[10]:
                raise Refused("elf_string_offset")
            raw = bytes(data[table + index : table + min(tags[10], index + 4097)])
            end = raw.find(b"\x00")
            if end < 0 or end > 4096:
                raise Refused("elf_string_unterminated_or_bound")
            try:
                return raw[:end].decode("ascii")
            except UnicodeError as error:
                raise Refused("elf_string_encoding") from error

        needed = [string(index) for index in needed]
        names = {tag: string(tags[tag]) for tag in (14, 15, 29) if tag in tags}
    else:
        names = {}
    if any(not name or "/" in name for name in needed) or len(set(needed)) != len(
        needed
    ):
        raise Refused("elf_needed_name_or_duplicate")
    if names.get(14) is not None and (not names[14] or "/" in names[14]):
        raise Refused("elf_soname")
    if names.get(15) or names.get(29):
        raise Refused("elf_search_path_requires_separate_policy")
    return {
        "interpreter": interpreter,
        "needed": needed,
        "soname": names.get(14),
        "rpath": names.get(15),
        "runpath": names.get(29),
        "machine": machine,
        "elfType": kind,
    }


def parse_fd(descriptor):
    """Read an already held regular descriptor; caller retains pathname custody."""
    before = os.fstat(descriptor)
    if not stat.S_ISREG(before.st_mode) or not 64 <= before.st_size <= MAX_FILE:
        raise Refused("elf_regular_file_bound")
    chunks, total = [], 0
    while total < before.st_size:
        chunk = os.pread(descriptor, min(65536, before.st_size - total), total)
        if not chunk:
            raise Refused("elf_short_read")
        chunks.append(chunk)
        total += len(chunk)
    after = os.fstat(descriptor)
    keys = (
        "st_dev",
        "st_ino",
        "st_mode",
        "st_uid",
        "st_gid",
        "st_size",
        "st_mtime_ns",
        "st_ctime_ns",
    )
    if any(getattr(before, key) != getattr(after, key) for key in keys):
        raise Refused("elf_changed_during_read")
    return parse_elf(b"".join(chunks))


def verify_graph(root, objects, resolutions, *, interpreter_paths=None):
    """Check an independently provisioned exact loader graph without executing it.

    Every object needs an edge map, including empty maps. PT_INTERP aliases may
    map to a canonical object only when the caller independently holds the alias.
    Returning successfully is structural closure, never loader-selection proof.
    """
    absolute(root)
    if (
        not isinstance(objects, dict)
        or not 1 <= len(objects) <= MAX_OBJECTS
        or not isinstance(resolutions, dict)
        or set(resolutions) != set(objects)
    ):
        raise Refused("elf_graph_shape")
    for name in objects:
        absolute(name)
    aliases = {} if interpreter_paths is None else interpreter_paths
    if not isinstance(aliases, dict):
        raise Refused("elf_interpreter_alias_shape")
    for alias, target in aliases.items():
        absolute(alias)
        absolute(target)
        if target not in objects:
            raise Refused("elf_interpreter_binding_missing")
    parsed, metadata, seen_aliases, pending = {}, {}, set(), [root]

    def describe(name):
        if name not in metadata:
            metadata[name] = parse_elf(objects[name])
        return metadata[name]

    while pending:
        name = pending.pop()
        if name in parsed:
            continue
        if name not in objects:
            raise Refused("elf_object_binding_missing")
        row = describe(name)
        parsed[name] = row
        edges = resolutions[name]
        if not isinstance(edges, dict) or set(edges) != set(row["needed"]):
            raise Refused("elf_needed_edge_set")
        for needed, target in edges.items():
            absolute(target)
            if target not in objects:
                raise Refused("elf_dependency_binding_missing")
            target_row = describe(target)
            if target_row["soname"] not in (None, needed):
                raise Refused("elf_dependency_soname_mismatch")
            pending.append(target)
        if row["interpreter"]:
            literal = row["interpreter"]
            target = aliases.get(literal, literal)
            if literal in aliases:
                seen_aliases.add(literal)
            if target not in objects:
                raise Refused("elf_interpreter_binding_missing")
            pending.append(target)
    if not parsed[root]["interpreter"]:
        raise Refused("elf_root_interpreter_required")
    if set(parsed) != set(objects) or seen_aliases != set(aliases):
        raise Refused("elf_unused_manifest_input")
    return parsed
