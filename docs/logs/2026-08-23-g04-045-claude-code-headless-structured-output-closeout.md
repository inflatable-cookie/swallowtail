# 2026-08-23 g04.045 Claude Code Headless Structured Output Closeout

Status: stopped after evidence; merged
Owner: Tom
Milestone: g04.045
Cards: 124-126
Branch: `t3code/review-headless-structured-output`
Worktree: `/Users/tom/.t3/worktrees/swallowtail/t3code-0098ce5a`
Base: `1fe9066676a1568b1910a80656879dc395c9a50e` (`origin/main` at dispatch)
Worker implementation head: `9c81b327`
PR: [#44](https://github.com/inflatable-cookie/swallowtail/pull/44)
Merge head: `8a2640ea4543430efa4766eeae9f2b0e4eee54eb`

## Outcome

Card 124 completed an exact evidence pass against
`@anthropic-ai/claude-code@2.1.238`. Research 192 admits no deliver-now schema
row. Cards 125 and 126 are blocked and were not executed. The headless adapter,
its existing prepared facade, schema-absent fixtures, guide capability claim,
and unreleased API baseline remain unchanged except for the new evidence corpus
and its focused fixture assertion. No install, login, credential or account
inspection, provider prompt, provider request, or paid operation was used.

## Evidence Stop

The exact package exposes `--json-schema` and locally validates an object schema.
On the selected stream-JSON Plan-mode command it adds a model-visible
`StructuredOutput` tool beside `Read`, `Glob`, and `Grep`. The native binary
contains validation and retry signals, so Contract 040 classifies enforcement
as `HarnessValidated`. Official SDK guidance and exact local CLI probes
establish draft-07 at the validation boundary; full keyword subset and
CLI-to-SDK runtime applicability remain unqualified. The retry maximum is not
bindable from the CLI/package surface, and no valid provider-produced
structured terminal result was qualified without a live prompt. The sanitized
no-auth specimen also records a `success` subtype with `is_error: true`, exit
`1`, and no `structured_output`; subtype alone is not acceptance truth.

Research 192's deliver-now table is empty. The existing
`claude-code.headless.stream-json.v1` behavior remains the sole claim, with no
new Contract 029 point. The guide stays unchanged because card 126, which owns
guide capability claims, did not execute.

## Changed Route-Local Surfaces

- `docs/research/192-claude-code-headless-structured-output-evidence.md`:
  promoted exact official/package evidence, deterministic specimens, schema,
  enforcement, retry, terminal, compatibility, and empty deliver-now
  dispositions
- `crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-code-2.1.238/headless-structured-output.json`:
  added sanitized exact-version schema evidence corpus
- `crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-code-2.1.238/README.md`:
  indexed the new corpus
- `crates/swallowtail-adapter-claude-agent/tests/claude_code_headless_identity.rs`:
  added deterministic corpus assertions
- this closeout log

The worker PR left `crates/swallowtail-adapter-claude-agent/src/**`, existing
headless and response-only fixtures, the prepared-integration guide, public API
baseline, and all shared surfaces unchanged. This post-merge closeout updates
only programme, triage, indexes, milestone/card status, and Next Task truth; it
does not change architecture, contracts, route/feature matrices, changelog,
release, or package API claims.

## Shared-Surface Delta

The worker records the required shared delta here without editing those
surfaces. There is no admitted capability delta:

- architecture and Contracts 039/040: unchanged; no new schema capability
- route/feature matrix: keep headless structured output unchanged / not
  deliverable
- programme and triage: record g04.045 as stopped after evidence
- indexes: refresh status text for Research 192, cards, and this closeout
- changelog and release: unchanged; nothing shipped
- package API baseline and matrix assertions: unchanged
- `docs/roadmaps/README.md` and g04 Next Task: move the pointer after this
  reviewed stop; no next feature family is selected by this lane

The orchestrator applied this shared-surface delta after PR 44 merged. The next
planning checkpoint is g04.046 compilation from the remaining promoted
per-route feature inventory; no feature family is preselected here.

## Validation

Passed:

- `cargo fmt -p swallowtail-adapter-claude-agent`
- `effigy validate:focused swallowtail-adapter-claude-agent` — 102 tests passed
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

`effigy doctor` remains the expected inherited failure: 371 god-file findings
(326 warnings, 45 errors) plus one generated-in-src warning. The graph index
was refreshed locally and is current; no new doctor finding was introduced by
the lane.

Cards 125-126 binding-only gates were not run because there is no typed input,
prepared plan, request policy, driver/argv binding, structured parser, or
dispatch to exercise. There is no package API or example delta.

## Unresolved

A future lane may reopen this route only with an exact package surface that
names the accepted schema dialect/subset, exposes an immutable attempt bound,
and proves valid structured terminal, failure, usage, cancellation, and
cleanup truth for the full Plan-mode composition. Later versions remain
`UnverifiedNewer` and do not inherit the evidence.
