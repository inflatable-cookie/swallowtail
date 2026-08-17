# 217 Figmatic Claude Code Protocol Adoption

Status: completed
Owner: Tom
Created: 2026-08-12
Updated: 2026-08-17
Milestone: `../068-claude-code-response-only-protocol-compatibility.md`

## Goal

Adopt the exact Swallowtail protocol-compatible response-only commit in
Figmatic and resume the packaged `g04.005` mutation-runway smoke.

## Source Identity

- Swallowtail commit: `fd2d95e83f6d7f8bdfe7852d3393eb6031d15cf2`
- package: `swallowtail-adapter-claude-agent`
- route: `swallowtail.claude-code.response-only`
- compatibility: qualified from `2.1.227` through the latest evidenced point;
  later stable releases provisional unless explicitly denied

## Acceptance

- [x] link the exact source through `effigy deps link cargo`
- [x] Figmatic reaches generation through its unchanged prepared facade
- [x] observed executable-version diagnostics are retained with smoke evidence
- [x] returned text stays under Figmatic's existing parsing, validation,
      compilation, gates, and operator acceptance
- [x] no API key, schema, tool, MCP, retry, continuation, fallback, alternate
      route, or new provider authority is added

## Evidence

Operator closeout from the Figmatic thread: packaged `g04.005` mutation-runway
smoke ran against Swallowtail pin `fd2d95e83f6d7f8bdfe7852d3393eb6031d15cf2`.
Figmatic remains the evidence owner. This repository was not mutated.
