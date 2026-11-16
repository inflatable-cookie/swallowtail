#!/usr/bin/env python3
"""Private selected-invocation contract and owned metadata model. Never launches."""

import argparse
import copy
import hashlib
import json
from pathlib import Path, PurePosixPath
import re

ROOT = Path(__file__).resolve().parents[1]
SOURCE = "docs/knowledge/operations/claude-agent-acp-source-flow.json"
SOURCE_SHA = "0808ba8ee5ee6f66c7bd295d052cd25fd19a28261d38a38f891be9aca212a642"
KINDS = ("auth-status", "sdk-transport", "registry-ps")
CONTEXTS = ("new", "load", "resume", "prompt")
BASE_ARGV = [
    "--output-format",
    "stream-json",
    "--verbose",
    "--input-format",
    "stream-json",
    "--permission-prompt-tool=stdio",
]


class Refused(ValueError):
    pass


def canonical(value):
    return json.dumps(
        value, sort_keys=True, separators=(",", ":"), allow_nan=False
    ).encode()


def digest(value):
    return hashlib.sha256(canonical(value)).hexdigest()


def read(file):
    if file.stat().st_size > 4 * 1024 * 1024:
        raise Refused("record_size")

    def unique(pairs):
        value = {}
        for key, item in pairs:
            if key in value:
                raise Refused("duplicate_key")
            value[key] = item
        return value

    def invalid_number(_):
        raise Refused("non_json_number")

    return json.loads(
        file.read_bytes(), object_pairs_hook=unique, parse_constant=invalid_number
    )


def build():
    raw = (ROOT / SOURCE).read_bytes()
    if hashlib.sha256(raw).hexdigest() != SOURCE_SHA:
        raise Refused("reviewed_source_record_changed")
    source = json.loads(raw)
    entry_file = "crates/swallowtail-adapter-claude-agent/tests/guard_shared_software/reviewed/entry.py"
    entry_sha = "9f0e48b497b3d6a93513d010277582a9a90dd9375e2f9ea5c1bffdbc4f1ba2f1"
    if hashlib.sha256((ROOT / entry_file).read_bytes()).hexdigest() != entry_sha:
        raise Refused("reviewed_entry_source_changed")
    profiles = {}
    for hop in source["hops"]:
        graph_file = (
            ROOT
            / "crates/swallowtail-adapter-claude-agent/sidecar/claude-agent-acp-guard-graphs"
            / (hop["hop"] + ".json")
        )
        if hashlib.sha256(graph_file.read_bytes()).hexdigest() != hop["graphSha256"]:
            raise Refused("source_graph_changed")
        graph = read(graph_file)
        found = []
        for caller, key in hop["selected"].items():
            syntax = source["syntax"]["files"].get(key, {})
            for call in syntax.get("calls", []):
                kind = call["policy"]
                if kind not in KINDS:
                    continue
                found.append(kind)
                profiles[hop["hop"] + "/" + kind] = {
                    "hop": hop["hop"],
                    "kind": kind,
                    "resolver": "held-registry-ps"
                    if kind == "registry-ps"
                    else "captured-platform-native",
                    "graphSha256": hop["graphSha256"],
                    "caller": {
                        "path": caller,
                        "sha256": syntax["sha256"],
                        "api": call["imported"],
                        "callsite": call["expression"],
                    },
                    "argumentExpressions": call["arguments"][:3],
                    "images": graph["native"] if kind != "registry-ps" else None,
                    "timeoutMs": 5000
                    if kind == "auth-status"
                    else 1000
                    if kind == "registry-ps"
                    else None,
                    "stdioOrigin": "explicit-three-pipes"
                    if kind == "sdk-transport"
                    else "execFile-default-pipes",
                    "unresolvedActualBinding": None,
                }
        if sorted(found) != sorted(KINDS):
            raise Refused("selected_call_coverage")
    return {
        "schema": "swallowtail.acp239.selected-invocation.v1",
        "scope": "source requirements and owned metadata counterexamples only",
        "reviewedSource": {
            "head": "c1e07dae22f0206fecc3fefcc931b62a0111e3cd",
            "review": 5916735544,
            "path": SOURCE,
            "sha256": SOURCE_SHA,
        },
        "contexts": list(CONTEXTS),
        "validatorSources": {
            name: hashlib.sha256((ROOT / name).read_bytes()).hexdigest()
            for name in (
                "scripts/claude-acp-invocation.py",
                "scripts/tests/claude-acp-invocation.py",
            )
        },
        "profiles": profiles,
        "entrySource": {
            "path": entry_file,
            "sha256": entry_sha,
            "head": "f9025d2197333c53aed2266627dfa39d05bff7e3",
            "scope": "historical owned software entry, not current host admission",
        },
        "excluded": ["cli-delegation", "auth-logout", "sdk-other-exec", "sdk-where"],
        "resolution": {
            "clientExtraArgs": {},
            "settingsOverride": None,
            "executableArgs": [],
            "customSpawn": False,
        },
        "sdkSubprofileArgv": BASE_ARGV,
        "limits": "No implicit phase order/count. Enumerate every launch and entry allowance once; no refund on refusal, release failure, revoke or unknown delivery.",
        "actualPlan": None,
        "remainingBindings": [
            "FULL_ACP_SDK_OPTION_TAIL_AND_SETTINGS_RESOLUTION",
            "EXACT_RUNTIME_EXECUTABLE_ENV_CWD_FD_AND_STARTUP_CUSTODY",
            "SAME_INVOCATION_TRUSTED_CALLSITE_AND_ENTRY_OBSERVATION",
            "REVIEWED_SHARED_STARTUP_MIGRATION",
            "ACTUAL_SIBLING_HOST_KERNEL_TRANSPORT_CANONICAL_FIXTURES",
            "OPAQUE_NATIVE_AND_PYTHON_RUNTIME_CLOSURE",
            "SEPARATE_EXACT_ORIGINAL_AUTHORITY",
        ],
        "admission": "closed",
        "originalCommand": None,
        "qualifiedCeiling": "0.100.0",
    }


def exact(value, expected, reason):
    if canonical(value) != canonical(expected):
        raise Refused(reason)


def fields(value, names, reason):
    if type(value) is not dict or set(value) != set(names.split()):
        raise Refused(reason)


def absolute(value):
    if type(value) is not str or not value or "\0" in value:
        raise Refused("absolute_path")
    p = PurePosixPath(value)
    if not p.is_absolute() or ".." in p.parts or str(p) != value:
        raise Refused("absolute_path")


def launch(reference, profile):
    fields(
        reference,
        "caller api executable image argv env cwd stdio detached shell timeoutMs windowsHide signalRef descriptors",
        "launch_fields",
    )
    exact(reference["caller"], profile["caller"], "caller_drift")
    exact(reference["api"], profile["caller"]["api"], "api_drift")
    absolute(reference["executable"])
    absolute(reference["cwd"])
    for flag in ("detached", "shell"):
        exact(reference[flag], False, flag)
    exact(reference["stdio"], ["pipe", "pipe", "pipe"], "stdio")
    exact(reference["timeoutMs"], profile["timeoutMs"], "timeout")
    kind = profile["kind"]
    exact(
        reference["windowsHide"],
        kind == "sdk-transport",
        "windows_hide",
    )
    args = reference["argv"]
    if type(args) is not list or any(type(x) is not str or "\0" in x for x in args):
        raise Refused("argv")
    if kind == "auth-status":
        exact(args, ["auth", "status", "--json"], "auth_argv_or_settings_unbound")
    elif kind == "sdk-transport":
        # A source-shaped owned subprofile, not the full ACP option resolver.
        # Every unresolved tail/override refuses rather than guessing it.
        exact(args, BASE_ARGV, "sdk_option_tail_unbound")
    elif (
        len(args) != 4
        or args[:3] != ["-o", "pid=,lstart=", "-p"]
        or len(args[3]) > 4096
        or not re.fullmatch(r"[0-9]+(,[0-9]+)*", args[3])
    ):
        raise Refused("registry_argv")
    if type(reference["env"]) is not dict or any(
        type(k) is not str
        or type(v) is not str
        or not k
        or "=" in k
        or "\0" in k + v
        or re.match(
            r"^(NODE_|LD_|DYLD_|BUN_|OPENSSL_|CLAUDE_CODE_EXECUTABLE$)", k, re.I
        )
        for k, v in reference["env"].items()
    ):
        raise Refused("environment")
    if kind == "registry-ps" and (
        reference["env"].get("LC_ALL"),
        reference["env"].get("TZ"),
    ) != ("C", "UTC"):
        raise Refused("registry_locale")
    image = reference["image"]
    fields(image, "platform sha256 bytes", "image_fields")
    if (
        type(image["bytes"]) is not int
        or image["bytes"] <= 0
        or type(image["sha256"]) is not str
        or not re.fullmatch(r"[a-f0-9]{64}", image["sha256"])
    ):
        raise Refused("image_identity")
    if kind != "registry-ps":
        expected = profile["images"].get(image["platform"])
        if expected is None:
            raise Refused("native_platform")
        exact(
            {k: image[k] for k in ("sha256", "bytes")},
            {k: expected[k] for k in ("sha256", "bytes")},
            "native_image",
        )
    elif image["platform"] not in ("darwin-arm64", "linux-x64"):
        raise Refused("registry_platform")
    if kind == "sdk-transport":
        if type(reference["signalRef"]) is not str or not reference["signalRef"]:
            raise Refused("signal_binding")
    else:
        exact(reference["signalRef"], None, "unexpected_signal")
    descriptors = reference["descriptors"]
    if (
        type(descriptors) is not list
        or len(descriptors) != 3
        or any(type(x) is not str or not x for x in descriptors)
        or len(set(descriptors)) != 3
    ):
        raise Refused("descriptor_bindings")


def actors(value, entry_command):
    fields(value, "prepared-owner group-owner checked-acp entry", "actor_roles")
    for actor in value.values():
        fields(actor, "pid parent group start command", "actor_fields")
        if (
            any(
                type(actor[k]) is not int or actor[k] <= 0
                for k in ("pid", "parent", "group")
            )
            or type(actor["start"]) is not str
            or not actor["start"]
            or type(actor["command"]) is not list
            or not actor["command"]
            or any(type(x) is not str for x in actor["command"])
        ):
            raise Refused("actor_identity_or_exact_argv_missing")
    owner, group, acp, entry = (
        value[k] for k in ("prepared-owner", "group-owner", "checked-acp", "entry")
    )
    if (
        len({x["pid"] for x in value.values()}) != 4
        or group["parent"] != owner["pid"]
        or acp["parent"] != owner["pid"]
        or group["group"] != group["pid"]
        or acp["group"] != group["pid"]
        or entry["parent"] != acp["pid"]
        or entry["group"] != group["pid"]
    ):
        raise Refused("actual_sibling_topology_required")
    exact(entry["command"], entry_command, "entry_command")
    if "--cli" in acp["command"]:
        raise Refused("cli_excluded")


class OwnedPreEntry:
    """No OS operations, callbacks, original mode, authority or observation API."""

    def __init__(self, manifest, packet):
        exact(manifest, build(), "source_manifest")
        fields(
            packet,
            "schema scope hop sourceManifestSha256 originalActivation references",
            "packet_fields",
        )
        exact(
            packet["schema"], "swallowtail.acp239.owned-invocations.v1", "packet_schema"
        )
        exact(packet["scope"], "owned-metadata-counterexamples", "not_owned_metadata")
        exact(packet["sourceManifestSha256"], digest(manifest), "source_binding")
        exact(packet["originalActivation"], False, "original_mode_closed")
        if type(packet["hop"]) is not str or packet["hop"] not in {
            p["hop"] for p in manifest["profiles"].values()
        }:
            raise Refused("packet_hop")
        if type(packet["references"]) is not list or len(packet["references"]) > 64:
            raise Refused("finite_reference_list")
        self.references = {}
        for ref in packet["references"]:
            fields(
                ref,
                "id context fixture fixtureBinding profile resolution launch entryCommand entrySourceSha256 entryConfigSha256 actors",
                "reference_fields",
            )
            if (
                type(ref["id"]) is not str
                or not ref["id"]
                or ref["id"] in self.references
                or ref["context"] not in CONTEXTS
                or ref["fixture"] not in ("absent", "disabled", "managed-deny")
                or type(ref["profile"]) is not str
                or ref["profile"] not in manifest["profiles"]
            ):
                raise Refused("reference_identity_or_selected_scope")
            if ref["profile"].split("/", 1)[0] != packet["hop"]:
                raise Refused("packet_hop_mix")
            for key in ("fixtureBinding", "entryConfigSha256"):
                if type(ref[key]) is not str or not re.fullmatch(
                    r"[a-f0-9]{64}", ref[key]
                ):
                    raise Refused("unresolved_fixture_or_entry_configuration")
            exact(
                ref["resolution"],
                {
                    **manifest["resolution"],
                    "resolver": manifest["profiles"][ref["profile"]]["resolver"],
                },
                "unresolved_or_unsupported_branch",
            )
            launch(ref["launch"], manifest["profiles"][ref["profile"]])
            if (
                type(ref["entryCommand"]) is not list
                or not ref["entryCommand"]
                or any(
                    type(x) is not str or not x or "\0" in x
                    for x in ref["entryCommand"]
                )
            ):
                raise Refused("entry_command")
            absolute(ref["entryCommand"][0])
            exact(
                ref["entrySourceSha256"],
                manifest["entrySource"]["sha256"],
                "entry_source",
            )
            actors(ref["actors"], ref["entryCommand"])
            self.references[ref["id"]] = copy.deepcopy(ref)
        self.used, self.entered = set(), set()
        self.revoked = False

    def reserve(self, allowance, observed):
        if self.revoked or allowance not in self.references or allowance in self.used:
            raise Refused("revoked_unknown_or_spent_launch")
        self.used.add(allowance)  # A refused/unknown attempt is never refunded.
        try:
            ref = self.references[allowance]
            exact(
                observed,
                {k: ref[k] for k in ("context", "fixture", "fixtureBinding", "launch")},
                "actual_launch_drift",
            )
        except Refused:
            self.revoked = True
            raise
        return {
            "matchesReference": True,
            "originalAllowed": False,
            "hostObserved": False,
        }

    def pre_entry(self, allowance, observed):
        if self.revoked or allowance not in self.used or allowance in self.entered:
            raise Refused("unreserved_revoked_or_spent_entry")
        self.entered.add(allowance)
        ref = self.references[allowance]
        try:
            exact(
                observed,
                {
                    k: ref[k]
                    for k in (
                        "context",
                        "fixture",
                        "fixtureBinding",
                        "launch",
                        "actors",
                        "entryCommand",
                        "entrySourceSha256",
                        "entryConfigSha256",
                    )
                },
                "entry_changed_before_effect",
            )
        except Refused:
            self.revoked = True
            raise
        return {
            "matchesReference": True,
            "originalAllowed": False,
            "hostObserved": False,
            "kernelEnforced": False,
        }

    def revoke(self):
        self.revoked = True


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("mode", choices=("build", "verify"))
    parser.add_argument("--output", type=Path)
    parser.add_argument("--record", type=Path)
    args = parser.parse_args()
    manifest = build()
    if args.mode == "build":
        args.output.write_text(json.dumps(manifest, indent=2, sort_keys=True) + "\n")
    else:
        exact(read(args.record), manifest, "source_manifest")
    print(
        json.dumps(
            {
                "profiles": len(manifest["profiles"]),
                "sha256": digest(manifest),
                "admission": "closed",
            }
        )
    )


if __name__ == "__main__":
    main()
