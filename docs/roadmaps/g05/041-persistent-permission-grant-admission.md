# g05.041 Persistent Permission Grant Admission

Status: planned; backlog stub only; no dispatch authorization
Owner: coordinator
Created: 2026-09-07
Updated: 2026-09-09
Depends on: Contract 041; the Card129 feature-matrix cross audit
Governing refs: Contracts 012, 017, 019, 028, 029, 037, 041, 047, 051, 057, 058, 060-062
Former card: 130 Persistent Permission Grant Admission (folded g05.038; no scope change)

## Outcome

Build the producer seam, if the operator promotes it, for persistent
permission grants across the exact provider routes that expose a durable
permission policy. One-shot Allow/Deny exchange remains a separate capability.

## Ready-State Rubric

- [x] Objective is bounded enough to finish without fresh planning decisions if promoted.
- [x] Governing refs point at current canonical surfaces.
- [ ] Scope, completion conditions, and review oracle are explicit (below).
- [ ] Operator promotion exists (absent: this task has no dispatch authorization).
- [ ] No unresolved planning gaps beyond that promotion gate.

## Decisions

None. Promotion, route admission, and matrix changes are operator and
evidence gates, not task decisions.

## Dispatch manifest

- **State:** planned backlog stub; not in any dispatch manifest; no worker, no siblings, no concurrency.
- **Completion:** an explicit producer-owned grant boundary and route capability per admitted route, with consumer policy, provider identity, exact admission, revocation, expiry, and audit semantics preserved; each route qualified independently before any matrix cell changes.
- **Owned mutable paths:** none until promoted; on promotion, the exact producer seam paths Chatterbox compiles.
- **Reserved closeout surfaces:** `docs/roadmaps/README.md`, `docs/roadmaps/g05/README.md`, `docs/roadmaps/generation-index.md`, `docs/logs/README.md`.
- **Worker:** none (no dispatch authorization).
- **Excluded:** dispatch, runtime implementation, provider probes, consumer policy changes, release changes, matrix claim changes.
- **Escalation:** operator via Chatterbox for promotion.

## Work

1. If promoted: define an explicit producer-owned grant boundary and route capability.
2. Preserve consumer policy, provider identity, exact admission, revocation, expiry, and audit semantics.
3. Qualify each route independently before changing a matrix cell.

## Acceptance and review oracle

| Invariant | Adversarial counterexample | Required proof |
| --- | --- | --- |
| Grant support is route-exact | A persistent grant is claimed without that route's evidence | Per-route provider-free proof plus exact-head review |
| One-shot authority never widens | A persistent grant silently extends one-shot Allow/Deny | Boundary fixtures separating the two capabilities |
| Matrix honesty | An unavailable cell is reported as a provider limitation while the seam is unbuilt | Matrix names this task as the producer gap |

- Contract 041 and the route-specific evidence settle whether a persistent
  grant is supported without widening one-shot authority.
- Each admitted route has provider-free proof and an exact-head review.
- The feature matrix changes only after the owning route evidence is frozen.

## Out Of Scope

No dispatch, runtime implementation, provider probe, consumer policy, release,
or matrix claim change is authorized by this stub. Its presence only gives a
future producer gap a durable task reference.

## Review Oracle

An unavailable persistent-grant cell names this task because the producer has
not built the seam; it must not be reported as a provider limitation merely
because the current producer rule withholds it.

## Stop conditions

- No operator promotion: no dispatch, no implementation, no claim change.
- A route cannot support persistent grants without widening one-shot authority.

## Evidence

On completion (if ever promoted), record: admitted routes, validation actually run, PR link, reviewed exact head, merge commit, and material limits.

## Next task

None. This stub authorizes no successor; promotion requires a consumer requirement and the operator's direction.
