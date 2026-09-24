# g06.021 Command Code Current-Release Qualification

Owner: Tom
Created: 2026-09-24
Depends on: Contract 029; Research 317, 330
Vision tags: route currentness, Command Code

## Outcome

Move the exact `QualifiedOnly` `command-code.npm` point from `1.54.0` to the
official stable current at run time (`1.65.0` at planning), provider-free.

## Why It Matters

Command Code is the largest known currentness gap: eleven minor releases
behind. Tom chose it as the next currentness lane on 2026-09-24.

## Ready-State Rubric

- [x] Research 317 froze the shipped-tree ledger through `1.54.0`; Research
      330 holds the paid-model live acceptance bound to `1.54.0`.
- [x] Official npm `latest` is `1.65.0` at planning.
- [x] Scope, acceptance, validation, evidence and stop conditions are explicit.

## Decisions

- Freeze identity and a shipped-tree ledger for every published stable after
  `1.54.0` through current latest, re-probed before push.
- Compatible hops advance the exact point with the
  `command-code.agent-event-ndjson-v1` revision. A selected-surface change is
  adapted here — milestone or revision — not held, per Contract 029 No
  Terminal Stop.
- Research 330 live evidence stays bound to `1.54.0`. Live-derived cells that
  depend on it stay gated at the new point; a live requalification is a
  separate authorization and one budgeted attempt.
- Provider-free: no login, prompt, install, or binary execution.

## Dispatch manifest

- **State:** ready; one family lane.
- **Completion:** the exact point names the run's official stable, or the
  adaptation is implemented; independent exact-head review accepts the head.
- **Owned mutable paths:** `crates/swallowtail-adapter-command-code/**`;
  the Command Code prepared guide and its route and feature matrix rows; `CHANGELOG.md` `[Unreleased]`; one new research record and its index
  line; the family identity and claim logs and their index lines;
  `PAPERCUTS.md` append only.
- **Reserved closeout surfaces:** lifecycle task record and generated
  projections; submitted-handoff deletion belongs to the repository hook.
- **Excluded:** this card's prose; other families; contracts; Next Task and
  generation runway; any live gate; release, tag, publication.
- **Worker evidence:** the authenticated run result and the research record.
- **Escalation:** operator via Chatterbox for a consumer-visible narrowing;
  Queue coordinator for mechanical blockers.

## Work

1. Re-probe official npm; freeze identity and the shipped-tree ledger per hop.
2. Classify each hop on the selected invocation, AgentEvent, result, usage,
   failure and lifecycle surfaces.
3. Advance the point, or implement the adaptation the evidence needs.
4. Update guide and matrices, keeping live-derived cells gated.
5. Run the named validation.

## Acceptance and review oracle

| Invariant | Adversarial counterexample | Required proof |
| --- | --- | --- |
| Every hop is classified | A published stable is skipped | ledger lists every published point |
| Live evidence is not stretched | A `1.54.0` live cell is claimed at the new point | those cells stay gated |
| The ceiling moves | The run stops at `1.54.0` without an escalated narrowing | claim names the run's official stable |

## Stop conditions

Stop and ask if identity disagrees across channels, if a hop resets the major
line, or if adaptation needs a consumer-visible narrowing.

## Evidence

Research 317 and 330; the new research record and fixtures. Focused and
affected-package validation for `swallowtail-adapter-command-code`, route and
docs QA.

## Next Task

Chatterbox asks the operator whether a budgeted live requalification at the
new point is worth one attempt.
