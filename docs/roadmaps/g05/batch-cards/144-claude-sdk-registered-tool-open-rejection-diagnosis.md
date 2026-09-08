# 144 Claude SDK Registered-Tool Open-Rejection Diagnosis

Status: ready; Card 132 returned a typed open failure and the operator directed Chatterbox to apply its decision tree
Owner: Tom
Created: 2026-09-08
Milestone: `../035-shared-harness-capability-and-producer-boundary.md`
Depends on: card 125 merged at `1cbc21ad`; card 132 stopped with Research 296; Desktop PR 170 accepted and merged

## Goal

Turn Card 132's opaque live `open_rejected` into a deterministic producer
diagnosis and an observation-derived failed-open receipt before any fresh live
gate can be considered. Keep both registered-tool cells unqualified.

## Scope

1. Freeze Research 296's exact Swallowtail source, Desktop lifecycle, capsule,
   artifact, and tuple identities in bounded route documentation and fixtures.
2. Add an additive Claude SDK prepared-route open surface that preserves the
   existing `open_session` behavior while returning a structured safe
   failed-open receipt: route code, exact bounded sidecar subcode when present,
   failure stage, whether provider readiness was reached, and observed cleanup
   disposition. For registered opens, include bounded evidence for bridge
   admission freeze, courier/listener/task join, lease/resource release, and
   survivor posture. Never expose paths, endpoints, bearer material,
   credentials, environment values, provider content, or arbitrary stderr.
3. Drive the exact `0.3.259` sidecar path provider-free through construction,
   initialization, account/readiness, required-MCP status, deadline, and
   cleanup failure shapes. Prove the outer stable diagnostic, structured
   subcode, receipt, and existing source-compatible API agree.
4. Reproduce the exact Card 132 registered-open request against the frozen fake
   SDK/sidecar surface. If it identifies a concrete producer defect, repair it
   in the same batch and prove ordinary open, registered omission, selected
   skill, permission, and teardown behavior unchanged. If it does not, close
   with the narrowed unresolved boundary and the exact evidence a future gate
   must collect; do not invent a provider limitation.
5. Update the Claude integration guide, Contract 063 failed-open evidence
   boundary, `[Unreleased]`, and the current additive public API baseline.
   Keep `registered_tools` and `consumer_tool_exchange` `No` and owned by this
   producer gap until a separately authorized passing live capsule exists.

Owned mutable paths: `crates/swallowtail-adapter-claude-agent/**`;
`release-baselines/public-api-0.4.4/swallowtail-adapter-claude-agent.txt`
additively; the Claude SDK failed-open section in Contract 063 and its guide;
the Claude matrix cross references/reasons only; `CHANGELOG.md` `[Unreleased]`;
this card's `## Result`; `PAPERCUTS.md` append only. Queue closeout owns shared
roadmap/index/handoff status surfaces.

Forbidden: other adapter/runtime/kernel behavior unless an additive
provider-neutral failure-evidence primitive is demonstrably required and
returned to Chatterbox first; Desktop or other consumer repositories; live
native Claude or provider execution (fixture Node sidecars remain allowed);
credential/global configuration mutation;
qualification or availability changes; tag, release, parity, or broader
acceptance claims.

## Acceptance Criteria

- [ ] existing open APIs remain source-compatible and preserve their exact
      stable outer codes
- [ ] an additive route-specific surface returns the bounded sidecar subcode,
      open stage, readiness posture, and positively observed cleanup evidence
      without message parsing or forbidden material
- [ ] provider-free fixtures cover construction, initialization, account/MCP
      readiness, deadline, joined cleanup, and unconfirmed cleanup
- [ ] the exact Card 132 request is reproduced provider-free and either yields
      a repaired producer defect or a precise unresolved boundary
- [ ] both Contract 061 cells remain unqualified; guide, contract, changelog,
      matrix producer-gap ownership, and API baseline agree

## Validation

- `cargo fmt -p swallowtail-adapter-claude-agent -- --check`
- `effigy validate:focused swallowtail-adapter-claude-agent`
- `effigy package:verify-affected swallowtail-adapter-claude-agent`
- `effigy validate:card116-mediated-stdio`
- `effigy package:api`
- `effigy qa:routes`
- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`

## Review Oracle

Invariant: a failed registered open is useful evidence without being a support
claim—the consumer can identify the exact bounded rejection stage/subcode and
cleanup truth from typed fields. Smallest counterexample: extracting
`construction_failed` by parsing `diagnostic().message()`.

## Stop Conditions

- deterministic evidence cannot distinguish the failure without another live
  provider attempt;
- a repair would weaken strict MCP configuration, permission mediation,
  registered-tool correlation, or joined cleanup;
- structured evidence requires exposing forbidden material; or
- the change needs a provider-neutral runtime failure redesign rather than a
  bounded Claude route addition.

Return the exact contradiction to Chatterbox. Do not run or request a live
retry from this card.

## Auto-Continuation

No. Stop at exact-head independent review. The queue owns merge and closeout.
