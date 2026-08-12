# 217 Figmatic Claude Code Protocol Adoption

Status: ready
Owner: Tom
Created: 2026-08-12
Milestone: `../068-claude-code-response-only-protocol-compatibility.md`

## Goal

Adopt the exact Swallowtail protocol-compatible response-only commit in
Figmatic and resume the packaged `g04.005` mutation-runway smoke.

## Source Identity

- Swallowtail commit: `IMPLEMENTATION_COMMIT_PENDING`
- package: `swallowtail-adapter-claude-agent`
- route: `swallowtail.claude-code.response-only`
- compatibility: qualified from `2.1.227` through the latest evidenced point;
  later stable releases provisional unless explicitly denied

## Acceptance

- link the exact source through `effigy deps link cargo`
- Figmatic reaches generation through its unchanged prepared facade
- observed executable-version diagnostics are retained with smoke evidence
- returned text stays under Figmatic's existing parsing, validation,
  compilation, gates, and operator acceptance
- no API key, schema, tool, MCP, retry, continuation, fallback, alternate route,
  or new provider authority is added
