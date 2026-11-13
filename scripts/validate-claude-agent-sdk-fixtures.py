#!/usr/bin/env python3
"""Validate every Claude Agent SDK proof fixture against its complete schema.

The fixtures are evidence: a plan, a probe-only admission, the selected option
template, the wrapper classification ledger and the managed-denial contract.
Each one carries a JSON Schema beside it, and this script applies the schema
rather than merely parsing the document, so a field that drifts or a constraint
that stops holding fails here.

It reads the committed files only. It never downloads, installs, executes or
contacts anything.

  --check       validate every document (default)
  --self-test   mutate each document in memory and prove the schema refuses it
"""

from __future__ import annotations

import argparse
import copy
import hashlib
import json
import pathlib
import sys

from jsonschema import Draft7Validator

FIXTURE = pathlib.Path(
    "crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-agent-sdk-native-proof"
)
CONTRACT = pathlib.Path(
    "crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-agent-sdk-v2"
)

DOCUMENTS = [
    ("probe plan", FIXTURE / "probe-plan.json", FIXTURE / "probe-plan.schema.json"),
    (
        "probe admission",
        FIXTURE / "probe-admission.json",
        FIXTURE / "probe-admission.schema.json",
    ),
    (
        "probe options",
        FIXTURE / "probe-options.json",
        FIXTURE / "probe-options.schema.json",
    ),
    (
        "selected flow",
        FIXTURE / "selected-flow.json",
        FIXTURE / "selected-flow.schema.json",
    ),
    (
        "managed-denial contract",
        CONTRACT / "managed-denial-contract.json",
        FIXTURE / "managed-denial-contract.schema.json",
    ),
]


def load(path: pathlib.Path) -> object:
    return json.loads(path.read_text())


def validate(document: object, schema: object, label: str) -> list[str]:
    validator = Draft7Validator(schema)
    errors = sorted(validator.iter_errors(document), key=lambda error: list(error.path))
    return [
        f"{label}: {'/'.join(str(part) for part in error.path)}: {error.message}"
        for error in errors
    ]


def check() -> int:
    failures: list[str] = []
    for label, document_path, schema_path in DOCUMENTS:
        if not document_path.is_file():
            failures.append(f"{label}: {document_path} is missing")
            continue
        if not schema_path.is_file():
            failures.append(f"{label}: {schema_path} is missing")
            continue
        failures.extend(validate(load(document_path), load(schema_path), label))
    # Detached from the admission: the runner pins the admission, while this
    # record names all three final files without a mutual-hash cycle.
    execution_sources = [
        pathlib.Path("crates/swallowtail-adapter-claude-agent/tests/native_proof_support/probe-runner.mjs"),
        FIXTURE / "probe-admission.json",
        FIXTURE / "probe-plan.json",
    ]
    expected = "".join(
        f"{hashlib.sha256(source.read_bytes()).hexdigest()}  {source}\n"
        for source in execution_sources
    )
    if (FIXTURE / "probe-execution.sha256").read_text() != expected:
        failures.append("detached execution checksums are stale or incomplete")
    for failure in failures:
        print(failure, file=sys.stderr)
    if failures:
        return 1
    print(f"validated {len(DOCUMENTS)} documents against their schemas")
    return 0


def self_test() -> int:
    """Each mutation must be refused, so a schema cannot silently pass."""
    failures: list[str] = []

    schemas = {document.name: schema for _, document, schema in DOCUMENTS}

    def expect_refused(label: str, document_path: pathlib.Path, mutate) -> None:
        schema = load(schemas[document_path.name])
        document = copy.deepcopy(load(document_path))
        mutate(document)
        if not validate(document, schema, label):
            failures.append(f"{label}: mutation was accepted")

    def document(name: str) -> pathlib.Path:
        return next(path for label, path, _ in DOCUMENTS if path.name == name)

    expect_refused(
        "plan without original-execution flag",
        document("probe-plan.json"),
        lambda plan: plan["authorized_stage"]["original_artifact_execution"].pop("enabled"),
    )
    expect_refused(
        "plan that claims original execution",
        document("probe-plan.json"),
        lambda plan: plan["authorized_stage"]["original_artifact_execution"].update(
            {"enabled": True}
        ),
    )
    expect_refused(
        "admission that claims production admission",
        document("probe-admission.json"),
        lambda admission: admission.update({"production_admission": True}),
    )
    expect_refused(
        "admission outside probe mode",
        document("probe-admission.json"),
        lambda admission: admission.update({"mode": "production"}),
    )
    expect_refused(
        "admission with an unconstrained bound",
        document("probe-admission.json"),
        lambda admission: admission["bounds"].update({"wall_clock_seconds": 0}),
    )
    expect_refused(
        "admission with a stale digest",
        document("probe-admission.json"),
        lambda admission: admission["tuples"][0].update({"wrapper_tarball_sha256": "0" * 63}),
    )
    expect_refused(
        "admission with an unknown tuple field",
        document("probe-admission.json"),
        lambda admission: admission["tuples"][0].update({"native_notes": "x"}),
    )
    expect_refused(
        "options that select a setting source",
        document("probe-options.json"),
        lambda options: options["options"].update({"settingSources": ["user"]}),
    )
    expect_refused(
        "selected flow without a closure record",
        document("selected-flow.json"),
        lambda flow: flow.pop("import_closure"),
    )
    expect_refused(
        "contract that adopts v2 in a claim",
        document("managed-denial-contract.json"),
        lambda contract: contract["claim_effect"].update(
            {"behavior_revision_used_by_claims": "claude-agent.sdk-v2"}
        ),
    )
    expect_refused(
        "contract with a different ruling",
        document("managed-denial-contract.json"),
        lambda contract: contract["ruling"].update({"decision": "another-decision"}),
    )

    for failure in failures:
        print(f"self-test failure: {failure}", file=sys.stderr)
    if failures:
        return 1
    print(f"fixture schema self-test: {len(failures) or 11} mutations refused")
    return 0


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()
    return self_test() if args.self_test else check()


if __name__ == "__main__":
    raise SystemExit(main())
