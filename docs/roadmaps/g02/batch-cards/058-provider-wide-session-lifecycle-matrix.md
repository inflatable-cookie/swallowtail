# 058 Provider-Wide Session Lifecycle Matrix

Status: completed
Owner: Tom
Created: 2026-07-26
Updated: 2026-07-27
Milestone: `../019-provider-session-lifecycle-acceptance-and-handoff.md`

## Objective

Classify every production route as supported, unsupported, or not applicable
for provider-session archive, restore, and deletion.

## Governing Refs

- Research 036
- Contract 038
- completed roadmaps 016-018
- provider route matrix and exact compatibility claims

## Scope

1. Extend the exact 22-route matrix with persistent-session applicability,
   management binding, archive, restore, delete, deletion strength, version
   posture, and driver-owned-cleanup columns.
2. Mark Codex, Claude Agent, and OpenCode from production evidence.
3. Mark Kimi and Gemini ACP explicitly unsupported under their selected
   transports.
4. Mark non-persistent routes not applicable.
5. Keep Alibaba conversation and Anthropic Managed Agent automatic cleanup
   separate.
6. Add a machine check for route count, uniqueness, and lifecycle posture.

## Acceptance Criteria

- [x] every production route appears exactly once
- [x] unsupported and not applicable remain distinct
- [x] no private CLI, SDK, REST, filesystem, or UI route substitutes for the
      selected driver
- [x] no local Nucleus capability is attributed to Swallowtail
- [x] deletion strength and exact version evidence are visible
- [x] driver-owned cleanup cannot satisfy user-directed management

## Validation

- route matrix inventory check
- docs links and formatting
- focused adapter claim checks
- `git diff --check`

## Stop Conditions

- a route cannot be classified from canonical evidence
- classification requires a new provider or transport policy
- a matrix row would imply automatic routing or fallback

## Auto-Continuation

Yes after roadmaps 016-018 close. Continue to card 059.

## Outcome

The production guide now carries a second exact 22-route matrix for provider
session lifecycle. Codex app-server, Claude Agent ACP, and OpenCode HTTP/SSE
are supported only for their qualified actions. Kimi Code and Gemini CLI ACP
are explicitly unsupported on their selected transports. The remaining
seventeen routes are not applicable under their current operation shapes.

Every row records management-binding availability, archive, restore, delete,
deletion strength, version posture, and driver-owned cleanup. The guide
forbids alternate private-route substitution and states that cleanup grants no
user-directed management authority.

The route check now validates both matrices independently: exact route count,
uniqueness, canonical inventory, lifecycle classification, management
binding, action posture, deletion strength, and non-empty version and cleanup
evidence.

## Validation Evidence

- route and lifecycle matrix check: 22 canonical routes passed
- Codex lifecycle corpus: 5 passed
- Claude Agent prepared management: 5 passed
- OpenCode deletion range: 4 passed
- OpenCode prepared deletion: 5 passed
