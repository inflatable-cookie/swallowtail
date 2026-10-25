#!/usr/bin/env python3
"""Inventory and routing guard for Swallowtail's maintained Rust routes.

Run: `effigy validate:oxide-routing`.

Every maintained local Rust route uses Oxide. Tom's 2026-10-25 ruling, relayed
by the Oxide Supervisor, removed the per-selector `:cargo` escape selectors and
the project-specific runner escape: this guard fails on either coming back.
docs/guides/validation-tiers.md owns the rules. This guard classifies every
maintained selector and script that reaches Cargo:

* switched - the selector runs `bash scripts/oxide-rust.sh <same arguments>`,
  or runs a maintained script that routes its own Cargo commands through the
  helper on the same arguments;
* script - the selector runs one maintained script that owns its route
  (`SCRIPT_CLASSES`), including the private-target and temporary-workspace
  drivers that enter once through the helper's `--host-tool` mode;
* delegated - a maintained script that reaches Cargo only by running another
  routed script;
* genuine - hosted CI, a route whose subject is the pinned original
  invocation, or a review/test of a Cargo invocation itself.

Anything else that spells Cargo is a routing defect, as is a `:cargo` escape
selector, the retired runner escape, a selector that passes `--host-tool`
itself instead of letting the tool own its one lease, a script that disagrees
with its classification, a drifted digest for a route that this guard freezes,
a private target without its ownership and cleanup, a foreign-workspace driver
without its own build directory, or a changed release include, workflow,
target ignore rule or agent block. It reads maintained files only, so it
proves wiring and classification, not that every routed route was executed.

Usage:
  oxide-selector-routing.py             check (nonzero on a defect), print the
                                        receipt
  oxide-selector-routing.py --inventory also print one row per classified
                                        selector or script
"""

import hashlib
from pathlib import Path
import re
import shlex
import sys

ROOT = Path(__file__).resolve().parents[2]
MANIFEST = ROOT / "effigy.toml"
RELEASE_INCLUDE = ROOT / "config" / "release.toml"
GITIGNORE = ROOT / ".gitignore"
AGENTS = ROOT / "AGENTS.md"
WORKFLOWS = ROOT / ".github" / "workflows"

# The routed workspace test recipe the release include documents.
RELEASE_TEST_RECIPE = (
    '# test = "nice -n 5 bash scripts/oxide-rust.sh nextest run '
    '--workspace --all-features --locked"'
)

# The retired escape forms. Both are checked by name so the guard names what
# came back rather than only that something did.
RETIRED_SIBLING_SUFFIX = ":cargo"
RETIRED_RUNNER_ESCAPE = "SWALLOWTAIL_RUST_RUNNER"
EFFIGY_ESCAPE = "EFFIGY_OXIDE_ESCAPE"

HELPER = "scripts/oxide-rust.sh"
ALLOWED_VERBS = (
    "check",
    "clippy",
    "nextest",
    "test",
    "build",
    "fmt",
    "deny",
    "doc",
    "metadata",
    "public-api",
)
# Environment assignments a selector may put before its route. `nice` similarly
# runs the whole route at reduced priority.
SELECTOR_ENV = re.compile(
    r"""^[A-Za-z_][A-Za-z0-9_]*=(?:"[^"]*"|'[^']*'|[^\s]+)$"""
)
SELECTOR_ENV_NAMES = ("RUSTUP_TOOLCHAIN", "SWALLOWTAIL_LIVE_", "RUSTDOCFLAGS")

# Selectors whose subject is the pinned original invocation, or which a review
# keeps on plain Cargo on purpose.
GENUINE = {
    "observe:copilot-acp-private-assessment": (
        "scripts/copilot-acp-original-entry.sh",
        "reviewed original-entry live assessment; its subject is the pinned "
        "original invocation and it needs original authority",
    ),
    "validate:format-selectors:test": (
        "scripts/tests/format-selectors.sh",
        "its subject is the exact `cargo fmt` argument list, so it takes "
        "Effigy's one-run escape and asserts the plain-Cargo argv rather than "
        "a routed one",
    ),
}

# Selectors that run a maintained proof of the routing itself, with the reason
# each is not a product route.
PROOF_SELECTORS = {
    "validate:oxide-routing": "the hermetic routing proof of the shared helper",
}

# Maintained scripts that reach Cargo, with the class each must keep.
SCRIPT_CLASSES = {
    HELPER: "helper",
    "scripts/check-msrv.sh": "switched",
    "scripts/check-package-metadata.sh": "switched",
    "scripts/check-release-floor.sh": "switched",
    "scripts/format-packages.sh": "switched",
    "scripts/generate-public-api-baseline.sh": "switched",
    "scripts/validate-focused-packages.sh": "switched",
    "scripts/validation/package-scope.sh": "switched",
    "scripts/check-public-api.sh": "delegated",
    "scripts/run-grok-acp-client-mcp-probe.sh": "host-tool",
    "scripts/run-with-owned-cargo-target.sh": "host-tool",
    "scripts/release/backports/sdk-v0.5.2/runner.sh": "foreign-workspace",
    "scripts/verify-affected-packages.sh": "foreign-workspace",
    "scripts/verify-lease-api-fixture.sh": "foreign-workspace",
    "scripts/verify-source-consumer.sh": "foreign-workspace",
    "scripts/tests/format-selectors.sh": "genuine",
    "scripts/tests/oxide-rust-routing.sh": "fake-proof",
    "scripts/copilot-acp-original-entry.sh": "genuine",
}

SCRIPT_REASONS = {
    "helper": (
        "the shared routing helper: the only place that spells plain Cargo, "
        "and the owner of Effigy's escape and the CI branch"
    ),
    "switched": "routes its Cargo commands through scripts/oxide-rust.sh",
    "delegated": "reaches Cargo only by running another routed script",
    "host-tool": (
        "Cargo-running tool: re-executes itself once through the helper's "
        "host-tool mode, so it holds one `oxide exec --` lease"
    ),
    "foreign-workspace": (
        "private-target or temporary-workspace proof: one `oxide exec --` "
        "lease, its own target and its own build directory for workspaces "
        "outside this checkout"
    ),
    "genuine": (
        "its subject is the Cargo invocation or a pinned original entry, so it "
        "keeps plain Cargo"
    ),
    "fake-proof": "hermetic proof of the helper with fake tools only",
}

# Routes whose arguments and scope were frozen when they moved to Oxide:
# selector -> sha256 of its command list. A deliberate change updates the
# digest in the same commit, so an edited filter, package list, profile or test
# binary cannot pass as a routing-only change.
FROZEN_ROUTES = {
    "check:lease-api-fixture": "c6fdb7bbbb927e11739933b34e77139bd7692efc9e6514ed28ab9a7195fc724c",
    "check:sdk-courier-timeout": "ba9f90cf3dd74e199741424b90bb59f2f9aad70804770ccd3b72b1333b56a7ed",
    "check:sdk-patch-backport": "1534d2c04ffcf0f10cb02108f4a074b0add733cc947950267a8e08b0da35600e",
    "format:sdk-patch-backport": "72fe81d697eaddbfc6ccdf14437a65ce3779d653a944d71006e7bb1358d79fec",
    "package:release-floor": "0cb71d915a9fe88738d5aaa5ce1e2ca84e50aedc931de4c77d409c5afcd50aea",
    "package:source-consumer": "ba60e274b11fcd880914c93f8a9668d54dc3c5755e694cccd8748aa226abc123",
    "package:verify-affected": "7bf8687e2b046979607f0f126b3d9b18a811ed4d4db0fd9cdbefde747a890d27",
    "test:rust": "2e3dff1832737681769bbb4efc952ddfcc49db54b33ce5de2b116f17633b242b",
    "validate:current-claude-agent-sdk-node": "e6fe50168218c42873a09b99cca8e42246220ae13284874fbcc69ba200b43ecf",
    "validate:current-grok-build-acp": "73c2d8f0d4733a909d93001397a0aa56dbb9de44360dd79d38cbc4b302ecab9a",
    "validate:focused": "c14780de9a78c0e78c9badd6ff76fee8910471c2067691a5fd51c27ac5d7571b",
    "validate:sdk-courier-idle": "3407911648ad02cb060b4c391cf44ac3fe7daafed946fd8f036ac4f5c0c50154",
    "validate:sdk-courier-timeout": "a8ad8d435a62eb1fb33fe7bd1b44deca4fb4681b12b9314f98d5f38f6d939be8",
    "validate:sdk-patch-backport": "961d8dc37f61806046f31958a642e25d79551f1fe25081a55b9c3f0e988eb055",
    "validate:sdk-registered-lifetime": "9f1b942a2d9698dcec791cc469ae5c732e9508dced5fd9c522ce65eeb4c96559",
    "validate:sdk-unsolicited-close": "0a0ee775a6fcb39fd05435acdb97ec5da39a322df26ef6c39c6a78ed59c1d210",
}

CARGO_WORD = "cargo"
SAFE_PRECEDING = {"nice", "env", "time", "timeout", "exec", "sudo"}
ENV_ASSIGNMENT = re.compile(r"^[A-Za-z_][A-Za-z0-9_]*=")
HELPER_INVOCATION = re.compile(
    r"(?<![A-Za-z0-9_./-])bash scripts/oxide-rust\.sh "
)
ENV_TOKENS = re.compile(r"""[A-Za-z_][A-Za-z0-9_]*=(?:"[^"]*"|'[^']*'|[^\s]+)""")
SCRIPT_REFERENCE = re.compile(r"(?<![A-Za-z0-9_./-])(scripts/[A-Za-z0-9_./-]+\.sh)")


class RoutingDefect(Exception):
    pass


def fail(message):
    raise RoutingDefect(message)


def unquote(text):
    """Decode one TOML basic string, including its backslash escapes."""
    if len(text) < 2 or not text.startswith('"') or not text.endswith('"'):
        fail("unsupported TOML string shape: %r" % text)
    body = text[1:-1]
    decoded = []
    index = 0
    escapes = {
        '"': '"',
        "\\": "\\",
        "n": "\n",
        "t": "\t",
        "r": "\r",
        "b": "\b",
        "f": "\f",
    }
    while index < len(body):
        character = body[index]
        if character != "\\":
            decoded.append(character)
            index += 1
            continue
        index += 1
        if index >= len(body):
            fail("truncated escape in %r" % text)
        escape = body[index]
        if escape in escapes:
            decoded.append(escapes[escape])
            index += 1
            continue
        if escape in ("u", "U"):
            width = 4 if escape == "u" else 8
            digits = body[index + 1 : index + 1 + width]
            if len(digits) != width:
                fail("short unicode escape in %r" % text)
            decoded.append(chr(int(digits, 16)))
            index += 1 + width
            continue
        fail("unsupported escape \\%s in %r" % (escape, text))
    return "".join(decoded)


def parse_array(lines, index):
    elements = []
    cursor = index + 1
    while cursor < len(lines):
        stripped = lines[cursor].strip()
        if stripped == "]":
            return elements, cursor
        if stripped.startswith("{"):
            if stripped.endswith(","):
                stripped = stripped[:-1]
            if not stripped.endswith("}"):
                fail("unsupported multi-line inline table: %r" % lines[cursor])
            elements.append(None)
        elif stripped.startswith('"'):
            if stripped.endswith(","):
                stripped = stripped[:-1]
            elements.append(unquote(stripped))
        else:
            fail("unsupported array line in [tasks]: %r" % lines[cursor])
        cursor += 1
    fail("unterminated array in [tasks]")


def parse_tasks(path):
    lines = path.read_text().splitlines()
    tasks = {}
    in_tasks = False
    index = 0
    while index < len(lines):
        line = lines[index]
        if line.startswith("["):
            in_tasks = line.strip() == "[tasks]"
            index += 1
            continue
        if not in_tasks:
            index += 1
            continue
        if not line.strip() or line.lstrip().startswith("#"):
            index += 1
            continue
        match = re.match(r'^"([^"]+)" = (.+)$', line)
        if not match:
            fail("unparsed [tasks] line: %r" % line)
        name, rest = match.group(1), match.group(2)
        if name in tasks:
            fail("duplicate selector: %s" % name)
        if rest.startswith('"'):
            tasks[name] = [unquote(rest)]
            index += 1
        elif rest.strip() == "[":
            tasks[name], index = parse_array(lines, index)
            index += 1
        elif rest.strip().startswith("{") and rest.strip().endswith("}"):
            tasks[name] = [None]
            index += 1
        else:
            fail("unsupported value for %s: %r" % (name, rest))
    return tasks


def command_word(command):
    words = shlex.split(command)
    for position, word in enumerate(words):
        if position == 0 and ENV_ASSIGNMENT.match(word):
            continue
        return word
    return None


def spells_cargo(command):
    return command_word(command) == CARGO_WORD


def helper_prefix_and_tail(command):
    """Split one helper invocation into its env prefix and its arguments."""
    match = HELPER_INVOCATION.search(command)
    if match is None:
        return None
    prefix = command[: match.start()]
    tail = command[match.end() :]
    residue = ENV_TOKENS.sub("", prefix).strip()
    if residue:
        fail("unsupported prefix before the helper: %r" % command)
    for assignment in ENV_TOKENS.findall(prefix):
        name = assignment.split("=", 1)[0]
        allowed = any(
            name == prefix_name or name.startswith(prefix_name)
            for prefix_name in SELECTOR_ENV_NAMES
        )
        if not allowed:
            fail(
                "selector routes through the helper with an unlisted environment "
                "assignment %s: %r" % (name, command)
            )
    return tail


def check_helper_command(command):
    """One selector's helper route: verb, no `+toolchain`, no second lease."""
    tail = helper_prefix_and_tail(command)
    if tail is None:
        fail("not a helper invocation: %r" % command)
    if "--host-tool" in tail.split():
        fail(
            "selector passes --host-tool itself; a Cargo-running tool owns its "
            "one lease inside the tool, so the selector is not the place for it: "
            "%r" % command
        )
    verb = tail.split(" ", 1)[0]
    if verb not in ALLOWED_VERBS:
        fail("switched command routes an unsupported verb: %r" % command)
    if tail.split(" ", 1)[1:2] and tail.split(" ")[1].startswith("+"):
        fail(
            "helper route carries a leading `+toolchain`; a pin travels as "
            "RUSTUP_TOOLCHAIN=<name> for this route: %r" % command
        )


def script_path(command):
    """The maintained script one selector runs, if any."""
    words = shlex.split(command)
    position = 0
    while position < len(words) and ENV_ASSIGNMENT.match(words[position]):
        position += 1
    if position < len(words) and words[position] in ("nice",):
        position += 1
        while position < len(words) and words[position].startswith("-"):
            position += 2
    if position < len(words) and words[position] == "env":
        position += 1
        while position < len(words) and ENV_ASSIGNMENT.match(words[position]):
            position += 1
    if position + 1 >= len(words):
        return None
    if words[position] not in ("bash", "sh"):
        return None
    candidate = words[position + 1]
    if not candidate.endswith(".sh"):
        return None
    return candidate


def script_cargo_lines(path):
    """Lines in a shell script that run `cargo` as a command word."""
    found = []
    for number, line in enumerate(path.read_text().splitlines(), 1):
        stripped = line.strip()
        if not stripped or stripped.startswith("#"):
            continue
        try:
            words = shlex.split(stripped, comments=True)
        except ValueError:
            words = stripped.split()
        for position, word in enumerate(words):
            if word != CARGO_WORD:
                continue
            preceding = words[:position]
            if all(
                item in SAFE_PRECEDING
                or item.isdigit()
                or item.startswith("-")
                or (ENV_ASSIGNMENT.match(item) and not item.startswith("="))
                for item in preceding
            ):
                found.append((number, " ".join(words[position : position + 3])))
            break
    return found


def check_no_retired_escapes(tasks):
    text = MANIFEST.read_text()
    if RETIRED_RUNNER_ESCAPE in text:
        fail(
            "the retired project escape %s is back in effigy.toml; Effigy's "
            "%s=cargo is the only local escape" % (RETIRED_RUNNER_ESCAPE, EFFIGY_ESCAPE)
        )
    if EFFIGY_ESCAPE in text:
        fail(
            "%s must not be persisted in effigy.toml; it is a one-run "
            "environment escape" % EFFIGY_ESCAPE
        )
    for name in tasks:
        if name.endswith(RETIRED_SIBLING_SUFFIX):
            fail(
                "%s is a plain-Cargo escape selector; those were removed, so "
                "route the selector through Oxide or delete it" % name
            )


def check_frozen_routes(tasks):
    for name, digest in FROZEN_ROUTES.items():
        if name not in tasks:
            fail("frozen route %s no longer exists" % name)
        commands = [command for command in tasks[name] if command is not None]
        actual = hashlib.sha256("\x00".join(commands).encode("utf-8")).hexdigest()
        if actual != digest:
            fail(
                "frozen route %s changed its command list; review the scope, "
                "filter, profile and package list, then refresh its digest "
                "deliberately (actual %s)" % (name, actual)
            )


def classify_selectors(tasks):
    observed = {}
    for name, commands in tasks.items():
        strings = [command for command in commands if command is not None]
        helpers = [command for command in strings if HELPER_INVOCATION.search(command)]
        cargo = [command for command in strings if spells_cargo(command)]
        scripts = [command for command in strings if script_path(command)]

        if name in GENUINE:
            script, genuine_reason = GENUINE[name]
            words = shlex.split(strings[0]) if strings else []
            if len(strings) != 1 or words[:2] != ["bash", script]:
                fail("genuine route %s no longer runs %s" % (name, script))
            if cargo or helpers:
                fail("genuine route %s must stay on plain Cargo" % name)
            observed[name] = ("genuine", genuine_reason)
            continue

        if cargo:
            fail(
                "selector %s spells a Cargo command but is neither routed nor a "
                "documented genuine exception" % name
            )

        for command in strings:
            if "--host-tool" in shlex.split(command):
                fail(
                    "selector %s passes --host-tool itself; the tool owns its "
                    "one lease inside the tool" % name
                )

        if helpers:
            for command in helpers:
                check_helper_command(command)
            observed[name] = ("switched", "ordinary local Rust route")
            continue

        if scripts:
            routed = []
            for command in scripts:
                path = script_path(command)
                kind = SCRIPT_CLASSES.get(path)
                if kind is None:
                    continue
                if kind == "genuine":
                    fail(
                        "selector %s runs %s, which stays a genuine plain-Cargo "
                        "route; record the selector in GENUINE with its reason"
                        % (name, path)
                    )
                if kind in ("fake-proof", "helper"):
                    if name not in PROOF_SELECTORS:
                        fail(
                            "selector %s runs %s, which is not a product route"
                            % (name, path)
                        )
                    routed.append((path, "proof"))
                    continue
                routed.append((path, kind))
            if routed:
                kinds = {kind for _path, kind in routed}
                if kinds == {"proof"}:
                    observed[name] = (
                        "proof",
                        PROOF_SELECTORS[name],
                    )
                    continue
                observed[name] = (
                    "script",
                    "maintained script owns the route: "
                    + ", ".join(
                        path for path, kind in routed if kind != "proof"
                    ),
                )
                continue
        # Any other selector runs no maintained Cargo route.

    return observed


def executable_lines(text):
    """The script's lines with comments and blanks removed."""
    return [
        line
        for line in text.splitlines()
        if line.strip() and not line.lstrip().startswith("#")
    ]


def check_scripts(referenced_scripts):
    observed = {}
    for path in sorted((ROOT / "scripts").rglob("*.sh")):
        relative = str(path.relative_to(ROOT))
        text = path.read_text()
        executable = "\n".join(executable_lines(text))
        has_helper = HELPER in executable
        has_exec = "oxide exec --" in executable
        cargo_lines = script_cargo_lines(path)
        if not has_helper and not has_exec and not cargo_lines:
            continue
        expected = SCRIPT_CLASSES.get(relative)
        if expected is None:
            fail(
                "maintained script reaches Cargo without a classification: %s"
                % relative
            )
        observed[relative] = (expected, SCRIPT_REASONS[expected])

        if RETIRED_RUNNER_ESCAPE in text:
            fail(
                "%s still names the retired %s escape"
                % (relative, RETIRED_RUNNER_ESCAPE)
            )

        if expected == "helper":
            if relative != HELPER:
                fail("only %s is the shared helper, not %s" % (HELPER, relative))
            for required in (
                EFFIGY_ESCAPE,
                "GITHUB_ACTIONS",
                "oxide exec --",
                "is not on PATH",
            ):
                if required not in text:
                    fail("%s lost %r from its routing contract" % (relative, required))
            if "cargo" not in text:
                fail("%s no longer names the plain-Cargo branch" % relative)
            continue

        if expected == "switched":
            if not has_helper:
                fail("%s is classified switched but never calls the helper" % relative)
            if cargo_lines:
                fail("%s still runs plain Cargo: %r" % (relative, cargo_lines))
            continue

        if expected == "delegated":
            if has_helper or has_exec or cargo_lines:
                fail(
                    "%s is classified delegated but reaches Cargo itself" % relative
                )
            delegated = [
                referenced
                for referenced in SCRIPT_REFERENCE.findall(text)
                if SCRIPT_CLASSES.get(referenced) in ("switched", "host-tool")
            ]
            if not delegated:
                fail("%s no longer runs a routed script" % relative)
            continue

        if expected in ("host-tool", "foreign-workspace"):
            if not has_helper or "--host-tool" not in text:
                fail(
                    "%s must re-execute itself through the helper's --host-tool "
                    "mode for one lease" % relative
                )
            if has_exec:
                fail("%s bypasses the shared host-tool routing" % relative)
            if text.count(HELPER) != executable.count(HELPER):
                fail("%s mentions the helper only in comments" % relative)
            if executable.count(HELPER) != 1:
                fail(
                    "%s names the helper %d times; one tool takes exactly one "
                    "lease" % (relative, executable.count(HELPER))
                )
            if "SWALLOWTAIL_OXIDE_EXEC_CHILD" not in executable:
                fail("%s lost its re-exec guard, so it would lease twice" % relative)
            if "mktemp -d" not in text:
                fail("%s no longer creates its own scratch root" % relative)
            if "rm -rf" not in text and "task-owned fixture root" not in text:
                fail(
                    "%s neither removes its own scratch root nor reports it as "
                    "retained evidence" % relative
                )
            if expected == "foreign-workspace":
                for required in ("CARGO_TARGET_DIR", "CARGO_BUILD_BUILD_DIR"):
                    if required not in text:
                        fail(
                            "%s builds a workspace outside this checkout, so it "
                            "must keep its own %s inside its scratch root instead "
                            "of using this project's target or slot"
                            % (relative, required)
                        )
            elif "CARGO_BUILD_BUILD_DIR" in text:
                fail(
                    "%s runs this checkout's own builds, so it keeps the held "
                    "slot's build directory" % relative
                )
            if relative == "scripts/run-with-owned-cargo-target.sh":
                for required in (
                    "swallowtail-sdk-courier.",
                    ".swallowtail-run-owned",
                    'export CARGO_TARGET_DIR="$target"',
                    'rm -rf -- "$target"',
                ):
                    if required not in text:
                        fail(
                            "%s lost its private-target custody: %r"
                            % (relative, required)
                        )
            continue

        if expected == "genuine":
            if has_helper:
                fail("%s must keep plain Cargo" % relative)
            continue

        if expected == "fake-proof":
            if cargo_lines:
                fail("%s invokes real Cargo instead of fake tools" % relative)
            if "mktemp -d" not in text:
                fail("%s no longer keeps its fakes in its own scratch root" % relative)
            continue

        fail("unknown script class %s for %s" % (expected, relative))

    for relative in SCRIPT_CLASSES:
        if relative in observed:
            continue
        if relative in referenced_scripts:
            observed[relative] = (
                SCRIPT_CLASSES[relative],
                SCRIPT_REASONS[SCRIPT_CLASSES[relative]],
            )
            continue
        fail(
            "stale script classification: %s is neither reached nor classified"
            % relative
        )
    return observed


def check_syntax_selector(tasks):
    """The routing syntax selector must compile every maintained script."""
    commands = [
        command for command in tasks.get("check:oxide-routing", []) if command is not None
    ]
    if not commands:
        fail("check:oxide-routing no longer exists")
    covered = set()
    for command in commands:
        words = shlex.split(command)
        if len(words) == 3 and words[:2] == ["bash", "-n"]:
            covered.add(words[2])
    missing = sorted(set(SCRIPT_CLASSES) - covered)
    if missing:
        fail(
            "check:oxide-routing does not syntax-check every maintained script: %s"
            % ", ".join(missing)
        )


def check_release_include():
    text = RELEASE_INCLUDE.read_text()
    if RELEASE_TEST_RECIPE not in text:
        fail("release include lost its routed workspace test recipe")
    for line in text.splitlines():
        stripped = line.strip().lstrip("#").strip()
        if re.match(r"^(?:nice -n 5\s+)?(?:env\s+\S+\s+)?cargo\s", stripped):
            fail(
                "release include keeps a plain-Cargo gate: %r; route it through "
                "the helper" % line.strip()
            )
    if 'fmt = "bash scripts/oxide-rust.sh fmt --all -- --check"' not in text:
        fail("release include lost its routed format gate")
    if 'metadata = "effigy package:metadata"' not in text:
        fail("release include lost its accepted gate shape")


def check_workflows():
    workflows = sorted(WORKFLOWS.glob("*.yml"))
    if not workflows:
        fail("no hosted workflow is checked; .github/workflows owns the CI route")
    for path in workflows:
        text = path.read_text()
        relative = str(path.relative_to(ROOT))
        if "oxide" in text:
            fail(
                "%s must stay on plain Cargo: hosted runners have no Oxide and "
                "the helper owns the GITHUB_ACTIONS branch" % relative
            )
        if "cargo " not in text:
            fail("%s no longer runs a plain-Cargo CI route" % relative)


def check_gitignore():
    lines = [line.strip() for line in GITIGNORE.read_text().splitlines()]
    if "/target/" in lines:
        fail(
            "`.gitignore` keeps the directory-only `/target/` rule; a first "
            "conversion needs the exact non-directory `/target`"
        )
    if "/target" not in lines:
        fail("`.gitignore` does not ignore `/target`")
    if "/.oxide-outputs/" not in lines:
        fail("`.gitignore` lost the init-owned `/.oxide-outputs/` rule")
    for expected in ("/node_modules/", ".effigy", ".agents.local.env"):
        if expected not in lines:
            fail("`.gitignore` lost its %s rule" % expected)


def check_agent_block():
    text = AGENTS.read_text()
    if text.count("<!-- oxide:begin -->") != 1:
        fail("AGENTS.md must carry exactly one generated Oxide block")
    if text.count("<!-- oxide:end -->") != 1:
        fail("AGENTS.md must close the generated Oxide block once")
    block = text.split("<!-- oxide:begin -->", 1)[1].split("<!-- oxide:end -->", 1)[0]
    if "Write `oxide` wherever you would write `cargo`" not in block:
        fail("AGENTS.md Oxide block is not the generated guidance")
    if "Managed by `oxide init`" not in block:
        fail("AGENTS.md Oxide block lost its managed marker")
    for retired in (RETIRED_SIBLING_SUFFIX, RETIRED_RUNNER_ESCAPE):
        if retired in text:
            fail("AGENTS.md still names the retired %r routing form" % retired)


def main(argv):
    inventory = "--inventory" in argv
    unknown = [argument for argument in argv if argument != "--inventory"]
    if unknown:
        fail("unknown argument: %s" % " ".join(unknown))

    tasks = parse_tasks(MANIFEST)
    referenced_scripts = set()
    for commands in tasks.values():
        for command in commands:
            if command is None:
                continue
            for word in shlex.split(command):
                if word.startswith("scripts/") and word.endswith(".sh"):
                    referenced_scripts.add(word)
    check_no_retired_escapes(tasks)
    observed = classify_selectors(tasks)
    check_frozen_routes(tasks)
    for name in GENUINE:
        if name not in tasks:
            fail("stale genuine entry: %s no longer exists" % name)
    scripts = check_scripts(referenced_scripts)
    check_syntax_selector(tasks)
    check_release_include()
    check_workflows()
    check_gitignore()
    check_agent_block()

    counts = {"switched": 0, "script": 0, "proof": 0, "genuine": 0}
    for kind, _reason in observed.values():
        counts[kind] += 1
    other = len(tasks) - sum(counts.values())

    if inventory:
        for name in sorted(observed):
            kind, reason = observed[name]
            print("%-9s %-58s %s" % (kind, name, reason))
        for relative in sorted(scripts):
            kind, reason = scripts[relative]
            print("script    %-58s %s" % (relative, reason))

    print("route inventory:")
    print("  selectors=%d" % len(tasks))
    print("  switched=%d" % counts["switched"])
    print("  script_routes=%d" % counts["script"])
    print("  proof_routes=%d" % counts["proof"])
    print("  genuine=%d" % counts["genuine"])
    print("  other=%d" % other)
    print("  cargo_escapes=0")
    print("  frozen_routes=%d" % len(FROZEN_ROUTES))
    print("  maintained_scripts=%d" % len(scripts))
    print("  release_include=routed, workspace test recipe routed")
    print("  workflows=plain Cargo")
    print("  target_ignore=/target")
    print("  agent_block=generated")
    print("oxide selector routing passed")


if __name__ == "__main__":
    try:
        main(sys.argv[1:])
    except RoutingDefect as defect:
        print("oxide routing defect: %s" % defect, file=sys.stderr)
        sys.exit(1)
