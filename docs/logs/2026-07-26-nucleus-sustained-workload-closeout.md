# Nucleus Sustained Workload Closeout

Date: 2026-07-26
Card: `../roadmaps/g02/batch-cards/042-nucleus-sustained-workload-hardening.md`

## Outcome

Card 042 completed its approved reset through Nucleus's normal Agent Chat
path. The reset used the frozen Nucleus, Swallowtail, Codex, ChatGPT access,
model, fixture, topology, and host tuple.

The rebuilt debug bundle executable had SHA-256
`a7be57b5741a5bff1c6e37878b3690c5534c6f4bcf55926b98df4391174b0092`.
Each launch started that executable with the isolated proof environment.
Computer Use targeted the full application path. A pre-turn check proved one
exact Nucleus PID with only the isolated database open.

## Valid Workload

The accepted valid tranche completed:

- 50 serial read-only turns
- 2 durable conversations reopened across 5 launches
- 10 app-server lifecycles
- 35 ordinary completions
- 10 read-only `task_ledger` callback completions
- 3 normal UI cancellations
- 2 controlled 30-second deadlines

The isolated database moved from the card 041 baseline of 14 total,
12 completed, 1 cancelled, and 1 timed-out turn to:

- 64 total
- 57 completed
- 4 cancelled
- 3 timed out
- 0 failed
- 0 active
- 0 unexpected

That is the exact valid delta of 50 total, 45 completed, 3 cancelled, and
2 timed out.

## Reset Accounting

The stopped first tranche remains part of consumption evidence:

- 10 provider turns
- 2 launches and catalogue selections
- 2 provider-session lifecycles

Combined with the valid tranche, the reset ended exactly at 60 provider turns,
7 launches and catalogue selections, and 12 provider sessions. The 18 marked
normal-state message records remain preserved.

No more than 2 live app-server children existed. Both children present after
the final idle turn joined application shutdown. The fixture remained clean at
`04f7eb371e4e3ac0010a69d3f96052a7becbe43a`. Nucleus remained clean at
`dd7952152f979511f82a9423b4e32590622e2f47`.

## Risks

- Codex supplied no stable rate, quota, usage, or billed-cost summary for the
  ChatGPT subscription-backed route.
- The proof does not widen the frozen Codex version, model, access, host, or
  topology qualification.
- Bounded workspace-write proof remains separately gated.
- Shared bundle identity remains unsafe as proof identity. The accepted
  control is the exact executable, full app path, and PID/database check.

## Next

Card 043 remains planned. Before a provider call, confirm Soundcheck roadmap
authority and obtain explicit approval of its fresh test database,
screenshots, search-enabled workflows, and exact 20-attempt, 4-launch,
2-hour envelope.
