#!/usr/bin/env python3
"""Bounded selected-flow classification for `@anthropic-ai/claude-agent-sdk`.

The `claude-agent.sdk` route loads exactly one file from the published package:
the root export resolves to `sdk.mjs`, and `sdk.mjs` only imports Node
builtins. This script classifies the *selected* surfaces of that one file, so a
hop above the accepted pair can be described without open-ended reverse
engineering and without claiming anything about the bundled native binary.

It is evidence scaffolding, not a qualification. Running it never downloads a
package, never executes vendor code, and never contacts a provider. Every fact
it reports is bound to the exact bytes named by the committed fixture:

  crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-agent-sdk-0.3.322/

Modes
-----
  --corpus DIR   Extract hops from an already extracted package tree. DIR holds
                 `pkg-<version>/package/` directories, or pass repeated
                 `--tarball <version>=<path>` instead.
  --self-test    Exercise identity verification, masking, surface extraction and
                 the fail-closed paths against synthetic fixtures. No vendor
                 bytes are needed or read.

Fail-closed rules
-----------------
  * A hop whose file ledger does not reproduce the committed SHA-256 values is
    refused, so a substituted or re-packed tree cannot be classified.
  * A selected anchor that is absent, ambiguous (more than one definition), or
    larger than the bounded window is reported as `unclassified`, never as
    unchanged.
  * The output records the resolved entry point and the import closure; a
    dynamic import or a `core-*.mjs` edge makes the closure `unbounded`.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import pathlib
import re
import sys

FIXTURE = pathlib.Path(
    "crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-agent-sdk-0.3.322"
)

# The sidecar's selected option set, in the words of the shipped asset. These
# are reachability inputs: a surface only matters if this option set can reach
# it. Keep them in sync with `sidecar/claude-agent-sdk-sidecar.mjs`.
SIDECAR_SELECTED_OPTIONS = {
    "prompt": "async iterable, so streamInput is the selected input path",
    "pathToClaudeCodeExecutable": "set",
    "spawnClaudeCodeProcess": "set",
    "env": "explicit child environment, never the inherited process environment",
    "settingSources": "[] (user, project and local disk tiers disabled)",
    "skills": "[]",
    "plugins": "[]",
    "agents": "{}",
    "hooks": "{} (nothing sets onFailure, so hasHookSetToBlock() is false)",
    "canUseTool": "set (permission-prompt-tool=stdio)",
    "allowedTools": "never set",
    "disallowedTools": "set",
    "tools": "set",
    "strictMcpConfig": "true",
    "managedSettings": "never set",
    "settings": "never set",
    "includePartialMessages": "false",
}

# Every surface the classification depends on. `anchor` must occur exactly once
# for a definition-style surface; member anchors may occur more than once and
# are reported with all offsets.
SURFACES = (
    {
        "name": "query.promptHold.assignment",
        "anchor": "this.promptHold=this.initialization.then(",
        "kind": "member",
        "before": 700,
        "after": 700,
        "note": "the only assignment site of promptHold",
    },
    {
        "name": "query.writeBehindPromptHold",
        "anchor": "writeBehindPromptHold(e,t){",
        "kind": "definition",
        "before": 0,
        "after": 1200,
        "note": "the hold barrier every prompt-hold branch funnels through",
    },
    {
        "name": "query.writePrompt",
        "anchor": "writePrompt(e){",
        "kind": "definition",
        "before": 0,
        "after": 700,
        "note": "string-prompt write path; not selected by an iterable prompt",
    },
    {
        "name": "query.initializeWritten.gate",
        "anchor": "this.initializeWritten=new Promise(",
        "kind": "member",
        "before": 500,
        "after": 900,
        "note": "the initialize-written promise and its resolver",
    },
    {
        "name": "query.streamInput",
        "anchor": "async streamInput(e){",
        "kind": "definition",
        "before": 0,
        "after": 2600,
        "note": "the selected input path for this route",
    },
    {
        "name": "query.writeStreamMessage",
        "anchor": "writeStreamMessage(e){",
        "kind": "definition",
        "before": 0,
        "after": 400,
        "note": "run-state reset plus transport write",
    },
    {
        "name": "query.endRun",
        "anchor": "endRun(){",
        "kind": "definition",
        "before": 0,
        "after": 700,
        "note": "run-end settlement",
    },
    {
        "name": "query.armRunEndCeiling",
        "anchor": "armRunEndCeiling(){",
        "kind": "definition",
        "before": 0,
        "after": 900,
        "note": "run-end ceiling",
    },
    {
        "name": "query.processControlRequest.can_use_tool",
        "anchor": 'e.request.subtype==="can_use_tool"',
        "kind": "member",
        "before": 260,
        "after": 2200,
        "note": "the native permission request the callback answers",
    },
    {
        "name": "transport.argv.base",
        "anchor": '"--output-format","stream-json"',
        "kind": "member",
        "before": 200,
        "after": 3400,
        "note": "native argv construction",
    },
    {
        "name": "transport.argv.managed_settings",
        "anchor": '"--managed-settings"',
        "kind": "member",
        "before": 200,
        "after": 260,
        "note": "SDK-option parent tier on the native argv",
    },
    {
        "name": "transport.argv.await_initialize_gate",
        "anchor": '"await-initialize"',
        "kind": "member",
        "before": 900,
        "after": 400,
        "note": "the gate that keeps --await-initialize off this route",
    },
    {
        "name": "settings.managed_projection",
        "anchor": "denyInsteadOfPrompting===!0",
        "kind": "member",
        "before": 1400,
        "after": 900,
        "note": "restrictive managed-policy projection of the parent tier",
    },
    {
        "name": "settings.resolve.sources",
        "anchor": "parentManaged:",
        "kind": "member",
        "before": 700,
        "after": 700,
        "note": "how settingSources and Options.managedSettings become tiers",
    },
    {
        "name": "settings.restrictive_key_table",
        "anchor": '["permissions","denyInsteadOfPrompting"]',
        "kind": "member",
        "before": 300,
        "after": 300,
        "note": "the managed-policy key table entry",
    },
    {
        "name": "telemetry.key_sanitizer_function",
        "anchor": "cleaned_key",
        "kind": "enclosing_function",
        "before": 0,
        "after": 0,
        "note": "the gated recursive key sanitizer added at 0.3.321",
    },
    {
        "name": "telemetry.key_sanitizer_legacy",
        "anchor": "!==\"__proto__\")t[o]=",
        "kind": "enclosing_function",
        "before": 0,
        "after": 0,
        "note": "the ungated sanitizer the earlier hops carry",
    },
    {
        "name": "telemetry.sanitizer_gate",
        "anchor": "cleaned_key",
        "kind": "member",
        "before": 460,
        "after": 120,
        "note": "the telemetry-gated numeric-key condition inside the sanitizer",
    },
    {
        "name": "telemetry.numeric_key_test",
        "anchor": "(?:0|[1-9]\\d{0,9})",
        "kind": "member",
        "before": 160,
        "after": 220,
        "note": "numeric-key predicate used by the sanitizer gate",
    },
    {
        "name": "telemetry.limits_gate",
        "anchor": ".onPast(",
        "kind": "member",
        "before": 220,
        "after": 160,
        "note": "telemetry limits gate the sanitizer consults",
    },
)

# Which surfaces the sidecar's option set can reach. `selected` values are
# source-proven from the two files; `native` marks behavior that lives in the
# bundled binary and cannot be classified from these bytes.
REACHABILITY = {
    "query.streamInput": ("selected", "iterable prompt is written through streamInput"),
    "query.writeStreamMessage": ("selected", "streamInput writes each message here"),
    "query.endRun": ("selected", "run-end settlement is observed by the driver"),
    "query.armRunEndCeiling": ("selected", "ceiling is armed for every turn"),
    "query.processControlRequest.can_use_tool": (
        "selected",
        "canUseTool is set, so the SDK answers a native can_use_tool request here",
    ),
    "query.writePrompt": (
        "unselected",
        "string-prompt only; the sidecar passes an async iterable",
    ),
    "query.promptHold.assignment": (
        "unselected",
        "assigned only under hasHookSetToBlock(), which hooks:{} cannot satisfy",
    ),
    "query.writeBehindPromptHold": (
        "unselected",
        "every call site is gated on promptHold being defined",
    ),
    "query.initializeWritten.gate": (
        "selected",
        "streamInput awaits it before the first write; resolved by the first initialize",
    ),
    "transport.argv.base": ("selected", "every native spawn uses this argv"),
    "transport.argv.managed_settings": (
        "unselected",
        "emitted only for Options.managedSettings, which the sidecar never sets",
    ),
    "transport.argv.await_initialize_gate": (
        "unselected",
        "requires both pathToClaudeCodeExecutable and spawnClaudeCodeProcess to be unset",
    ),
    "settings.managed_projection": (
        "native",
        "the parent tier is empty for this route, but the native reads its own "
        "administrator tier, so managed policy is not excluded by these bytes",
    ),
    "settings.resolve.sources": (
        "selected",
        "settingSources maps only user, project and local; the administrator tier "
        "is not one of them",
    ),
    "settings.restrictive_key_table": (
        "native",
        "the key is a managed-policy key; honor and bypass behavior are native",
    ),
    "telemetry.key_sanitizer_function": (
        "unselected",
        "its callers are the tag and custom-title writes, not the turn path",
    ),
    "telemetry.key_sanitizer_legacy": (
        "unselected",
        "its callers are the tag and custom-title writes, not the turn path",
    ),
    "telemetry.sanitizer_gate": (
        "unselected",
        "only consulted from the sanitizer, whose callers are control-path writes",
    ),
    "telemetry.numeric_key_test": (
        "unselected",
        "only consulted from the sanitizer gate",
    ),
    "telemetry.limits_gate": (
        "unselected",
        "only consulted from the sanitizer gate",
    ),
}

TOKEN = re.compile(
    r"[A-Za-z_$][A-Za-z0-9_$]*|\d[\dA-Za-z_.]*"
    r'|"(?:[^"\\\n]|\\.)*"|\'(?:[^\'\\\n]|\\.)*\'|`(?:[^`\\]|\\.)*`|\s+|.',
    re.S,
)
IDENTIFIER = re.compile(r"[A-Za-z_$][A-Za-z0-9_$]*")
GATE_SHAPE = re.compile(
    r'[A-Za-z_$][A-Za-z0-9_$]*!=="__proto__"&&\([^\n]{0,200}?cleaned_key"\)'
)


class Refused(Exception):
    """A fail-closed classification refusal."""


def sha256_bytes(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def mask(source: str) -> str:
    """Identifiers, string literals and numbers become opaque markers.

    Member names survive after a `.`, because the selected surfaces of this
    route are member names. Whitespace collapses so minifier layout cannot
    masquerade as a change.
    """
    out = []
    tokens = TOKEN.findall(source)
    for index, token in enumerate(tokens):
        previous = tokens[index - 1] if index else ""
        before_previous = tokens[index - 2] if index > 1 else ""
        member_access = previous == "." and before_previous != "."
        if token.isspace():
            out.append(" ")
        elif token[0] in "\"'`":
            out.append('"S"')
        elif token[0].isdigit():
            out.append("0")
        elif IDENTIFIER.fullmatch(token):
            out.append(token if member_access else "ID")
        else:
            out.append(token)
    return "".join(out)


def collapsed(text: str) -> str:
    return re.sub(r"\s+", " ", text).strip()


def brace_body(source: str, start: int) -> tuple[int, str]:
    """Return `(end, body)` for the brace-balanced body at or after `start`."""
    opening = source.index("{", start)
    depth = 0
    index = opening
    while index < len(source):
        character = source[index]
        if character == "{":
            depth += 1
        elif character == "}":
            depth -= 1
            if depth == 0:
                return index + 1, source[start:index + 1]
        index += 1
    raise Refused("unbalanced braces in selected surface")


def mask_tokens(text: str) -> list[str]:
    """The masked token sequence of a window, minus its final partial token.

    A byte-bounded window can cut the last token in half at different places
    once upstream code shifts, so the final token is dropped before comparison.
    """
    tokens = [token for token in mask(text).split(" ") if token]
    return tokens[:-1] if tokens else tokens


def enclosing_function(source: str, offset: int) -> tuple[int, int] | None:
    """The byte range of the nearest `function` declaration before `offset`."""
    keyword = source.rfind("function ", 0, offset)
    if keyword < 0:
        return None
    try:
        end, _body = brace_body(source, keyword)
    except Refused:
        return None
    return keyword, end


def statement_window(
    source: str, offset: int, anchor: str, surface: dict
) -> tuple[int, int, bool]:
    """Align a member window to statement boundaries so upstream shifts
    cannot move what the window contains.

    The window starts just after the previous `;`, `{` or `}` and ends just
    after the next `;` at bracket depth zero. If either side would exceed the
    declared byte budget, the byte-bounded window is used instead and the
    caller reports `statement_aligned: false`.
    """
    lower = max(0, offset - surface["before"])
    boundary = max(
        source.rfind(";", lower, offset),
        source.rfind("{", lower, offset),
        source.rfind("}", lower, offset),
    )
    start = boundary + 1 if boundary >= 0 else lower
    limit = min(len(source), offset + len(anchor) + surface["after"])
    depth = 0
    index = offset + len(anchor)
    end = limit
    while index < limit:
        character = source[index]
        if character in "([{":
            depth += 1
        elif character in ")]}":
            depth = max(0, depth - 1)
        elif character == ";" and depth == 0:
            end = index + 1
            break
        index += 1
    if end - start > surface["before"] + len(anchor) + surface["after"]:
        return lower, limit, False
    return start, end, True


def extract(source: str, surface: dict) -> dict:
    anchor = surface["anchor"]
    offsets = [match.start() for match in re.finditer(re.escape(anchor), source)]
    if not offsets:
        return {"status": "absent", "offsets": [], "occurrences": 0}
    if surface["kind"] == "definition" and len(offsets) != 1:
        return {"status": "ambiguous", "offsets": offsets, "occurrences": len(offsets)}
    windows = []
    for offset in offsets:
        if surface["kind"] == "definition":
            start = offset
            try:
                end, text = brace_body(source, offset)
            except Refused:
                return {"status": "unbalanced", "offsets": offsets, "occurrences": len(offsets)}
            bounded = True
        elif surface["kind"] == "enclosing_function":
            span = enclosing_function(source, offset)
            if span is None:
                return {"status": "unbounded", "offsets": offsets, "occurrences": len(offsets)}
            start, end = span
            text = source[start:end]
            if len(text) > 12000:
                return {
                    "status": "unbounded",
                    "offsets": offsets,
                    "occurrences": len(offsets),
                    "bytes": len(text),
                }
            bounded = True
        else:
            start, end, bounded = statement_window(source, offset, anchor, surface)
            text = source[start:end]
        if len(text) > 12000:
            return {
                "status": "unbounded",
                "offsets": offsets,
                "occurrences": len(offsets),
                "bytes": len(text),
            }
        tokens = mask_tokens(text)
        windows.append(
            {
                "offset": start,
                "bytes": len(text),
                "statement_aligned": bounded,
                "masked_sha256": sha256_bytes(collapsed(" ".join(tokens)).encode()),
                "masked_multiset_sha256": sha256_bytes(
                    collapsed(" ".join(sorted(tokens))).encode()
                ),
            }
        )
    sequences = [window["masked_sha256"] for window in windows]
    multisets = [window["masked_multiset_sha256"] for window in windows]
    return {
        "status": "classified",
        "offsets": offsets,
        "occurrences": len(offsets),
        "masked_sequence_sha256": sequences,
        "masked_multiset_sha256": multisets,
        "windows": windows,
    }


NODE_BUILTINS = {
    "assert", "async_hooks", "buffer", "child_process", "cluster", "console",
    "constants", "crypto", "dgram", "diagnostics_channel", "dns", "domain",
    "events", "fs", "fs/promises", "http", "http2", "https", "inspector",
    "module", "net", "os", "path", "perf_hooks", "process", "punycode",
    "querystring", "readline", "repl", "stream", "string_decoder", "sys",
    "timers", "tls", "trace_events", "tty", "url", "util", "v8", "vm",
    "wasi", "worker_threads", "zlib",
}

# Bundled fourth-party edges reached only by a code path this route never
# selects. Recorded so the closure report is honest about them.
SELECTED_AWAY_REQUIRE = [
    "ajv/dist/runtime/equal",
    "ajv/dist/runtime/ucs2length",
    "ajv/dist/runtime/uri",
    "ajv/dist/runtime/validation_error",
    "ajv-formats/dist/formats",
]


def import_closure(source: str) -> dict:
    specifiers = set(re.findall(r'from\s*"([^"]+)"', source))
    node_builtins = sorted(
        value for value in specifiers if value.startswith("node:") or value in NODE_BUILTINS
    )
    vendor = sorted(
        value
        for value in specifiers
        if not value.startswith("node:") and value not in NODE_BUILTINS
    )
    requires = sorted(set(re.findall(r'\brequire\s*\(\s*"([^"]+)"', source)))
    dynamic = len(re.findall(r"\bimport\s*\(", source))
    chunks = sorted(set(re.findall(r'"(\./core-[^"]+\.mjs)"', source)))
    return {
        "static_node_builtins": node_builtins,
        "static_vendor_specifiers": vendor,
        "dynamic_import_sites": dynamic,
        "literal_require_specifiers": requires,
        "literal_require_selected_away": all(
            value in SELECTED_AWAY_REQUIRE or value in NODE_BUILTINS
            for value in requires
        ),
        "core_chunk_edges": chunks,
        "import_meta_sites": len(re.findall(r"import\.meta", source)),
        "bounded": dynamic == 0 and not chunks,
    }


def resolve_entry(package_root: pathlib.Path) -> dict:
    manifest = json.loads((package_root / "package.json").read_text())
    exports = manifest.get("exports", {})
    root = exports.get(".") if isinstance(exports, dict) else None
    if isinstance(root, dict):
        target = root.get("import") or root.get("default") or root.get("require")
    else:
        target = root
    if not isinstance(target, str):
        raise Refused("package root export does not resolve to one file")
    entry = package_root / target.lstrip("./")
    if not entry.is_file():
        raise Refused(f"root export {target!r} is not a packaged file")
    return {"specifier": target, "file": entry.name}


def load_corpus(args) -> dict[str, pathlib.Path]:
    corpus: dict[str, pathlib.Path] = {}
    if args.corpus:
        root = pathlib.Path(args.corpus)
        for directory in sorted(root.glob("pkg-*/package")):
            version = directory.parent.name.removeprefix("pkg-")
            corpus[version] = directory
    for spec in args.tarball or []:
        version, _, path = spec.partition("=")
        if not version or not path:
            raise Refused(f"--tarball expects <version>=<path>, got {spec!r}")
        import tarfile

        directory = pathlib.Path(args.extract_to or ".").resolve() / f"pkg-{version}/package"
        directory.mkdir(parents=True, exist_ok=True)
        with tarfile.open(path) as archive:
            archive.extractall(directory.parent)
        corpus[version] = directory
    if not corpus:
        raise Refused("no hops supplied; pass --corpus or --tarball")
    return corpus


def verify_identity(fixture: pathlib.Path, corpus: dict[str, pathlib.Path]) -> dict:
    inventory = json.loads((fixture / "dist-inventory.json").read_text())
    identity = json.loads((fixture / "identity.json").read_text())
    tarballs = {
        hop["sdk_version"]: hop["tarball_sha256"] for hop in identity["hops"]
    }
    verified = {}
    for version, package_root in corpus.items():
        expected = inventory["per_file_sha256"].get(version)
        if expected is None:
            raise Refused(f"hop {version} is not in the committed file ledger")
        actual = {}
        for path in sorted(package_root.iterdir()):
            if path.is_file():
                actual[path.name] = sha256_bytes(path.read_bytes())
        if set(actual) != set(expected):
            missing = sorted(set(expected) - set(actual))
            extra = sorted(set(actual) - set(expected))
            raise Refused(
                f"hop {version} file set diverges from the ledger: "
                f"missing={missing} extra={extra}"
            )
        mismatched = [name for name in expected if expected[name] != actual[name]]
        if mismatched:
            raise Refused(f"hop {version} bytes diverge from the ledger: {mismatched[:5]}")
        verified[version] = {
            "files": len(actual),
            "sdk_mjs_sha256": actual["sdk.mjs"],
            "sdk_mjs_bytes": (package_root / "sdk.mjs").stat().st_size,
            "tarball_sha256_frozen": tarballs[version],
        }
    return verified


def changed_spans(corpus: dict, versions: list, surface: dict, limit: int = 400) -> list:
    """Exact removed/added byte spans for a surface, adjacent hops only.

    The span is the maximal common prefix and suffix of the two windows; what
    remains is the changed middle. Every span reports its byte offsets inside
    the two `sdk.mjs` files, so a reviewer can read exactly the code that moved
    without trusting a diff algorithm's alignment.
    """
    spans = []
    for left, right in zip(versions, versions[1:]):
        windows = {
            version: surface_window((corpus[version] / "sdk.mjs").read_text(), surface)
            for version in (left, right)
        }
        if windows[left] is None or windows[right] is None:
            spans.append({"hop": f"{left}->{right}", "status": "absent"})
            continue
        (offset_a, text_a), (offset_b, text_b) = windows[left], windows[right]
        shorter = min(len(text_a), len(text_b))
        prefix = 0
        while prefix < shorter and text_a[prefix] == text_b[prefix]:
            prefix += 1
        suffix = 0
        while (
            suffix < shorter - prefix
            and text_a[len(text_a) - 1 - suffix] == text_b[len(text_b) - 1 - suffix]
        ):
            suffix += 1
        middle_a = text_a[prefix:len(text_a) - suffix]
        middle_b = text_b[prefix:len(text_b) - suffix]
        spans.append(
            {
                "hop": f"{left}->{right}",
                "status": "identical" if not middle_a and not middle_b else "changed",
                "removed_offset": offset_a + prefix,
                "added_offset": offset_b + prefix,
                "removed_bytes": len(middle_a),
                "added_bytes": len(middle_b),
                "removed_preview": middle_a[:limit],
                "added_preview": middle_b[:limit],
                "previews_truncated": len(middle_a) > limit or len(middle_b) > limit,
            }
        )
    return spans


def surface_window(source: str, surface: dict):
    anchor = surface["anchor"]
    offset = source.find(anchor)
    if offset < 0:
        return None
    if surface["kind"] == "definition":
        try:
            _end, text = brace_body(source, offset)
        except Refused:
            return None
        return offset, text
    if surface["kind"] == "enclosing_function":
        span = enclosing_function(source, offset)
        if span is None:
            return None
        start, end = span
        return start, source[start:end]
    start, end, _bounded = statement_window(source, offset, anchor, surface)
    return start, source[start:end]


def classify(fixture: pathlib.Path, corpus: dict[str, pathlib.Path]) -> dict:
    verified = verify_identity(fixture, corpus)
    versions = list(verified)
    report = {
        "schema": "claude-agent-sdk-selected-flow-v1",
        "fixture": str(fixture),
        "versions": versions,
        "identity": verified,
        "entry": {},
        "import_closure": {},
        "surfaces": {},
        "reachability_inputs": SIDECAR_SELECTED_OPTIONS,
        "classification": {},
    }
    for version, package_root in corpus.items():
        entry = resolve_entry(package_root)
        source = (package_root / entry["file"]).read_text()
        report["entry"][version] = entry
        report["import_closure"][version] = import_closure(source)
        per_version = {}
        for surface in SURFACES:
            per_version[surface["name"]] = extract(source, surface)
        report["surfaces"][version] = per_version

    for surface in SURFACES:
        name = surface["name"]
        states = {
            version: report["surfaces"][version][name]["status"] for version in versions
        }
        sequences = {
            version: report["surfaces"][version][name].get("masked_sequence_sha256")
            for version in versions
        }
        multisets = {
            version: report["surfaces"][version][name].get("masked_multiset_sha256")
            for version in versions
        }
        present = [version for version in versions if states[version] == "classified"]
        distinct = sorted({tuple(sequences[version]) for version in present})
        changed_hops = []
        reordered_hops = []
        if len(present) > 1:
            for left, right in zip(versions, versions[1:]):
                if left not in present or right not in present:
                    continue
                if sequences[left] == sequences[right]:
                    continue
                hop = f"{left}->{right}"
                if multisets[left] == multisets[right]:
                    reordered_hops.append(hop)
                else:
                    changed_hops.append(hop)
        report["classification"][name] = {
            "note": surface["note"],
            "status_per_version": states,
            "present_in": present,
            "absent_in": [version for version in versions if states[version] != "classified"],
            "masked_sequence_sha256": sequences,
            "masked_multiset_sha256": multisets,
            "structurally_identical_where_present": len(distinct) <= 1,
            "changed_hops": changed_hops,
            "reordered_only_hops": reordered_hops,
            "selected_path": REACHABILITY[name][0],
            "reason": REACHABILITY[name][1],
            "bounded": all(
                state in ("classified", "absent") for state in states.values()
            ),
        }
        if changed_hops:
            report["classification"][name]["changed_spans"] = changed_spans(
                corpus, versions, surface
            )
    # The sanitizer gate is the condition itself: report the exact predicate
    # shape per hop so an added telemetry marker cannot hide in a renamed name.
    for version in versions:
        source = (corpus[version] / "sdk.mjs").read_text()
        report["classification"].setdefault("telemetry.sanitizer_predicate", {})[
            version
        ] = {
            "gate_present": bool(GATE_SHAPE.search(source)),
            "source": (GATE_SHAPE.search(source).group(0) if GATE_SHAPE.search(source) else None),
        }
    closure_bounded = all(
        report["import_closure"][version]["bounded"]
        and report["import_closure"][version]["literal_require_selected_away"]
        for version in versions
    )
    report["closure_bounded"] = closure_bounded
    return report


def self_test() -> int:
    """Prove the fail-closed paths against synthetic fixtures only."""
    import tempfile

    failures = []

    def check(name, condition):
        if not condition:
            failures.append(name)

    with tempfile.TemporaryDirectory() as directory:
        root = pathlib.Path(directory)
        fixture = root / "fixture"
        fixture.mkdir()
        hop = root / "pkg-0.0.1/package"
        hop.mkdir(parents=True)
        source = (
            "// Version: 0.0.1\n"
            'import{createRequire as o5}from"node:module";\n'
            "function cD(e){return /^(?:0|[1-9]\\d{0,9})$/.test(e)}\n"
            "function uD(e){let{limits:t}=C7;return t===void 0?!1:!0}\n"
            "function Ki(e){if(typeof e===\"string\")return e;let t={};"
            "for(let[n,r]of Object.entries(e)){let o=Ki(n);"
            "if(o!==\"__proto__\"&&(o===n||!cD(o)||Number(o)<64||!uD(\"cleaned_key\")))"
            "t[o]=Ki(r)}return t}\n"
            "writePrompt(e){return this.afterInitialize(()=>this.writeStreamMessage(e))}\n"
            'async streamInput(e){for await(let r of e)await this.writeStreamMessage(r)}\n'
            "writeStreamMessage(e){return this.transport.write(e)}\n"
            "endRun(){return}\n"
            "armRunEndCeiling(){return}\n"
            "writeBehindPromptHold(e,t){return n}\n"
            'this.promptHold=this.initialization.then(()=>!0)\n'
            'this.initializeWritten=new Promise((g)=>{this.markInitializeWritten=g})\n'
            'e.request.subtype==="can_use_tool"\n'
            '"--output-format","stream-json"\n'
            '"--managed-settings"\n'
            '"await-initialize"\n'
            "permissions.denyInsteadOfPrompting]===!0\n"
            'parentManaged:t?.tier??null,allowedSources:(e.settingSources??_Xe).map(l=>hXe[l])\n'
            '["permissions","denyInsteadOfPrompting"],restrictive:!0\n'
            "Unicode sanitization reached maximum iterations\n"
        )
        (hop / "sdk.mjs").write_text(source)
        (hop / "package.json").write_text(
            json.dumps({"name": "@anthropic-ai/claude-agent-sdk", "exports": {".": "./sdk.mjs"}})
        )
        (hop / "extra.txt").write_text("fixture\n")
        files = {
            name: sha256_bytes((hop / name).read_bytes())
            for name in ("sdk.mjs", "package.json", "extra.txt")
        }
        (fixture / "dist-inventory.json").write_text(
            json.dumps({"per_file_sha256": {"0.0.1": files}})
        )
        (fixture / "identity.json").write_text(
            json.dumps({"hops": [{"sdk_version": "0.0.1", "tarball_sha256": "0" * 64}]})
        )

        report = classify(fixture, {"0.0.1": hop})
        check("entry resolves", report["entry"]["0.0.1"]["file"] == "sdk.mjs")
        check("closure bounded", report["closure_bounded"])
        check(
            "sanitizer gate detected",
            report["classification"]["telemetry.sanitizer_predicate"]["0.0.1"][
                "gate_present"
            ],
        )
        check(
            "streamInput classified",
            report["surfaces"]["0.0.1"]["query.streamInput"]["status"] == "classified",
        )
        check(
            "promptHold marked unselected",
            report["classification"]["query.promptHold.assignment"]["selected_path"]
            == "unselected",
        )

        # A tampered tree must be refused, not classified.
        (hop / "sdk.mjs").write_text(source + "// tampered\n")
        try:
            classify(fixture, {"0.0.1": hop})
            failures.append("tampered tree was not refused")
        except Refused:
            pass

        # An unledgered hop must be refused.
        try:
            verify_identity(fixture, {"9.9.9": hop})
            failures.append("unledgered hop was not refused")
        except Refused:
            pass

        # Masking must ignore identifier renames and catch a structural change.
        left = mask("a.b(c=>d);")
        right = mask("x.b(y=>z);")
        changed = mask("x.b(y=>z,w);")
        check("masking ignores renames", left == right)
        check("masking catches structure", right != changed)

    if failures:
        for failure in failures:
            print(f"self-test failure: {failure}", file=sys.stderr)
        return 1
    print("selected-flow self-test: ok")
    return 0


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--corpus")
    parser.add_argument("--tarball", action="append", default=[])
    parser.add_argument("--extract-to")
    parser.add_argument("--fixture", default=str(FIXTURE))
    parser.add_argument("--json", help="write the report here")
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()
    if args.self_test:
        return self_test()
    try:
        corpus = load_corpus(args)
        report = classify(pathlib.Path(args.fixture), corpus)
    except Refused as refusal:
        print(f"refused: {refusal}", file=sys.stderr)
        return 2
    payload = json.dumps(report, indent=2, sort_keys=True)
    if args.json:
        pathlib.Path(args.json).write_text(payload + "\n")
        print(f"wrote {args.json}")
    else:
        print(payload)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
