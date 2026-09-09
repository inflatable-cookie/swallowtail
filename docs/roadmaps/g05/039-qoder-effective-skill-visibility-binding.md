# g05.039 Qoder Effective Skill Visibility Binding

Status: planned; gated on a non-empty Research 256 deliver-now disposition
Owner: Tom
Created: 2026-08-28
Updated: 2026-09-09
Depends on: completed g05.002 card 004 evidence (Research 256 honest empty set); Contract 058
Governing refs: Contracts 020, 029, 032, 033, 037, 041, 047, 058
Vision tags: harness skills, selected-session truth, consumer integration
Former card: 005 Qoder Effective Skill Visibility Binding (folded g05.038; no scope change)

## Outcome

Bind the exact Qoder roster through Contract 058 without widening structured
run authority — only rows admitted by Research 256.

## Ready-State Rubric

- [x] Objective is bounded enough to finish without fresh planning decisions.
- [x] Governing refs point at current canonical surfaces.
- [ ] Scope, acceptance criteria, validation, evidence, and stop conditions are explicit (below).
- [ ] A non-empty Research 256 deliver-now disposition exists (currently honest empty; task stays planned).
- [ ] No unresolved planning gaps or operator intent checkpoints beyond that gate.

## Decisions

None beyond g05.002. Binding executes only after a non-empty deliver-now
disposition; the honest empty set keeps this task planned with no production
capability landing.

## Dispatch manifest

- **State:** planned; one lane; serial before g05.040; no concurrent siblings.
- **Completion:** bounded observation records, capability, prepared-plan agreement, decoder mapping, identity, provenance, completeness, freshness, and safe failure behavior for admitted rows only; existing Qoder run behavior unchanged when unrequested.
- **Owned mutable paths:** Qoder adapter observation/binding paths, route fixtures, guide cells; this task file.
- **Reserved closeout surfaces:** `docs/roadmaps/README.md`, `docs/roadmaps/g05/README.md`, `docs/roadmaps/generation-index.md`, `docs/logs/README.md`.
- **Worker:** adapter worker with Contract 058 literacy; no provider credentials.
- **Excluded:** prompts, scans, installs, mutations, inferred provenance; widening the Qoder run; model-visibility inference.
- **Escalation:** operator via Chatterbox for disposition questions.

## Work

1. Implement only admitted Research 256 rows as bounded observation records with capability, prepared-plan agreement, decoder mapping, identity, provenance, completeness, freshness, and safe failure behavior.
2. Preserve unchanged omission; keep empty and unavailable distinct.
3. Prove existing Qoder run behavior is unchanged when unrequested.

## Acceptance and review oracle

| Invariant | Adversarial counterexample | Required proof |
| --- | --- | --- |
| Only admitted rows land | An unadmitted roster row is observable | Fixture diff against the admitted set |
| No widened authority | A prompt, scan, install, mutation, or inferred provenance appears | Negative fixtures plus unchanged-run proof |
| Empty stays empty | Empty and unavailable collapse into one state | Distinct-state fixtures |

- [ ] only admitted Research 256 rows are implemented
- [ ] no prompt, scan, install, mutation, or inferred provenance is added
- [ ] empty and unavailable stay distinct
- [ ] existing Qoder run behavior is unchanged when unrequested

## Validation

- `effigy validate:focused swallowtail-core swallowtail-runtime swallowtail-adapter-qoder`
- `effigy package:verify-affected swallowtail-core swallowtail-runtime swallowtail-adapter-qoder`
- `git diff --check`

## Stop conditions

- Research 256 stays honest-empty: task remains planned, no production capability lands.
- Positive membership requires login, paid work, or ambient host mutation.
- `skills` or `plugins` is partial, lazy, unbounded, or not selected-run truth.
- A project or global row requires Swallowtail or adapter scanning instead of the exact harness roster surface.

## Evidence

On completion, record: admitted rows, validation actually run, PR link, reviewed exact head, merge commit, and material limits.

## Next task

g05.040 closes the first Contract 058 proof after this task lands. If Research 256 stays empty, both tasks remain planned with no dispatch.
