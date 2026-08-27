# g04.085 Parallel Per-Route Feature Qualification III

Status: ready
Owner: Tom
Created: 2026-08-27
Depends on: g04.084 closeout; live per-route feature inventory
Vision tags: explicit behavior, route-local controls, evidence before claims
Contract refs: 011, 020, 024, 026, 029, 034, 037, 040, 041, 047, 052
Research: 237-240

## Problem

Twenty-six original feature candidates remain active after g04.084. Four
useful questions belong to distinct route packages and can be qualified
without shared worker writes. Autocompaction, personality, sandboxing, and Plan
mode remain exact route-local controls; none implies another route's behavior.

## Goal

Qualify four exact route-local controls concurrently. Each worker promotes one
Research file with a closed deliver-now table or an honest empty set. Production
binding, shared disposition changes, and any later acceptance work remain serial
orchestrator responsibilities.

## Parallel Lanes

| Lane | Original item | Card | Research | Route package | Question |
| --- | ---: | ---: | ---: | --- | --- |
| A | 4 | 238 | 237 | `swallowtail-adapter-claude-agent` | headless `--autocompact` |
| B | 20 | 239 | 238 | `swallowtail-adapter-codex` | app-server personality |
| C | 31 | 240 | 239 | `swallowtail-adapter-gemini` | headless `--sandbox` |
| D | 41 | 241 | 240 | `swallowtail-adapter-cline` | ACP Plan mode |

Each lane owns only its card, Research file, reserved lane log, and optional new
route-local frozen evidence under its package. It must not edit this milestone,
the live inventory, programme, triage, feature matrices, shared indexes,
generation state, or the roadmaps front door.

## Goals

- [ ] card 238 promotes exact Claude Code headless autocompaction evidence or
      an empty set
- [ ] card 239 promotes exact Codex app-server personality evidence or an empty
      set
- [ ] card 240 promotes exact Gemini CLI headless sandbox evidence or an empty
      set
- [ ] card 241 promotes exact Cline ACP Plan-mode evidence or an empty set
- [ ] every lane preserves omission and separates requested, dispatched,
      accepted, effective, returned, and observed truth where applicable
- [ ] no lane changes production code, public API, contracts, currentness,
      release state, generation ownership, or g04 closure

## Non-Goals

- production binding or acceptance cards
- sibling-route or provider-wide promotion
- generic compaction, personality, sandbox, or Plan vocabulary
- provider prompts, credentials, account inspection, paid work, or ambient
  configuration mutation
- consumer-account Gemini access, hosted OAuth, new routes, releases,
  generation rollover, or g04 closure

## Execution

Cards 238-241 may execute concurrently from the same pushed planning base.
Workers open four independent PRs. Merge remains serial: after one PR lands,
restack every remaining PR onto current `main` before a fast-forward-only merge.

After all four evidence PRs land, the orchestrator promotes their results in one
serial shared-state batch. Move an original item to closed only when its evidence
gives it a durable delivered, stopped, obsolete, not-applicable, or withheld
disposition. Compile separate route-local binding and acceptance roadmaps only
for non-empty exact sets; do not implement from an evidence worker branch.

## Acceptance Criteria

- [ ] Research 237-240 each contain frozen sources, exact route boundaries,
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

- [238 Claude Code Headless Autocompaction Evidence](batch-cards/238-claude-code-headless-autocompaction-evidence.md)
- [239 Codex App-Server Personality Evidence](batch-cards/239-codex-app-server-personality-evidence.md)
- [240 Gemini CLI Headless Sandbox Evidence](batch-cards/240-gemini-cli-headless-sandbox-evidence.md)
- [241 Cline ACP Plan-Mode Evidence](batch-cards/241-cline-acp-plan-mode-evidence.md)

## References

- [Per-Route Feature Inventory](per-route-feature-inventory.md)
- [Per-Route Feature Completion Programme](per-route-feature-completion.md)
- [Advanced Route Features](../../triage/2026-08-21-advanced-route-features.md)
- [Contract 034 Harness Options](../../contracts/034-negotiated-harness-session-options.md)
- [Contract 040 Generation Controls](../../contracts/040-generation-control-application-and-enforcement.md)
- [Contract 052 Integration Documentation](../../contracts/052-consumer-and-operator-integration-documentation.md)
