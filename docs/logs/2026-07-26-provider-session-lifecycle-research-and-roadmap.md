# Provider Session Lifecycle Research And Roadmap

Date: 2026-07-26

## Decision

Keep two independent lifecycle planes:

- Nucleus owns local thread archive, restore, deletion, persistence, and UI.
- Swallowtail optionally manages one bound inactive provider session when the
  exact driver, transport, version, capability, and access permit it.

Runtime attachment close, provider-native active close, reversible archive,
restore, history removal, provider data deletion, hard deletion, and
driver-owned cleanup remain separate.

## Evidence

Research 036 checked:

- Codex app-server archive, unarchive, and hard-delete methods
- stable ACP v1 close and soft-or-hard delete semantics
- Claude Agent ACP advertised and implemented close/delete behavior
- OpenCode HTTP session deletion
- Kimi and Gemini ACP absence under the selected transports
- T3 Code's consumer-local archive/delete and provider-stop boundary
- current Swallowtail bindings, close paths, qualified ranges, and
  driver-owned cleanup

The evidence supports Codex, Claude Agent ACP, and OpenCode as the first
applicable routes. Kimi and Gemini ACP remain explicit unsupported mappings.
Other production routes are not applicable or retain their existing
driver-owned cleanup.

## Promotion

- Research 036 is promoted.
- Contract 038 defines the durable binding, capability, action,
  deletion-strength, effect-truth, version, authority, diagnostics, and
  conformance rules.
- Contract 017 now points explicit user-directed lifecycle to Contract 038.
- System architecture records the contracted realization gap and dependency
  direction.

## Roadmap

g02 remains active and grows from 14 to 19 roadmaps:

- g02.015 — provider-neutral management foundation
- g02.016 — Codex lifecycle range and production proof
- g02.017 — ACP lifecycle refresh and Claude Agent proof
- g02.018 — OpenCode deletion proof
- g02.019 — provider-wide acceptance and Nucleus handoff

Cards 046-060 provide the execution runway. Card 046 is ready. Provider
implementations remain gated behind cards 046-048.

## Validation

- `effigy qa:docs` passes
- `effigy qa:northstar` passes
- `git diff --check` passes
- `effigy doctor` retains the known 21 oversized-file findings: 14 warnings
  and 7 errors

## Held Boundaries

- no Nucleus edit
- no provider call or live authentication
- no provider history browser or binding import
- no active-session management or global registry
- no implicit provider deletion
- no package publication, push, tag, workflow, owner, or release mutation
