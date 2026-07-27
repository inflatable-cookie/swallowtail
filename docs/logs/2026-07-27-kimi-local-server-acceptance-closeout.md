# Kimi Local Server Acceptance Closeout

Date: 2026-07-27
Roadmap: g02.020
Card: 065

## Changed

- promoted `kimi-code.local-server` as Swallowtail's 23rd production route
- kept Kimi ACP and local-server driver, transport, access, topology,
  lifecycle, and version identities separate
- published exact route and provider-session lifecycle rows
- documented attached and owned-foreground use without a container or sandbox
  claim
- added attached preparation, owned lifecycle, interactive, and ACP-binding
  import examples
- added a local-server guide and bounded Nucleus adoption inputs
- extended local and future-candidate package gates for the Kimi route

## Package Proof

All 23 crates assembled into `.crate` archives from the current source
snapshot. The extracted workspace compiled every target, including the four
new examples.

The extracted Kimi adapter then ran 21 deterministic tests:

- 2 route and compatibility tests
- 4 lifecycle and topology tests
- 8 cross-transport binding-import tests
- 7 interactive, callback, failure, and cleanup tests

The package content audit and stable-diagnostic redaction corpus passed. The
proof emitted no provider token, session id, working path, prompt, or payload.
No live credential or provider endpoint was used.

## Acceptance

- `kimi-code.acp` remains interactive ACP with load/replay/resume and no
  provider-session management
- `kimi-code.local-server` provides REST/WebSocket v2 interactive sessions and
  inactive archive/restore
- neither route provides hard delete
- local-server attached topology preserves the external server
- owned topology starts and joins one foreground Kimi child
- later stable releases remain visible unverified-newer attempts, not
  guaranteed support and not automatic hard denials
- session handle close preserves provider state; management is a separate
  post-close effect

## Validation

- full Kimi adapter: 56 deterministic tests passed; one live probe ignored
- full workspace tests passed
- strict workspace Clippy passed
- route, lifecycle, docs, Northstar, formatting, Rust, examples, package, and
  diff checks passed
- doctor retained 37 known findings, including 9 pre-existing oversized-file
  errors
- no candidate replacement, publication, push, tag, or consumer edit occurred

## Continuation

Roadmap g02.020 and cards 061-065 are complete. Card 060 remains in bounds as
the separate Nucleus thread-lifecycle handoff, but it stays planned until the
operator authorizes that consumer-facing lane. Card 059 remains paused behind
canonical source history. No automatic continuation is active.
