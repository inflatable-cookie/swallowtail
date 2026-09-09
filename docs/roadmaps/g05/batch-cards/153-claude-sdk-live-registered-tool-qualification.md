# 153 Claude SDK Live Registered-Tool Qualification

Status: ready
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

- [ ] the registered-tool projection is qualified only for the exact accepted tuple
- [ ] deterministic tests bind the claim to every Research 301 evidence identity
- [ ] Allow proves one unchanged `{}` dispatch and correlated `{"ok":true}` result
- [ ] Deny, cancellation, and stale/foreign controls remain zero-dispatch evidence
- [ ] route-qualified degraded cleanup is recorded truthfully and never renamed clean
- [ ] only `registered_tools` and `consumer_tool_exchange` become available
- [ ] omission and every unrelated capability remain byte-identical in meaning

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

Pending.
