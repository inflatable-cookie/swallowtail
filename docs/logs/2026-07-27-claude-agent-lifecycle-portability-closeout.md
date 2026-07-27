# Claude Agent Lifecycle Portability Closeout

Date: 2026-07-27
Card:
`../roadmaps/g02/batch-cards/054-claude-agent-lifecycle-portability-closeout.md`

## Outcome

Roadmap g02.017 is complete.

The Claude production stdio driver passes one management contract across all
four qualified behavior segments. The matrix retains the unpublished
`0.58.0` exclusion and requires explicit acceptance for unverified-newer
`0.62.0` deletion.

Missing capability stops before delete. Missing target, provider rejection,
disconnect after dispatch, and malformed success remain unconfirmed after the
effect boundary. Cancellation and deadline distinguish pre-dispatch failure
from post-dispatch uncertainty. Process join precedes resource release and
credential release. The prepared path exposes only caller-asserted inactive
management; an active-target request is not constructible.

The real remote WebSocket ACP transport carries the same qualified initialize
and delete records under local and remote-authoritative host identities. A
remote disconnect opens one connection and has no process service, retry,
reconnect, or stdio fallback.

This is transport portability evidence. Remote ACP remains provider-neutral
and unauthenticated. Swallowtail does not claim an authenticated remote Claude
endpoint.

## Validation

- Claude Agent package: 25 tests pass
- ACP protocol, remote transport, runtime, and provider-session testkit suites
  pass
- changed-crate clippy, Rust, format, docs, Northstar, and diff checks pass
- default validation uses no live authentication
- inherited doctor findings remain unchanged

## Continuation

Card 055 is ready. It traces OpenCode deletion across the exact maintained
range before any production delete path is authorized. Cards 056-060 remain
in bounds.
