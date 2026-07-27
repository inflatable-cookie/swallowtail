# 015 Provider Session Management Foundation

Status: completed
Owner: Tom
Created: 2026-07-26
Depends on: g02.007 and Contract 038
Vision tags: consumer boundary, persistent sessions, lifecycle truth
Contract refs: 008-011, 017, 029, 037-038
Planning state: cards 046-048 complete

## Problem

Swallowtail can close runtime attachments and delete driver-owned remote
resources. It cannot archive, restore, or delete one consumer-authorized
persistent provider session without adapter-specific assembly.

## Goals

- [x] Add the smallest provider-neutral management identity and capability
      vocabulary.
- [x] Add one side-effect-free plan and low-level runtime role for inactive
      bound provider sessions.
- [x] Preserve exact deletion strength, uncertainty, version, host, and access
      truth.
- [x] Add reusable deterministic conformance before a provider adapter.

## Execution Plan

### Batch 15.1 — Core Records

- [x] Execute card 046.

### Batch 15.2 — Runtime Role

- [x] Execute card 047 over the accepted public record shape.

### Batch 15.3 — Conformance

- [x] Execute card 048 after the low-level role compiles.

## Acceptance Criteria

- [x] arbitrary provider session ids grant no management authority
- [x] archive, restore, delete, provider-native close, and attachment close
      remain distinct
- [x] deletion strength and provider-defined descendant scope are explicit
- [x] unsupported and incompatible actions fail before effects
- [x] after-effect uncertainty cannot become confirmation
- [x] no consumer persistence, global registry, history browser, retry, or
      fallback enters the shared crates

## Runway

This milestone advances g02's additive API-stabilization goal. Roadmaps
016-018 apply the accepted role to Codex, ACP/Claude Agent, and OpenCode.
Roadmap 019 is the provider-wide checkpoint and Nucleus handoff. Card 049 is
the sole ready continuation.
