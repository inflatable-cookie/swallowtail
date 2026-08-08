#!/usr/bin/env python3
"""Reject non-literal InterfaceVersion parse expects in production Rust source.

A provider-reachable `InterfaceVersion::new(...).expect(...)` panics on
malformed provider text. Version-parse expects must stay on string literals
or compile-time constants; this scan fails any statement that parses a
non-literal version and then expects. Comments, `#[cfg(test)]` modules,
test-support crates (testkit), tests, and examples are excluded.

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


def fail(message: str) -> None:
    print(f"version-parse expect check failed: {message}", file=sys.stderr)
    raise SystemExit(1)


def fail(message: str) -> None:
    print(f"version-parse expect check failed: {message}", file=sys.stderr)
    raise SystemExit(1)


def skip_string(text: str, i: int) -> int:
    """Returns the position after a regular or raw string starting at i."""
    n = len(text)
    if text[i] == "r" and i + 1 < n:
        hashes = 0
        j = i + 1
        while j < n and text[j] == "#":
            hashes += 1
            j += 1
        if j < n and text[j] == '"':
            j += 1
            while j < n:
                if text[j] == '"' and text.startswith("#" * hashes + '"', j + 1):
                    return j + 1 + hashes + 1
                j += 1
            return n
    if text[i] != '"':
        return i + 1
    i += 1
    while i < n:
        if text[i] == "\\":
            i += 2
        elif text[i] == '"':
            return i + 1
        else:
            i += 1
    return n


def mask_comments_and_strings(text: str) -> str:
    """Replaces comments with spaces while preserving string contents.

    Positions are preserved so findings map back to original line numbers.
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
        elif text[i] == '"' or (text[i] == "r" and i + 1 < n and text[i + 1] in '#"'):
            i = skip_string(text, i)
        else:
            i += 1
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
            char = text[i]
            if char == "{":
                depth += 1
            elif char == "}":
                depth -= 1
            elif char == '"' or (char == "r" and i + 1 < len(text) and text[i + 1] in '#"'):
                i = skip_string(text, i) - 1
            i += 1
        if depth == 0:
            out[start:i] = " " * (i - start)
    return "".join(out)


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
        literal = argument < len(text) and text[argument] == '"'
        constant = argument < len(text) and re.fullmatch(
            r"(?:[A-Za-z_][A-Za-z0-9_]*::)*[A-Z][A-Z0-9_]*", text[argument]
        )

        depth = 1
        cursor = position
        while cursor < len(text) and depth > 0:
            char = text[cursor]
            if char == "(":
                depth += 1
            elif char == ")":
                depth -= 1
            elif char == '"' or (char == "r" and cursor + 1 < len(text) and text[cursor + 1] in '#"'):
                cursor = skip_string(text, cursor) - 1
            cursor += 1
        if depth != 0:
            continue

        statement = cursor
        depth = 0
        while statement < len(text):
            char = text[statement]
            if char == '"' or (char == "r" and statement + 1 < len(text) and text[statement + 1] in '#"'):
                statement = skip_string(text, statement) - 1
            elif char == "(":
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


def main() -> int:
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
