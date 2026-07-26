# 042 Nucleus Sustained Workload Hardening

Status: active
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

## Approval Gate

The next decision covers only the read-only sustained workload:

- 50 planned turns, 55 attempts maximum;
- 5 native launches, 4 restarts, and 10 provider sessions;
- 35 ordinary turns, 10 inspections, 3 cancellations, and 2 deadlines;
- 1 active turn, no more than 2 live provider children, and 4 hours maximum;
- the exact card 041 Codex, ChatGPT subscription, model, fixture, topology, and
  no-fallback tuple; and
- no workspace, task, SCM, forge, proposal, provider-account, fixture, or Git
  mutation.

Codex supplied no stable rate, quota, usage, or billed-cost summary during card
041. Approval therefore accepts a bounded subscription-backed workload whose
consumption is controlled by turn and time ceilings rather than exact cost
evidence.

The 10 bounded task attempts remain excluded. They require a later disposable
worktree and provider-write grant.

Operator pause: 2026-07-26. Do not resume this card implicitly after the Codex
discovery-diagnostics repair.

Approval accepted: 2026-07-26. The operator explicitly approved the complete
read-only envelope after Nucleus returned to a clean checkpoint.

## Frozen Execution Tuple

- Nucleus: `dd7952152f979511f82a9423b4e32590622e2f47`
- Swallowtail: `a3fbc14b8a76bad074e8542223497c840cb73ffe`
- Codex: `codex-cli 0.145.0`, direct target SHA-256
  `1da3f4e0e96028b8a771814293c3033dafd1971f943f6c7e79b0897fe705f590`
- host: macOS `26.5.2`, arm64, local authoritative
- access: ChatGPT interactive OAuth and subscription allowance; no API-key
  environment
- route: exact `gpt-5.4-mini`, low reasoning, no fallback
- fixture: exact read-only commit
  `04f7eb371e4e3ac0010a69d3f96052a7becbe43a`
- state: isolated Nucleus proof root retained from card 041

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
