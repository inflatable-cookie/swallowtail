#!/usr/bin/env python3
"""Inventory and routing guard for Swallowtail's Effigy selectors.

Run: `effigy validate:oxide-routing`.

Every maintained selector whose route spells a Cargo command is classified
here as one of two kinds:

* switched - the selector runs `bash scripts/oxide-rust.sh <cargo args>` and
  keeps a `<selector>:cargo` sibling whose command is the same Cargo arguments
  with no Oxide anywhere on the route;
* exception - the selector stays on plain Cargo for a stated reason.

A selector that spells Cargo and fits neither is a routing defect, and so is a
switched selector with a missing, extra or reordered sibling. The script also
checks the release include, the first-conversion ignore rule and the generated
Oxide agent block. It reads only maintained files plus the release include, so
it is a static inventory: it proves wiring, not that every moved selector was
executed.

Usage:
  oxide-selector-routing.py            check the inventory (exits nonzero on
                                       any defect, prints the receipt)
  oxide-selector-routing.py --inventory
                                       also print one row per classified
                                       selector
"""

from pathlib import Path
import re
import shlex
import sys

ROOT = Path(__file__).resolve().parents[2]
MANIFEST = ROOT / "effigy.toml"
RELEASE_INCLUDE = ROOT / "config" / "release.toml"
GITIGNORE = ROOT / ".gitignore"
AGENTS = ROOT / "AGENTS.md"

RUNNER = "bash scripts/oxide-rust.sh "
ALLOWED_VERBS = ("check", "clippy", "nextest", "test", "build")
ENV_ASSIGNMENT = re.compile(r"^[A-Za-z_][A-Za-z0-9_]*=")

# Selectors that stay on plain Cargo, with the reason that keeps them there.
# `format:` and `probe:` are matched by prefix (see PREFIX_EXCEPTIONS); every
# other exception is named exactly so a retired one fails this check.
EXCEPTIONS = {
    "test:rust": (
        "whole-workspace nextest keeps the milestone shape, and it runs the "
        "courier and Grok registered-tool test binaries, which build a nested "
        "Cargo target of their own"
    ),
    "validate:sdk-registered-lifetime": (
        "selects claude_agent_sdk_driver registered-tool tests, whose nested "
        "courier build requires the run-owned private target"
    ),
    "validate:sdk-unsolicited-close": (
        "selects claude_agent_sdk_driver registered-tool tests, whose nested "
        "courier build requires the run-owned private target"
    ),
    "validate:current-grok-build-acp": (
        "selects acp_suite registered-tool tests, which build the courier into "
        "their own nested Cargo target and would inherit Oxide's slot build "
        "directory"
    ),
    "lint:current-pi-sdk-sidecar": (
        "one step selects a toolchain with a leading `cargo +1.95.0`; Oxide "
        "refuses a leading +toolchain and the MSRV proof keeps its exact pin"
    ),
    "lint:current-copilot-cli-acp": (
        "one step selects a toolchain with a leading `cargo +1.95.0`; Oxide "
        "refuses a leading +toolchain and the MSRV proof keeps its exact pin"
    ),
    "security:dependencies": (
        "`cargo deny` is a supply-chain metadata tool, not a build route"
    ),
    "package:docs": (
        "RUSTDOCFLAGS missing-docs gate on the release and CI route"
    ),
    "check:copilot-acp-offline-proof": (
        "static proof cluster mixing Python validation steps with one Cargo "
        "compile step; not an ordinary local build entry point"
    ),
    "validate:copilot-acp-offline-proof": (
        "static proof cluster mixing Python validation steps with one Cargo "
        "test step; not an ordinary local test entry point"
    ),
    "check:current-copilot-cli-acp": (
        "static proof cluster mixing Python ledger checks with one Cargo "
        "compile step; not an ordinary local build entry point"
    ),
    "validate:current-copilot-cli-acp": (
        "static proof cluster mixing a Python ledger check with one Cargo "
        "test step; not an ordinary local test entry point"
    ),
    "validate:copilot-acp-private-assessment": (
        "static proof cluster mixing a Cargo test step with a Python "
        "self-test; not an ordinary local test entry point"
    ),
    "check:copilot-acp-authenticated-proof": (
        "static proof cluster mixing Python plan validation with one Cargo "
        "compile step; not an ordinary local build entry point"
    ),
    "validate:copilot-acp-authenticated-proof": (
        "static proof cluster mixing a Python self-test with one Cargo test "
        "step; not an ordinary local test entry point"
    ),
}

PREFIX_EXCEPTIONS = {
    "format:": (
        "formatting only; `cargo fmt` is plain passthrough in Oxide and "
        "writes no build output"
    ),
    "probe:": (
        "live provider probe; it needs explicit operator authority and is not "
        "an ordinary local build route"
    ),
}


# Selectors that reach Cargo through a maintained script instead of spelling a
# Cargo command word themselves. Each stays on plain Cargo for the stated
# reason, and this table is the inventory that keeps them visible: a script
# wrapper must not acquire the Oxide route by accident.
SCRIPT_WRAPPED = {
    "validate:focused": (
        "scripts/validate-focused-packages.sh",
        "whole-package milestone proof includes the nested-Cargo courier tests",
    ),
    "format:package": (
        "scripts/format-packages.sh",
        "formatting only; package-scoped cargo fmt and its check pass",
    ),
    "check:sdk-patch-backport": (
        "scripts/release/backports/sdk-v0.5.2/runner.sh",
        "release backport runner owns its private targets and pinned toolchains",
    ),
    "validate:sdk-patch-backport": (
        "scripts/release/backports/sdk-v0.5.2/runner.sh",
        "release backport runner owns its private targets and pinned toolchains",
    ),
    "format:sdk-patch-backport": (
        "scripts/release/backports/sdk-v0.5.2/runner.sh",
        "release backport runner owns its private targets and pinned toolchains",
    ),
    "check:sdk-courier-timeout": (
        "scripts/run-with-owned-cargo-target.sh",
        "private run-owned target plus cleanup; Oxide owns storage placement",
    ),
    "validate:sdk-courier-timeout": (
        "scripts/run-with-owned-cargo-target.sh",
        "private run-owned target plus cleanup; Oxide owns storage placement",
    ),
    "validate:sdk-courier-idle": (
        "scripts/run-with-owned-cargo-target.sh",
        "private run-owned target plus cleanup; Oxide owns storage placement",
    ),
    "package:metadata": (
        "scripts/check-package-metadata.sh",
        "cargo metadata topology gate on the release and CI route",
    ),
    "package:api": (
        "scripts/check-public-api.sh",
        "cargo-public-api baseline gate; a metadata tool, not a build route",
    ),
    "package:msrv": (
        "scripts/check-msrv.sh",
        "pinned-toolchain cargo check through rustup run",
    ),
    "package:release-floor": (
        "scripts/check-release-floor.sh",
        "pinned-MSRV cargo clippy and test through rustup run",
    ),
    "package:source-consumer": (
        "scripts/verify-source-consumer.sh",
        "external source consumer with its own generated lock and target",
    ),
    "package:verify-affected": (
        "scripts/verify-affected-packages.sh",
        "package archive assembly and a private compile target",
    ),
}


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


def runner_arguments(command):
    if not command.startswith(RUNNER):
        fail("switched command does not use %s: %r" % (RUNNER.strip(), command))
    return command[len(RUNNER) :]


def exception_reason(name):
    if name in EXCEPTIONS:
        return EXCEPTIONS[name]
    for prefix, reason in PREFIX_EXCEPTIONS.items():
        if name.startswith(prefix):
            return reason
    return None


def classify(tasks):
    switched = {}
    observed = {}
    for name, commands in tasks.items():
        strings = [command for command in commands if command is not None]
        cargo = [command for command in strings if command_word(command) == "cargo"]
        runner = [command for command in strings if command.startswith(RUNNER)]
        reason = exception_reason(name)

        if name.endswith(":cargo"):
            if not cargo:
                fail("sibling %s no longer spells a Cargo command" % name)
            if runner:
                fail("sibling %s puts Oxide back on the plain route" % name)
            observed[name] = ("sibling", "plain Cargo twin of a switched selector")
            continue

        if name in SCRIPT_WRAPPED:
            script, reason = SCRIPT_WRAPPED[name]
            if runner:
                fail("script-wrapped selector %s acquired the Oxide route" % name)
            path = ROOT / script
            if not path.is_file():
                fail("script-wrapped selector %s names a missing script" % name)
            text = path.read_text()
            if "oxide-rust.sh" in text:
                fail("%s routes through the Oxide helper" % script)
            if "cargo " not in text:
                fail("%s no longer runs a Cargo command" % script)
            if cargo:
                fail("%s both wraps a script and spells Cargo" % name)
            observed[name] = ("script", reason)
            continue

        if reason is not None and name not in EXCEPTIONS:
            # A prefix rule only covers selectors that actually spell Cargo.
            if not cargo:
                if runner:
                    fail("switched selector %s matches a prefix exception" % name)
                continue
        if reason is not None:
            if not cargo:
                fail("exception %s no longer spells a Cargo command" % name)
            if runner:
                fail("exception %s must stay on plain Cargo" % name)
            if name + ":cargo" in tasks:
                fail("exception %s needs no :cargo sibling" % name)
            observed[name] = ("exception", reason)
            continue

        if runner:
            if len(runner) != len(strings):
                fail(
                    "switched selector %s mixes Oxide routing with another "
                    "command: %r" % (name, strings)
                )
            if cargo:
                fail("switched selector %s keeps a bare cargo command" % name)
            arguments = []
            for command in runner:
                rest = runner_arguments(command)
                verb = command_word(rest)
                if verb not in ALLOWED_VERBS:
                    fail(
                        "switched selector %s routes an unsupported verb %r"
                        % (name, verb)
                    )
                arguments.append(rest)
            switcher = {
                "name": name,
                "cargo": ["cargo " + rest for rest in arguments],
                "reason": "ordinary local build/check/clippy/test route",
            }
            switched[name] = switcher
            observed[name] = ("switched", switcher["reason"])
            continue

        if cargo:
            fail(
                "selector %s spells a Cargo command but is neither switched "
                "nor a documented exception" % name
            )

    return switched, observed


def check_siblings(tasks, switched):
    for name, switcher in switched.items():
        sibling = name + ":cargo"
        if sibling not in tasks:
            fail("switched selector %s has no %s sibling" % (name, sibling))
        commands = [command for command in tasks[sibling] if command is not None]
        if commands != switcher["cargo"]:
            fail(
                "sibling %s does not repeat the plain Cargo arguments:\n"
                "  actual:   %r\n  expected: %r"
                % (sibling, commands, switcher["cargo"])
            )
        for command in commands:
            if "oxide" in command:
                fail("sibling %s puts Oxide back on the plain route" % sibling)
            if command_word(command) != "cargo":
                fail("sibling %s is not a plain Cargo command: %r" % (sibling, command))
        if sibling in switched:
            fail("sibling %s is itself switched" % sibling)
        if exception_reason(sibling) is not None:
            fail("sibling %s is also classified as an exception" % sibling)

    for name in tasks:
        if name.endswith(":cargo") and name[: -len(":cargo")] not in switched:
            fail("%s is an orphan :cargo sibling" % name)


def check_exception_entries(tasks):
    for name in EXCEPTIONS:
        if name not in tasks:
            fail("stale exception entry: %s no longer exists" % name)
    for name in SCRIPT_WRAPPED:
        if name not in tasks:
            fail("stale script-wrapped entry: %s no longer exists" % name)


def check_release_include():
    text = RELEASE_INCLUDE.read_text()
    if "oxide" in text:
        fail("release include must keep its plain-Cargo gates")
    if '"cargo fmt --all -- --check"' not in text:
        fail("release include lost its plain-Cargo format gate")
    if 'metadata = "effigy package:metadata"' not in text:
        fail("release include lost its accepted gate shape")


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


def main(argv):
    inventory = "--inventory" in argv
    unknown = [argument for argument in argv if argument != "--inventory"]
    if unknown:
        fail("unknown argument: %s" % " ".join(unknown))

    tasks = parse_tasks(MANIFEST)
    switched, observed = classify(tasks)
    check_siblings(tasks, switched)
    check_exception_entries(tasks)
    check_release_include()
    check_gitignore()
    check_agent_block()

    prefix_counts = {}
    kind_counts = {"exception": 0, "script": 0, "sibling": 0, "switched": 0}
    for name, (kind, _reason) in observed.items():
        kind_counts[kind] += 1
        if kind != "exception":
            continue
        label = "exact"
        for prefix in PREFIX_EXCEPTIONS:
            if name.startswith(prefix):
                label = prefix.rstrip(":")
        prefix_counts[label] = prefix_counts.get(label, 0) + 1

    if inventory:
        for name in sorted(tasks):
            if name not in observed:
                continue
            kind, reason = observed[name]
            printf_row = "%-12s %-55s %s" % (kind, name, reason)
            print(printf_row)

    print("selector inventory:")
    print("  selectors=%d" % len(tasks))
    print("  switched=%d" % kind_counts["switched"])
    print("  cargo_siblings=%d" % kind_counts["sibling"])
    print("  exceptions=%d" % kind_counts["exception"])
    print("  script_wrapped=%d" % kind_counts["script"])
    print(
        "  other=%d"
        % (len(tasks) - sum(kind_counts[key] for key in kind_counts))
    )
    for label in sorted(prefix_counts):
        print("  exception:%s=%d" % (label, prefix_counts[label]))
    print("  release_include=plain-cargo")
    print("  target_ignore=/target")
    print("  agent_block=generated")
    print("oxide selector routing passed")


if __name__ == "__main__":
    try:
        main(sys.argv[1:])
    except RoutingDefect as defect:
        print("oxide routing defect: %s" % defect, file=sys.stderr)
        sys.exit(1)
