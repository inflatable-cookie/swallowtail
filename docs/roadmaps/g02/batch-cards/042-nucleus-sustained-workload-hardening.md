# 042 Nucleus Sustained Workload Hardening

Status: planned
Owner: Tom
Created: 2026-07-25
Milestone: `../014-consumer-scale-application-proof-and-hardening.md`

## Objective

Run the accepted repeated Nucleus workload, fix Swallowtail defects
fixture-first, and replay until the exact envelope passes.

## Entry Gates

- card 041 complete
- exact operation, duration, spend, concurrency, and stop budgets accepted
- disposable workspace and write authority approved separately when used

## Scope

1. Run 50 Agent Chat attempts across 2 durable conversations, 5 native
   launches, 4 restarts, and 10 app-server lifecycles.
2. Complete 35 ordinary turns, 10 read-only callbacks, 3 cancellations, and 2
   controlled deadlines.
3. Run one active turn at a time with no more than 2 live app-server children.
4. Permit at most 5 exact-scenario reruns, 55 turns total, and 4 hours wall
   time.
5. Add exactly 10 bounded task attempts only under the separate disposable
   worktree and provider-write grant in card 040.
6. Preserve exact rate, usage, cleanup, failure, persistence, and safe
   correlation evidence.
7. Reduce Swallowtail failures to offline fixtures, fix, run repository QA,
   then replay the failed application scenario.

## Acceptance Criteria

- [ ] the complete card 040 workload passes
- [ ] 50 planned chat turns pass within the 55-turn and 4-hour ceilings
- [ ] no detached task, leaked process, callback loss, or silent fallback
- [ ] provider and application state reconcile after every planned failure
- [ ] every Swallowtail fix has deterministic regression coverage
- [ ] reruns use the same accepted envelope
- [ ] unresolved capability, auth, topology, or version risks are explicit

## Stop Conditions

- spend, duration, provider rate, or workspace bounds are exceeded
- a run would touch non-disposable user state
- provider truth cannot be reconciled safely
- repeated failure lacks a deterministic reproduction path
