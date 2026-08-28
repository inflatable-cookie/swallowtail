# g04.090 Residual Per-Route Feature Qualification

Status: complete
Owner: Tom
Created: 2026-08-28
Updated: 2026-08-28
Depends on: g04.089 closeout and remainder audit
Vision tags: per-route features, exact evidence, residual qualification
Contract refs: 006, 013-016, 020, 023, 029, 034, 037, 040, 047, 052
Research: 253-254 promoted

## Problem

The audited 85-item ledger has no delivery row. Sixteen stale remainder items
are already covered, inapplicable, owned by another programme, or blocked by a
settled route boundary. Bedrock items 79-80 remain parked. Only Goose ACP mode
and Kiro ACP agent selection retain bounded exact-route evidence questions.

## Goal

Run two independent evidence-only lanes in parallel. Each worker promotes a
closed exact-route deliver-now table or an honest empty set. Workers do not
change production code, public API, shared planning, matrices, or sibling
lanes.

## Selected Lanes

| Lane | Original item | Card | Research | Package | Evidence question |
| --- | ---: | ---: | ---: | --- | --- |
| A | 47 | 256 | 253 | `swallowtail-adapter-goose` | exact ACP Goose mode membership, selection, application, confirmation, and authority |
| B | 49 | 257 | 254 | `swallowtail-adapter-kiro` | exact ACP `--agent` profile membership, failure, confirmation, and omission |

## Parallel Boundary

Each worker owns only its assigned card, reserved Research file, reserved log,
and optional new frozen evidence under a unique adapter-local path. Workers do
not edit this milestone, inventory, programme, triage, matrices, indexes, sole
Next Task, or production surfaces.

Run both workers concurrently through manual harness handoffs. Integrate
evidence PRs serially A then B. Restack B onto current pushed `main`; merge only
by fast-forward after exact-head review, green CI, and operator authorisation.

## Acceptance Criteria

- [x] card 256 promotes exact Goose ACP mode evidence or an empty set
- [x] card 257 promotes exact Kiro ACP agent-profile evidence or an empty set
- [x] each result separates requested, configured, dispatched, accepted,
      effective, returned, observed, and persisted truth as applicable
- [x] auto-approval, trust-all, permission widening, and ambient profile
      mutation stay withheld
- [x] provider labels do not become portable `HarnessMode` values by implication
- [x] no worker uses credentials, provider prompts, paid work, install/update,
      account inspection, or ambient host mutation
- [x] no production binding starts from a worker lane
- [x] shared closeout reconciles both original items only after their PRs land

## Stop Conditions

- a lane needs a contract or product-policy decision rather than evidence
- exact route/version identity or authoritative source retrieval cannot close
- a live account, credential, provider prompt, paid operation, install, update,
  or ambient mutation becomes necessary
- workers need the same mutable file or a sibling route/package claim
- a result widens tool, write, approval, trust, or profile authority by default
- a result would flatten provider vocabulary into an unsupported portable claim

## Batch Cards

- [256 Goose ACP Mode Evidence](batch-cards/256-goose-acp-mode-evidence.md)
- [257 Kiro ACP Agent-Profile Evidence](batch-cards/257-kiro-acp-agent-profile-evidence.md)

## Result

Both lanes closed with honest empty deliver-now sets. PR 111 landed Goose by
fast-forward at `9e317e20`. PR 110 was then restacked and landed Kiro by
fast-forward at `96b937d1`. Each exact head passed hosted CI before merge.
Original items 47 and 49 close as evidence stops. No production binding
follows. g04 remains open at operator direction.

## References

- [Per-Route Feature Completion](per-route-feature-completion.md)
- [Per-Route Feature Inventory](per-route-feature-inventory.md)
- [Advanced Route Features](../../triage/2026-08-21-advanced-route-features.md)
- [Goose ACP Prepared Integration](../../guides/goose-acp-prepared-integration.md)
- [Kiro ACP Prepared Integration](../../guides/kiro-acp-prepared-integration.md)
- [Contract 013 Interactive Session Access Policy](../../contracts/013-interactive-session-access-policy.md)
- [Contract 023 Harness Isolation](../../contracts/023-harness-operation-isolation-and-native-boundary.md)
- [Contract 029 Interface Version Qualification](../../contracts/029-interface-version-qualification-and-compatibility.md)
- [Contract 034 Negotiated Harness Session Options](../../contracts/034-negotiated-harness-session-options.md)
- [Contract 040 Generation-Control Application](../../contracts/040-generation-control-application-and-enforcement.md)
