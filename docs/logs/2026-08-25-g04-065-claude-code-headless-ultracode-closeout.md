# 2026-08-25 g04.065 Claude Code Headless Ultracode Closeout

Status: stopped after evidence
Owner: Tom
Milestone: g04.065
Cards: 181-183
Branch: `t3code/claude-code-headless-ultracode`
Worktree: `/Users/tom/.t3/worktrees/swallowtail/t3code-b3abaea4`
Base: `7687ea8e1d972925122f3753734bb53bc0ad2d5b` (`origin/main` at dispatch)
PR: [#64](https://github.com/inflatable-cookie/swallowtail/pull/64)

## Outcome

Card 181 completed an exact evidence pass across `@anthropic-ai/claude-code`
`2.1.202`, `2.1.203`, `2.1.220`, and `2.1.241` with the qualified ceiling at
`2.1.241`. Research 212 admits no deliver-now Ultracode row. Cards 182 and
183 are blocked and were not executed. The headless adapter, prepared facade,
ordinary effort dispatch, guide, and API baseline remain unchanged except for
the new evidence corpus and its focused fixture assertion. No install, login,
credential capture, account inspection, provider request, or paid operation was
authorized for binding claims.

## Evidence Stop

Official documentation describes Ultracode as `--effort ultracode` from
`v2.1.203+`: a product setting that sends `xhigh` and enables dynamic workflow
orchestration. Exact `--help` at every probed point, including the qualified
ceiling `2.1.241`, advertises only `low|medium|high|xhigh|max`.

Local parser behavior:

- `2.1.202` rejects `--effort ultracode` with the frozen unknown-value warning
  and falls back to default effort.
- `2.1.203`, `2.1.220`, and `2.1.241` accept `--effort ultracode` without
  warning, but help still omits it.

Extracted implementation strings confirm Ultracode is coupled to dynamic
workflow orchestration and gated by model or organization `xhigh` availability.
The selected headless command fixes Plan mode, `Read,Glob,Grep`, empty MCP,
and no-session persistence, but this lane does not prove that Ultracode's
workflow effects are disabled or fully contained inside the route's owned child
and joined cleanup truth. Model eligibility, entitlement, and effective
behavior therefore remain unproved without account or provider work.

Research 212's deliver-now table is empty. Ultracode must not enter portable
`ReasoningMode` as a seventh value or as an alias for `xhigh`. The existing
`claude-code.headless.stream-json.v1` behavior remains the sole claim.

## Changed Route-Local Surfaces

- `docs/research/212-claude-code-headless-ultracode-evidence.md`: promoted
  exact official/package evidence, parser/help truth, workflow/topology gaps,
  portable-reasoning disposition, and empty deliver-now table
- `crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-code-2.1.241/headless-ultracode.json`:
  added sanitized exact-version Ultracode evidence corpus
- `crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-code-2.1.241/README.md`:
  indexed the new corpus
- `crates/swallowtail-adapter-claude-agent/tests/claude_code_headless_identity.rs`:
  added deterministic corpus assertions
- this closeout log

The worker leaves `crates/swallowtail-adapter-claude-agent/src/**`, the guide,
public API baseline, route/feature matrices, changelog, and all shared surfaces
unchanged.

## Shared-Surface Delta

There is no admitted capability delta:

- architecture and Contracts 033/040/044: unchanged; no Ultracode binding
- route/feature matrix: keep headless Ultracode unchanged / not deliverable
- programme and triage: record g04.065 as stopped after evidence
- indexes: refresh status text for Research 212, cards, and this closeout
- changelog and release: unchanged; nothing shipped
- package API baseline: unchanged

## Validation

Passed:

- `cargo fmt -p swallowtail-adapter-claude-agent`
- `effigy validate:focused swallowtail-adapter-claude-agent` — 103 tests passed
- `effigy package:verify-affected swallowtail-adapter-claude-agent`
- `effigy check:examples`
- `effigy qa:routes`
- `effigy qa:northstar`
- `effigy qa:docs:index:research`
- `effigy qa:docs:index:logs`
- `effigy qa:docs:index:roadmaps`
- `effigy qa:docs:index:roadmaps:g04`
- `effigy qa:docs:index:roadmaps:batch-cards`
- `effigy qa:docs:next-action:roadmaps`
- `effigy package:api`
- `git diff --check`

`effigy doctor` remains the inherited baseline: 378 god-file findings (332
warnings, 46 errors) plus one generated-in-src warning. No new doctor finding
was introduced by the lane.

## Continuation

Keep g04 open. Reassess the remaining promoted per-route feature inventory for
the next serial lane unless the operator supplies a different direction.
Contract 029 currentness remains standing.
