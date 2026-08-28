# g04.087 Fourth Parallel Per-Route Feature Qualification

Status: complete
Owner: Tom
Created: 2026-08-27
Updated: 2026-08-28
Depends on: g04.086 closeout
Vision tags: per-route features, exact evidence, parallel qualification
Contract refs: 006, 013-016, 020, 023, 029, 033-034, 037, 040, 047, 052
Research: 241-244

## Problem

The normalized 85-item feature ledger has 22 active qualification candidates
and no active delivery row. The next lowest original ids span repeated Claude
and Codex families plus independent Cursor and Gemini ACP surfaces. Production
work would be speculative until exact route, version, value, authority,
application, confirmation, omission, and lifecycle truth is closed.

## Goal

Run four independent evidence-only lanes in parallel. Each lane promotes a
closed exact-route deliver-now table or an honest empty set. No worker changes
production code, public API, matrices, shared planning state, or another lane.

## Selected Lanes

| Lane | Original item | Card | Research | Package | Evidence question |
| --- | ---: | ---: | ---: | --- | --- |
| A | 6 | 244 | 241 | `swallowtail-adapter-claude-agent` | Claude Code headless spend cap |
| B | 18 | 245 | 242 | `swallowtail-adapter-codex` | Codex app-server Fast mode |
| C | 28 | 246 | 243 | `swallowtail-adapter-cursor` | Cursor ACP model parameters |
| D | 29 | 247 | 244 | `swallowtail-adapter-gemini` | Gemini CLI ACP sandbox |

These are the lowest still-open original ids that can form four package-
distinct lanes. Items 8-9 and 11-12 remain behind lane A because they share
the Claude package. Item 21 remains behind lane B because it shares Codex.

## Parallel Boundary

Each worker owns only its assigned card, reserved Research file, reserved log,
and optional new frozen evidence under a unique adapter-local path. Workers do
not edit this milestone, the inventory, programme, triage, matrices, indexes,
the sole Next Task, or production surfaces.

Run the four workers concurrently. Integrate evidence PRs serially in lane
order A, B, C, D. Restack later heads onto current pushed `main`; merge only by
fast-forward after exact-head review, green CI, and operator authorisation.

## Acceptance Criteria

- [x] card 244 promotes exact Claude headless spend-cap evidence or an empty set
- [x] card 245 promotes exact Codex app-server Fast evidence or an empty set
- [x] card 246 promotes exact Cursor ACP model-parameter evidence or an empty set
- [x] card 247 promotes exact Gemini ACP sandbox evidence or an empty set
- [x] each result separates requested, configured, dispatched, accepted,
      effective, returned, observed, and billed/contained truth as applicable
- [x] no worker uses credentials, provider prompts, paid work, install/update,
      account inspection, or ambient host mutation
- [x] no production binding starts before the orchestrator promotes and
      sequences a non-empty exact result
- [x] shared closeout reconciles all four original items only after their PRs
      land

## Outcome

- card 244 / Research 241: honest empty Claude headless spend-cap set; the
  local catalog-priced USD ledger does not close against subscription billing
- card 245 / Research 242: honest empty Codex app-server Fast set; soft
  unsupported drop, request-wire divergence, catalogue substitution, missing
  cold persistence, and an ambient feature gate prevent binding
- card 246 / Research 243: honest empty Cursor ACP model-parameter set;
  membership is account-gated and the selected ACP path lacks independent
  parameter selection and confirmation
- card 247 / Research 244: honest empty Gemini ACP sandbox set; ambient
  precedence, pre-ACP re-exec/stdin behavior, backend readiness, and missing
  ACP confirmation prevent binding

PRs 99, 97, 98, and 96 landed fast-forward-only in lane order through
`e40a5407`. Each final head passed hosted CI. No production binding follows.
Original items 6, 18, 28, and 29 close as evidence stops.

## Stop Conditions

- a lane needs a contract or product-policy decision rather than evidence
- exact route/version identity or authoritative source retrieval cannot close
- a live account, credential, provider prompt, paid operation, install, update,
  sandbox backend start, or host mutation becomes necessary
- workers need the same mutable file or a sibling route/package claim
- a result would flatten billing, model parameters, sandboxing, or provider
  vocabulary into an unsupported portable claim

## Batch Cards

- [244 Claude Code Headless Spend-Cap Evidence](batch-cards/244-claude-code-headless-spend-cap-evidence.md)
- [245 Codex App-Server Fast-Mode Evidence](batch-cards/245-codex-app-server-fast-mode-evidence.md)
- [246 Cursor ACP Model-Parameter Evidence](batch-cards/246-cursor-acp-model-parameter-evidence.md)
- [247 Gemini CLI ACP Sandbox Evidence](batch-cards/247-gemini-cli-acp-sandbox-evidence.md)

## References

- [Per-Route Feature Completion](per-route-feature-completion.md)
- [Per-Route Feature Inventory](per-route-feature-inventory.md)
- [Advanced Route Features](../../triage/2026-08-21-advanced-route-features.md)
- [Contract 020 Model Catalogue Observation](../../contracts/020-model-catalogue-observation-and-availability-boundary.md)
- [Contract 023 Harness Isolation](../../contracts/023-harness-operation-isolation-and-native-boundary.md)
- [Contract 029 Interface Version Qualification](../../contracts/029-interface-version-qualification-and-compatibility.md)
- [Contract 034 Negotiated Harness Session Options](../../contracts/034-negotiated-harness-session-options.md)
- [Contract 037 Prepared Integration](../../contracts/037-prepared-consumer-integration.md)
- [Contract 040 Generation-Control Application](../../contracts/040-generation-control-application-and-enforcement.md)
