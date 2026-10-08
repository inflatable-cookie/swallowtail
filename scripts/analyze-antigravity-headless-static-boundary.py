#!/usr/bin/env python3
"""Map selected Antigravity Mach-O/Go control-flow without executing it."""

from __future__ import annotations

import argparse
import bisect
import hashlib
import json
import struct
import sys
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parent.parent
BASELINE_IDENTITY = (
    ROOT
    / "crates/swallowtail-adapter-antigravity/tests/fixtures/antigravity-cli-1.2.11/identity.json"
)
NEWER_INVENTORY = (
    ROOT
    / "crates/swallowtail-adapter-antigravity/tests/fixtures/antigravity-cli-1.3.1/dist-inventory.json"
)
VERSIONS = (
    "1.2.11",
    "1.2.12",
    "1.2.13",
    "1.2.14",
    "1.2.15",
    "1.2.16",
    "1.2.17",
    "1.3.0",
    "1.3.1",
)
RETRY_LOADER = (
    "google3/third_party/jetski/cli/backend/backend.applyModelAPIMaxRetriesOverride"
)
BACKEND_BUILD = (
    "google3/third_party/jetski/cli/backend/backend.(*ServerBackend).buildCascadeConfigInternal"
)
RUNNER_SUFFIX = {
    version: (
        "google3/third_party/jetski/cli/printmode/printmode.Run"
        if version in ("1.2.11", "1.2.12", "1.2.13")
        else "google3/third_party/jetski/cli/printmode/printmode.run"
    )
    for version in VERSIONS
}
API_RETRY_SUFFIX = {
    version: (
        "google3/third_party/gemini_coder/framework/core/core.generateWithAPIRetry"
        if version in ("1.2.11", "1.2.12", "1.2.13", "1.2.14", "1.2.15")
        else "google3/third_party/oneharness/internal/core/core.generateWithAPIRetry"
    )
    for version in VERSIONS
}
SELECTED_CALL_MARKERS = (
    "runAttempt",
    "sleepWithContext",
    "appendErrorStep",
    "hasMoreRetries",
    "attemptGenerate",
    "analyzeError",
    "isMidStreamError",
    "ExtractRetryDelayFromError",
    "extractRetryInfoFromHTTPError",
    "extractErrorInfoReasonFromHTTPError",
    "isGeminiAPIHardQuotaError",
    "isHardQuotaExhaustedMessage",
    "hasErrorInfoReason",
    "classifyGenAIAPIError",
    "ParseAgentScriptItem",
    "CleanJSONContent",
    "resolveJSONConfig",
    "resolvePathToURI",
    "resolveAgentPaths",
    "resolveProjectPaths",
    "addWorkspaceFolders",
    "discoverAgentCustomizations",
    "discoverPaths",
    "getWorkingDirContexts",
    "computeWorkingDirContexts",
    "discoverFromContexts",
    "statCached",
    "ResolveDepotPath",
    "ParseAbsolutePath",
    "EvalSymlinks",
    "Path.Join",
    "path.Join",
    "path.Clean",
    "NewManager",
)


class EvidenceError(ValueError):
    """Raised when an artifact or mapping is ambiguous or has the wrong identity."""


def unique_function(functions: list[dict[str, Any]], exact_name: str) -> dict[str, Any]:
    matches = [function for function in functions if function["name"] == exact_name]
    if len(matches) != 1:
        raise EvidenceError(
            f"expected one function named {exact_name!r}; found {len(matches)}"
        )
    return matches[0]


def unique_suffix(functions: list[dict[str, Any]], suffix: str) -> dict[str, Any]:
    matches = [function for function in functions if function["name"].endswith(suffix)]
    if len(matches) != 1:
        raise EvidenceError(f"expected one function ending {suffix!r}; found {len(matches)}")
    return matches[0]


def optional_suffix(functions: list[dict[str, Any]], suffix: str) -> dict[str, Any] | None:
    matches = [function for function in functions if function["name"].endswith(suffix)]
    if len(matches) > 1:
        raise EvidenceError(f"expected at most one function ending {suffix!r}; found {len(matches)}")
    return matches[0] if matches else None


def verify_digest(data: bytes, expected: str, label: str) -> str:
    actual = hashlib.sha256(data).hexdigest()
    if actual != expected:
        raise EvidenceError(f"{label} SHA-256 mismatch: expected {expected}, got {actual}")
    return actual


def parse_macho(data: bytes) -> tuple[list[dict[str, Any]], dict[str, Any]]:
    if len(data) < 32 or data[:4] != b"\xcf\xfa\xed\xfe":
        raise EvidenceError("expected a little-endian 64-bit Mach-O executable")
    cpu_type = struct.unpack_from("<I", data, 4)[0]
    if cpu_type != 0x0100000C:
        raise EvidenceError(f"expected arm64 Mach-O; CPU type was 0x{cpu_type:08x}")

    command_count = struct.unpack_from("<I", data, 16)[0]
    command_offset = 32
    sections: list[dict[str, Any]] = []
    for _ in range(command_count):
        if command_offset + 8 > len(data):
            raise EvidenceError("truncated Mach-O load command")
        command, command_size = struct.unpack_from("<II", data, command_offset)
        if command_size < 8 or command_offset + command_size > len(data):
            raise EvidenceError("invalid Mach-O load-command size")
        if command == 0x19:  # LC_SEGMENT_64
            section_count = struct.unpack_from("<I", data, command_offset + 64)[0]
            section_offset = command_offset + 72
            for _ in range(section_count):
                if section_offset + 80 > command_offset + command_size:
                    raise EvidenceError("truncated Mach-O section table")
                section_name = data[section_offset : section_offset + 16].split(b"\0", 1)[0]
                segment_name = data[section_offset + 16 : section_offset + 32].split(b"\0", 1)[0]
                address, size = struct.unpack_from("<QQ", data, section_offset + 32)
                file_offset = struct.unpack_from("<I", data, section_offset + 48)[0]
                if file_offset + size > len(data) and size:
                    raise EvidenceError("Mach-O section extends past the file")
                sections.append(
                    {
                        "segment": segment_name.decode("ascii", "replace"),
                        "name": section_name.decode("ascii", "replace"),
                        "address": address,
                        "size": size,
                        "file_offset": file_offset,
                    }
                )
                section_offset += 80
        command_offset += command_size

    text = unique_section(sections, "__text")
    pcln = unique_section(sections, "__lrodata_gopcln")
    header = parse_go_pcln(data, text, pcln)
    return sections, {"text": text, "pcln": pcln, "header": header}


def unique_section(sections: list[dict[str, Any]], name: str) -> dict[str, Any]:
    matches = [section for section in sections if section["name"] == name]
    if len(matches) != 1:
        raise EvidenceError(f"expected one Mach-O section {name!r}; found {len(matches)}")
    return matches[0]


def parse_go_pcln(
    data: bytes, text: dict[str, Any], pcln: dict[str, Any]
) -> dict[str, Any]:
    start = pcln["file_offset"]
    end = start + pcln["size"]
    candidates: list[tuple[int, tuple[int, ...]]] = []
    cursor = start
    while True:
        cursor = data.find(b"\xf1\xff\xff\xff", cursor, end)
        if cursor < 0:
            break
        if cursor + 72 <= end and data[cursor + 7] == 8:
            values = struct.unpack_from("<QQQQQQQQ", data, cursor + 8)
            nfunc, _nfiles, _textstart, nameoff, cuoff, fileoff, pctaboff, pclnoff = values
            if (
                1000 < nfunc < 300000
                and all(value < pcln["size"] for value in (nameoff, cuoff, fileoff, pctaboff, pclnoff))
                and pclnoff + 8 * nfunc <= pcln["size"]
            ):
                candidates.append((cursor, values))
        cursor += 1
    if len(candidates) != 1:
        raise EvidenceError(f"expected one Go pcHeader; found {len(candidates)}")

    header_offset, values = candidates[0]
    nfunc, _nfiles, _textstart, nameoff, _cuoff, _fileoff, _pctaboff, pclnoff = values
    functions: list[dict[str, Any]] = []
    previous_entry = -1
    for index in range(nfunc):
        table_offset = header_offset + pclnoff + index * 8
        pc, func_offset = struct.unpack_from("<II", data, table_offset)
        func_data = header_offset + pclnoff + func_offset
        if func_data + 8 > end:
            raise EvidenceError(f"Go function table entry {index} points outside pclntab")
        entry_offset, name_offset = struct.unpack_from("<Ii", data, func_data)
        if index and entry_offset <= previous_entry:
            raise EvidenceError("Go function entries are not strictly ordered")
        previous_entry = entry_offset
        if entry_offset >= text["size"]:
            raise EvidenceError(f"Go function {index} lies outside __text")
        name_start = header_offset + nameoff + name_offset
        name_end = data.find(b"\0", name_start, end)
        if name_end < 0:
            raise EvidenceError(f"Go function {index} has an unterminated name")
        next_pc = (
            struct.unpack_from("<I", data, table_offset + 8)[0]
            if index + 1 < nfunc
            else text["size"]
        )
        if next_pc <= entry_offset or next_pc > text["size"]:
            raise EvidenceError(f"Go function {index} has an invalid extent")
        name = data[name_start:name_end].decode("utf-8", "replace")
        functions.append(
            {
                "name": name,
                "address": text["address"] + entry_offset,
                "end": text["address"] + next_pc,
                "size": next_pc - entry_offset,
                "entry_offset": entry_offset,
                "pc_table_offset": pc,
            }
        )
    return {"function_count": nfunc, "header_file_offset": header_offset, "functions": functions}


def function_at(functions: list[dict[str, Any]], starts: list[int], address: int) -> str | None:
    index = bisect.bisect_right(starts, address) - 1
    if index >= 0 and address < functions[index]["end"]:
        return functions[index]["name"]
    return None


def direct_call_graph(
    data: bytes, text: dict[str, Any], functions: list[dict[str, Any]]
) -> dict[str, list[str]]:
    starts = [function["address"] for function in functions]
    graph: dict[str, list[str]] = {}
    for function in functions:
        raw_start = text["file_offset"] + function["address"] - text["address"]
        raw_end = raw_start + function["size"]
        raw = data[raw_start:raw_end]
        calls: list[str] = []
        for offset in range(0, len(raw) - 3, 4):
            instruction = struct.unpack_from("<I", raw, offset)[0]
            if instruction & 0xFC000000 != 0x94000000:  # ARM64 BL only; BR/BLR are indirect.
                continue
            immediate = instruction & 0x03FFFFFF
            if immediate & (1 << 25):
                immediate -= 1 << 26
            target = function_at(functions, starts, function["address"] + offset + (immediate << 2))
            if target is not None:
                calls.append(target)
        graph[function["name"]] = calls
    return graph


def reachable(graph: dict[str, list[str]], start: str, target: str) -> bool:
    seen: set[str] = set()
    pending = [start]
    while pending:
        current = pending.pop()
        if current == target:
            return True
        if current in seen:
            continue
        seen.add(current)
        pending.extend(graph.get(current, ()))
    return False


def require_edge(graph: dict[str, list[str]], source: str, target: str) -> None:
    if target not in graph.get(source, ()):
        raise EvidenceError(f"missing direct call edge: {source} -> {target}")


def section_string_references(
    data: bytes,
    sections: list[dict[str, Any]],
    text: dict[str, Any],
    functions: list[dict[str, Any]],
    marker: bytes,
) -> dict[str, list[int]]:
    starts = [function["address"] for function in functions]
    string_addresses: set[int] = set()
    cursor = 0
    while True:
        cursor = data.find(marker, cursor)
        if cursor < 0:
            break
        section = next(
            (
                item
                for item in sections
                if item["file_offset"] <= cursor < item["file_offset"] + item["size"]
            ),
            None,
        )
        if section is not None:
            string_addresses.add(section["address"] + cursor - section["file_offset"])
        cursor += 1

    instructions: dict[int, list[int]] = {}
    code = data[text["file_offset"] : text["file_offset"] + text["size"]]
    for offset in range(0, len(code) - 4, 4):
        instruction = struct.unpack_from("<I", code, offset)[0]
        if instruction & 0x9F000000 != 0x90000000:  # ADRP
            continue
        register = instruction & 31
        immhi = (instruction >> 5) & 0x7FFFF
        immlo = (instruction >> 29) & 0x3
        immediate = (immhi << 2) | immlo
        if immediate & (1 << 20):
            immediate -= 1 << 21
        page = (text["address"] + offset) & ~0xFFF
        base = page + (immediate << 12)
        for add_offset in range(offset + 4, min(offset + 20, len(code)), 4):
            add = struct.unpack_from("<I", code, add_offset)[0]
            if add & 0x7F000000 == 0x11000000 and ((add >> 5) & 31) == register:
                low = (add >> 10) & 0xFFF
                if (add >> 22) & 1:
                    low <<= 12
                instructions.setdefault(base + low, []).append(text["address"] + offset)
                break

    found: dict[str, list[int]] = {}
    for address in sorted(string_addresses):
        for pc in instructions.get(address, ()):
            owner = function_at(functions, starts, pc)
            if owner is not None:
                found.setdefault(owner, []).append(pc)
    return found


def expected_identities() -> dict[str, dict[str, Any]]:
    baseline = json.loads(BASELINE_IDENTITY.read_text())
    inventory = json.loads(NEWER_INVENTORY.read_text())
    identities = {
        "1.2.11": {
            "binary_sha256": baseline["official"]["assets"]["mac_arm64"][
                "extracted_cli_sha256"
            ],
            "binary_size": baseline["official"]["assets"]["mac_arm64"][
                "extracted_cli_size"
            ],
            "archive_sha256": baseline["official"]["assets"]["mac_arm64"]["sha256"],
            "tag_commit": baseline["official"]["github_commit"],
        }
    }
    for version in VERSIONS[1:]:
        item = inventory["versions"][version]
        member = item["mac_arm64"]["archive_members"]
        if len(member) != 1 or member[0]["path"] != "antigravity":
            raise EvidenceError(f"{version} inventory does not have one mapped antigravity file")
        identities[version] = {
            "binary_sha256": member[0]["sha256"],
            "binary_size": member[0]["size"],
            "archive_sha256": item["mac_arm64"]["download_sha256"],
            "tag_commit": item["tag_commit"],
        }
    return identities


def analyze_artifact(version: str, path: Path, expected: dict[str, Any]) -> dict[str, Any]:
    if not path.is_file() or path.is_symlink():
        raise EvidenceError(f"{version} artifact must be a regular non-symlink file: {path}")
    data = path.read_bytes()
    digest = verify_digest(data, expected["binary_sha256"], version)
    if len(data) != expected["binary_size"]:
        raise EvidenceError(f"{version} byte size mismatch: expected {expected['binary_size']}, got {len(data)}")
    sections, image = parse_macho(data)
    functions = image["header"]["functions"]
    graph = direct_call_graph(data, image["text"], functions)

    def exact(suffix: str) -> dict[str, Any]:
        return unique_suffix(functions, suffix)

    mapped = {
        "main": exact("main.main"),
        "entrypoint": exact("google3/third_party/jetski/cli/entrypoints/entrypoints.launchCLI"),
        "project_paths": exact("google3/third_party/jetski/cli/entrypoints/entrypoints.resolveProjectPaths"),
        "add_workspace_folders": exact("google3/third_party/jetski/cli/entrypoints/entrypoints.addWorkspaceFolders"),
        "print_runner": exact(RUNNER_SUFFIX[version]),
        "stream_input": exact("google3/third_party/jetski/cli/printmode/printmode.runStreamInput"),
        "new_session": exact("google3/third_party/jetski/cli/printmode/printmode.newSession"),
        "apply_agent_mode": exact("google3/third_party/jetski/cli/printmode/printmode.applyAgentMode"),
        "store_select_agent": exact("google3/third_party/jetski/cli/store/store.(*Manager).SelectAgentByName"),
        "session_turn": exact("google3/third_party/jetski/cli/printmode/printmode.(*session).runTurn"),
        "store_send_user_message": exact("google3/third_party/jetski/cli/store/store.(*Manager).SendUserMessage"),
        "server_send_user_message": exact("google3/third_party/jetski/cli/backend/backend.(*ServerBackend).SendUserMessage"),
        "server_get_agents": exact("google3/third_party/jetski/cli/backend/backend.(*ServerBackend).GetAgentCustomizations"),
        "agent_manager": exact("google3/third_party/jetski/cortex/customizations/customizations.(*Manager).GetAgentCustomizations"),
        "agent_discovery": exact("google3/third_party/jetski/cortex/customizations/customizations.(*Manager).getAgentCustomizations"),
        "manager_discover_agent_customizations": exact("google3/third_party/jetski/cortex/customizations/customizations.(*Manager).discoverAgentCustomizations"),
        "manager_discover_paths": exact("google3/third_party/jetski/cortex/customizations/customizations.(*Manager).discoverPaths"),
        "manager_discover_paths_core": exact("google3/third_party/jetski/cortex/customizations/customizations.(*Manager).discoverPathsCore"),
        "manager_discover_paths_core_with_contexts": exact("google3/third_party/jetski/cortex/customizations/customizations.(*Manager).discoverPathsCoreWithContexts"),
        "manager_get_working_dir_contexts": exact("google3/third_party/jetski/cortex/customizations/customizations.(*Manager).getWorkingDirContexts"),
        "manager_compute_working_dir_contexts": exact("google3/third_party/jetski/cortex/customizations/customizations.(*Manager).computeWorkingDirContexts"),
        "manager_discover_from_contexts": exact("google3/third_party/jetski/cortex/customizations/customizations.(*Manager).discoverFromContexts"),
        "agent_script_item": exact("google3/third_party/jetski/cortex/customizations/customizations.getAgentScriptItem"),
        "retry_loader": unique_function(functions, RETRY_LOADER),
        "backend_build": unique_function(functions, BACKEND_BUILD),
        "retry_auth_chain": exact("google3/third_party/jetski/cli/backend/backend.(*ServerBackendConfig).chainedAuthOrDefault"),
        "retry_classifier": exact("google3/third_party/jetski/cortex/utils/utils.IsRetryableAPIError"),
        "api_retry_loop": unique_function(functions, API_RETRY_SUFFIX[version]),
        "planner_api_retry": exact("google3/third_party/gemini_coder/framework/generator/generator.(*PlannerGenerator).generateWithAPIRetry"),
        "agent_get": exact("google3/third_party/jetski/cortex/customizations/customizations.(*Manager).GetDefinedAgentByName"),
        "agent_load_md": exact("google3/third_party/jetski/cortex/customizations/customizations.loadMDAgent"),
        "agent_path_entry": exact("google3/third_party/jetski/cortex/customizations/customizations.resolveAgentPaths"),
        "agent_path_resolver": exact("google3/third_party/jetski/cortex/customizations/customizations.resolveAgentPathsWithBase"),
        "agent_path_uri": exact("google3/third_party/jetski/cortex/customizations/customizations.resolvePathToURI"),
        "config_entry": exact("google3/third_party/jetski/cortex/customizations/customizations.resolveEntry"),
        "config_directory": exact("google3/third_party/jetski/cortex/customizations/customizations.resolveDir"),
        "load_config": exact("google3/third_party/jetski/cortex/customizations/customizations.loadConfigFile"),
        "resolve_json_entry": exact("google3/third_party/jetski/cortex/customizations/customizations.resolveJSONConfig.func1"),
        "permission_config": exact("google3/third_party/jetski/cli/backend/backend.(*ServerBackend).populatePermissionConfig"),
        "permission_grant_factory": exact("google3/third_party/jetski/cortex/permissions/permissions.NewPermissionGrantStore"),
        "permission_grant_parser": exact("google3/third_party/jetski/cortex/permissions/permissions.grantsFromConfig"),
        "permission_manager": exact("google3/third_party/jetski/cortex/permissions/permissions.(*permissionManager).EnsurePermissions"),
        "permission_callback": exact("google3/third_party/jetski/cortex/permissions/permissions.(*permissionManager).EnsurePermissions.func1"),
        "path_join": exact("google3/third_party/jetski/fs/fs.Path.Join"),
    }
    default_agent_path = optional_suffix(
        functions,
        "google3/third_party/jetski/cortex/customizations/customizations.setDefaultAgentPath",
    )
    if (version == "1.3.1") != (default_agent_path is not None):
        raise EvidenceError(f"{version} setDefaultAgentPath presence does not match the frozen hop map")
    if default_agent_path is not None:
        mapped["set_default_agent_path"] = default_agent_path
    custom_manager_init = optional_suffix(
        functions,
        "google3/third_party/jetski/cli/backend/backend.(*ServerBackend).newCustomizationManagerWithOptions",
    )
    if version in ("1.3.0", "1.3.1") and custom_manager_init is None:
        raise EvidenceError(f"{version} custom-manager initialization presence does not match the frozen hop map")
    if custom_manager_init is not None:
        mapped["custom_manager_init"] = custom_manager_init

    mapped_names = {item["name"] for item in mapped.values() if item is not None}
    direct_calls = {
        role: [
            target
            for target in graph[item["name"]]
            if target in mapped_names
            or any(marker in target for marker in SELECTED_CALL_MARKERS)
        ]
        for role, item in mapped.items()
        if item is not None
    }
    checks = [
        ("entrypoint", "print_runner"),
        ("entrypoint", "project_paths"),
        ("entrypoint", "add_workspace_folders"),
        ("print_runner", "stream_input"),
        ("manager_discover_paths", "manager_discover_paths_core"),
        ("manager_discover_paths_core", "manager_get_working_dir_contexts"),
        ("manager_discover_paths_core", "manager_discover_paths_core_with_contexts"),
        ("manager_discover_paths_core_with_contexts", "manager_discover_from_contexts"),
        ("agent_discovery", "manager_discover_agent_customizations"),
    ]
    for source_role, target_role in checks:
        require_edge(graph, mapped[source_role]["name"], mapped[target_role]["name"])
    require_edge(
        graph,
        mapped["backend_build"]["name"],
        mapped["retry_loader"]["name"],
    )
    require_edge(
        graph,
        mapped["agent_load_md"]["name"],
        exact("google3/third_party/jetski/cortex/customizations/customizations.resolveAgentPaths")["name"],
    )
    require_edge(
        graph,
        mapped["agent_get"]["name"],
        mapped["agent_script_item"]["name"],
    )
    require_edge(
        graph,
        mapped["agent_script_item"]["name"],
        mapped["agent_load_md"]["name"],
    )
    require_edge(
        graph,
        mapped["agent_path_entry"]["name"],
        mapped["agent_path_resolver"]["name"],
    )
    require_edge(
        graph,
        mapped["agent_path_resolver"]["name"],
        mapped["agent_path_uri"]["name"],
    )
    require_edge(
        graph,
        mapped["server_get_agents"]["name"],
        mapped["agent_manager"]["name"],
    )
    require_edge(
        graph,
        mapped["agent_manager"]["name"],
        mapped["agent_discovery"]["name"],
    )
    if custom_manager_init is not None:
        require_edge(
            graph,
            custom_manager_init["name"],
            exact("path/filepath.EvalSymlinks")["name"],
        )
    require_edge(
        graph,
        mapped["config_entry"]["name"],
        mapped["config_directory"]["name"],
    )
    require_edge(
        graph,
        mapped["load_config"]["name"],
        exact("google3/third_party/jetski/cortex/customizations/customizations.resolveJSONConfig")["name"],
    )
    require_edge(
        graph,
        mapped["resolve_json_entry"]["name"],
        mapped["config_entry"]["name"],
    )
    require_edge(
        graph,
        mapped["permission_grant_factory"]["name"],
        mapped["permission_grant_parser"]["name"],
    )
    require_edge(
        graph,
        mapped["new_session"]["name"],
        mapped["apply_agent_mode"]["name"],
    )
    require_edge(
        graph,
        mapped["new_session"]["name"],
        mapped["store_select_agent"]["name"],
    )
    require_edge(
        graph,
        mapped["session_turn"]["name"],
        mapped["store_send_user_message"]["name"],
    )
    require_edge(
        graph,
        mapped["server_send_user_message"]["name"],
        exact("google3/third_party/jetski/cli/backend/backend.(*ServerBackend).buildCascadeConfigAndAgentSpec")["name"],
    )
    if version in ("1.2.16", "1.2.17", "1.3.0", "1.3.1"):
        require_edge(
            graph,
            mapped["config_directory"]["name"],
            mapped["path_join"]["name"],
        )
    if default_agent_path is not None:
        require_edge(graph, mapped["agent_load_md"]["name"], default_agent_path["name"])

    for role in ("entrypoint", "print_runner", "stream_input", "session_turn"):
        if not reachable(graph, mapped["entrypoint"]["name"], mapped[role]["name"]):
            raise EvidenceError(f"{version} selected headless direct-call path cannot reach {role}")

    retry_xrefs = section_string_references(
        data, sections, image["text"], functions, b"AGY_CLI_MODEL_API_MAX_RETRIES"
    )
    retry_references = retry_xrefs.get(RETRY_LOADER, [])
    if sum(map(len, retry_xrefs.values())) != 1 or len(retry_references) != 1:
        raise EvidenceError(
            f"{version} retry marker must have one code reference in the shared loader"
        )
    key_xrefs = section_string_references(data, sections, image["text"], functions, b"GEMINI_API_KEY")
    key_reference_functions = {
        name: pcs
        for name, pcs in key_xrefs.items()
        if any(token in name for token in ("defaultEnvVarProvider", "getAPIKeyFromEnv", "chainedAuthOrDefault"))
    }
    required_key_paths = (
        "defaultEnvVarProvider",
        "getAPIKeyFromEnv",
        "chainedAuthOrDefault",
    )
    if not all(any(token in name for name in key_reference_functions) for token in required_key_paths):
        raise EvidenceError(f"{version} is missing a mapped GEMINI_API_KEY auth path")

    return {
        "version": version,
        "tag_commit": expected["tag_commit"],
        "archive_sha256": expected["archive_sha256"],
        "binary_sha256": digest,
        "binary_size": len(data),
        "architecture": "Mach-O arm64",
        "go_function_count": image["header"]["function_count"],
        "pclntab_header_file_offset": image["header"]["header_file_offset"],
        "mapped_functions": {
            role: {
                "name": function["name"],
                "address": f"0x{function['address']:x}",
                "size": function["size"],
                "code_sha256": hashlib.sha256(
                    data[
                        image["text"]["file_offset"]
                        + function["address"]
                        - image["text"]["address"] : image["text"]["file_offset"]
                        + function["end"]
                        - image["text"]["address"]
                    ]
                ).hexdigest(),
            }
            for role, function in mapped.items()
            if function is not None
        },
        "selected_direct_calls": direct_calls,
        "retry_marker_reference_pc": f"0x{retry_references[0]:x}",
        "gemini_api_key_reference_functions": {
            name: [f"0x{pc:x}" for pc in pcs] for name, pcs in sorted(key_reference_functions.items())
        },
        "required_key_paths": list(required_key_paths),
        "unproven_indirect_handoffs": [
            {
                "from": mapped["store_send_user_message"]["name"],
                "to": mapped["server_send_user_message"]["name"],
                "direct_call_reachable": reachable(
                    graph,
                    mapped["store_send_user_message"]["name"],
                    mapped["server_send_user_message"]["name"],
                ),
            },
            {
                "from": mapped["backend_build"]["name"],
                "to": mapped["api_retry_loop"]["name"],
                "direct_call_reachable": reachable(
                    graph,
                    mapped["backend_build"]["name"],
                    mapped["api_retry_loop"]["name"],
                ),
            },
            {
                "from": mapped["new_session"]["name"],
                "to": mapped["agent_get"]["name"],
                "direct_call_reachable": reachable(
                    graph,
                    mapped["new_session"]["name"],
                    mapped["agent_get"]["name"],
                ),
            },
            {
                "from": mapped["permission_manager"]["name"],
                "to": mapped["permission_callback"]["name"],
                "direct_call_reachable": reachable(
                    graph,
                    mapped["permission_manager"]["name"],
                    mapped["permission_callback"]["name"],
                ),
            },
        ],
        "static_limits": [
            "The decoder follows direct ARM64 BL edges; indirect BLR and RPC dispatch are recorded as unresolved boundaries.",
            "The strings are addressed as names only; no environment values are read or logged.",
        ],
    }


def analyze(root: Path) -> dict[str, Any]:
    if not root.is_dir():
        raise EvidenceError(f"artifact root is not a directory: {root}")
    identities = expected_identities()
    artifacts = []
    for version in VERSIONS:
        artifacts.append(
            analyze_artifact(version, root / version / "antigravity", identities[version])
        )
    return {
        "schema": 1,
        "method": "Hash and stat each official extracted Mac ARM64 executable; parse its Mach-O and Go pclntab tables; decode direct ARM64 BL calls and ADRP+ADD literal references without executing the file.",
        "source_identity": {
            "baseline": "frozen Research 353 identity fixture for 1.2.11",
            "newer": "frozen Research 380 dist-inventory.json for 1.2.12 through 1.3.1",
        },
        "artifact_count": len(artifacts),
        "artifacts": artifacts,
    }


def run_self_tests() -> None:
    def must_fail(action: Any, case: str) -> None:
        try:
            action()
        except EvidenceError:
            return
        raise EvidenceError(f"self-test failed to refuse {case}")

    one = [{"name": "pkg.run", "address": 10, "end": 20}]
    must_fail(lambda: unique_function(one + one, "pkg.run"), "ambiguous function identity")
    must_fail(lambda: verify_digest(b"wrong", "0" * 64, "synthetic"), "wrong artifact identity")
    if reachable({"root": ["middle"], "isolated": ["leaf"]}, "root", "leaf"):
        raise EvidenceError("self-test failed to refuse an unreachable function")
    must_fail(
        lambda: require_edge({"root": ["middle"]}, "root", "missing"),
        "unreachable direct call edge",
    )
    must_fail(lambda: parse_macho(b"not a Mach-O"), "wrong binary format")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("artifact_root", nargs="?", type=Path)
    parser.add_argument("output", nargs="?", type=Path)
    parser.add_argument("--self-test", action="store_true")
    arguments = parser.parse_args()
    try:
        if arguments.self_test:
            run_self_tests()
            print("synthetic refusal cases passed; no provider evidence was generated")
            return 0
        if arguments.artifact_root is None:
            parser.error("artifact_root is required unless --self-test is used")
        result = analyze(arguments.artifact_root)
        rendered = json.dumps(result, indent=2, sort_keys=True) + "\n"
        if arguments.output is None:
            sys.stdout.write(rendered)
        else:
            arguments.output.parent.mkdir(parents=True, exist_ok=True)
            arguments.output.write_text(rendered)
            print(f"wrote {len(result['artifacts'])} exact artifact mappings to {arguments.output}")
    except (EvidenceError, OSError, KeyError, ValueError, struct.error) as error:
        print(f"static boundary analysis failed closed: {error}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
