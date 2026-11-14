#!/usr/bin/env python3
"""Source-only ACP admission preparation. No vendor loading or original command."""

import argparse
import ast
import hashlib
import json
import os
from pathlib import Path
import platform
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]
FIXTURE = (
    ROOT
    / "crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-agent-acp-0.106.0"
)
GRAPHS = (
    ROOT
    / "crates/swallowtail-adapter-claude-agent/sidecar/claude-agent-acp-guard-graphs"
)
HOPS = (
    "0.101.0",
    "0.101.1",
    "0.102.0",
    "0.103.0",
    "0.104.0",
    "0.104.1",
    "0.105.0",
    "0.106.0",
)
PYTHON_INPUT = "docs/knowledge/operations/claude-agent-acp-python-inputs.json"
SHARED = "crates/swallowtail-adapter-claude-agent/tests/guard_shared_software/reviewed"
INPUTS = [
    PYTHON_INPUT,
    "scripts/claude-acp-admission.py",
    "scripts/tests/claude-acp-admission.py",
    "crates/swallowtail-adapter-claude-agent/sidecar/claude-agent-acp-guard-loader.mjs",
    "crates/swallowtail-host-local/src/captured_source.rs",
    "crates/swallowtail-adapter-claude-agent/tests/acp_guard_shared_software.rs",
    "crates/swallowtail-adapter-claude-agent/tests/guard_shared_software/host.py",
    SHARED + "/loader.mjs",
    SHARED + "/bootstrap-native.mjs",
    SHARED + "/entry.py",
    SHARED + "/proposal.py",
    "crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-agent-acp-0.106.0/registry.json",
    "crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-agent-acp-0.106.0/import-edge-inventory.json",
    "crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-agent-acp-0.106.0/native-permission-control.json",
]


def digest(file):
    p = Path(file)
    if not p.is_file() or p.stat().st_size > 64 * 1024 * 1024:
        raise ValueError("bounded regular input unavailable: " + str(p))
    return hashlib.sha256(p.read_bytes()).hexdigest()


def canonical(value):
    return json.dumps(value, sort_keys=True, separators=(",", ":")).encode()


def inspect_python(executable):
    """Read selected installed files/linkage only; never execute the target."""
    if platform.system() != "Darwin" or platform.machine() != "arm64":
        raise ValueError("Darwin arm64 inspection only; no platform claim")
    executable = executable.resolve(strict=True)
    suffix = "Resources/Python.app/Contents/MacOS/Python"
    if not str(executable).endswith("/" + suffix):
        raise ValueError("exact framework-app layout required")
    framework = executable.parents[4]
    stdlib = framework / "lib/python3.9"
    if not stdlib.is_dir():
        raise ValueError("explicit Python3.9 stdlib layout unavailable")
    files, links, missing = {}, [], []

    def file_record(p):
        info = p.stat()
        return {
            "sha256": digest(p),
            "bytes": info.st_size,
            "uid": info.st_uid,
            "gid": info.st_gid,
            "mode": info.st_mode,
        }

    pending = [executable]
    while pending:
        binary = pending.pop().resolve(strict=True)
        if str(binary).startswith(("/System/", "/usr/lib/")):
            continue
        key = str(binary.relative_to(framework))
        if key in files:
            continue
        if len(files) >= 8:
            raise ValueError("selected Mach-O inspection limit")
        files[key] = file_record(binary)

        def otool(flag):
            result = subprocess.run(
                ["/usr/bin/otool", flag, str(binary)],
                env={"PATH": "/usr/bin:/bin"},
                stdin=subprocess.DEVNULL,
                capture_output=True,
                timeout=5,
                check=True,
            )
            if len(result.stdout) + len(result.stderr) > 65536:
                raise ValueError("linkage output limit")
            return result.stdout.decode()

        # -L includes a dylib's LC_ID_DYLIB. Identify it with -D instead of
        # falsely treating its own @rpath install-name as a missing dependency.
        identities = {
            line.strip()
            for line in otool("-D").splitlines()
            if line.startswith(("@", "/")) and not line.endswith(":")
        }
        linkage = otool("-L")
        for line in linkage.splitlines():
            if not line.startswith("\t"):
                continue
            name = line.strip().split(" (", 1)[0]
            if name in identities:
                continue
            if name.startswith(("/System/", "/usr/lib/")):
                edge = {
                    "parent": key,
                    "installName": name,
                    "class": "trusted-system-loader-input; not extracted/attested",
                }
            else:
                if name.startswith("@executable_path/"):
                    target = executable.parent / name[len("@executable_path/") :]
                elif name.startswith("@loader_path/"):
                    target = binary.parent / name[len("@loader_path/") :]
                elif name.startswith("@"):
                    row = {
                        "parent": key,
                        "installName": name,
                        "gate": "UNRESOLVED_PYTHON_MACHO_SEARCH",
                    }
                    if row not in missing:
                        missing.append(row)
                    continue
                else:
                    target = Path(name)
                target = target.resolve(strict=True)
                if not target.is_relative_to(framework):
                    raise ValueError("non-system dependency outside selected framework")
                edge = {
                    "parent": key,
                    "installName": name,
                    "target": str(target.relative_to(framework)),
                    "class": "non-system framework dependency; inventory only",
                }
                pending.append(target)
            if edge not in links:
                links.append(edge)

    modules = set()
    source_files = [
        ROOT / SHARED / "entry.py",
        ROOT / SHARED / "proposal.py",
        ROOT
        / "crates/swallowtail-adapter-claude-agent/tests/guard_shared_software/host.py",
    ]
    for source in source_files:
        for node in ast.walk(ast.parse(source.read_bytes())):
            if isinstance(node, ast.Import):
                modules.update(x.name for x in node.names)
            elif isinstance(node, ast.ImportFrom) and node.module:
                modules.add(node.module)
    if len(modules) > 40:
        raise ValueError("direct Python import limit")
    direct, cache = {}, {}
    for module in sorted(modules):
        parts = module.split(".")
        candidates = []
        for count in range(1, len(parts) + 1):
            relative = Path(*parts[:count])
            candidates.extend(
                [
                    stdlib / relative.with_suffix(".py"),
                    stdlib / relative / "__init__.py",
                ]
            )
        selected = [p for p in candidates if p.is_file()]
        direct[module] = {
            "scope": "literal module and package-prefix candidates; not transitive/runtime selection",
            "sources": {
                str(p.relative_to(framework)): file_record(p) for p in selected
            },
        }
        for source in selected:
            for p in sorted(
                (source.parent / "__pycache__").glob(source.stem + ".cpython-39*.pyc")
            ):
                cache[str(p.relative_to(framework))] = file_record(p)
    if len(cache) > 128:
        raise ValueError("direct bytecode candidate limit")
    return {
        "schema": "swallowtail.acp227.python-inputs.v1",
        "inspectionSourceSha256": digest(Path(__file__)),
        "directEntrySourceSha256": {
            str(p.relative_to(ROOT)): digest(p) for p in source_files
        },
        "scope": "bounded read-only direct inputs, not complete runtime custody",
        "frameworkLayout": "Xcode Python3.framework/Versions/3.9",
        "interpreter": str(executable.relative_to(framework)),
        "machOFiles": files,
        "machOLinks": links,
        "directImportRoots": sorted({name.split(".")[0] for name in modules}),
        "directImportCandidates": direct,
        "observedBytecodeCandidates": cache,
        "stdlibZipCandidate": {
            "relativePath": "lib/python39.zip",
            "present": (framework / "lib/python39.zip").is_file(),
        },
        "startup": ["-I", "-B", "-S"],
        "bytecodeRule": "-B prevents writes, not reads; existing cache and zip selection remain host inputs",
        "unresolved": missing
        + [
            "CPYTHON_TRANSITIVE_IMPORT_EXTENSION_AND_BYTECODE_CUSTODY",
            "CPYTHON_PREFIX_ZIP_PATH_CONFIGURATION",
            "TRUSTED_HOST_SYSTEM_LOADER_IDENTITY_AND_UPDATE_CUSTODY",
        ],
        "admitted": False,
    }


def read_json(file):
    def unique(pairs):
        value = {}
        for key, item in pairs:
            if key in value:
                raise ValueError("duplicate preparation key")
            value[key] = item
        return value

    return json.loads(Path(file).read_bytes(), object_pairs_hook=unique)


def build():
    observation = read_json(ROOT / PYTHON_INPUT)
    if (
        observation.get("schema") != "swallowtail.acp227.python-inputs.v1"
        or observation.get("admitted") is not False
    ):
        raise ValueError("Python inventory cannot grant admission")
    if observation.get("inspectionSourceSha256") != digest(Path(__file__)):
        raise ValueError(
            "Python inspection source changed; fresh read-only inventory required"
        )
    for source, expected in observation["directEntrySourceSha256"].items():
        if digest(ROOT / source) != expected:
            raise ValueError("Python entry-source inventory changed")
    registry = json.loads((FIXTURE / "registry.json").read_bytes())["versions"]
    hops = []
    for hop in HOPS:
        path = GRAPHS / (hop + ".json")
        graph = json.loads(path.read_bytes())
        sdk = registry[hop]["dependencies"]["@anthropic-ai/claude-agent-sdk"]
        edges = [
            {
                **e,
                "classification": "mediated-path-only"
                if e["target"] == "node:child_process"
                else "caller-bound module facade"
                if e["target"] == "node:module"
                else "ambient process capability; raw binding/execve not proved prohibited",
            }
            for e in graph["builtins"]
            if e["target"] in ("node:child_process", "node:module", "node:process")
        ]
        hops.append(
            {
                "hop": hop,
                "graphSha256": digest(path),
                "vendorSha256": graph["vendor"],
                "sdk": sdk,
                "selectedFiles": len(graph["files"]),
                "staticClosureSha256": hashlib.sha256(
                    canonical({k: graph[k] for k in ("files", "imports", "builtins")})
                ).hexdigest(),
                "executionImportEdges": edges,
                "nativeCallers": graph["native_parents"],
                "requireParents": graph["require_parents"],
                "platformPrograms": graph["platform_programs"],
                "nativeImages": graph["native"],
                "dynamicCautions": graph["source_cautions"],
                "unselectedImports": graph["denied_dynamic"],
            }
        )
    return {
        "schema": "swallowtail.acp227.admission-preparation.v1",
        "acceptedSoftwareHead": "5bb3579c3824763fd9b3339d0707c4a820c74281",
        "acceptedReview": 5904122860,
        "sharedSourceHead": "f9025d2197333c53aed2266627dfa39d05bff7e3",
        "inputs": {p: digest(ROOT / p) for p in INPUTS},
        "hops": hops,
        "edgeClasses": {
            "startup": "Checked host binds prefix/runtime/source identities and rejects unsafe startup environment before spawn. This is not a kernel payload sandbox.",
            "capturedImport": "Exact retained ESM/CJS/JSON static closure; resolution misses refuse. This is not complete semantic/native closure.",
            "childProcess": "Production facade mediates held native path/hash and bounded caller ps; permits spawn/spawnSync/execFile/execFileSync. Exact argv/env/cwd/stdio/detached policy, one-shot and pre-entry release are NOT supplied by path matching.",
            "fakeRecipe": "Accepted first-party recipe substitutes f902 caller-bound one-shot facade and Python entry. It does not attest the original SDK launch argv or native behavior.",
            "acpCallers": "All four child_process import parents per hop require selected-call/argv classification; graph reachability cannot label ACP auth/status/index callers unreachable.",
            "workerVm": "Absent worker_threads/vm module edges refuse captured resolution. This proves no original-native/ambient-process worker prohibition.",
            "dynamicCode": "Node startup disallows string code generation. Function cautions remain selected-compatibility unknowns; do not relax flags from fake success.",
            "addons": "Node --no-addons is required; original embedded native addon/dynamic-loader behavior remains opaque.",
            "rawProcess": "getBuiltinModule is disabled by production bootstrap, but process.binding/_linkedBinding/execve are not established blocked there. Broad normal builtins remain capabilities; no hostile-JavaScript sandbox claim.",
            "native": "Frozen native SHA/size by hop/platform identifies bytes only. Descendant exec, workers, dynamic code and permitted-executable argv bypass remain opaque/unadmitted; no further native disassembly is inferred.",
        },
        "pythonInputs": observation,
        "compositionDelta": {
            "owner": "swallowtail#234",
            "frozenPacketMutation": False,
            "pendingDecision": "78e36a73-4254-42a2-b872-71235fbdf4f9",
            "actors": [
                {"role": "prepared-owner", "parent": "host monitor"},
                {"role": "group-owner", "parent": "prepared-owner", "group": "own PID"},
                {
                    "role": "checked-acp",
                    "parent": "prepared-owner",
                    "group": "group-owner PID",
                },
                {
                    "role": "native-entry",
                    "parent": "checked-acp",
                    "group": "group-owner PID",
                },
            ],
            "requirements": [
                "Host observes actual PID/parent/group/start/launch before release; stand-in parent==group-owner is invalid for ACP.",
                "Admit each selected original caller and exact one-attempt argv/env/cwd/native identity before payload effects; cover permitted-executable bypass.",
                "Kernel task budget counts threads and encloses helper, checked ACP and descendants; deny migration/controller changes before untrusted entry.",
                "Transport and canonical absent/disabled/managed-deny fixtures must compose with actual prepared ACP on the chosen platform.",
                "Keep native/driver/helper software waits, adoptee rights, pidfd/cgroup settlement and fixture cleanup separate.",
                "Bind ACP installed compatible Node and Python inputs independently of SDK22 evidence; preserve trusted-host/system-loader boundary.",
            ],
        },
        "remainingGates": [
            "SELECTED_ORIGINAL_CALLER_ARGV_AND_PREENTRY_ADMISSION",
            "OPAQUE_NATIVE_EXEC_WORKER_LOADER_CLOSURE",
            "PYTHON_COMPLETE_NON_SYSTEM_INPUT_CUSTODY",
            "ACTUAL_SIBLING_KERNEL_TRANSPORT_CANONICAL_FIXTURE_ADMISSION",
            "SEPARATE_EXACT_ORIGINAL_EXECUTION_AUTHORITY",
        ],
        "originalCommand": None,
        "qualifiedCeiling": "0.100.0",
        "admission": "closed",
    }


def verify(record):
    if record != build():
        raise ValueError("admission preparation record changed or incomplete")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("mode", choices=["inspect-python", "build", "verify"])
    parser.add_argument("--python-executable", type=Path)
    parser.add_argument("--record", type=Path)
    parser.add_argument("--output", type=Path)
    args = parser.parse_args()
    if args.mode == "inspect-python":
        if not args.python_executable or not args.output:
            parser.error("exact Python executable/output required")
        value = inspect_python(args.python_executable)
    else:
        if args.mode == "verify":
            if not args.record:
                parser.error("record required")
            verify(read_json(args.record))
            print(
                "Exact source/edge/runtime preparation matches; admission stays closed"
            )
            return
        if not args.output:
            parser.error("output required")
        value = build()
    args.output.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n")
    print(
        json.dumps(
            {
                "mode": args.mode,
                "outputSha256": digest(args.output),
                "admission": "closed",
            }
        )
    )


if __name__ == "__main__":
    main()
