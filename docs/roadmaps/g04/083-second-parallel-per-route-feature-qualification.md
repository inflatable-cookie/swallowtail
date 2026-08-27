# g04.083 Parallel Per-Route Feature Qualification II

Status: ready
Owner: Tom
Created: 2026-08-27
Depends on: g04.082 closeout; live per-route feature inventory
Vision tags: explicit behavior, route-local controls, evidence before claims
Contract refs: 011, 020, 024, 026, 029, 037, 040, 041, 047, 052
Research: 233-236

## Problem

Thirty original feature candidates remain active after g04.082. Four useful
questions belong to distinct route packages and can be qualified without shared
worker writes. Their vendor controls use overlapping words such as Fast,
thinking, and reasoning, but they do not share semantics or evidence.

## Goal

Qualify four exact route-local controls concurrently. Each worker promotes one
Research file with a closed deliver-now table or an honest empty set. Production
binding, shared disposition changes, and any later acceptance work remain serial
orchestrator responsibilities.

## Parallel Lanes

| Lane | Original item | Card | Research | Route package | Question |
| --- | ---: | ---: | ---: | --- | --- |
| A | 2 | 232 | 233 | `swallowtail-adapter-claude-agent` | headless Fast mode |
| B | 15 | 233 | 234 | `swallowtail-adapter-codex` | exec Fast / service tier |
| C | 30 | 234 | 235 | `swallowtail-adapter-gemini` | ACP thinking configuration |
| D | 76 | 235 | 236 | `swallowtail-adapter-openai` | Realtime reasoning effort |

Each lane owns only its card, Research file, reserved lane log, and optional new
route-local frozen evidence under its package. It must not edit this milestone,
the live inventory, programme, triage, feature matrices, shared indexes,
generation state, or the roadmaps front door.

## Goals

- [ ] card 232 promotes exact Claude Code headless Fast evidence or an empty set
- [ ] card 233 promotes exact Codex exec Fast/service-tier evidence or an empty set
- [ ] card 234 promotes exact Gemini CLI ACP thinking evidence or an empty set
- [ ] card 235 promotes exact OpenAI Realtime reasoning evidence or an empty set
- [ ] every lane preserves omission and separates requested, dispatched,
      accepted, effective, returned, and observed truth where applicable
- [ ] no lane changes production code, public API, contracts, currentness,
      release state, generation ownership, or g04 closure

## Non-Goals

- production binding or acceptance cards
- sibling-route or provider-wide promotion
- generic Fast, service-tier, thinking, or reasoning vocabulary
- provider prompts, credentials, account inspection, paid work, or ambient
  configuration mutation
- consumer-account Gemini access, hosted OAuth, new routes, releases,
  generation rollover, or g04 closure

## Execution

Cards 232-235 may execute concurrently from the same pushed planning base.
Workers open four independent PRs. Merge remains serial: after one PR lands,
restack every remaining PR onto current `main` before a fast-forward-only merge.

After all four evidence PRs land, the orchestrator promotes their results in one
serial shared-state batch. Move an original item to closed only when its evidence
gives it a durable delivered, stopped, obsolete, not-applicable, or withheld
disposition. Compile separate route-local binding and acceptance roadmaps only
for non-empty exact sets; do not implement from an evidence worker branch.

## Acceptance Criteria

- [ ] Research 233-236 each contain frozen sources, exact route boundaries,
      closed tables, omission truth, lifecycle disposition, and a non-empty set
      or honest empty set
- [ ] evidence files and lane logs are self-contained and do not depend on a
      worker transcript
- [ ] lane PRs touch no shared mutable planning or matrix file
- [ ] deterministic validation uses only the assigned package plus named docs
      checks; no provider operation or credential is used
- [ ] the orchestrator can promote all four results without a hidden authority
      or ordering dependency

## Decision Gates

- Stop a lane with an honest empty set when exact support, membership,
  precedence, pre-effect rejection, or effective-state truth cannot be frozen.
- Stop and report rather than editing shared authority when a candidate needs a
  contract, currentness, product-policy, access, or sibling-route decision.
- Stop if evidence requires a provider prompt, credential, account inspection,
  paid operation, install/update, or ambient configuration mutation.
- Do not infer one route's support from another route in the same family.

## Batch Cards

- [232 Claude Code Headless Fast-Mode Evidence](batch-cards/232-claude-code-headless-fast-mode-evidence.md)
- [233 Codex Exec Fast Service-Tier Evidence](batch-cards/233-codex-exec-fast-service-tier-evidence.md)
- [234 Gemini CLI ACP Thinking Evidence](batch-cards/234-gemini-cli-acp-thinking-evidence.md)
- [235 OpenAI Realtime Reasoning-Effort Evidence](batch-cards/235-openai-realtime-reasoning-effort-evidence.md)

## References

- [Per-Route Feature Inventory](per-route-feature-inventory.md)
- [Per-Route Feature Completion Programme](per-route-feature-completion.md)
- [Advanced Route Features](../../triage/2026-08-21-advanced-route-features.md)
- [Contract 040 Generation Controls](../../contracts/040-generation-controls-and-input-authority.md)
- [Contract 052 Prepared Facades](../../contracts/052-consumer-and-operator-integration-documentation.md)
