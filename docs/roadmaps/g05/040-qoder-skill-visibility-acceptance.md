# g05.040 Qoder Skill Visibility Acceptance

Status: planned; serial after g05.039
Owner: Tom
Created: 2026-08-28
Updated: 2026-09-09
Depends on: g05.039; card 004 evidence; Contract 058
Governing refs: Contracts 020, 029, 032, 033, 037, 041, 047, 058
Vision tags: harness skills, selected-session truth, consumer integration
Former card: 006 Qoder Skill Visibility Acceptance (folded g05.038; no scope change)

## Outcome

Close the first Contract 058 proof with route fixtures, guide coverage, and
consumer-facing truth.

## Ready-State Rubric

- [x] Objective is bounded enough to finish without fresh planning decisions.
- [x] Governing refs point at current canonical surfaces.
- [ ] Scope, acceptance criteria, validation, evidence, and stop conditions are explicit (below).
- [ ] g05.039 has landed (currently planned behind the empty disposition).
- [ ] No unresolved planning gaps or operator intent checkpoints beyond that serial edge.

## Decisions

None beyond g05.002. Shared route and feature guidance updates land only
after the proof passes.

## Dispatch manifest

- **State:** planned; one lane; serial after g05.039; no concurrent siblings.
- **Completion:** positive global/project rows proved when admitted, complete-empty behavior, staleness, bounds, cancellation, cleanup, redaction, and absence behavior; guides updated only after proof passes.
- **Owned mutable paths:** Qoder adapter fixtures, shared route/feature guidance sections, matrix cells; this task file.
- **Reserved closeout surfaces:** `docs/roadmaps/README.md`, `docs/roadmaps/g05/README.md`, `docs/roadmaps/generation-index.md`, `docs/logs/README.md`.
- **Worker:** adapter worker with Contract 058 literacy; no provider credentials.
- **Excluded:** ambient scans, model prompts, guide claims ahead of fixtures.
- **Escalation:** operator via Chatterbox for disposition questions.

## Work

1. Prove positive global and project rows when admitted, plus complete-empty behavior, staleness, bounds, cancellation, cleanup, redaction, and absence behavior.
2. Update shared route and feature guidance only after the proof passes.
3. Verify route matrices and guide claims match fixtures exactly.

## Acceptance and review oracle

| Invariant | Adversarial counterexample | Required proof |
| --- | --- | --- |
| One exact roster is consumer-visible | No consumer can observe the admitted roster | End-to-end fixture with consumer visibility |
| Operator-installed skills included | Provenance excludes operator-installed skills | Provenance fixtures covering install scopes |
| No ambient observation | An ambient scan or model prompt occurs | Negative instrumentation proof |

- [ ] one exact effective roster is visible to a consumer
- [ ] provenance does not exclude operator-installed skills
- [ ] no ambient scan or model prompt occurs
- [ ] route matrices and guide claims match fixtures exactly

## Validation

- `effigy validate:focused swallowtail-adapter-qoder`
- `effigy package:verify-affected swallowtail-adapter-qoder`
- `effigy qa:docs`
- `git diff --check`

## Stop conditions

- g05.039 has not landed: this task remains planned.
- Positive rows require login, paid work, or ambient host mutation.
- Guide claims cannot match fixtures exactly.

## Evidence

On completion, record: proof fixtures, validation actually run, PR link, reviewed exact head, merge commit, and material limits.

## Next task

Return to Chatterbox after merge; the first Contract 058 proof is then closed subject to the g05.002 stop conditions.
