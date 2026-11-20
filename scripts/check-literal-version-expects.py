#!/usr/bin/env python3
"""Reject non-literal InterfaceVersion parse expects in production Rust source.

A provider-reachable `InterfaceVersion::new(...).expect(...)` panics on
malformed provider text. Version-parse expects must stay on string literals
or compile-time constants; this scan fails any statement that parses a
non-literal version and then expects. Comments, `#[cfg(test)]` modules,
test-support crates (testkit), tests, and examples are excluded.

Run `--self-test` to check the scanner's own tokenisation (raw strings, char
literals, lifetimes, constant detection) against synthetic sources.

Stop condition: the scan must not false-positive on macros or generated code.
"""

from __future__ import annotations

import re
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
CRATES = REPO / "crates"
MARKERS = ("InterfaceVersion::new(", "InterfaceVersionAxis::new(")
TEST_FILE = re.compile(r"(^|/)(tests?|examples?|.*_tests?)(/|\.rs$)")
# A compile-time constant, optionally path-qualified, such as `CURRENT` or
# `crate::versions::CURRENT`. It must match the whole argument expression.
CONSTANT_EXPRESSION = re.compile(r"(?:[A-Za-z_][A-Za-z0-9_]*::)*[A-Z][A-Z0-9_]*")
RAW_STRING_START = re.compile(r'r(#*)"')
# A char literal such as `'a'`, `'"'`, `'\''` or `'\u{1F600}'`. A lifetime such
# as `'a` never matches, because no closing quote follows the name.
CHAR_LITERAL = re.compile(r"'(?:\\.[^'\n]*|[^\\'\n])'")


def fail(message: str) -> None:
    print(f"version-parse expect check failed: {message}", file=sys.stderr)
    raise SystemExit(1)


def skip_literal(text: str, i: int) -> int | None:
    """Return the position after a string, raw string or char literal at i.

    Returns None when no literal starts at i. Char literals are skipped so that
    a quote inside `'"'` cannot open a string that swallows the rest of the file.
    """
    n = len(text)
    char = text[i]
    if char == '"':
        j = i + 1
        while j < n:
            if text[j] == "\\":
                j += 2
            elif text[j] == '"':
                return j + 1
            else:
                j += 1
        return n
    if char == "r":
        match = RAW_STRING_START.match(text, i)
        if match is None:
            return None
        # A raw string closes at a quote followed by the same number of hashes
        # as its opening, so `r#"..."#` ends at `"#` and not at the next `"`.
        closing = '"' + match.group(1)
        end = text.find(closing, match.end())
        return n if end == -1 else end + len(closing)
    if char == "'":
        match = CHAR_LITERAL.match(text, i)
        if match is not None:
            return match.end()
    return None


def is_string_start(text: str, i: int) -> bool:
    return text[i] == '"' or RAW_STRING_START.match(text, i) is not None


def mask_comments_and_strings(text: str) -> str:
    """Replaces comments and string contents with spaces.

    A string keeps its opening and closing quote, so the scan still sees a
    string literal where one stands, but a marker written inside a string is
    never code. Positions are preserved so findings map back to original lines.
    """
    out = list(text)
    i = 0
    n = len(text)
    while i < n:
        if text.startswith("//", i):
            end = text.find("\n", i)
            end = n if end == -1 else end
            out[i:end] = " " * (end - i)
            i = end
        elif text.startswith("/*", i):
            end = text.find("*/", i + 2)
            end = n if end == -1 else end + 2
            out[i:end] = " " * (end - i)
            i = end
        else:
            end = skip_literal(text, i)
            if end is None:
                i += 1
                continue
            if text[i] != "'" and end - i >= 2:
                out[i : end] = '"' + " " * (end - i - 2) + '"'
            i = end
    return "".join(out)


TEST_MODULE = re.compile(
    r"^\s*#\[cfg\(test\)\][^\n]*\n(?:\s*#\[path\s*=\s*\"[^\"]*\"\]\s*\n)?"
    r"\s*mod\s+\w+\s*\{",
    re.MULTILINE,
)


def mask_cfg_test_modules(text: str) -> str:
    """Masks `#[cfg(test)] mod tests { ... }` bodies so test code is skipped."""
    out = list(text)
    for match in TEST_MODULE.finditer(text):
        start = match.start()
        depth = 1
        i = match.end()
        while i < len(text) and depth > 0:
            end = skip_literal(text, i)
            if end is not None:
                i = end
                continue
            char = text[i]
            if char == "{":
                depth += 1
            elif char == "}":
                depth -= 1
            i += 1
        if depth == 0:
            out[start:i] = " " * (i - start)
    return "".join(out)


def argument_end(text: str, start: int) -> int:
    """Return the position of the comma or closing paren that ends an argument."""
    depth = 0
    i = start
    while i < len(text):
        end = skip_literal(text, i)
        if end is not None:
            i = end
            continue
        char = text[i]
        if char in "([{":
            depth += 1
        elif char in ")]}":
            if depth == 0:
                return i
            depth -= 1
        elif char == "," and depth == 0:
            return i
        i += 1
    return len(text)


def scan(text: str) -> list[tuple[int, str]]:
    findings: list[tuple[int, str]] = []
    position = 0
    while True:
        best = None
        for marker in MARKERS:
            index = text.find(marker, position)
            if index != -1 and (best is None or index < best[0]):
                best = (index, len(marker))
        if best is None:
            return findings
        index, marker_len = best
        position = index + marker_len

        argument = position
        while argument < len(text) and text[argument].isspace():
            argument += 1
        stop = argument_end(text, argument)
        expression = text[argument:stop].strip()
        # The argument is a literal only when one whole string literal is the
        # argument; `"1.0".repeat(2)` starts with a literal but is not one.
        literal_end = skip_literal(text, argument) if is_string_start(text, argument) else None
        literal = literal_end is not None and text[literal_end:stop].strip() == ""
        constant = CONSTANT_EXPRESSION.fullmatch(expression) is not None

        depth = 1
        cursor = position
        while cursor < len(text) and depth > 0:
            end = skip_literal(text, cursor)
            if end is not None:
                cursor = end
                continue
            char = text[cursor]
            if char == "(":
                depth += 1
            elif char == ")":
                depth -= 1
            cursor += 1
        if depth != 0:
            continue

        statement = cursor
        depth = 0
        while statement < len(text):
            end = skip_literal(text, statement)
            if end is not None:
                statement = end
                continue
            char = text[statement]
            if char == "(":
                depth += 1
            elif char == ")":
                depth -= 1
            elif char in ";,}" and depth <= 0:
                break
            statement += 1
        if ".expect(" in text[cursor:statement] and not literal and not constant:
            findings.append((index, text[index:statement].strip()))
    return findings


def production_file(path: Path) -> bool:
    if TEST_FILE.search(str(path.relative_to(REPO))):
        return False
    relative = path.relative_to(CRATES)
    if "tests" in relative.parts or "examples" in relative.parts:
        return False
    # Test-support infrastructure: its fixture helpers are not
    # provider-reachable and panics there are test failures.
    if relative.parts[0] == "swallowtail-testkit":
        return False
    return True


# Each case is (source, expected number of findings).
SELF_TEST_CASES: tuple[tuple[str, int], ...] = (
    # A raw string must end at its own closing hashes, not swallow later code.
    ('let a = r#"one"#; InterfaceVersion::new(value).expect("bad");', 1),
    ('let a = r##"x "# y"##; InterfaceVersion::new(value).expect("bad");', 1),
    # Raw-string arguments are literals, and raw-string contents are not code.
    ('InterfaceVersion::new(r#"1.0.0"#).expect("bad");', 0),
    ('let s = r#"InterfaceVersion::new(value).expect("bad")"#;', 0),
    # A quote inside a char literal must not open a string.
    ("let q = '\"'; InterfaceVersion::new(value).expect(\"bad\");", 1),
    # Constants, including path-qualified ones, are allowed.
    ('InterfaceVersion::new(SUPPORTED_VERSION).expect("bad");', 0),
    ('InterfaceVersion::new(crate::versions::SUPPORTED).expect("bad");', 0),
    # A constant-looking first letter is not enough: the whole argument must
    # be a constant.
    ('InterfaceVersion::new(VERSION.trim()).expect("bad");', 1),
    ('InterfaceVersion::new(VERSION_TEXT.as_str()).expect("bad");', 1),
    ('InterfaceVersion::new("1.0.0".repeat(2)).expect("bad");', 1),
    ('InterfaceVersion::new(value).expect("bad");', 1),
    ('InterfaceVersion::new("1.0.0").expect("bad");', 0),
    # Lifetimes are not char literals.
    ("fn f<'a>(v: &'a str) { InterfaceVersion::new(v).expect(\"bad\"); }", 1),
    # Comments and cfg(test) modules are excluded.
    ('// InterfaceVersion::new(value).expect("bad")', 0),
    (
        '#[cfg(test)]\nmod tests {\n    fn t() { InterfaceVersion::new(value).expect("bad"); }\n}\n',
        0,
    ),
)


def self_test() -> int:
    failures: list[str] = []
    for source, expected in SELF_TEST_CASES:
        found = scan(mask_cfg_test_modules(mask_comments_and_strings(source)))
        if len(found) != expected:
            failures.append(
                f"expected {expected} finding(s), found {len(found)}: {source!r}"
            )
    if failures:
        for failure in failures:
            print(f"self-test: {failure}", file=sys.stderr)
        fail(f"{len(failures)} scanner self-test case(s) failed")
    print(f"version-parse expect self-test passed: {len(SELF_TEST_CASES)} cases")
    return 0


def main() -> int:
    if sys.argv[1:] == ["--self-test"]:
        return self_test()
    findings: list[tuple[Path, int, str]] = []
    for path in sorted(CRATES.glob("*/src/**/*.rs")):
        if not production_file(path):
            continue
        text = path.read_text(encoding="utf-8")
        masked = mask_cfg_test_modules(mask_comments_and_strings(text))
        for index, statement in scan(masked):
            line = text.count("\n", 0, index) + 1
            findings.append((path, line, statement))
    if findings:
        for path, line, statement in findings:
            print(f"{path}:{line}: {statement}", file=sys.stderr)
        fail(f"{len(findings)} non-literal version-parse expect(s) found")
    print(
        "version-parse expect check passed: "
        "no non-literal InterfaceVersion parse is expected in production source"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
