# g04.089 Sixth Parallel Per-Route Feature Qualification

Status: complete
Owner: Tom
Created: 2026-08-28
Updated: 2026-08-28
Depends on: g04.088 closeout
Vision tags: per-route features, exact evidence, parallel qualification
Contract refs: 006, 013-016, 020, 023, 029, 034, 037, 040, 047, 052
Research: 249-252 reserved

## Problem

The normalized 85-item ledger has 14 active qualification candidates and no
active delivery row. The next four package-distinct controls are official
surfaces, but exact route, version, authority, application, confirmation,
omission, and lifecycle truth is not closed.

## Goal

Run four independent evidence-only lanes in parallel. Each worker promotes a
closed exact-route deliver-now table or an honest empty set. Workers do not
change production code, public API, shared planning, matrices, or sibling
lanes.

## Selected Lanes

| Lane | Original item | Card | Research | Package | Evidence question |
| --- | ---: | ---: | ---: | --- | --- |
| A | 9 | 252 | 249 | `swallowtail-adapter-claude-agent` | Claude Code headless permission modes beyond Plan |
| B | 46 | 253 | 250 | `swallowtail-adapter-goose` | Goose ACP builtins |
| C | 48 | 254 | 251 | `swallowtail-adapter-kiro` | Kiro ACP effort |
| D | 54 | 255 | 252 | `swallowtail-adapter-mistral-vibe` | Mistral Vibe headless agent profiles beyond Plan |

These are the lowest still-open original ids that form four package-distinct
lanes. Items 11-12 remain behind lane A. Item 47 remains behind lane B. Item
49 remains behind lane C.

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

- [x] card 252 promotes exact Claude permission-mode evidence or an empty set
- [x] card 253 promotes exact Goose ACP builtin evidence or an empty set
- [x] card 254 promotes exact Kiro ACP effort evidence or an empty set
- [x] card 255 promotes exact Mistral Vibe agent-profile evidence or an empty set
- [x] each result separates requested, configured, dispatched, accepted,
      effective, returned, observed, and persisted truth as applicable
- [x] dangerous permission bypass and auto-approval stay withheld
- [x] no worker uses credentials, provider prompts, paid work, install/update,
      account inspection, or ambient host mutation
- [x] no production binding starts from a worker lane
- [x] shared closeout reconciles all four original items only after their PRs land

## Stop Conditions

- a lane needs a contract or product-policy decision rather than evidence
- exact route/version identity or authoritative source retrieval cannot close
- a live account, credential, provider prompt, paid operation, install, update,
  or ambient mutation becomes necessary
- workers need the same mutable file or a sibling route/package claim
- a result widens write, tool, extension, or approval authority by default
- a result would flatten provider vocabulary into an unsupported portable claim

## Batch Cards

- [252 Claude Code Headless Permission-Mode Evidence](batch-cards/252-claude-code-headless-permission-mode-evidence.md)
- [253 Goose ACP Builtin Evidence](batch-cards/253-goose-acp-builtin-evidence.md)
- [254 Kiro ACP Effort Evidence](batch-cards/254-kiro-acp-effort-evidence.md)
- [255 Mistral Vibe Headless Agent-Profile Evidence](batch-cards/255-mistral-vibe-headless-agent-profile-evidence.md)

## Result

All four lanes closed with honest empty deliver-now sets. PRs 109, 107, 106,
and 108 landed fast-forward-only in lane order through `e28979a0`; each exact
head passed hosted CI before merge. Original items 9, 46, 48, and 54 close as
evidence stops. No production binding follows.

## References

- [Per-Route Feature Completion](per-route-feature-completion.md)
- [Per-Route Feature Inventory](per-route-feature-inventory.md)
- [Advanced Route Features](../../triage/2026-08-21-advanced-route-features.md)
- [Contract 013 Interactive Session Access Policy](../../contracts/013-interactive-session-access-policy.md)
- [Contract 020 Model Catalogue Observation](../../contracts/020-model-catalogue-observation-and-availability-boundary.md)
- [Contract 023 Harness Isolation](../../contracts/023-harness-operation-isolation-and-native-boundary.md)
- [Contract 029 Interface Version Qualification](../../contracts/029-interface-version-qualification-and-compatibility.md)
- [Contract 034 Negotiated Harness Session Options](../../contracts/034-negotiated-harness-session-options.md)
- [Contract 040 Generation-Control Application](../../contracts/040-generation-control-application-and-enforcement.md)
