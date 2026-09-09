# 153 Claude SDK Live Registered-Tool Qualification

Status: complete; PR 298 merged at `b35e4c385746f2dc1145630d8ce2076f5dbb7b0f` (reviewed head `34ef6c322134a2b19dacb747b0e8724b1f702415`)
Owner: Claude Agent SDK adapter worker
Created: 2026-09-09
Milestone: `../035-shared-harness-capability-and-producer-boundary.md`
Depends on: Card 152 merged at `24f88fb8`; Research 301; accepted Desktop Card 318 capsule

## Goal

Consume the accepted Card 318 live evidence and qualify the existing
`claude-agent.sdk` mediated-stdio registered-tool route for the exact tuple that
ran.

## Scope

1. Replace `real_route_gate_pending` with an exact
   `RegisteredToolRouteQualification::Qualified` disposition. Bind it to SDK
   `0.3.259`, native `2.1.259` Darwin arm64 and its frozen digest, Node
   `22.23.2`, sidecar tag `0.4.4`, private-loopback plus mediated-stdio, MCP
   `2025-11-25`, and the existing carrier revision. Do not widen any axis.
2. Freeze Research 301's source-linked Swallowtail SHA, Desktop task, capsule,
   PR, accepted head, independent review, merge, closeout, courier, attempt
   counts, dispatch counts, correlation, and cleanup facts in deterministic
   route tests and documentation. Do not copy credentials, machine paths, raw
   provider prose, or unbounded capsule content.
3. Reconcile Contract 061 projection and the Claude SDK guide for registered
   tools and consumer tool exchange only. Preserve the existing registered
   permission, progress, selected-skill, omission, and lifecycle semantics.
4. Change only `registered_tools` and `consumer_tool_exchange` to available in
   the feature matrix and remove their completed Card 152 producer-gap
   references. Leave every other cell and cross classification unchanged.
5. Update `[Unreleased]` and the additive public API baseline only where the
   exact qualification changes those surfaces.

Owned mutable paths: `crates/swallowtail-adapter-claude-agent/**`;
`release-baselines/public-api-0.4.4/swallowtail-adapter-claude-agent.txt`
additively; the `claude-agent.sdk` rows/sections in Contract 061 and provider
guides/matrix; `CHANGELOG.md` `[Unreleased]`; this card's `## Result`;
`PAPERCUTS.md` append only. Queue closeout owns shared roadmap, index, handoff,
and factual-log status surfaces.

Forbidden: provider execution; Desktop edits; another live run; credentials or
account state; dependency/version extension; other adapter/runtime/kernel
behavior; unrelated matrix claims; candidate preparation; tag; release.

## Acceptance Criteria

- [x] the registered-tool projection is qualified only for the exact accepted tuple
- [x] deterministic tests bind the claim to every Research 301 evidence identity
- [x] Allow proves one unchanged `{}` dispatch and correlated `{"ok":true}` result
- [x] Deny, cancellation, and stale/foreign controls remain zero-dispatch evidence
- [x] route-qualified degraded cleanup is recorded truthfully and never renamed clean
- [x] only `registered_tools` and `consumer_tool_exchange` become available
- [x] omission and every unrelated capability remain byte-identical in meaning

## Validation

- `cargo fmt -p swallowtail-adapter-claude-agent -- --check`
- `effigy validate:focused swallowtail-adapter-claude-agent`
- `effigy package:verify-affected swallowtail-adapter-claude-agent`
- `effigy package:api`
- `effigy qa:routes`
- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`
- exact-head independent review

## Review Oracle

Invariant: every available Claude SDK registered-tool row is bound to the
exact tuple and accepted capsule that proved it. Smallest counterexample: an
adjacent SDK/native/Node/platform point becomes qualified, an adverse control
is described as a successful tool dispatch, or another feature-matrix cell
moves.

## Stop Conditions

- the capsule identities do not bind to source `24f88fb8` and the implemented carrier;
- qualification requires changing the lease, admission, correlation, cleanup, or omission contract; or
- a claimed dimension needs another live provider observation.

Return the exact contradiction to Chatterbox. Do not rerun the provider.

## Auto-Continuation

No. Stop at exact-head independent review. The queue owns merge and canonical
closeout. Candidate preparation and release remain separate operator gates.

## Result

The route is qualified. `claude_agent_sdk_registered_tool_qualification()` now
returns `Qualified(CLAUDE_AGENT_SDK_REGISTERED_TOOL_ROUTE)` with exactly the
dimensions the accepted Card 318 capsules proved: `ExactOneShot` one-shot
permission (one Allow dispatched `desktop/reconcile` exactly once with
unchanged `{}` and correlated its fixed `{"ok":true}` result; one Deny
completed with zero dispatches; the cancellation and stale/foreign controls
dispatched zero times and ended provider-failed as pass evidence),
`NoProgress`, and `NotCarried` selected-skill delivery. The mediation-kind row
moved from `Unknown`/`Unavailable` with `real_route_gate_pending` to
`Supported`/`Available` at `RouteValidation`.

The tuple cannot widen. This route pins SDK `0.3.259`, native `2.1.259`, Node
`22.23.2`, the `0.4.4` sidecar source tag, the carrier revision, and MCP
`2025-11-25` exactly, so the one axis a compiled route can vary is its
platform: qualification is scoped to the Darwin arm64 target the capsules ran
on, and every other target projects the unqualified truth with the new
`platform_not_admitted` reason. Off-platform targets keep the exact pre-card
omission and row shape.

Route tests freeze Research 301's identities: source-linked Swallowtail
`24f88fb8`, Desktop task `4356b461`, PR 181 head `18b70c91`, review comment
`5599408741`, merge `807f7a3f`, closeout `03e71e90`, capsule SHA-256
`c4de15a8`, courier `sha256:12db9fe3`, and the native Darwin arm64 binary
digest `884baa38`, plus the four-attempt, one-Allow-dispatch, zero-control-
dispatch counts, the `claude-sonnet-5` default-permission persistence-off
tuple, and the route-qualified degraded macOS cleanup posture (never
`Clean`). The frozen transcript gains the exact live Allow case: one dispatch
of the unchanged empty arguments object and the correlated `{"ok":true}`
result, and a dedicated test captures the dispatched payload bytes as exactly
`{}`. The existing zero-dispatch Deny, cancellation, stale-replay, and
foreign-lease cases are unchanged and still pass.

`registered_tools` and `consumer_tool_exchange` reconcile to `yes` with their
completed Card 152 producer-gap entries and reasons removed (the pinned
cross-classification board passes again; only the card 130
`persistent_permission_grants` gap remains on this route). Every other cell,
cross classification, and TSV row is untouched. Contract 063's claude-agent.sdk placement
row, its failed-open paragraph, and its live-gate paragraph now record the
run; the guide's Registered Tools, Failed-Open Receipts, and Selected Skill
Bundles sections carry the same result; `[Unreleased]` records it; the
unreleased 0.4.4 API baseline adds `CLAUDE_AGENT_SDK_REGISTERED_TOOL_ROUTE`
and `CLAUDE_AGENT_SDK_REGISTERED_TOOL_PLATFORM_NOT_ADMITTED_CODE`.

One deviation worth naming: `CLAUDE_AGENT_SDK_REAL_ROUTE_GATE_PENDING_CODE`
could not be deleted. Unlike Grok's card 143, this route's pending code
predates the immutable v0.4.3 public surface, and `effigy package:api` rightly
refuses its removal. The constant stays as documented dead vocabulary — no
row publishes it — and only additive items joined the 0.4.4 surface.

Validation: `cargo fmt -p swallowtail-adapter-claude-agent -- --check`,
`effigy validate:focused swallowtail-adapter-claude-agent` (500 tests),
`effigy package:verify-affected swallowtail-adapter-claude-agent`,
`effigy package:api`, `effigy qa:routes`, `effigy qa:docs`,
`effigy qa:northstar`, and `git diff --check` all pass at the reported head.
