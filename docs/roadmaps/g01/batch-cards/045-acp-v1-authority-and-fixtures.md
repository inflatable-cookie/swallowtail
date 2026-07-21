# 045 ACP v1 Authority And Fixtures

Status: completed
Owner: Tom
Updated: 2026-07-20
Milestone: `../014-gemini-acp-proof.md`

## Objective

Promote exact ACP v1 lifecycle and callback rules and build deterministic
protocol fixtures.

## Scope

- initialization, authentication, new/load, prompt/update, cancellation,
  permissions, filesystem, terminal, modes, and extensions
- Gemini-specific capability inventory

## Out Of Scope

- universal ACP feature claims
- production driver

## Acceptance Criteria

- [x] baseline and optional methods are distinct
- [x] protocol version and capability negotiation fail closed
- [x] host/consumer callback authority maps to existing contracts
- [x] card 046 needs no fresh protocol decision

## Evidence

- official ACP authority rechecked at wire version `1`; stable schema release
  `schema-v1.19.0` is pinned by SHA-256 without vendoring the 198 KB schema
- Gemini CLI stable `0.51.0`, tag commit, ACP SDK `0.16.1`, dispatcher, and
  session-manager evidence are pinned independently
- Contract 015 separates negotiation, optional capabilities, permission
  requests, filesystem/terminal host callbacks, consumer tools, extensions,
  authentication, cancellation, and cleanup
- the first subset excludes authentication mutation, load/resume, mode/model
  mutation, MCP injection, filesystem writes, terminals, and native close
- current Google access evidence excludes consumer membership; later live
  profiles require exact paid API-key, Cloud, or enterprise access records
- bounded deterministic transcripts cover initialize, version mismatch, new
  session, prompt updates, permission cancellation, filesystem read, unknown
  extensions, authentication failure, and disconnect
- eight focused fixture tests pass without a Gemini binary or credential

## Validation

- protocol fixture tests
- `git diff --check`

## Stop Conditions

- ACP or Gemini authority is contradictory on required lifecycle behavior

## Auto-Continuation

Yes, only after card 046 is marked ready.
