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
  profile input and the low-level driver, immutable prepared evidence, a
  fail-closed qualified-version gate, and one canonical `--max-turns <n>`
  appended to the existing command.
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

`UnverifiedNewer` points reject a selection before process work with
`swallowtail.claude_code.headless.preparation.maximum_turns_unqualified`. No
artifact for one was probed, so the parser, the loop guard, and the terminal
shape are unproved there. Omission still runs on those versions, unchanged.

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
  `ClaudeCodeMaximumTurns` and its three new seams
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
`git diff --check`. Adapter tests are 113, up from 108. Doctor matches the
inherited baseline exactly: 380 god-file findings (334 warnings, 46 errors)
plus one generated-in-src warning.

- PR: pending
- branch: `t3code/e222dbf8`
- worktree: `/Users/tom/.t3/worktrees/swallowtail/t3code-e222dbf8`

## Next

Reassess the remaining per-route feature inventory and compile the next
numbered route-local milestone. Do not release, move currentness, roll g04, or
close the generation from this lane.
