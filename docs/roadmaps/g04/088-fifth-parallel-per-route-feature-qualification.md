# g04.088 Fifth Parallel Per-Route Feature Qualification

Status: ready
Owner: Tom
Created: 2026-08-28
Updated: 2026-08-28
Depends on: g04.087 closeout
Vision tags: per-route features, exact evidence, parallel qualification
Contract refs: 006, 013-016, 020, 023, 029, 033-034, 037, 040, 047, 052
Research: 245-248 reserved

## Problem

The normalized 85-item ledger has 18 active qualification candidates and no
active delivery row. The next four package-distinct items expose plausible
official controls, but exact route, version, authority, application,
confirmation, omission, and lifecycle truth is not closed.

## Goal

Run four independent evidence-only lanes in parallel. Each worker promotes a
closed exact-route deliver-now table or an honest empty set. Workers do not
change production code, public API, shared planning, matrices, or sibling
lanes.

## Selected Lanes

| Lane | Original item | Card | Research | Package | Evidence question |
| --- | ---: | ---: | ---: | --- | --- |
| A | 8 | 248 | 245 | `swallowtail-adapter-claude-agent` | Claude Code headless advisor |
| B | 21 | 249 | 246 | `swallowtail-adapter-codex` | Codex app-server Plan-mode effort |
| C | 34 | 250 | 247 | `swallowtail-adapter-grok` | Grok Build ACP web-search disable |
| D | 42 | 251 | 248 | `swallowtail-adapter-cline` | Cline ACP model selection |

These are the lowest still-open original ids that form four package-distinct
lanes. Item 9 and response-only items 11-12 remain behind lane A because they
share the Claude package.

## Parallel Boundary

Each worker owns only its assigned card, reserved Research file, reserved log,
and optional new frozen evidence under a unique adapter-local path. Workers do
not edit this milestone, inventory, programme, triage, matrices, indexes, sole
Next Task, or production surfaces.

Run the four workers concurrently through manual harness handoffs. Integrate
evidence PRs serially A, B, C, D. Restack later heads onto current pushed
`main`; merge only by fast-forward after exact-head review, green CI, and
operator authorisation.

## Acceptance Criteria

- [ ] card 248 promotes exact Claude headless advisor evidence or an empty set
- [ ] card 249 promotes exact Codex app-server Plan-effort evidence or an empty set
- [ ] card 250 promotes exact Grok ACP web-search-disable evidence or an empty set
- [ ] card 251 promotes exact Cline ACP model-selection evidence or an empty set
- [ ] each result separates requested, configured, dispatched, accepted,
      effective, returned, and observed truth as applicable
- [ ] no worker uses credentials, provider prompts, paid work, install/update,
      account inspection, or ambient host mutation
- [ ] no production binding starts from a worker lane
- [ ] shared closeout reconciles all four original items only after their PRs land

## Stop Conditions

- a lane needs a contract or product-policy decision rather than evidence
- exact route/version identity or authoritative source retrieval cannot close
- a live account, credential, provider prompt, paid operation, install, update,
  or ambient mutation becomes necessary
- workers need the same mutable file or a sibling route/package claim
- a result would flatten provider vocabulary into an unsupported portable claim

## Batch Cards

- [248 Claude Code Headless Advisor Evidence](batch-cards/248-claude-code-headless-advisor-evidence.md)
- [249 Codex App-Server Plan-Mode Effort Evidence](batch-cards/249-codex-app-server-plan-mode-effort-evidence.md)
- [250 Grok Build ACP Web-Search Disable Evidence](batch-cards/250-grok-build-acp-web-search-disable-evidence.md)
- [251 Cline ACP Model-Selection Evidence](batch-cards/251-cline-acp-model-selection-evidence.md)

## References

- [Per-Route Feature Completion](per-route-feature-completion.md)
- [Per-Route Feature Inventory](per-route-feature-inventory.md)
- [Advanced Route Features](../../triage/2026-08-21-advanced-route-features.md)
- [Contract 020 Model Catalogue Observation](../../contracts/020-model-catalogue-observation-and-availability-boundary.md)
- [Contract 023 Harness Isolation](../../contracts/023-harness-operation-isolation-and-native-boundary.md)
- [Contract 029 Interface Version Qualification](../../contracts/029-interface-version-qualification-and-compatibility.md)
- [Contract 034 Negotiated Harness Session Options](../../contracts/034-negotiated-harness-session-options.md)
- [Contract 040 Generation-Control Application](../../contracts/040-generation-control-application-and-enforcement.md)
