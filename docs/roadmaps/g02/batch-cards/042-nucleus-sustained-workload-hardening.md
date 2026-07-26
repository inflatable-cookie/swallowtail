# 042 Nucleus Sustained Workload Hardening

Status: completed
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

## Launch-Target Stop

The first sustained launch stopped after 10 provider turns. Effigy launched the
current dev executable with the isolated proof environment, but Computer Use
resolved the shared `dev.nucleus.desktop` identity to the existing
`target/debug/bundle/macos/Nucleus.app`. That bundle lacked the proof
environment and used normal Nucleus state.

Observed effects:

- 9 synthetic turns completed and 1 was cancelled
- the normal state contains 18 uniquely marked synthetic message records
- isolated proof evidence stayed at the card 041 baseline of 14 turns
- the read-only fixture remained unchanged
- no Nucleus or Nucleus-owned Codex child remained after shutdown
- no normal-state record was deleted or rewritten during cleanup

A diagnostic SQLite query initially opened normal state in default CLI mode
before the audit switched to `-readonly`; it checkpointed database metadata but
added, removed, or changed no logical record.

The failed tranche consumes 10 provider-turn attempts, 2 native launches and
catalogue selections, and 2 provider-session lifecycles. It does not satisfy
the sustained workload.

Resume requires:

1. an operator decision to preserve the clearly marked synthetic normal-state
   records or authorize a separately planned recovery
2. one exact launch and UI-control target that cannot resolve away from the
   proof environment
3. explicit reset approval for a maximum of 60 provider turns, 7 native
   launches and catalogue selections, and 12 provider-session lifecycles while
   retaining the original 50 valid outcomes, serial execution, read-only
   effects, and 4-hour active-time ceiling

Reset accepted: 2026-07-26. Preserve the marked synthetic normal-state records.
Rebuild the exact committed debug bundle, launch its executable with the proof
environment, and bind Computer Use to the full bundle path. Before any turn,
prove one Nucleus PID and an open isolated database path. The accepted reset is
60 provider turns, 7 launches and catalogue selections, 12 provider sessions,
50 valid outcomes, serial execution, read-only effects, and 4 hours active
time.

## Scope

1. [x] Run 50 Agent Chat attempts across 2 durable conversations, 5 native
   launches, 4 restarts, and 10 app-server lifecycles.
2. [x] Complete 35 ordinary turns, 10 read-only callbacks, 3 cancellations, and 2
   controlled deadlines.
3. [x] Run one active turn at a time with no more than 2 live app-server children.
4. [x] Permit at most 5 exact-scenario reruns, 55 turns total, and 4 hours wall
   time.
5. [x] Keep the 10 bounded task attempts excluded because no disposable
   worktree and provider-write grant was accepted.
6. [x] Preserve exact rate, usage, cleanup, failure, persistence, and safe
   correlation evidence.
7. [x] Reduce Swallowtail failures to offline fixtures before replay. No new
   Swallowtail failure appeared in the sustained reset.

## Execution Evidence

The accepted reset completed on 2026-07-26 through the rebuilt debug bundle:

- bundle executable SHA-256
  `a7be57b5741a5bff1c6e37878b3690c5534c6f4bcf55926b98df4391174b0092`
- 5 valid launches, 4 restarts, 5 catalogue selections, and 10 app-server
  lifecycles
- 50 serial read-only turns across the 2 retained durable conversations
- 35 ordinary completions, 10 read-only `task_ledger` completions,
  3 cancellations, and 2 controlled deadlines
- exact isolated evidence delta: 50 total, 45 completed, 3 cancelled,
  2 timed out, 0 failed, and 0 unexpected
- exact final isolated evidence: 64 total, 57 completed, 4 cancelled,
  3 timed out, 0 failed, 0 active, and 0 unexpected
- no more than 2 live app-server children; both joined application shutdown
- unchanged fixture commit
  `04f7eb371e4e3ac0010a69d3f96052a7becbe43a`

Every launch used an environment-bound executable and full bundle-path UI
target. Before its first turn, one exact Nucleus PID had only the isolated
database open. The frozen Nucleus, Swallowtail, Codex, access, model, host,
topology, and no-fallback tuple remained unchanged.

Including the stopped tranche, the reset consumed exactly 60 provider-turn
attempts, 7 launches and catalogue selections, and 12 provider sessions. The
18 marked normal-state messages remain preserved. No workspace, task, SCM,
forge, proposal, fixture, provider-account, Nucleus source, publication, push,
tag, or release mutation occurred.

## Acceptance Criteria

- [x] the approved read-only sustained workload from card 040 passes
- [x] 50 planned chat turns pass within the 55-turn and 4-hour ceilings
- [x] no detached task, leaked process, callback loss, or silent fallback
- [x] provider and application state reconcile after every planned failure
- [x] every preceding Swallowtail fix has deterministic regression coverage
- [x] reruns use the same accepted envelope
- [x] unresolved capability, auth, topology, or version risks are explicit

## Remaining Risks

- Codex exposes no stable rate, quota, usage, or billed-cost summary for this
  ChatGPT subscription-backed route.
- The proof covers Codex `0.145.0`, exact `gpt-5.4-mini`, local stdio, and the
  frozen macOS host. It does not widen qualified version, model, access, or
  topology support.
- Bounded workspace-write execution remains unproved and separately gated.
- The shared bundle identifier remains unsuitable as proof identity; the
  accepted control is the exact environment-bound executable plus full app
  path and PID/database check.

## Stop Conditions

- spend, duration, provider rate, or workspace bounds are exceeded
- a run would touch non-disposable user state
- provider truth cannot be reconciled safely
- repeated failure lacks a deterministic reproduction path
