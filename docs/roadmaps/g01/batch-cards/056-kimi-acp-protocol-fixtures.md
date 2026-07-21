# 056 Kimi ACP Protocol Fixtures

Status: complete
Owner: Tom
Updated: 2026-07-20
Milestone: `../018-kimi-code-acp-portability-proof.md`

## Objective

Freeze the card 055 Kimi ACP subset in bounded deterministic fixtures without a
binary, account, credential, or production driver.

## Governing References

- `../../../research/006-kimi-code-acp-currentness-and-persistent-session-evidence.md`
- `../../../contracts/015-acp-v1-negotiation-and-client-callbacks.md`
- `../../../contracts/017-provider-owned-session-load-replay-and-host-containment.md`
- `../018-kimi-code-acp-portability-proof.md`

## Scope

- Kimi Code `0.28.1`, ACP adapter `0.3.4`, SDK `0.23.0`, wire `1`, and
  schema `v1.19.1` as independent pins
- exact initialize and capability negotiation
- one bound new session plus provider-owned load/resume and replay evidence
- bounded prompt/update, cancellation, and host-authorized write-callback
  transcripts
- disconnect, unknown capability, version drift, redaction, and cleanup cases

## Out Of Scope

- production process mapping
- live login or provider inference
- capability claims not promoted by card 055
- provider permission approval or process-containment implementation

## Acceptance Criteria

- [x] shared ACP decoder remains provider-neutral
- [x] Kimi capability and extension evidence stays adapter-local
- [x] replay and write cases fail closed on correlation or authority mismatch
- [x] fixtures need no installed binary or credential
- [x] no fixture treats a write callback as tool approval or process isolation

## Validation

- focused protocol fixture tests
- focused clippy
- `git diff --check`

## Evidence

- exact pins and source hashes live in
  `crates/swallowtail-protocol-acp/tests/fixtures/acp-v1-kimi-code-0.28.1/protocol.json`
- 17 deterministic artifacts cover initialize, new, load, resume, prompt,
  cancellation, writes, drift, auth failure, and disconnect
- 10 Kimi-specific assertions and all 20 ACP crate tests pass
- focused clippy, formatting, fixture whitespace, and diff checks pass
- no shared ACP source, provider binary, account, credential, or live endpoint
  changed or ran

## Stop Conditions

- the exact versioned subset cannot be reproduced deterministically

## Auto-Continuation

No. Card 057 is ready for an evidence-first containment proof.
