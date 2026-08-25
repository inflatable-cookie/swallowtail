# g04.063 Kimi Code Headless Reasoning Effort

Status: stopped
Owner: Tom
Created: 2026-08-25
Updated: 2026-08-25
Depends on: g04.032; per-route feature completion programme
Vision tags: explicit selection, provider truth, installed-route isolation
Contract refs: 011, 029, 033, 037, 040, 044, 052
Research: 017, 046, 056, 066, 068, 074, 159, 179, 207, 208, 210

## Problem

`kimi-code.headless` selects an exact model but exposes no reasoning control.
Exact Kimi Code 0.38.0 documentation describes a thinking-effort setting,
per-model supported/default effort metadata, and a process-temporary model
environment surface. It also describes fallback to a model default when a
configured effort is unsupported. A route binding is unsafe until exact
source evidence proves a process-local key, value set, version range, model
agreement, precedence, and fail-closed behavior.

## Generation Runway Goal

Qualify and bind the smallest exact reasoning-effort subset on the existing
`kimi-code.headless` one-prompt route. Keep selection typed, model-bound, and
process-local. Preserve omission and reject any selected value that could be
substituted, clamped, shadowed, or silently defaulted.

## Goals

- [ ] freeze exact official documentation and selected package source
- [ ] identify the exact process-local environment/config key and precedence
- [ ] freeze supported/default effort metadata for each exact selected model
- [ ] classify invalid, unsupported, aliased, clamped, and fallback behavior
- [ ] promote Research 210 with an exact deliver-now table or honest empty set
- [ ] expose only Research 210-admitted typed `ReasoningMode` values
- [ ] bind one immutable selection through input, plan, evidence, and child env
- [ ] fail before process creation when exact value agreement is not proved
- [ ] preserve model selection, access, retention, recovery, and retry truth
- [ ] keep thinking content out of stream-json output and portable activity
- [ ] prove omission preserves existing child arguments, environment, and run

## Non-Goals

- Kimi Code ACP or local-server promotion, or the Python `kimi-cli` product
- raw configuration, raw environment maps, config-file mutation, or a
  synthetic Kimi home/config root
- plan, yolo, agent, tool, permission, sandbox, memory, or multi-agent controls
- thought content, chain-of-thought, reasoning summaries, or activity claims
- a new shared capability, contract, runtime abstraction, or public raw value
- live account work, login, credential use, package install, prompt execution,
  currentness, release, merge, generation rollover, or g04 closure

## Named Scope

The lane is restricted to route `kimi-code.headless`, driver
`swallowtail.kimi.headless`, exact executable axis `kimi-code.executable`,
delegated membership OAuth reference access, and the qualified compatibility
range `0.29.0..=0.37.2` under legacy v1 corpus. Exact `0.38.0` default
headless dispatch uses agent-core-v2 `runV2Print`; Research 210 retracted
headless qualified ceiling to `0.37.2` and left `0.38.0` visible
`UnverifiedNewer` until v2 stream-json is independently qualified.

The route already selects one exact model and launches one prompt with
`--output-format stream-json`. The candidate control is optional portable
`ReasoningSelection` only where Research 210 proves an exact selected
model/version/value row. No value is admitted by this planning document.

The only acceptable transport is an exact process-local child environment
binding owned by the adapter. It must not mutate user configuration, invent a
config root, or expose a caller-provided environment map. Inherited ambient
configuration must not be able to replace the prepared selection. If Kimi
falls back to `default_effort`, aliases a value, clamps it, ignores it, or
selects a different model/provider, preparation or dispatch fails closed.

Claim strength remains layered. Deterministic construction may prove planned
or dispatched selection. Source-defined parsing or fixture confirmation may
prove accepted/effective/observed behavior only when Research 210 names that
boundary. Neither prose nor output quality proves reasoning depth.

## Execution Plan

### Batch 63.1 — Exact Headless Effort Evidence

- [x] Execute card 176.
- [x] freeze exact version/model/value/transport/precedence/fallback truth
- [x] promote Research 210 with a non-empty exact table or honest empty set

### Batch 63.2 — Conditional Route-Local Binding

- [ ] Execute card 177 only when Research 210 admits a non-empty set. Blocked:
      Research 210 empty deliver-now set.
- [ ] bind only the exact typed rows through the existing child process

### Batch 63.3 — Route-Local Acceptance

- [ ] Execute card 178 only after card 177. Blocked: card 177.
- [ ] prove dispatch, failure, omission, lifecycle, docs, and API truth

## Acceptance Criteria

- [ ] only Research 210 exact version/model/value rows prepare and dispatch
- [ ] the selected model and effort agree at every claimed boundary
- [ ] unsupported, shadowed, substituted, or defaulted selection fails closed
- [ ] omission preserves existing child arguments, environment, and behavior
- [ ] no user config, ambient durable state, or generic provider setting mutates
- [ ] no thinking content or reasoning-summary activity becomes public
- [ ] retention, managed recovery, access, cancellation, and retry remain exact
- [ ] default QA performs no account, credential, install, prompt, or paid work
- [ ] g04.063 closes only this route-local family; g04 remains active

## Lane Runway

- predecessor: g04.062 Anthropic Messages adaptive thinking
- this milestone: Kimi Code headless reasoning effort
- execution topology: one serial worker lane, cards 176-178
- generation boundary: g04 remains open; no closure or rollover is authorized

## Decision Gates

- Stop if the exact process-local key, source precedence, model/value support,
  or version floor cannot be proved.
- Stop if Kimi can silently substitute, clamp, alias, ignore, or default the
  selected value on a candidate row.
- Stop if safe delivery requires user config mutation, a synthetic config
  root, ambient-state ownership, live provider work, public thought content,
  or a new shared contract/capability.

## Batch Cards

- [176-kimi-code-headless-reasoning-effort-evidence.md](batch-cards/176-kimi-code-headless-reasoning-effort-evidence.md)
- [177-kimi-code-headless-reasoning-effort-binding.md](batch-cards/177-kimi-code-headless-reasoning-effort-binding.md)
- [178-kimi-code-headless-reasoning-effort-acceptance.md](batch-cards/178-kimi-code-headless-reasoning-effort-acceptance.md)

## References

- [Per-Route Feature Completion Programme](./per-route-feature-completion.md)
- [Advanced Route Features](../../triage/2026-08-21-advanced-route-features.md)
- [Research 179 Kimi Code 0.38.0 Identity](../../research/179-kimi-code-0-38-0-identity.md)
- [Research 210 Kimi Code Headless Reasoning Effort](../../research/210-kimi-code-headless-reasoning-effort-evidence.md)
- [Contract 040 Generation Controls](../../contracts/040-generation-control-application-and-enforcement.md)
- [Contract 044 Observable Agent Activity](../../contracts/044-observable-agent-activity-and-disclosure.md)
- [Kimi Code configuration files at 0.38.0](https://github.com/MoonshotAI/kimi-code/blob/%40moonshot-ai%2Fkimi-code%400.38.0/docs/en/configuration/config-files.md)
- [Kimi Code command reference at 0.38.0](https://github.com/MoonshotAI/kimi-code/blob/%40moonshot-ai%2Fkimi-code%400.38.0/docs/en/reference/kimi-command.md)
