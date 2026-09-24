# g06.030 Ollama Runtime Current-Release Qualification

Owner: Tom
Created: 2026-09-24
Depends on: Contract 029; Research 342; completed g06.023
Vision tags: route currentness, Ollama

## Outcome

Raise `ollama.runtime` from `0.34.2` through the official GitHub stable
current at run time (`v0.34.4` at planning), provider-free.

## Why It Matters

g06.023 landed `0.34.2` after official had already moved. Ollama is the last
family behind its official stable after the 2026-09-24 currentness sweep. Tom
approved the run on 2026-09-24.

## Ready-State Rubric

- [x] Research 342 froze identity and selected-file hashes through `0.34.2`.
- [x] Official GitHub `ollama/ollama` latest is `v0.34.4`, not a prerelease.
- [x] Scope, acceptance, validation, evidence and stop conditions are explicit.

## Decisions

- Freeze identity for every published stable after `0.34.2` through current
  latest, re-probed before push, using Research 342's method.
- Compatible hops extend the Maintained window. A selected-surface change is
  adapted in this task under Contract 029 No Terminal Stop.
- Keep the existing exclusions (`0.32.2`, `0.32.10`) and the attached-runtime
  route shape.
- Take the next free research number at commit time and recheck it against
  `main` before push.
- Provider-free: no install, model pull, prompt, or binary execution.

## Dispatch manifest

- **State:** ready; one family lane.
- **Completion:** the window names the run's official stable, or the
  adaptation is implemented; independent exact-head review accepts the head.
- **Owned mutable paths:** `crates/swallowtail-adapter-ollama/**`; the Ollama
  prepared guide and its route and feature matrix rows; `CHANGELOG.md`
  `[Unreleased]`; one new research record and its index line; the family
  identity and claim logs and their index lines; `PAPERCUTS.md` append only.
- **Reserved closeout surfaces:** lifecycle task record and generated
  projections; submitted-handoff deletion belongs to the repository hook.
- **Excluded:** this card's prose; other families; contracts; Next Task and
  generation runway; release, tag, publication.
- **Worker evidence:** the authenticated run result and the research record.
- **Escalation:** operator via Chatterbox for a consumer-visible narrowing;
  Queue coordinator for mechanical blockers.

## Acceptance and review oracle

| Invariant | Adversarial counterexample | Required proof |
| --- | --- | --- |
| Every hop is classified | A published stable is skipped | identity lists every published point |
| The ceiling moves | The run ends at `0.34.2` without an escalated narrowing | window names the run's official stable |
| No number collision | The record reuses an existing research number | unique number checked against `main` |

## Stop conditions

Stop and ask if identity disagrees across channels, if a hop resets the minor
line with a breaking API change, or if adaptation needs a consumer-visible
narrowing.

## Evidence

Research 342; the new record. Focused and affected-package validation for
`swallowtail-adapter-ollama`, route and docs QA.

## Next Task

Chatterbox reconciles the claim after closeout.
