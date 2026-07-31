# g03 Compatibility Maintenance And Consumer-Proven Hardening

Status: active
Owner: Tom
Created: 2026-07-31

## Purpose

Keep Swallowtail useful across real provider and harness release ranges while
hardening defects and integration friction proven by consuming applications.

g03 does not chase every upstream release. Each Swallowtail release carries
its qualified ranges. Versions above a qualified upper milestone remain
visible as unverified newer unless exact evidence requires rejection.

## Generation Runway

| Goal | State | Governing refs | First milestone |
| --- | --- | --- | --- |
| Establish a repeatable currentness inventory for installed harnesses and shared protocols. | completed | Contracts 011, 029, 036 | `g03.001` |
| Extend exact compatibility segments only where current evidence finds material drift or useful newer support. | active | Contracts 011, 029, 037 | `g03.002` |
| Keep deterministic corpora and conformance aligned with behavior milestones rather than package semver alone. | planned | Contracts 011, 029, 036 | later g03 |
| Turn consumer-reproduced defects and integration friction into portable regression evidence. | active | Contracts 002, 037, 044-045 | `g03.003` |
| Reassess prepared-facade usability from multi-consumer proof without importing product policy. | planned | Contracts 002, 037 | later g03 |
| Periodically reconcile deferred gates, route truth, and generation capacity. | planned | Contract 001 | recurring checkpoints |

## Current Checkpoint

- g02 closed at 49 roadmaps
- its only unfinished implementation lane, Pi RPC load and resume, moved to
  shared backlog behind the unchanged cwd-bound attachment gate
- no active spec governs g03
- Nucleus owns its delegated child-work, typed question, plan, and task-list
  adoption
- provider-session management binding persistence remains deferred
- registry publication remains outside the active roadmap until the operator
  revisits it after sustained application usage
- warning-only structural reduction remains opportunistic, not a generation
  goal
- Research 074 inventories 13 installed/attached harness route ids and bounds
  the first external currentness source set
- the 2026-07-31 currentness pass leaves Codex and stable ACP unchanged,
  classifies Claude Agent, Gemini, Pi, and Qwen range candidates, keeps Pi
  continuity blocked, and confirms OpenCode's optional live selector is stale
- roadmap g03.002 selects Claude Agent through `0.64.0` and Gemini CLI
  `0.53.0` as the first fixture-first maintenance tranche
- Nucleus reproduced Codex `0.146.0` rejecting a legal numeric request ID at
  activity resolution after a typed callback answer; g03.003 repaired the
  mismatch with strict type-aware deterministic coverage
- the operator elected to complete the resulting portable request-reference
  representation contract immediately; g03.004 now preserves text versus
  signed-integer identity portably and has restored card 004

## Milestones

- [001 Installed Harness And Protocol Currentness Baseline](001-installed-harness-and-protocol-currentness-baseline.md) — completed
- [002 Claude And Gemini ACP Range Maintenance](002-claude-and-gemini-acp-range-maintenance.md) — active
- [003 Codex Request-ID Canonicalization](003-codex-request-id-canonicalization.md) — completed
- [004 Provider Request Reference Representation](004-provider-request-reference-representation.md) — completed

## Next

Execute card 004. Freeze exact Claude Agent `0.62.0..=0.64.0` and Gemini CLI
`0.53.0` artifacts and behavior groups before changing production claims.

## Generation Boundary

g03 begins at roadmap 001 and normally runs for 30-50 numbered roadmaps. A
consumer defect, provider release, or completed maintenance tranche does not
create another generation. Rollover needs a substantial run plus an explicit
sequencing reset.
