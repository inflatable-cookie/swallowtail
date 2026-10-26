#!/usr/bin/env python3
"""Decode selected Grok ACP dispatch tables from official runtimes without executing them."""

from __future__ import annotations

import argparse
import bisect
import hashlib
import json
import struct
import sys
from concurrent.futures import ProcessPoolExecutor, as_completed
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parent.parent
FIXTURE = ROOT / "crates/swallowtail-adapter-grok/tests/fixtures/grok-1.0.70-acp"
PRIOR_IDENTITY = (
    ROOT / "crates/swallowtail-adapter-grok/tests/fixtures/grok-1.0.46/identity.json"
)

VERSIONS = tuple(f"1.0.{minor}" for minor in range(46, 71))
PLATFORMS = ("linux-x64", "darwin-arm64")

STDIO_METHODS = (
    "initialize",
    "authenticate",
    "session/new",
    "session/load",
    "session/prompt",
    "session/cancel",
    "session/set_model",
)
CLIENT_CALLBACKS = (
    "session/request_permission",
    "session/update",
    "fs/write_text_file",
    "fs/read_text_file",
    "terminal/create",
    "terminal/output",
    "terminal/wait_for_exit",
    "terminal/kill",
)
PERMISSION_KINDS = (
    "allow_once",
    "allow_always",
    "reject_once",
    "reject_always",
)
ACP_METHODS = tuple(
    sorted(
        set(STDIO_METHODS)
        | set(CLIENT_CALLBACKS)
        | {
            "session/set_mode",
            "session/set_config_option",
            "session/close",
            "session/fork",
            "session/resume",
            "session/list",
            "session/delete",
            "session/rename",
            "session/export",
            "session/import",
            "session/compact",
            "terminal/release",
            "x.ai/ask_user_question",
            "x.ai/session/upsert",
            "x.ai/session/close",
            "x.ai/session/update",
            "_x.ai/session/close",
            "_x.ai/session/update",
            "session/request_user_input",
        },
        key=len,
        reverse=True,
    )
)
SERDE_TYPES = (
    "InitializeRequest",
    "InitializeResponse",
    "NewSessionRequest",
    "NewSessionResponse",
    "LoadSessionRequest",
    "PromptRequest",
    "PromptResponse",
    "UsageUpdate",
    "SessionNotification",
    "RequestPermissionRequest",
    "RequestPermissionResponse",
    "AgentCapabilities",
    "PromptCapabilities",
    "SessionModelState",
    "AuthCapabilities",
)
FIELD_PACKS = {
    "initialize_request_fields": b"protocolVersionclientCapabilitiesclientInfo",
    "initialize_response_fields": b"agentCapabilitiesauthMethodsagentInfo",
    "agent_capabilities_fields": b"loadSessionpromptCapabilitiesmcpCapabilitiessessionCapabilities",
    "session_model_state_fields": b"currentModelIdavailableModels",
    "permission_kinds": b"allow_onceallow_alwaysreject_oncereject_always",
}
ALREADY_PRESENT_UNMAPPED = {
    "session/set_mode",
    "session/set_config_option",
    "session/close",
    "session/fork",
    "session/resume",
    "session/list",
    "session/delete",
    "session/rename",
    "session/export",
    "session/import",
    "session/compact",
    "fs/write_text_file",
    "fs/read_text_file",
    "terminal/create",
    "terminal/output",
    "terminal/wait_for_exit",
    "terminal/kill",
    "terminal/release",
    "x.ai/ask_user_question",
    "x.ai/session/upsert",
    "x.ai/session/close",
    "x.ai/session/update",
    "_x.ai/session/close",
    "_x.ai/session/update",
    "authenticate",
}
SWALLOWTAIL_PROCESS_LIFECYCLE = {
    "join_drain_release": (
        "Swallowtail begin_close is process.close_stdin plus process.request_stop. "
        "The vendor session/close JSON-RPC method stays unmapped."
    ),
    "session_close_rpc": "unmapped; already present at 1.0.46",
}


class EvidenceError(ValueError):
    """Raised when an artifact cannot be decoded or disagrees with frozen identity."""


def sha256_bytes(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def sha256_lines(values: list[str]) -> str:
    return hashlib.sha256(("\n".join(values) + "\n").encode()).hexdigest()


def concat(parts: tuple[str, ...]) -> bytes:
    return "".join(parts).encode()


def pack_offsets(blob: bytes, parts: tuple[str, ...]) -> dict[str, int]:
    offsets: dict[str, int] = {}
    cursor = 0
    for part in parts:
        offsets[part] = cursor
        cursor += len(part.encode())
    if cursor != len(blob):
        raise EvidenceError("pack offset reconstruction drifted")
    return offsets


def greedy_method_packs(data: bytes) -> list[tuple[int, tuple[str, ...]]]:
    encoded = [(name, name.encode()) for name in ACP_METHODS]
    found: dict[int, tuple[str, ...]] = {}
    for name, blob in encoded:
        start = 0
        while True:
            index = data.find(blob, start)
            if index < 0:
                break
            left, right = index, index + len(blob)
            sequence = [name]
            growing = True
            while growing:
                growing = False
                for other, other_blob in encoded:
                    other_len = len(other_blob)
                    if data[right : right + other_len] == other_blob:
                        sequence.append(other)
                        right += other_len
                        growing = True
                    elif (
                        left >= other_len
                        and data[left - other_len : left] == other_blob
                    ):
                        sequence.insert(0, other)
                        left -= other_len
                        growing = True
            if len(sequence) >= 2:
                found[left] = tuple(sequence)
            start = index + 1
    return sorted(found.items())


def serde_element_counts(data: bytes) -> dict[str, int]:
    counts: dict[str, int] = {}
    for name in SERDE_TYPES:
        marker = f"struct {name} with ".encode()
        index = data.find(marker)
        if index < 0:
            continue
        rest = data[index + len(marker) : index + len(marker) + 16]
        digits = []
        for byte in rest:
            if 48 <= byte <= 57:
                digits.append(chr(byte))
            else:
                break
        if digits:
            counts[name] = int("".join(digits))
    return counts


def presence_map(data: bytes, keys: list[str]) -> dict[str, bool]:
    return {key: data.find(key.encode()) >= 0 for key in keys}


def parse_macho_sections(data: bytes) -> tuple[list[dict[str, Any]], int]:
    magic = struct.unpack_from("<I", data, 0)[0]
    if magic != 0xFEEDFACF:
        raise EvidenceError("not a little-endian 64-bit Mach-O")
    ncmds = struct.unpack_from("<I", data, 16)[0]
    offset = 32
    sections: list[dict[str, Any]] = []
    text_vmaddr = 0
    for _ in range(ncmds):
        cmd, cmdsize = struct.unpack_from("<II", data, offset)
        if cmd == 0x19:
            segname = data[offset + 8 : offset + 24].split(b"\x00", 1)[0].decode()
            vmaddr = struct.unpack_from("<Q", data, offset + 24)[0]
            nsects = struct.unpack_from("<I", data, offset + 64)[0]
            if segname == "__TEXT":
                text_vmaddr = vmaddr
            sect_off = offset + 72
            for _sect in range(nsects):
                sectname = data[sect_off : sect_off + 16].split(b"\x00", 1)[0].decode()
                sseg = data[sect_off + 16 : sect_off + 32].split(b"\x00", 1)[0].decode()
                addr, size = struct.unpack_from("<QQ", data, sect_off + 32)
                file_off = struct.unpack_from("<I", data, sect_off + 48)[0]
                sections.append(
                    {
                        "seg": sseg,
                        "name": sectname,
                        "addr": addr,
                        "size": size,
                        "offset": file_off,
                    }
                )
                sect_off += 80
        offset += cmdsize
    return sections, text_vmaddr


def compact_unwind_starts(
    data: bytes, unwind: dict[str, Any], text_vmaddr: int
) -> list[int]:
    blob = data[unwind["offset"] : unwind["offset"] + unwind["size"]]
    (
        version,
        _common_off,
        _common_count,
        _pers_off,
        _pers_count,
        index_off,
        index_count,
    ) = struct.unpack_from("<IIIIIII", blob, 0)
    if version != 1:
        raise EvidenceError(f"compact unwind version {version}")
    starts: list[int] = []
    for index in range(index_count - 1):
        function_offset, page_off, _lsda = struct.unpack_from(
            "<III", blob, index_off + index * 12
        )
        if page_off == 0:
            continue
        kind = struct.unpack_from("<I", blob, page_off)[0]
        if kind == 2:
            entry_page_offset, entry_count = struct.unpack_from(
                "<HH", blob, page_off + 4
            )
            for entry in range(entry_count):
                foff, _enc = struct.unpack_from(
                    "<II", blob, page_off + entry_page_offset + entry * 8
                )
                starts.append(text_vmaddr + foff)
        elif kind == 3:
            entry_page_offset, entry_count, _enc_off, _enc_count = struct.unpack_from(
                "<HHHH", blob, page_off + 4
            )
            for entry in range(entry_count):
                word = struct.unpack_from(
                    "<I", blob, page_off + entry_page_offset + entry * 4
                )[0]
                starts.append(text_vmaddr + function_offset + (word & 0xFFFFFF))
        else:
            raise EvidenceError(f"unknown compact-unwind page kind {kind}")
    starts.sort()
    return starts


def section_vma(sections: list[dict[str, Any]], file_offset: int) -> int:
    for section in sections:
        start = section["offset"]
        end = start + section["size"]
        if start <= file_offset < end:
            return section["addr"] + (file_offset - start)
    raise EvidenceError(f"file offset {file_offset} is outside mapped sections")


def function_at(starts: list[int], pc: int) -> tuple[int, int] | None:
    index = bisect.bisect_right(starts, pc) - 1
    if index < 0:
        return None
    start = starts[index]
    end = starts[index + 1] if index + 1 < len(starts) else start
    return start, end


def macho_adrp_xrefs(
    data: bytes, text: dict[str, Any], targets: dict[int, str]
) -> dict[str, list[int]]:
    code = data[text["offset"] : text["offset"] + text["size"]]
    text_addr = text["addr"]
    found: dict[str, list[int]] = {name: [] for name in set(targets.values())}
    limit = len(code) - 8
    for offset in range(0, limit, 4):
        instruction = struct.unpack_from("<I", code, offset)[0]
        if instruction & 0x9F000000 != 0x90000000:
            continue
        register = instruction & 31
        immhi = (instruction >> 5) & 0x7FFFF
        immlo = (instruction >> 29) & 0x3
        immediate = (immhi << 2) | immlo
        if immediate & (1 << 20):
            immediate -= 1 << 21
        page = (text_addr + offset) & ~0xFFF
        base = page + (immediate << 12)
        for add_offset in range(offset + 4, min(offset + 20, len(code)), 4):
            add = struct.unpack_from("<I", code, add_offset)[0]
            if add & 0x7F000000 != 0x11000000:
                continue
            if ((add >> 5) & 31) != register:
                continue
            low = (add >> 10) & 0xFFF
            if (add >> 22) & 1:
                low <<= 12
            vma = base + low
            name = targets.get(vma)
            if name is not None:
                found[name].append(text_addr + offset)
            break
    return {name: pcs for name, pcs in found.items() if pcs}


def parse_elf_sections(data: bytes) -> list[dict[str, Any]]:
    if data[:4] != b"\x7fELF":
        raise EvidenceError("not ELF")
    if data[4] != 2 or data[5] != 1:
        raise EvidenceError("need ELF64 little-endian")
    e_shoff = struct.unpack_from("<Q", data, 40)[0]
    e_shentsize = struct.unpack_from("<H", data, 58)[0]
    e_shnum = struct.unpack_from("<H", data, 60)[0]
    e_shstrndx = struct.unpack_from("<H", data, 62)[0]
    str_off = struct.unpack_from("<Q", data, e_shoff + e_shstrndx * e_shentsize + 24)[0]
    sections: list[dict[str, Any]] = []
    for index in range(e_shnum):
        base = e_shoff + index * e_shentsize
        name_off, sh_type = struct.unpack_from("<II", data, base)
        flags, addr, offset, size = struct.unpack_from("<QQQQ", data, base + 8)
        end = data.find(b"\x00", str_off + name_off)
        name = data[str_off + name_off : end].decode()
        sections.append(
            {
                "name": name,
                "type": sh_type,
                "flags": flags,
                "addr": addr,
                "offset": offset,
                "size": size,
            }
        )
    return sections


def decode_eh_pointer(
    data: bytes, offset: int, encoding: int, pc: int, datarel: int
) -> tuple[int, int]:
    application = encoding & 0x70
    fmt = encoding & 0x0F
    if encoding == 0xFF:
        return 0, offset
    if fmt == 0x03:
        value = struct.unpack_from("<I", data, offset)[0]
        offset += 4
    elif fmt == 0x0B:
        value = struct.unpack_from("<i", data, offset)[0]
        offset += 4
    elif fmt == 0x04:
        value = struct.unpack_from("<Q", data, offset)[0]
        offset += 8
    elif fmt == 0x0C:
        value = struct.unpack_from("<q", data, offset)[0]
        offset += 8
    else:
        raise EvidenceError(f"unsupported DWARF pointer format 0x{encoding:02x}")
    if application == 0x10:
        value += pc
    elif application == 0x30:
        value += datarel
    return value & 0xFFFFFFFFFFFFFFFF, offset


def elf_eh_frame_starts(data: bytes, hdr: dict[str, Any]) -> list[int]:
    blob = data[hdr["offset"] : hdr["offset"] + hdr["size"]]
    if blob[0] != 1:
        raise EvidenceError("eh_frame_hdr version")
    ptr_enc, count_enc, table_enc = blob[1], blob[2], blob[3]
    datarel = hdr["addr"]
    cursor = 4
    _eh_ptr, cursor = decode_eh_pointer(
        blob, cursor, ptr_enc, hdr["addr"] + cursor, datarel
    )
    count, cursor = decode_eh_pointer(
        blob, cursor, count_enc, hdr["addr"] + cursor, datarel
    )
    starts: list[int] = []
    for _ in range(count):
        loc, cursor = decode_eh_pointer(
            blob, cursor, table_enc, hdr["addr"] + cursor, datarel
        )
        _fde, cursor = decode_eh_pointer(
            blob, cursor, table_enc, hdr["addr"] + cursor, datarel
        )
        starts.append(loc)
    starts.sort()
    return starts


def elf_lea_xrefs(
    data: bytes, text: dict[str, Any], targets: dict[int, str]
) -> dict[str, list[int]]:
    code = data[text["offset"] : text["offset"] + text["size"]]
    text_addr = text["addr"]
    found: dict[str, list[int]] = {name: [] for name in set(targets.values())}
    offset = 1
    limit = len(code) - 6
    while offset < limit:
        index = code.find(b"\x8d", offset)
        if index < 0 or index >= limit:
            break
        if code[index - 1] & 0xF0 == 0x40 and code[index + 1] & 0xC7 == 0x05:
            disp = struct.unpack_from("<i", code, index + 2)[0]
            pc = text_addr + index - 1
            target = (pc + 7 + disp) & 0xFFFFFFFFFFFFFFFF
            name = targets.get(target)
            if name is not None:
                found[name].append(pc)
            offset = index + 6
            continue
        offset = index + 1
    return {name: pcs for name, pcs in found.items() if pcs}


def classify_extra_pack(methods: tuple[str, ...]) -> str:
    if all(name in STDIO_METHODS or name in CLIENT_CALLBACKS for name in methods):
        return "selected-duplicate"
    if all(
        name in ALREADY_PRESENT_UNMAPPED
        or name in STDIO_METHODS
        or name in CLIENT_CALLBACKS
        for name in methods
    ):
        return "already-present-unmapped"
    return "new-or-unknown"


def xref_functions(
    data: bytes,
    sections: list[dict[str, Any]],
    starts: list[int],
    text: dict[str, Any],
    pack_file_offset: int,
    parts: tuple[str, ...],
    xrefs: dict[str, list[int]],
) -> dict[str, list[dict[str, Any]]]:
    blob = concat(parts)
    relative = pack_offsets(blob, parts)
    pack_vma = section_vma(sections, pack_file_offset)
    records: dict[str, list[dict[str, Any]]] = {}
    for name, rel in relative.items():
        functions: list[dict[str, Any]] = []
        seen: set[int] = set()
        for pc in xrefs.get(name, ()):
            owner = function_at(starts, pc)
            if owner is None:
                continue
            start, end = owner
            if start in seen:
                continue
            seen.add(start)
            size = end - start if end > start else 0
            file_start = text["offset"] + (start - text["addr"])
            body = data[file_start : file_start + size] if size > 0 else b""
            functions.append(
                {
                    "vma": f"0x{start:x}",
                    "size": size,
                    "body_sha256": sha256_bytes(body) if body else None,
                    "xref_pc": f"0x{pc:x}",
                }
            )
        records[name] = {
            "pack_vma": f"0x{pack_vma + rel:x}",
            "functions": functions,
        }
    return records


def analyze_binary(
    path: Path, platform: str, expected_sha256: str, keys: list[str]
) -> dict[str, Any]:
    data = path.read_bytes()
    digest = sha256_bytes(data)
    if digest != expected_sha256:
        raise EvidenceError(f"{path.name} sha256 {digest} != {expected_sha256}")
    stdio_blob = concat(STDIO_METHODS)
    client_blob = concat(CLIENT_CALLBACKS)
    perm_blob = concat(PERMISSION_KINDS)
    stdio_off = data.find(stdio_blob)
    client_off = data.find(client_blob)
    perm_off = data.find(perm_blob)
    if stdio_off < 0 or client_off < 0 or perm_off < 0:
        raise EvidenceError(f"{path.name} missing a selected interned pack")
    packs = greedy_method_packs(data)
    extra = []
    for offset, methods in packs:
        if methods in (STDIO_METHODS, CLIENT_CALLBACKS):
            continue
        extra.append(
            {
                "file_offset": offset,
                "methods": list(methods),
                "classification": classify_extra_pack(methods),
            }
        )
    presence = presence_map(data, keys)
    present = sorted(key for key, found in presence.items() if found)
    record: dict[str, Any] = {
        "executable_sha256": digest,
        "executable_size": len(data),
        "stdio_pack": list(STDIO_METHODS),
        "stdio_pack_file_offset": stdio_off,
        "client_pack": list(CLIENT_CALLBACKS),
        "client_pack_file_offset": client_off,
        "permission_pack": list(PERMISSION_KINDS),
        "permission_pack_file_offset": perm_off,
        "field_packs_present": {
            name: data.find(blob) >= 0 for name, blob in FIELD_PACKS.items()
        },
        "stop_reason_variants_present": {
            name: data.find(name.encode()) >= 0
            for name in (
                "end_turn",
                "cancelled",
                "refusal",
                "max_tokens",
                "max_turn_requests",
            )
        },
        "serde_element_counts": serde_element_counts(data),
        "extra_method_packs": extra,
        "selected_literal_presence_digest": sha256_lines(present),
        "xref": {},
        "function_count": 0,
        "text_size": 0,
    }
    selected_names = set(STDIO_METHODS) | {
        "session/request_permission",
        "session/update",
    }
    if platform == "darwin-arm64":
        sections, text_vmaddr = parse_macho_sections(data)
        text = next(
            section
            for section in sections
            if section["name"] == "__text" and section["seg"] == "__TEXT"
        )
        unwind = next(
            section for section in sections if section["name"] == "__unwind_info"
        )
        starts = compact_unwind_starts(data, unwind, text_vmaddr)
        targets: dict[int, str] = {}
        for pack_off, parts in (
            (stdio_off, STDIO_METHODS),
            (client_off, CLIENT_CALLBACKS),
        ):
            blob = concat(parts)
            for name, rel in pack_offsets(blob, parts).items():
                if name in selected_names:
                    targets[section_vma(sections, pack_off) + rel] = name
        xrefs = macho_adrp_xrefs(data, text, targets)
        record["function_count"] = len(starts)
        record["text_size"] = text["size"]
        record["xref"]["stdio"] = xref_functions(
            data, sections, starts, text, stdio_off, STDIO_METHODS, xrefs
        )
        record["xref"]["client"] = xref_functions(
            data,
            sections,
            starts,
            text,
            client_off,
            CLIENT_CALLBACKS,
            xrefs,
        )
    elif platform == "linux-x64":
        sections = parse_elf_sections(data)
        text = next(section for section in sections if section["name"] == ".text")
        hdr = next(
            section for section in sections if section["name"] == ".eh_frame_hdr"
        )
        starts = elf_eh_frame_starts(data, hdr)
        targets = {}
        for pack_off, parts in (
            (stdio_off, STDIO_METHODS),
            (client_off, CLIENT_CALLBACKS),
        ):
            blob = concat(parts)
            for name, rel in pack_offsets(blob, parts).items():
                if name in selected_names:
                    targets[section_vma(sections, pack_off) + rel] = name
        xrefs = elf_lea_xrefs(data, text, targets)
        record["function_count"] = len(starts)
        record["text_size"] = text["size"]
        record["xref"]["stdio"] = xref_functions(
            data, sections, starts, text, stdio_off, STDIO_METHODS, xrefs
        )
        record["xref"]["client"] = xref_functions(
            data,
            sections,
            starts,
            text,
            client_off,
            CLIENT_CALLBACKS,
            xrefs,
        )
    else:
        raise EvidenceError(f"unsupported platform {platform}")
    return record


def expected_hashes() -> dict[tuple[str, str], str]:
    identity = json.loads((FIXTURE / "identity.json").read_text())
    prior = json.loads(PRIOR_IDENTITY.read_text())
    hashes: dict[tuple[str, str], str] = {}
    hop46 = next(hop for hop in prior["hops"] if hop["version"] == "1.0.46")
    hashes[("1.0.46", "linux-x64")] = hop46["linux_x64"]["executable_sha256"]
    hashes[("1.0.46", "darwin-arm64")] = hop46["darwin_arm64"]["executable_sha256"]
    for hop in identity["hops"]:
        hashes[(hop["version"], "linux-x64")] = hop["linux_x64"]["executable_sha256"]
        hashes[(hop["version"], "darwin-arm64")] = hop["darwin_arm64"][
            "executable_sha256"
        ]
    return hashes


def load_literal_keys() -> list[str]:
    protocol = json.loads((FIXTURE / "protocol.json").read_text())
    keys = list(protocol["selected_literal_presence"].keys())
    if len(keys) != 92:
        raise EvidenceError(f"expected 92 selected literals, found {len(keys)}")
    return keys


def document() -> dict[str, Any]:
    return {
        "axis": "grok-build.executable",
        "route": "grok-build.acp",
        "baseline": "1.0.46",
        "official_stable": "1.0.70",
        "method": (
            "Hash each official decompressed Linux x64 ELF and Darwin arm64 Mach-O. "
            "Locate the interned concatenated JSON-RPC method tables and serde "
            "element-count strings. Decode Darwin compact-unwind function ranges and "
            "ADRP+ADD literal xrefs, and Linux .eh_frame_hdr ranges with RIP-relative "
            "LEA xrefs, without executing the file."
        ),
        "no_execution": True,
        "selected_stdio_methods": list(STDIO_METHODS),
        "selected_client_callbacks": list(CLIENT_CALLBACKS),
        "selected_permission_kinds": list(PERMISSION_KINDS),
        "swallowtail_process_lifecycle": SWALLOWTAIL_PROCESS_LIFECYCLE,
        "compatibility_predicate": (
            "Interned stdio, client, and permission tables plus serde layouts and the "
            "frozen 1.0.46 selected-literal presence digest are recorded observations. "
            "They do not qualify a hop. Selected handler xref function bodies must match "
            "1.0.46 on both platforms to raise the ACP ceiling; they differ on every hop "
            "1.0.47 through 1.0.70, so those points stay UnverifiedNewer."
        ),
        "hops": {},
    }


def _job(
    version: str, platform: str, path: str, digest: str, keys: list[str]
) -> tuple[str, str, dict[str, Any]]:
    record = analyze_binary(Path(path), platform, digest, keys)
    return version, platform, record


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binaries", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--versions", nargs="*", default=list(VERSIONS))
    parser.add_argument("--jobs", type=int, default=3)
    args = parser.parse_args(argv)
    hashes = expected_hashes()
    keys = load_literal_keys()
    result = document()
    pending: list[tuple[str, str, str, str, list[str]]] = []
    for version in args.versions:
        result["hops"][version] = {}
        for platform in PLATFORMS:
            path = args.binaries / f"{platform}-{version}"
            if not path.is_file():
                raise EvidenceError(f"missing {path}")
            pending.append(
                (version, platform, str(path), hashes[(version, platform)], keys)
            )
    with ProcessPoolExecutor(max_workers=max(1, args.jobs)) as pool:
        futures = [pool.submit(_job, *item) for item in pending]
        for future in as_completed(futures):
            version, platform, record = future.result()
            result["hops"][version][platform.replace("-", "_")] = record
            print(
                f"analyze {platform} {version} funcs={record['function_count']}",
                flush=True,
            )
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(result, indent=2) + "\n")
    print(f"wrote {args.output}", flush=True)
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except EvidenceError as error:
        print(f"error: {error}", file=sys.stderr)
        raise SystemExit(1)
