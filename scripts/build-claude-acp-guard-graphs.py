#!/usr/bin/env python3
"""Reconstruct guarded manifests from frozen syntax and verified package bytes.

Reads only source and metadata; never imports or executes a vendor module.
Input directories use the keys from import-edge-inventory.json.
"""
import argparse
import hashlib
import json
import posixpath
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
FIXTURE = ROOT / "crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-agent-acp-0.106.0"
HOPS = ("0.101.0", "0.101.1", "0.102.0", "0.103.0", "0.104.0", "0.104.1", "0.105.0", "0.106.0")
BUILTINS = set("async_hooks buffer child_process crypto events fs fs/promises http http2 https module net os path path/win32 process readline stream stream/web string_decoder timers/promises url util zlib".split())


def export_target(value, mode):
    if isinstance(value, str):
        return value
    if isinstance(value, dict):
        for key in (mode, "node", "default"):
            if key in value:
                result = export_target(value[key], mode)
                if result:
                    return result
    return None


def build(inputs, hop):
    inventory = json.loads((FIXTURE / "import-edge-inventory.json").read_text())["packages"]
    registry = json.loads((FIXTURE / "registry.json").read_text())["versions"][hop]
    prefixes = {"@agentclientprotocol/claude-agent-acp": "acp", "@agentclientprotocol/sdk": "acpsdk", "@anthropic-ai/claude-agent-sdk": "agentsdk", "diff": "diff", "zod": "zod"}
    keys = {"acp": hop, **{prefixes[name]: name.replace("/", "_").replace("@", "_") + "_" + version for name, version in registry["dependencies"].items()}}
    retained = json.loads((FIXTURE / "guard-package-metadata.json").read_text())
    metadata = {}
    for prefix, key in keys.items():
        raw = (inputs / key / "package.json").read_bytes() if inputs else None
        if raw is not None and hashlib.sha256(raw).hexdigest() != retained[key]["sha256"]:
            raise ValueError("package metadata identity mismatch: " + key)
        metadata[prefix] = retained[key]["metadata"]
    files, edges, builtins, denied, cautions = {}, set(), set(), [], []

    def resolve(parent, specifier, mode):
        if specifier.removeprefix("node:") in BUILTINS:
            return "node:" + specifier.removeprefix("node:")
        prefix, relative = parent.split("/", 1)
        if specifier.startswith("."):
            target = posixpath.normpath(posixpath.join(posixpath.dirname(relative), specifier))
            if target.startswith("../"):
                raise ValueError("escaping relative import")
            return prefix + "/" + target
        for name, target_prefix in prefixes.items():
            if specifier == name or specifier.startswith(name + "/"):
                subpath = "." + specifier[len(name):]
                exports = metadata[target_prefix].get("exports", {})
                value = exports.get(subpath) if isinstance(exports, dict) else exports if subpath == "." else None
                if value is None and isinstance(exports, dict):
                    for pattern, candidate in exports.items():
                        if "*" in pattern:
                            before, after = pattern.split("*", 1)
                            if subpath.startswith(before) and subpath.endswith(after):
                                replacement = subpath[len(before):len(subpath)-len(after) if after else None]
                                value = json.loads(json.dumps(candidate).replace("*", replacement))
                                break
                target = export_target(value, mode)
                if target is None and subpath == ".":
                    target = metadata[target_prefix].get("main")
                if target is None:
                    return None
                return target_prefix + "/" + posixpath.normpath(target.removeprefix("./"))
        return None

    pending = ["acp/dist/index.js"]
    while pending:
        path = pending.pop()
        if path in files:
            continue
        prefix, relative = path.split("/", 1)
        row = inventory[keys[prefix]].get(relative)
        if inputs:
            digest = hashlib.sha256((inputs / keys[prefix] / relative).read_bytes()).hexdigest()
        elif row:
            digest = row["sha256"]
        elif relative == "package.json":
            digest = retained[keys[prefix]]["sha256"]
        else:
            raise ValueError("unrecorded JSON source")
        if row and row["sha256"] != digest:
            raise ValueError("source identity mismatch: " + path)
        if not row and not relative.endswith(".json"):
            raise ValueError("missing syntax inventory: " + path)
        module_format = "json" if relative.endswith(".json") else "commonjs" if relative.endswith(".cjs") or (relative.endswith(".js") and metadata[prefix].get("type") != "module") else "module"
        files[path] = {"sha256": digest, "format": module_format}
        if not row:
            continue
        for edge in row["edges"]:
            target = resolve(path, edge["specifier"], "require" if module_format == "commonjs" else "import")
            if target is None:
                if edge["kind"] == "dynamic":
                    denied.append({"parent": path, **edge, "policy": "unselected-optional-import-refused"})
                    continue
                raise ValueError(f"unresolved required import: {path} {edge}")
            if target.startswith("node:"):
                builtins.add((path, edge["specifier"], target))
            else:
                edges.add((path, edge["specifier"], target))
                pending.append(target)
        cautions.extend({"parent": path, **row} for row in row["cautions"])
    # The bundled SDK's first createRequire alias has one literal fs call for
    # WSL detection. Its second alias resolves the exact platform native. The
    # wrapper explicitly creates a require rooted at the captured SDK export.
    builtins.add(("agentsdk/sdk.mjs", "fs", "node:fs"))
    sdk = registry["dependencies"]["@anthropic-ai/claude-agent-sdk"]
    native_ledger = json.loads((FIXTURE / "dependencies.json").read_text())["native_manifests"][sdk]["platforms"]
    natives = {platform: {"sha256":native_ledger[platform]["checksum"], "bytes":native_ledger[platform]["size"], "specifier":"@anthropic-ai/claude-agent-sdk-"+platform+"/claude"} for platform in ("darwin-arm64", "linux-x64")}
    return {"vendor":registry["tarball_sha256"], "entry": "acp/dist/index.js", "idle": "acp/dist/idle-compaction.js", "files": dict(sorted(files.items())), "imports": [dict(zip(("parent", "specifier", "target"), row)) for row in sorted(edges)], "builtins": [dict(zip(("parent", "specifier", "target"), row)) for row in sorted(builtins)], "require_parents": [["acp/dist/claude-cli.js", "agentsdk/sdk.mjs"]], "native_parents": ["agentsdk/sdk.mjs"], "native": natives, "platform_programs": ([{"parent":"acp/dist/session-index/live-registry.js","name":"ps","arguments":"live-process-starts-v1"}] if "acp/dist/session-index/live-registry.js" in files else []), "denied_dynamic": sorted(denied, key=lambda row: (row["parent"], row["offset"])), "source_cautions": sorted(cautions, key=lambda row: (row["parent"], row["offset"], row["kind"]))}


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("inputs", type=Path, nargs="?")
    parser.add_argument("output", type=Path, nargs="?")
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    if args.check:
        proposal = json.loads((FIXTURE / "guarded-probe-proposal.json").read_text())
        for source, expected in proposal["component_stage"]["source_sha256"].items():
            if hashlib.sha256((ROOT / source).read_bytes()).hexdigest() != expected:
                raise ValueError("probe source drift: " + source)
        for hop in HOPS:
            expected = build(None, hop)
            actual = json.loads((ROOT / "crates/swallowtail-adapter-claude-agent/sidecar/claude-agent-acp-guard-graphs" / (hop + ".json")).read_text())
            if actual != expected:
                raise ValueError("independent graph reconstruction differs: " + hop)
        print("eight complete guarded graphs reconstructed from frozen source/import/metadata inventories")
        return
    if not args.inputs or not args.output:
        parser.error("inputs and output required outside --check")
    args.output.mkdir(exist_ok=True)
    for hop in HOPS:
        result = build(args.inputs, hop)
        (args.output / (hop + ".json")).write_text(json.dumps(result, sort_keys=True, separators=(",", ":")) + "\n")
        print(hop, len(result["files"]), len(result["imports"]), len(result["builtins"]), len(result["denied_dynamic"]))


if __name__ == "__main__":
    main()
