# 2026-08-26 g04.078 llama.cpp Owned Reasoning Controls Closeout

Status: complete and review-ready
Owner: Tom
Milestone: g04.078
Cards: 216, 217, 218

## Result

Research 225 admits a narrow non-empty deliver-now set: exactly one of the five
candidate reasoning values. Exact `llama-cpp.owned` `b10069-178a6c449` can
dispatch `--reasoning off` without changing omission, context-size argv,
lifecycle ordering, or Contract 029 identity.

- Card 216 froze exact `b10069` parser, precedence, application, and
  observation truth from tagged source. `--reasoning` accepts a wider value set
  than the README's `on|off|auto`; `auto` and `-1` store the parser default and
  write no template kwarg, so they are byte-equivalent to omission. Exact
  `tools/server/server-context.cpp` evaluates
  `enable_reasoning != 0 && template_supports_thinking`, so only `off`
  short-circuits before the template is probed. `on` shares the default's
  startup result and differs only through a request-time template-kwarg
  override. `--reasoning-budget` is discarded whole when the applied template
  has no thinking end tag, and that tag is per-request. Observation is closed:
  `/props` `chat_template_caps` has exactly eight keys and none reports
  thinking support, `task_params::to_json` emits no `reasoning_budget_tokens`
  in either branch, and the one `thinking = %d` line is `LOG_TRC`, above the
  default verbosity of `3`.
- Card 217 added closed adapter-local `LlamaCppReasoningSelection` with the one
  admitted variant `Disabled`, `with_reasoning` on serving selection, immutable
  prepared evidence, and configured driver argv. No raw string or integer
  enters the public surface and no rejected value is constructible.
  `StartServingRequest` is unchanged.
- Card 218 proved omission keeps the eleven-argument command, `--reasoning off`
  appends after any `--ctx-size N`, both selections compose without
  interference, input/evidence/driver/argv agree, and build-mismatch cleanup
  stays joined with the reasoning flag present.

The lane adds no new pre-process rejection class. The closed one-variant type
makes every unadmitted reasoning value unconstructible rather than rejected.
Prepared, access, host-service, deadline, and model-route mismatches keep
rejecting before artifact acquisition and process start, unchanged. The exact
executable build is still verified only after launch through `/props`
readiness, and card 218 proves a mismatch keeps the joined stop, wait,
endpoint-release, and artifact-release ordering with the reasoning flag
present. Research 225 records that this route has no preflight model/template
source; the admitted row needs none, because its applied server state is
template-independent. The completed roadmap and card criteria were corrected to
say exactly this rather than claim a pre-process exact-runtime gate the
implementation does not provide.

Claimed truth is dispatch plus applied server state. Effective and observed
reasoning behavior stay withheld: a chat template need not honor the render
variable, a consumer request may override it through `chat_template_kwargs`,
and no readiness channel reports reasoning state. No model reasoning
capability, portable control, or attached-route reasoning support is implied.

## Shared Closeout

- architecture, guide, and route/feature matrices record exact adapter-local
  owned `--reasoning off` dispatch, omission preserved, unchanged context-size
  rows, and effective/observed withheld; argv arithmetic is stated exactly, as
  omitting both selections preserving eleven arguments rather than omitting
  either
- the generation index, g04 index, and batch-card index move g04.078 to
  complete: 64 completed milestones, fourteen evidence stops, no ready
  milestone, and cards 216-218 under Completed
- Contract 029 exact `b10069-178a6c449` membership, driver id, behavior id, and
  configured-instance revision do not move
- the attached `b9910` route and its rejection of unqualified reasoning content
  are untouched
- programme, triage, and research/log/roadmap/g04/batch-card indexes reconcile
  Research 225, cards 216-218, and g04.078
- `CHANGELOG.md` records the unreleased `LlamaCppReasoningSelection` public
  adapter API
- the sole Next Task returns to remaining per-route inventory reassessment
- g04 remains active and unrolled until explicit operator direction

## Validation

Named card 218 gates passed: format, focused adapter tests (59, up from 53),
verify-affected, examples, public API, northstar,
research/logs/roadmap/g04/batch-card/next-action indexes, and
`git diff --check`. Doctor matches the inherited baseline exactly: 380
god-file findings (334 warnings, 46 errors) plus one generated-in-src warning.

The reasoning proofs first pushed `tests/owned_driver.rs` from warning to
error (380/333/47). Rather than log that as new debt, its selection proofs
moved to `tests/owned_driver/selections.rs`, alongside the existing
`owned_driver/failures.rs` module. The file is a warning again at 260 code
lines and the baseline is exact. The open g04.056 papercut records the
remaining `prepared_facades.rs` split and the stale g04.056 baseline figure.

CI needed one rerun. The first Stable run aborted in
`swallowtail-adapter-opencode` on a fixture-server `ConnectionReset` during
drop, which is the open 2026-08-22 OpenCode papercut and not this lane's code;
the branch touches no OpenCode file and the rerun passed all five checks.

- PR: https://github.com/inflatable-cookie/swallowtail/pull/77
- branch: `t3code/llama-cpp-reasoning-controls`
- worktree: `/Users/tom/.t3/worktrees/swallowtail/t3code-d2c680cc`

## Next

Reassess the remaining per-route feature inventory and compile the next
numbered route-local milestone. Do not release, move currentness, roll g04, or
close the generation from this lane.
