# 2026-08-27 g04.079 Claude Code Headless Maximum Turns Closeout

Status: complete and review-ready
Owner: Tom
Milestone: g04.079
Cards: 219, 220, 221

## Result

Research 226 admits a non-empty deliver-now set, so the conditional binding and
acceptance cards both ran. `claude-code.headless` now accepts one closed
adapter-local positive maximum over agentic tool-use turns on every published
version in the qualified `2.1.220..=2.1.241` window.

- Card 219 downloaded every published official npm package in the window and
  its `@anthropic-ai/claude-code-darwin-arm64` platform package, then probed
  each native executable through `--version`, `--help`, prompt-free argv cases,
  and extracted implementation source. No install, login, account inspection,
  provider request, paid operation, or ambient configuration write was used.
- Card 220 added `ClaudeCodeMaximumTurns`, `with_maximum_turns` on the run
  profile input, immutable prepared evidence, a fail-closed gate on the exact
  probed version set, and one canonical `--max-turns <n>` appended to the
  existing command.
- Card 221 proved dispatch, omission byte-equality, value and version
  rejection, low-level driver agreement, and the native limit-reached terminal
  shape.

## What The Exact Evidence Changed

Three findings moved the lane away from what the compiled milestone assumed.

**Help omission was the wrong signal.** The frozen `2.1.241` help does not
advertise `--max-turns`, and the milestone treated that as the open question.
The option is in fact registered at every published version and then explicitly
hidden with `hideHelp()`. Absence from help meant nothing; registration and
enforcement had to be proved separately, and both were.

**The native parser is far wider than the documented positive domain.** It
coerces the argument with `Number` and rejects only `NaN`. Zero, negatives,
signed zero, fractions, `Infinity`, exponent form, hexadecimal, grouped digits,
and the empty string all pass. The agent loop then guards with
`maxTurns && next > maxTurns` — a truthiness test, not a presence test — so a
resolved `0` disables enforcement outright and a negative value stops after the
first tool-use turn. Trusting Claude Code's own validation would have shipped an
inert bound. The adapter closes the domain to positive 32-bit integers instead,
which makes every degenerate value unconstructible rather than merely rejected.

**Explicit argv precedence is unconditional and needs no environment access.**
The resolver returns the argv value before it ever reads
`CLAUDE_CODE_MAX_TURNS`, including for argv values the environment itself would
reject. The `-p` probe confirms both branches at both window endpoints. That
settles the environment tension the milestone raised: production preparation
never has to inspect or scrub the opaque approved environment to make a
selection authoritative.

The converse is now written down rather than assumed. With the flag absent, an
ambient `CLAUDE_CODE_MAX_TURNS` is authoritative: a valid positive integer
silently caps the run, and an invalid value aborts Claude Code at startup with
exit `1` before any stream appears. That is existing route truth. Omission
preserves it exactly and still claims nothing about unlimited execution.

## Terminal Truth

Reaching the native bound emits one `error_max_turns` result carrying
`is_error`, `num_turns`, `stop_reason`, `usage`, and a
`Reached maximum number of turns (N)` message, with no `result` field, and the
process exits `1`. Under `--output-format stream-json` the headless print
switch adds nothing; the result message is the only carrier.

The existing decoder already handles this correctly. Any non-`success` subtype
records the provider diagnostic, and the terminal mapping orders that ahead of
the generic nonzero-exit path, so the run reports `ProviderFailed` with
`FailureOrigin::Provider`, no output, the usage observation still emitted, and
unchanged joined cleanup. No decoder or terminal change was needed and none was
made. The lane deliberately withheld a distinct diagnostic for
`error_max_turns`: adding one would widen terminal mapping beyond the boundary,
and consumers that need the distinction can read the exact subtype from the
stream.

## Boundaries Held

A counted turn is one tool-use round trip. It is never output tokens, tool
calls, provider requests, retries, cost, wall time, or context size, and no
portable budget vocabulary was introduced. `claude-code.response-only` and
`claude-agent.acp` are untouched. Model and reasoning selection, Plan
read-only authority, fixed `Read,Glob,Grep`, strict empty MCP, no session
persistence, `AmbientHost` isolation, ambient configuration posture, activity,
deadline, cancellation, retention, process ownership, and joined cleanup are
unchanged. Contract 029 identity, the qualified window, the behavior revision,
and the driver id do not move.

Requested, prepared, dispatched, parser-accepted, natively enforced, reached,
and observed state stay separate in the guide, the fixture, and the tests.
Swallowtail proves dispatch and rejects unqualified rows; it does not claim how
many turns a given prompt will use.

The version gate is the exact Research 226 probed set, not the route's
qualified window. The window is weaker in two ways: its claim permits later
stable points as `UnverifiedNewer`, and its segment is a semantic range
containing `2.1.230`, which was never published to npm. A `Qualified`
assessment for `2.1.230` describes the range, not an observed binary, so it
does not admit the feature. Both cases reject at preparation with
`swallowtail.claude_code.headless.preparation.maximum_turns_unqualified`.

`ClaudeCodePreparedRun::start_run` is the only surface that dispatches a bound.
`with_maximum_turns` on the low-level driver is crate-private, and
`low_level_driver` deliberately returns an unbound driver even when a bound was
prepared. The driver additionally re-checks the plan's version whenever a bound
is present, as a fail-closed guard on the internal seam.

Prepared and dispatched state therefore agree by construction rather than by
comparison, which is the only option available: neither `PreflightPlan` nor
`StructuredRunRequest` records a maximum-turn bound, so there is no immutable
execution input to compare against without a shared-contract change this lane
does not authorize. Keeping the bound and its `(plan, request)` pair together in
one path means they cannot disagree. Omission still runs on every version the
route otherwise permits.

## Shared Closeout

- architecture, the Claude guide, and the route/feature matrices record
  adapter-local `--max-turns` dispatch, the closed positive domain, the
  qualified-version gate, omission preserved with ambient precedence stated
  exactly, and the unchanged provider-failure terminal
- the g04 index, generation counts, and batch-card index move g04.079 to
  complete: 65 completed milestones, fourteen evidence stops, no ready
  milestone, and cards 219-221 under Completed
- programme, triage, and the research/log/roadmap/g04/batch-card indexes
  reconcile Research 226, cards 219-221, and g04.079
- `CHANGELOG.md` and the unreleased public-API baseline record
  `ClaudeCodeMaximumTurns` and its prepared seams
- the sole Next Task returns to remaining per-route inventory reassessment
- g04 remains active and unrolled until explicit operator direction

## Currentness Note, Not Movement

npm `latest` for `@anthropic-ai/claude-code` was `2.1.247` when this evidence
was collected, above the qualified ceiling `2.1.241` that Research 202 froze.
This lane did not probe, qualify, or move that. Currentness is a standing lane
under Contract 029 and is out of scope here. The maximum-turn selection is
gated to `Qualified` versions, so it declines on those points rather than
assuming them.

## Validation

Named card 221 gates passed: `cargo fmt`, focused adapter validation,
verify-affected, examples, public API, northstar, and the
research/logs/roadmap/g04/batch-card/next-action indexes, plus
`git diff --check`. Adapter tests are 116, up from 108. Doctor matches the
inherited baseline exactly: 380 god-file findings (334 warnings, 46 errors)
plus one generated-in-src warning.

Three files initially crossed the god-file warning threshold: the new
maximum-turn test cases, `claude_code_headless_identity.rs`, and
`prepared_code/profile.rs`. Rather than log that as new debt, each was split
along a natural seam — the Research 226 corpus assertions moved to their own
identity test, the dispatch and rejection cases separated, and
`ClaudeCodePreparedEvidence` plus `ClaudeCodePreparedRun` moved to
`prepared_code/profile/prepared.rs` alongside the existing `profile/plan.rs`.
The baseline is exact again. The open g04.056 papercut still records the
inherited structural debt.

### Review Round Two

`betterthanclay` requested changes again at `fb5a9221`. The version gate and
roadmap fixes held; one agreement gap remained and it was real.

`ClaudeCodePreparedRun::low_level_driver` was public and returned a driver
carrying that run's bound, while `start` could only check the supplied plan's
*version*. Preparing run A with maximum `1` and run B with maximum `30` on the
same admitted version, then calling
`run_a.low_level_driver().start_run(run_b.plan(), run_b.request(), …)`, passed
validation and dispatched `1` — silently contradicting run B. The same held when
run B omitted the selection entirely.

Comparison could not fix this, because no immutable execution input records the
bound. `low_level_driver` now returns an unbound driver and prepared `start_run`
builds the bound one internally, so the mismatch is unconstructible.
`an_extracted_driver_never_carries_another_runs_bound` proves all four
cross-pairings dispatch no `--max-turns`, and that prepared `start_run` still
dispatches `1`, `30`, and omission exactly.

### Review Round One

`betterthanclay` requested changes at `87692364`. All three findings were real
and are fixed.

1. **The prepared gate admitted an unprobed version.** The first gate accepted
   any `Qualified` point, and a test asserted `2.1.230` prepared a selection.
   That version is inside the semantic segment but was never published, so no
   artifact was probed. The gate now requires membership in the exact Research
   226 probed set, and the negative boundary is asserted.
2. **The public low-level setter bypassed the gate.**
   `ClaudeCodeHeadlessDriver::with_maximum_turns` is now crate-private, so
   prepared construction is the only path to a bound, and
   `start_run` re-checks the plan's version whenever a bound is present. Direct
   negative low-level tests prove a swapped-in unprobed plan rejects before any
   process work.
3. **Roadmap drift.** A duplicate g04.079 entry still read `ready` /
   `reserved`. Reconciled.

The public surface shrank by one item relative to the reviewed head; nothing
released was touched.

- PR: https://github.com/inflatable-cookie/swallowtail/pull/78
- branch: `t3code/claude-code-headless-max-turns`
- worktree: `/Users/tom/.t3/worktrees/swallowtail/t3code-e222dbf8`

## Next

Reassess the remaining per-route feature inventory and compile the next
numbered route-local milestone. Do not release, move currentness, roll g04, or
close the generation from this lane.
