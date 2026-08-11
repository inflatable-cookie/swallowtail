# 203 Claude Code Tool-Free Text Driver

Status: completed
Owner: Tom
Updated: 2026-08-11

## Goal

Implement the exact `2.1.227` response-only driver and deterministic protocol
corpus.

## Scope

- distinct descriptor, compatibility axis, exact version claim, and discovery
- exact command arguments with safe mode, empty tools/MCP, and no persistence
- bounded stream parser and ordinary text projection
- cancellation, deadline, terminal failure, and joined cleanup
- deterministic success, drift, malformed, bounds, cancellation, and timeout
  fixtures

## Out Of Scope

- prepared facade, guide, live probe, Figmatic edit, or release

## Acceptance Criteria

- [x] old headless command and fixtures remain unchanged
- [x] new parser accepts exactly one assistant text response and one-turn result
- [x] tools, MCP, user events, tool blocks, duplicate response, structured
      output, or missing text fail safely
- [x] process request has no working resource
- [x] all stable failures are redacted

## Validation

- `effigy validate:focused swallowtail-adapter-claude-agent`
