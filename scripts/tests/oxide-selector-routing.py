#!/usr/bin/env python3
"""Inventory and routing guard for Swallowtail's maintained Rust routes.

Run: `effigy validate:oxide-routing`.

The expanded Oxide adoption boundary (docs/guides/validation-tiers.md) makes
every maintained route without a genuine Cargo requirement use Oxide. This
guard classifies each maintained selector and script that reaches Cargo:

* switched - the selector runs `bash scripts/oxide-rust.sh <same arguments>`
  and keeps a `<selector>:cargo` sibling with no Oxide anywhere on its route;
  or the selector runs a maintained script that routes through that helper and
  keeps a sibling that forces the helper's plain-Cargo branch;
* pending - the route needs a private `CARGO_TARGET_DIR` or a temporary
  workspace, which waits for qualified Oxide one-use workspace support;
* genuine - hosted CI, a route whose subject is the pinned original
  invocation, or a review/test of a Cargo invocation itself;
* query - a shared scope precheck that keeps pending routes wholly off Oxide.

Anything else that spells Cargo is a routing defect, as is a switched selector
with a missing, changed, reordered or Oxide-tainted sibling, a stale entry, a
script that disagrees with its classification, or a changed release include,
target ignore rule or agent block. It reads maintained files only, so it
proves wiring and classification, not that every switched route was executed.

Usage:
  oxide-selector-routing.py             check (nonzero on a defect), print the
                                        receipt
  oxide-selector-routing.py --inventory also print one row per classified
                                        selector or script
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
PLAIN_ESCAPE = "SWALLOWTAIL_RUST_RUNNER=cargo "
RELEASE_PENDING_TEST = (
    '# test = "nice -n 5 cargo nextest run --workspace --all-features --locked"'
)
# (package, test binary) pairs whose binary contains tests that build their own
# nested Cargo target. Selecting the whole binary from a switched route would
# run those tests outside the run-owned private target, so such a route stays
# pending. Other packages' binaries with the same name are unaffected.
PRIVATE_TARGET_TEST_BINARIES = {
    ("swallowtail-adapter-claude-agent", "claude_agent_sdk_driver"),
    ("swallowtail-adapter-grok", "acp_suite"),
}


def selected_packages(command):
    words = shlex.split(command)
    packages = set()
    index = 0
    while index < len(words):
        word = words[index]
        if word in ("-p", "--package") and index + 1 < len(words):
            packages.add(words[index + 1])
            index += 2
            continue
        if word.startswith("--package="):
            packages.add(word.split("=", 1)[1])
        index += 1
    return packages


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
ENV_ASSIGNMENT = re.compile(r"^[A-Za-z_][A-Za-z0-9_]*=")

# Routes that wait for qualified Oxide one-use workspace support: they own a
# private CARGO_TARGET_DIR or a temporary consumer/package workspace, or their
# tests build such a target themselves.
PENDING = {
    "test:rust": (
        "the whole-workspace suite includes courier and Grok registered-tool "
        "tests that build a nested Cargo target of their own"
    ),
    "validate:focused": (
        "whole-package suites include those nested-Cargo tests and the route is "
        "the milestone package proof"
    ),
    "validate:sdk-registered-lifetime": (
        "selects Claude SDK registered-tool tests whose nested courier build "
        "requires the run-owned private target"
    ),
    "validate:sdk-unsolicited-close": (
        "selects Claude SDK registered-tool tests whose nested courier build "
        "requires the run-owned private target"
    ),
    "validate:current-grok-build-acp": (
        "selects Grok registered-tool tests that build a courier into their own "
        "nested target"
    ),
    "validate:current-claude-agent-sdk-node": (
        "its nextest filter selects the whole claude_agent_sdk_driver binary, "
        "which includes registered-tool courier tests that require the "
        "run-owned private target"
    ),
    "package:release-floor": (
        "runs the whole workspace test suite, including those private-target "
        "courier tests"
    ),
    "check:sdk-courier-timeout": (
        "run-with-owned-cargo-target.sh owns a fresh private CARGO_TARGET_DIR "
        "and its cleanup"
    ),
    "validate:sdk-courier-timeout": (
        "run-with-owned-cargo-target.sh owns a fresh private CARGO_TARGET_DIR "
        "and its cleanup"
    ),
    "validate:sdk-courier-idle": (
        "run-with-owned-cargo-target.sh owns a fresh private CARGO_TARGET_DIR "
        "and its cleanup"
    ),
    "package:source-consumer": (
        "generates a temporary consumer workspace and lock outside this "
        "checkout and builds it there"
    ),
    "package:verify-affected": (
        "assembles package archives and compiles extracted packages in a "
        "private temporary target"
    ),
    "check:sdk-patch-backport": (
        "the backport runner stages a private candidate and consumer clone with "
        "its own targets and pinned toolchains"
    ),
    "validate:sdk-patch-backport": (
        "the backport runner stages a private candidate and consumer clone with "
        "its own targets and pinned toolchains"
    ),
    "format:sdk-patch-backport": (
        "the backport runner formats inside a private candidate clone"
    ),
}

# Genuine Cargo exceptions: hosted CI keeps its unchanged raw-Cargo workflows,
# and this route's subject is the pinned original invocation, which it needs
# original authority to review. Genuine exceptions are otherwise script-level
# (see SCRIPT_CLASSES).
GENUINE = {
    "observe:copilot-acp-private-assessment": (
        "scripts/copilot-acp-original-entry.sh",
        "reviewed original-entry live assessment; its subject is the pinned "
        "original invocation and it needs original authority",
    ),
}

# Selectors whose maintained script routes through the helper. The selector
# text itself stays the script call; its sibling forces the plain branch.
SCRIPT_ROUTES = {
    "format:package": "scripts/format-packages.sh",
    "package:metadata": "scripts/check-package-metadata.sh",
    "package:api": "scripts/check-public-api.sh",
    "package:msrv": "scripts/check-msrv.sh",
}

# Maintained scripts that reach Cargo, with the class each must keep.
SCRIPT_CLASSES = {
    "scripts/check-msrv.sh": "switched",
    "scripts/check-package-metadata.sh": "switched",
    "scripts/check-release-floor.sh": "pending",
    "scripts/format-packages.sh": "switched",
    "scripts/generate-public-api-baseline.sh": "switched",
    "scripts/run-grok-acp-client-mcp-probe.sh": "host-tool",
    "scripts/oxide-rust.sh": "helper",
    "scripts/tests/oxide-rust-routing.sh": "fake-proof",
    "scripts/tests/format-selectors.sh": "genuine",
    "scripts/copilot-acp-original-entry.sh": "genuine",
    "scripts/validate-focused-packages.sh": "pending",
    "scripts/verify-affected-packages.sh": "pending",
    "scripts/verify-source-consumer.sh": "pending",
    "scripts/release/backports/sdk-v0.5.2/runner.sh": "pending",
    "scripts/run-with-owned-cargo-target.sh": "pending",
    "scripts/validation/package-scope.sh": "query",
}

SCRIPT_REASONS = {
    "switched": "routes its Cargo command through scripts/oxide-rust.sh",
    "host-tool": (
        "Cargo-running host tool: one `oxide exec --` lease, plain-Cargo branch "
        "for CI and the escape"
    ),
    "helper": "the shared routing helper",
    "fake-proof": "hermetic proof of the helper with fake tools only",
    "genuine": (
        "its subject is the Cargo invocation or a pinned original entry, so it "
        "keeps plain Cargo"
    ),
    "pending": "private target or temporary workspace; pending one-use support",
    "query": "scope precheck shared with pending routes; keeps them off Oxide",
}

CARGO_WORD = "cargo"
ROUTED_TEST_LANE = re.compile(
    r'oxide-rust\.sh"?\s*(?:\\\s*\n\s*)?(?:--host-tool\s+\S+\s*)?'
    r"\b(?:test|nextest)\b"
)
SAFE_PRECEDING = {"nice", "env", "time", "timeout", "exec", "sudo"}


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
    return command_word(command) == "cargo"


def helper_tail(command):
    """Return the arguments after the helper invocation, or None."""
    words = shlex.split(command)
    position = 0
    while position < len(words):
        word = words[position]
        if ENV_ASSIGNMENT.match(word):
            position += 1
            continue
        break
    if position + 2 >= len(words):
        return None
    if words[position] != "bash" or words[position + 1] != "scripts/oxide-rust.sh":
        return None
    return words[position + 2 :], words[:position]


HELPER_INVOCATION = re.compile(r"(?<![A-Za-z0-9_./-])bash scripts/oxide-rust\.sh ")
ENV_TOKENS = re.compile(r"""[A-Za-z_][A-Za-z0-9_]*=(?:"[^"]*"|'[^']*'|[^\s]+)""")


def plain_twin(command):
    """The plain-Cargo twin of one switched selector command.

    Only the runner word and the RUSTUP_TOOLCHAIN mapping change; every other
    byte, including quotes and spacing inside arguments, is preserved.
    """
    match = HELPER_INVOCATION.search(command)
    if match is None:
        return command
    prefix = command[: match.start()]
    tail = command[match.end() :]
    residue = ENV_TOKENS.sub("", prefix).strip()
    if residue:
        fail("unsupported prefix before the helper: %r" % command)
    toolchain = None
    kept = []
    for assignment in ENV_TOKENS.findall(prefix):
        if assignment.startswith("RUSTUP_TOOLCHAIN="):
            toolchain = assignment.split("=", 1)[1].strip("\"'")
        else:
            kept.append(assignment)
    verb = tail.split(" ", 1)[0]
    if verb not in ALLOWED_VERBS:
        fail("switched command routes an unsupported verb: %r" % command)
    plain = " ".join(kept + ["cargo"])
    if toolchain is not None:
        plain += " +" + toolchain
    return plain + " " + tail


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


def classify_selectors(tasks):
    switched = {}
    observed = {}
    for name, commands in tasks.items():
        strings = [command for command in commands if command is not None]
        helpers = [command for command in strings if HELPER_INVOCATION.search(command)]
        cargo = [command for command in strings if spells_cargo(command)]
        reason = PENDING.get(name)

        if name.endswith(":cargo"):
            parent = name[: -len(":cargo")]
            if parent in SCRIPT_ROUTES:
                wanted = [
                    PLAIN_ESCAPE + command
                    for command in tasks[parent]
                    if command is not None
                ]
                if strings != wanted:
                    fail("sibling %s does not force the plain branch" % name)
                if "oxide" in " ".join(strings):
                    fail("sibling %s puts Oxide back on the plain route" % name)
                observed[name] = ("sibling", "plain-Cargo twin of a script route")
                continue
            if not cargo:
                fail("sibling %s no longer spells a Cargo command" % name)
            if helpers:
                fail("sibling %s puts Oxide back on the plain route" % name)
            observed[name] = ("sibling", "plain-Cargo twin of a switched selector")
            continue

        if name in GENUINE:
            script, genuine_reason = GENUINE[name]
            words = shlex.split(strings[0]) if strings else []
            if len(strings) != 1 or words[:2] != ["bash", script]:
                fail("genuine route %s no longer runs %s" % (name, script))
            if cargo or helpers:
                fail("genuine route %s must stay on plain Cargo" % name)
            if name + ":cargo" in tasks:
                fail("genuine route %s needs no :cargo sibling" % name)
            observed[name] = ("genuine", genuine_reason)
            continue

        if reason is not None:
            if helpers:
                fail("pending route %s must stay on plain Cargo" % name)
            if name + ":cargo" in tasks:
                fail("pending route %s needs no :cargo sibling" % name)
            if not cargo:
                script_paths = []
                for command in strings:
                    words = shlex.split(command)
                    if len(words) < 2 or words[0] != "bash":
                        fail("pending route %s reaches Cargo unexpectedly" % name)
                    script_paths.append(words[1])
                if not script_paths:
                    fail("pending route %s no longer reaches Cargo" % name)
                for script in script_paths:
                    if SCRIPT_CLASSES.get(script) != "pending":
                        fail(
                            "pending route %s runs %s, which is not classified "
                            "pending" % (name, script)
                        )
            observed[name] = ("pending", reason)
            continue

        if name in SCRIPT_ROUTES:
            if helpers:
                fail("script route %s also calls the helper directly" % name)
            script = SCRIPT_ROUTES[name]
            expected = (name + ":cargo") in tasks
            if not expected:
                fail("script route %s has no :cargo sibling" % name)
            sibling = [
                command for command in tasks[name + ":cargo"] if command is not None
            ]
            wanted = [PLAIN_ESCAPE + command for command in strings]
            if sibling != wanted:
                fail(
                    "sibling %s:cargo does not force the plain branch:\n"
                    "  actual:   %r\n  expected: %r" % (name, sibling, wanted)
                )
            for command in sibling:
                if "oxide" in command:
                    fail("sibling %s:cargo puts Oxide back on the plain route" % name)
            observed[name] = ("script", "maintained script routes through the helper")
            switched[name] = {"script": script, "cargo": sibling}
            continue

        if helpers:
            if cargo:
                fail("switched selector %s keeps a bare cargo command" % name)
            for command in helpers:
                helper_tail(command)
                packages = selected_packages(command)
                for package, binary in sorted(PRIVATE_TARGET_TEST_BINARIES):
                    if not re.search(
                        r"binary\(\s*%s\s*\)" % re.escape(binary), command
                    ):
                        continue
                    if packages and package not in packages:
                        continue
                    fail(
                        "switched selector %s selects the whole %s test binary "
                        "of %s, which contains private-target tests; keep it "
                        "pending one-use support" % (name, binary, package)
                    )
            switched[name] = {"script": None, "cargo": [plain_twin(c) for c in strings]}
            observed[name] = ("switched", "ordinary local Rust route")
            continue

        if cargo:
            fail(
                "selector %s spells a Cargo command but is neither switched nor "
                "a documented pending or genuine exception" % name
            )

    return switched, observed


def check_siblings(tasks, switched):
    for name, switcher in switched.items():
        sibling = name + ":cargo"
        commands = [command for command in tasks[sibling] if command is not None]
        expected = switcher["cargo"]
        if commands != expected:
            fail(
                "sibling %s does not repeat the plain Cargo arguments:\n"
                "  actual:   %r\n  expected: %r" % (sibling, commands, expected)
            )
        if "oxide" in " ".join(commands):
            fail("sibling %s puts Oxide back on the plain route" % sibling)
        if switcher["script"] is None and not any(
            spells_cargo(command) for command in commands
        ):
            fail("sibling %s carries no plain Cargo command" % sibling)

    for name in tasks:
        if name.endswith(":cargo") and name[: -len(":cargo")] not in switched:
            fail("%s is an orphan :cargo sibling" % name)


def check_entry_table(tasks, table, label):
    for name in table:
        if name not in tasks:
            fail("stale %s entry: %s no longer exists" % (label, name))


def check_scripts(referenced_scripts):
    observed = {}
    for path in sorted((ROOT / "scripts").rglob("*.sh")):
        relative = str(path.relative_to(ROOT))
        text = path.read_text()
        has_helper = "oxide-rust.sh" in text
        helper_text = text
        has_exec = "oxide exec --" in text
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
        if expected == "helper":
            continue
        if expected == "switched":
            if not has_helper:
                fail("%s is classified switched but never calls the helper" % relative)
            if cargo_lines:
                fail("%s still runs plain Cargo: %r" % (relative, cargo_lines))
            if "--workspace" in helper_text and ROUTED_TEST_LANE.search(helper_text):
                fail(
                    "%s routes a whole-workspace test lane, which includes "
                    "private-target tests; keep it pending" % relative
                )
            continue
        if expected == "host-tool":
            if not has_helper or "--host-tool" not in text:
                fail(
                    "%s must use the helper's --host-tool mode for one lease" % relative
                )
            if "oxide exec --" in text:
                fail("%s bypasses the shared host-tool routing" % relative)
            continue
        if expected == "genuine":
            if has_helper:
                fail("%s must keep plain Cargo" % relative)
            continue
        if expected == "pending":
            if has_helper or has_exec:
                fail("%s must keep its private target on plain Cargo" % relative)
            if not cargo_lines:
                fail("%s no longer runs Cargo" % relative)
            continue
        if expected == "query":
            if has_helper or has_exec:
                fail("%s must stay a plain query" % relative)
            if not cargo_lines:
                fail("%s no longer runs Cargo" % relative)
            continue
        if expected == "fake-proof":
            if cargo_lines:
                fail("%s invokes real Cargo instead of fake tools" % relative)
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


def check_release_include():
    text = RELEASE_INCLUDE.read_text()
    # The commented whole-workspace test recipe runs the private-target
    # courier tests, so it stays on plain Cargo pending one-use support.
    if RELEASE_PENDING_TEST not in text:
        fail("release include lost its pending plain-Cargo workspace test recipe")
    if "cargo" in text.replace(RELEASE_PENDING_TEST, ""):
        fail("release include must route other Cargo commands through the helper")
    if re.search(r"oxide-rust\.sh\s+(nextest|test)\b.*--workspace", text):
        fail(
            "release include routes a whole-workspace test lane, which includes "
            "private-target tests; keep it pending"
        )
    if 'fmt = "bash scripts/oxide-rust.sh fmt --all -- --check"' not in text:
        fail("release include lost its routed format gate")
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
    referenced_scripts = set()
    for commands in tasks.values():
        for command in commands:
            if command is None:
                continue
            for word in shlex.split(command):
                if word.startswith("scripts/") and word.endswith(".sh"):
                    referenced_scripts.add(word)
    switched, observed = classify_selectors(tasks)
    check_siblings(tasks, switched)
    check_entry_table(tasks, PENDING, "pending")
    check_entry_table(tasks, GENUINE, "genuine")
    check_entry_table(tasks, SCRIPT_ROUTES, "script route")
    scripts = check_scripts(referenced_scripts)
    check_release_include()
    check_gitignore()
    check_agent_block()

    counts = {"switched": 0, "script": 0, "sibling": 0, "pending": 0, "genuine": 0}
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
    print("  cargo_siblings=%d" % counts["sibling"])
    print("  pending=%d" % counts["pending"])
    print("  genuine=%d" % counts["genuine"])
    print("  other=%d" % other)
    print("  maintained_scripts=%d" % len(scripts))
    print("  release_include=routed, workspace test recipe pending")
    print("  target_ignore=/target")
    print("  agent_block=generated")
    print("oxide selector routing passed")


if __name__ == "__main__":
    try:
        main(sys.argv[1:])
    except RoutingDefect as defect:
        print("oxide routing defect: %s" % defect, file=sys.stderr)
        sys.exit(1)
