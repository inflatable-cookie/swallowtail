#!/usr/bin/env python3
"""Pinned ACP callsite evidence. Parses source data, never loads vendor modules."""

import argparse
import base64
import hashlib
import io
import json
import os
from pathlib import Path, PurePosixPath
import re
import subprocess
import tarfile

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


def sha(data):
    return hashlib.sha256(data).hexdigest()


def read_json(file):
    def unique(pairs):
        value = {}
        for key, item in pairs:
            if key in value:
                raise ValueError("duplicate JSON key")
            value[key] = item
        return value

    return json.loads(file.read_bytes(), object_pairs_hook=unique)


def plan():
    registry = read_json(FIXTURE / "registry.json")["versions"]
    artifacts = read_json(FIXTURE / "dependencies.json")["artifacts"]
    extras = read_json(FIXTURE / "import-edge-artifacts.json")
    metadata = read_json(FIXTURE / "guard-package-metadata.json")
    inventory = read_json(FIXTURE / "import-edge-inventory.json")["packages"]
    packages, hops = {}, []
    for hop in HOPS:
        graph = read_json(GRAPHS / (hop + ".json"))
        sdk = registry[hop]["dependencies"]["@anthropic-ai/claude-agent-sdk"]
        keys = {
            "acp": hop,
            "agentsdk": "_anthropic-ai_claude-agent-sdk_" + sdk,
            "zod": "zod_4.6.7",
        }
        identities = {
            "acp": registry[hop],
            "agentsdk": artifacts["@anthropic-ai/claude-agent-sdk@" + sdk],
            "zod": {
                **extras["zod_4.6.7"],
                "tarball_sha256": extras["zod_4.6.7"]["sha256"],
            },
        }
        selected = {}
        for prefix, key in keys.items():
            row = identities[prefix]
            files = {
                p.split("/", 1)[1]: v["sha256"]
                for p, v in graph["files"].items()
                if p.startswith(prefix + "/")
            }
            files["package.json"] = metadata[key]["sha256"]
            if key not in packages:
                packages[key] = {
                    "url": row["tarball"],
                    "integrity": row["integrity"],
                    "sha256": row["tarball_sha256"],
                    "files": {},
                    "memberLimit": 4 * (len(inventory[key]) + 1),
                }
            packages[key]["files"].update(files)
            selected.update(
                {
                    p: key + "/" + p.split("/", 1)[1]
                    for p in graph["files"]
                    if p.startswith(prefix + "/")
                }
            )
        hops.append(
            {
                "hop": hop,
                "graphSha256": sha((GRAPHS / (hop + ".json")).read_bytes()),
                "selected": selected,
                "childProcessParents": sorted(
                    e["parent"]
                    for e in graph["builtins"]
                    if e["target"] == "node:child_process"
                ),
            }
        )
    return packages, hops


def archive_sources(raw, expected):
    if len(raw) > 8 * 1024 * 1024:
        raise ValueError("compressed source bound")
    if (
        sha(raw) != expected["sha256"]
        or "sha512-" + base64.b64encode(hashlib.sha512(raw).digest()).decode()
        != expected["integrity"]
    ):
        raise ValueError("frozen archive identity mismatch")
    files, seen, total = {}, set(), 0
    with tarfile.open(fileobj=io.BytesIO(raw), mode="r:gz") as archive:
        for count, member in enumerate(archive, 1):
            p = PurePosixPath(member.name)
            if (
                count > expected.get("memberLimit", 8192)
                or str(p) in seen
                or p.is_absolute()
                or ".." in p.parts
                or not p.parts
                or p.parts[0] != "package"
                or not (member.isfile() or member.isdir())
                or "\\" in member.name
                or str(p) != member.name.rstrip("/")
                or member.sparse
            ):
                raise ValueError("unsafe, duplicate or excessive archive member")
            seen.add(str(p))
            total += member.size
            if total > 64 * 1024 * 1024 or member.size < 0:
                raise ValueError("expanded source bound")
            name = str(p.relative_to("package"))
            if member.isfile() and name in expected["files"]:
                data = archive.extractfile(member).read(member.size + 1)
                if len(data) != member.size or sha(data) != expected["files"][name]:
                    raise ValueError("selected regular-file identity mismatch: " + name)
                files[name] = data
    if set(files) != set(expected["files"]):
        raise ValueError("incomplete selected source set")
    return files


def inspect(sources):
    parser = ROOT / "scripts/claude-acp-call-sites.cjs"
    result = subprocess.run(
        ["node", str(parser)],
        input=json.dumps(sources),
        text=True,
        capture_output=True,
        timeout=120,
        check=True,
        env={**os.environ, "NODE_OPTIONS": "", "NODE_DISABLE_COMPILE_CACHE": "1"},
    )
    if len(result.stdout.encode()) > 8 * 1024 * 1024:
        raise ValueError("parser output bound")
    return json.loads(result.stdout)


def source_view(text, needle, before=180, after=600):
    start = text.find(needle)
    if start < 0:
        raise ValueError("source anchor missing: " + needle)
    lo, hi = max(0, start - before), min(len(text), start + len(needle) + after)
    data = text[lo:hi].encode()
    return {
        "needle": needle,
        "startByte": len(text[:lo].encode()),
        "endByte": len(text[:hi].encode()),
        "sha256": sha(data),
        "text": data.decode(),
    }


def bundle_archives(bundles, packages):
    result = {}
    for bundle in bundles:
        if bundle.is_symlink() or bundle.stat().st_size > 8 * 1024 * 1024:
            raise ValueError("input bundle bound/type")
        with tarfile.open(bundle, "r:gz") as archive:
            for member in archive:
                key = member.name.removesuffix(".tgz")
                if (
                    not member.isfile()
                    or member.name != key + ".tgz"
                    or key not in packages
                    or key in result
                    or member.size > 8 * 1024 * 1024
                ):
                    raise ValueError("unknown, duplicate or unsafe source bundle entry")
                result[key] = archive.extractfile(member).read(member.size + 1)
                if sum(map(len, result.values())) > 32 * 1024 * 1024:
                    raise ValueError("combined bundle bound")
    if set(result) != set(packages):
        raise ValueError("incomplete source input bundle")
    return result


def build(inputs=None, bundles=()):
    packages, hops = plan()
    bundled = bundle_archives(bundles, packages) if bundles else None
    sources, receipt = {}, []
    for key, expected in sorted(packages.items()):
        raw = (
            bundled[key]
            if bundled is not None
            else (inputs / (key + ".tgz")).read_bytes()
        )
        files = archive_sources(raw, expected)
        sources.update(
            {
                key + "/" + name: data.decode()
                for name, data in files.items()
                if name.endswith((".js", ".mjs", ".cjs"))
            }
        )
        receipt.append(
            {
                "key": key,
                "url": expected["url"],
                "sha256": sha(raw),
                "integrity": expected["integrity"],
                "bytes": len(raw),
                "memberLimit": expected["memberLimit"],
                "selectedFiles": {
                    n: {"sha256": sha(b), "bytes": len(b)}
                    for n, b in sorted(files.items())
                },
            }
        )
    syntax = inspect(sources)
    for key, value in syntax["files"].items():
        for call in value["calls"]:
            if key.endswith("/dist/index.js"):
                call["policy"] = "cli-delegation"
            elif key.endswith("/dist/session-index/live-registry.js"):
                call["policy"] = "registry-ps"
            elif key.endswith("/sdk.mjs"):
                call["policy"] = (
                    "sdk-transport"
                    if call["imported"] == "spawn"
                    else "sdk-where"
                    if call["imported"] == "execFileSync"
                    else "sdk-other-exec"
                )
            else:
                call["policy"] = (
                    "auth-logout"
                    if '["auth", "logout"]' in call["expression"]["text"]
                    else "auth-status"
                )
    for hop in hops:
        selected = hop["selected"]
        actual = sorted(
            p
            for p, key in selected.items()
            if syntax["files"].get(key, {}).get("calls")
        )
        if actual != hop["childProcessParents"]:
            raise ValueError("invocation-parent coverage differs: " + hop["hop"])
    views = {}
    for key, text in sources.items():
        anchors = []
        if key.endswith("/dist/index.js"):
            anchors = [('process.argv.includes("--cli")', 0, 400)]
        elif key.endswith("/dist/claude-cli.js"):
            anchors = [("export async function claudeCliPath()", 0, 4200)]
        elif key.endswith("/dist/agent-command.js"):
            anchors = (
                [("export function cliArgs", 0, 300)]
                if "export function cliArgs" in text
                else []
            )
        elif key.endswith("/sdk.mjs"):
            anchors = [
                ("spawnLocalProcess(e)", 0, 650),
                ("pathToClaudeCodeExecutable:l", 150, 5600),
            ]
            # Minifier symbols differ between frozen SDKs; source-bound syntax
            # below retains every execFile caller and enclosing definition.
            anchors += [
                ('"/usr/bin/plutil"', 120, 420),
                ('Function("return this")', 150, 100),
            ]
            anchors += [
                ('.binding("util")', 250, 80),
                ('new Function("")', 170, 100),
                (".compile(e)", 220, 250),
            ]
            resolver = re.search(
                r"\.pathToClaudeCodeExecutable;if\(![A-Za-z_$][A-Za-z0-9_$]*\)", text
            )
            if resolver is None:
                raise ValueError("SDK resolver predicate missing")
            anchors.append((resolver.group(), 60, 1000))
        elif key.endswith(("/dist/acp-agent.js", "/dist/v1/acp-agent.js")):
            anchors = [
                ("const env = {", 0, 1500),
                ("pathToClaudeCodeExecutable: await claudeCliPath()", 3500, 1400),
                ("const q = query({", 200, 500),
            ]
        elif key.endswith("/v4/core/schemas.js"):
            anchors = [("const fastEnabled", 160, 1300)]
        elif key.endswith("/v4/core/util.js"):
            anchors = [("export const allowsEval", 0, 700)]
        elif key.endswith("/v4/core/compile.js"):
            anchors = [("Function", 180, 450)]
        if anchors:
            views[key] = [source_view(text, *a) for a in anchors]
    bootstrap = "crates/swallowtail-host-local/src/captured_source.rs"
    return {
        "schema": "swallowtail.acp239.source-flow.v1",
        "scope": "Exact selected ACP/Agent-SDK/Zod source calls, not generic JavaScript noninterference or runtime attestation; ACP protocol SDK/diff retain their frozen import inventories, not restored semantic closure",
        "inputs": {
            p: sha((ROOT / p).read_bytes())
            for p in (
                "scripts/claude-acp-source-flow.py",
                "scripts/claude-acp-call-sites.cjs",
                "scripts/tests/claude-acp-source-flow.py",
                "crates/swallowtail-adapter-claude-agent/sidecar/claude-agent-acp-guard-loader.mjs",
                bootstrap,
            )
        },
        "archives": receipt,
        "hops": hops,
        "syntax": syntax,
        "dataflowViews": views,
        "guardStartup": source_view(
            (ROOT / bootstrap).read_text(),
            "Object.defineProperty(process,'getBuiltinModule'",
            0,
            1150,
        ),
        "policies": POLICIES,
        "originalCommand": None,
        "admission": "closed",
        "qualifiedCeiling": "0.100.0",
    }


POLICIES = {
    "cli-delegation": {
        "classification": "proved-unreachable-with-no-cli-launch-argument",
        "predicate": "index executes spawn only under process.argv.includes('--cli'); actual original launch must bind argv without --cli",
        "command": "claudeCliPath(): env CLAUDE_CODE_EXECUTABLE, bundled directory, then SDK-rooted platform require.resolve",
        "argv": "process.argv.slice(2) filtering --cli; later cliArgs additionally filters --hide-claude-auth",
        "env": "omitted: Node process.env",
        "cwd": "omitted: Node process.cwd()",
        "stdio": "inherit",
        "detached": "omitted: Node default false",
        "mediation": "native path/hash check before original.spawn; no exact argv admission or one-shot credit",
    },
    "auth-status": {
        "classification": "precise-existing-mediation-before-effect; runtime-observation-required",
        "reachability": "initialize and session/prompt auth probes reach runProbe; gateway auth early return is conditional, not universal",
        "command": "await claudeCliPath()",
        "argv": "optional ['--settings',JSON.stringify(context.override)] then ['auth','status','--json']",
        "env": "context.env ? {...process.env,...context.env} : process.env",
        "cwd": "context.manager.getCwd() ?? process.cwd()",
        "stdio": "execFile buffered pipes/defaults",
        "detached": "omitted: Node default false",
        "timeoutMs": 5000,
        "mediation": "promisify(execFile) uses facade's custom symbol path, same native path/hash/env check; no selected-input/entry release attestation",
    },
    "auth-logout": {
        "classification": "precise-existing-mediation-before-effect; unselected-operation",
        "reachability": "explicit logout dispatch, not new/load/resume permission canonical proof; still a real reachable mapped caller",
        "command": "await claudeCliPath()",
        "argv": ["auth", "logout"],
        "env": "omitted: process.env",
        "cwd": "omitted: process.cwd()",
        "stdio": "execFile buffered pipes/defaults",
        "detached": "omitted: Node default false",
        "timeoutMs": None,
        "mediation": "same native facade; explicit logout/credentials effects not authorized by this source record",
    },
    "registry-ps": {
        "classification": "precise-existing-mediation-before-effect",
        "reachability": "LiveSessionRegistry.readFiles -> processStarts(needStarts) only for nonempty selected PIDs, not win32",
        "command": "ps translated to separately held ps.path",
        "argv": ["-o", "pid=,lstart=", "-p", "pids.join(',')"],
        "env": "{...process.env,LC_ALL:'C',TZ:'UTC'}",
        "cwd": "omitted: process.cwd()",
        "stdio": "execFile buffered pipes/defaults",
        "detached": "omitted: Node default false",
        "timeoutMs": 1000,
        "mediation": "caller-bound ps binding, four exact argv cells, decimal PID CSV <=4096 chars, timeout1000, then custody/hash/startup-env check",
    },
    "sdk-transport": {
        "classification": "precise-existing-mediation-before-effect; exact-runtime-inputs-required",
        "reachability": "ACP builds query options with native path and env; query creates ProcessTransport; custom spawn can override default only if supplied, never assume standalone SDK sidecar options",
        "command": "native extension test selects pathToClaudeCodeExecutable; script extension selects executable default bun/node",
        "argv": "native: [...executableArgs,...B]; script: [...executableArgs,path,...B]; B starts --output-format stream-json --verbose --input-format stream-json plus all selected option/extraArgs branches retained in dataflowViews",
        "env": "options.env or copy process.env; sets CLAUDE_CODE_ENTRYPOINT sdk-ts only if absent, deletes NODE_OPTIONS, normalizes DEBUG",
        "cwd": "options.cwd",
        "stdio": ["pipe", "pipe", "pipe"],
        "detached": "omitted: Node default false",
        "windowsHide": True,
        "mediation": "default spawn reaches facade path/hash/env check; dynamic option/extraArgs/env/cwd and custom callback require exact original pre-entry binding, not path-only credit",
    },
    "sdk-other-exec": {
        "classification": "precise-existing-mediation-before-effect; refused-nonnative-command",
        "git": "session/worktree enumeration: resolved git with -c core.hooksPath=/dev/null -c core.fsmonitor= worktree list --porcelain; cwd argument, timeout5000, windowsHide; process.env/default buffered pipes/detached false",
        "managed": "settings loader invokes /usr/bin/plutil -convert json -o - -- PATH then -lint -s -- PATH; timeout5000, encoding utf-8, optional maxBuffer2097152; Windows/WSL reg query HKLM/HKCU SOFTWARE\\Policies\\ClaudeCode /v Settings; inherited env/cwd/default pipes/detached false",
        "security": "Darwin sessionStore credential staging invokes security find-generic-password -a derivedAccount -w -s derivedService; timeout5000 encoding utf-8 windowsHide; inherited env/cwd/default pipes/detached false",
        "mediation": "these commands match neither held native nor caller-specific registry ps, so current facade refuses synchronously before OS spawn; caught failures/fallback may affect semantics and need original compatibility observation; do not silently allow utilities",
    },
    "sdk-where": {
        "classification": "proved-unreachable-on-selected-darwin-linux; otherwise-refused-before-effect",
        "versions": ["0.3.318", "0.3.319"],
        "command": "SYSTEMROOT or C:\\Windows joined System32/where.exe",
        "argv": "[requested executable name]",
        "env": "process.env",
        "cwd": "process.cwd() default",
        "stdio": "pipe",
        "detached": "default false",
        "timeoutMs": 5000,
        "reachability": "execFileSync Windows resolver only; platform predicate in source is win32; newer SDK removes this import; facade would refuse where.exe rather than admitting it",
    },
    "sdk-startup-builtin": {
        "classification": "proved-source-incompatible-with-current-throwing-bootstrap",
        "sitesPerSdk": 2,
        "versions": ["0.3.318", "0.3.319", "0.3.321", "0.3.322"],
        "call": "globalThis.process?.getBuiltinModule?.('async_hooks')",
        "effect": "module-scope optional calls have no surrounding catch; bootstrap defines a present throwing function, so optional chaining does not take either missing-API fallback; SDK evaluation throws before query",
        "next": "bootstrap-compatible adaptation needs planning disposition within shared bootstrap owner #240; this source record grants no change to startup policy or original execution",
    },
    "sdk-util-binding": {
        "classification": "proved-unreachable-under-standard-captured-esm-global-premise",
        "call": "Lodash generated CommonJS detector gates require('util').types then processAlias.binding('util')",
        "gate": "typeof undeclared module alias (n_ in318, o_ in319, b_ in321/322) is not object under normal Node ESM; namespace export identity also required; no module-alias binding declaration exists in these pinned files",
        "limits": "not an ambient process-binding prohibition; arbitrary injected globals and opaque native behavior remain outside this source premise",
    },
    "Function": {
        "classification": "source-bounded-fallback-or-explicit-runtime-gate",
        "prototype": "Function.prototype.call/bind/toString are not string generation",
        "zod": "SDK embedded and external Zod allowsEval probes catch Function denial; object schema fastpass requires allowsEval; Doc.compile and explicit z.compile still generate strings and remain denied/unselected compatibility gates",
        "sdkAjv": "Ajv Function compilation has no general interpreter fallback; getValidator(requestedSchema) path can require it; exact selected ACP/MCP compatibility under unchanged disallow flag remains observation-required",
        "lodash": "Function('return this') is short-circuited by normal Node global.Object===Object; this premise belongs to admitted startup, not arbitrary hostile globals",
        "startup": "retain --disallow-code-generation-from-strings; no vendor module evaluated by this analysis",
    },
    "ambient-native": {
        "classification": "exact-opaque-runtime-observation-required",
        "rawProcess": "syntax census records actual named/computed binding/_linkedBinding/execve/dlopen/getBuiltinModule/mainModule members; an empty census proves no explicit such member in restored pinned subset, not arbitrary computed property/constructor prohibition",
        "dynamic": "all ImportExpression/require/createRequire sites retained; module facade admits only exact captured parent/edge, native resolution returns held pathname",
        "workerVm": "graph has no worker_threads/vm import allowance; resolution refuses missing edge before loading; opaque native workers and arbitrary ambient capabilities remain unproved",
        "addon": "--no-addons remains required; no explicit native addon loader allowance; native descendant exec/threads/dynamic image loading/permitted-image argv bypass remain opaque",
        "shared": "actual sibling host/actors/transport/kernel/canonical fixtures and Python/non-system runtime custody remain #240-owned prerequisites; exact separate original authority still required",
    },
}


def verify(record, expected):
    if record != expected:
        raise ValueError("callsite/dataflow record differs from pinned reconstruction")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("mode", choices=["build", "verify"])
    sources = parser.add_mutually_exclusive_group(required=True)
    sources.add_argument("--inputs", type=Path)
    sources.add_argument("--bundle", type=Path, action="append", default=[])
    parser.add_argument("--output", type=Path)
    parser.add_argument("--record", type=Path)
    args = parser.parse_args()
    value = build(args.inputs, args.bundle)
    if args.mode == "verify":
        verify(read_json(args.record), value)
        print("Pinned invocation/dataflow matches; original admission remains closed")
    else:
        args.output.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n")
        print(
            json.dumps(
                {
                    "sha256": sha(args.output.read_bytes()),
                    "hops": len(value["hops"]),
                    "sourceFiles": len(value["syntax"]["files"]),
                }
            )
        )


if __name__ == "__main__":
    main()
