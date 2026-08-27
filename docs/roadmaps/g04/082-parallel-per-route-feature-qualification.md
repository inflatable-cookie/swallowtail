# g04.082 Parallel Per-Route Feature Qualification

Status: complete
Owner: Tom
Created: 2026-08-27
Depends on: g04.081 closeout; normalized per-route feature inventory
Vision tags: explicit behavior, route-local controls, evidence before claims
Contract refs: 011, 020, 024, 029, 037, 040, 041, 047, 052
Research: 229-232

## Problem

The normalized 85-item inventory has 34 active qualification candidates. Four
high-value candidates belong to distinct route packages and have no evidence
dependency, but ordinary full-delivery lanes converge on shared matrices,
programme state, indexes, and the sole Next Task.

## Goal

Qualify four route-local control families concurrently without concurrent
shared-state edits. Each worker promotes one exact Research file or an honest
empty set. Production binding and shared disposition changes wait for serial
orchestrator promotion.

## Parallel Lanes

| Lane | Card | Research | Route package | Question |
| --- | ---: | ---: | --- | --- |
| A | 228 | 229 | `swallowtail-adapter-codex` | app-server model verbosity |
| B | 229 | 230 | `swallowtail-adapter-gemini` | headless thinking configuration |
| C | 230 | 231 | `swallowtail-adapter-bedrock` | Runtime latency / service tier |
| D | 231 | 232 | `swallowtail-adapter-ollama` | attached think `max` |

Each lane owns only its card, Research file, reserved lane log, and any new
route-local frozen evidence corpus. It must not edit this milestone, the live
inventory, programme, triage, feature matrices, shared indexes, generation
state, or the roadmaps front door.

## Goals

- [x] card 228 promotes exact Codex app-server verbosity evidence or an empty set
- [x] card 229 promotes exact Gemini headless thinking evidence or an empty set
- [x] card 230 promotes exact Bedrock service-tier evidence or an empty set
- [x] card 231 promotes exact Ollama think-`max` evidence or an empty set
- [x] every lane preserves omission and distinguishes requested, dispatched,
      accepted, effective, and observed truth
- [x] no lane changes production code, public API, contracts, currentness,
      release state, generation ownership, or g04 closure

## Non-Goals

- production binding or acceptance cards
- sibling-route promotion
- generic Fast, effort, thinking, or service-tier vocabulary
- provider prompts, credentials, account inspection, paid work, or ambient
  configuration mutation
- new routes, releases, generation rollover, or g04 closure

## Execution

Cards 228-231 may execute concurrently from the same pushed planning base.
Workers open four independent PRs. Merge remains serial: after one PR lands,
restack every remaining PR onto current `main` before a fast-forward-only
merge.

After all four evidence PRs land, the orchestrator updates this milestone and
shared authority in one serial promotion batch. It moves each original item to
closed only when the evidence gives it a durable delivered, stopped, obsolete,
not-applicable, or withheld disposition. Non-empty exact sets receive separate
route-local binding and acceptance roadmaps; they are not auto-implemented by
this wave.

## Acceptance Criteria

- [x] Research 229-232 each contain frozen sources, route boundaries, exact
      tables, omission truth, lifecycle disposition, and a non-empty set or
      honest empty set
- [x] evidence files and lane logs are self-contained and do not depend on a
      worker transcript
- [x] lane PRs touch no shared mutable planning or matrix file
- [x] deterministic validation uses only the assigned package plus named docs
      checks; no provider operation or credential is used
- [x] the orchestrator can promote all four results without resolving a hidden
      authority or ordering dependency

## Outcome

All four lanes closed with honest empty deliver-now sets. PRs 81-84 landed
serially by fast-forward at `df8ba194`, `3ec2ac3a`, `33d3ade2`, and
`5d9fa3f5` after exact-head CI passed.

- Codex app-server exposes no typed, confirmable, catalogue-backed verbosity
  seam. Exec evidence does not transfer.
- Gemini headless thinking is settings-backed, but the qualified adapter binds
  no isolated settings seam and stream-json confirms no effective value.
- Bedrock request fields exist, but the route-open facade cannot close model,
  region, inference-profile, account, billing, or returned-state truth.
- Ollama's wire parser accepts `max` from `0.22.0`, but selected-model detail
  advertises only generic thinking and some server paths rewrite `max`.

Original inventory items 19, 32, 81, and 83 are evidence-stopped. No binding
or acceptance roadmap follows from this wave. g04 stays open.

## Decision Gates

- Stop a lane with an honest empty set when exact support, membership,
  precedence, pre-effect rejection, or effective-state truth cannot be frozen.
- Stop and report rather than editing shared authority when a candidate needs a
  contract, currentness, product-policy, access, or sibling-route decision.
- Stop if evidence requires a provider prompt, credential, account inspection,
  paid operation, install/update, or ambient configuration mutation.
- Do not infer one route's support from another route in the same family.

## Batch Cards

- [228 Codex App-Server Model Verbosity Evidence](batch-cards/228-codex-app-server-model-verbosity-evidence.md)
- [229 Gemini CLI Headless Thinking Evidence](batch-cards/229-gemini-cli-headless-thinking-evidence.md)
- [230 Bedrock Runtime Service-Tier Evidence](batch-cards/230-bedrock-runtime-service-tier-evidence.md)
- [231 Ollama Attached Think Max Evidence](batch-cards/231-ollama-attached-think-max-evidence.md)

## References

- [Per-Route Feature Inventory](per-route-feature-inventory.md)
- [Per-Route Feature Completion Programme](per-route-feature-completion.md)
- [Advanced Route Features](../../triage/2026-08-21-advanced-route-features.md)
- [Contract 040 Generation Controls](../../contracts/040-generation-controls-and-input-authority.md)
- [Contract 052 Prepared Facades](../../contracts/052-prepared-integration-facades.md)
