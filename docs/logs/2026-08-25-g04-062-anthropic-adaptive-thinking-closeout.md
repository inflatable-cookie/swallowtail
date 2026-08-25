# 2026-08-25 g04.062 Anthropic Adaptive Thinking Closeout

Status: worker-complete pending review
Owner: Tom
Milestone: g04.062
Cards: 173-175
Branch: `t3code/review-anthropic-adaptive-thinking`

## Result

Research 209 promoted one deliver-now row: exact `claude-opus-4-7` admits
adapter-local `AnthropicThinkingMode::adaptive()` with explicit omitted
display on one-attempt structured inference and fixed direct continuation.
The qualified wire is `thinking: {"display":"omitted","type":"adaptive"}`.
Omission stays byte-identical. Effort remains independent portable
`low|medium|high|xhigh|max`.

Cards 174-175 bound that row. Prepared input and evidence carry the
selection; the shared plan does not grow a thinking capability. Structured
attempts validate omitted thinking and start-complete redacted blocks, emit
no thought content or `ReasoningSummary` activity, and retain no private
continuation after terminal. Direct continuation captures the first-assistant
private sequence in bounded zeroizing memory and replays it unmodified before
`tool_use` on the correlated result attempt and later-turn history. Adaptive
skip with no thinking block is valid. `thinking_delta`, missing signatures,
and thinking without the adapter-local mode fail closed. Late thinking or
redacted blocks after public text/tool content fail closed instead of being
rewritten before `tool_use`. Duplicate signatures and private overflow fail
closed. Raw SSE frames store payload bytes in `RedactedBytes` with redacted
`Debug` and zeroizing drop; the decoder wraps drained frames in a zeroizing
guard before fallible decode and parses private fields without leaving an
ordinary JSON-string copy. Continuation replay encodes private blocks
directly into a redacted request body under that owner for every return path,
including later-turn splicing. POST upload reads that body in place through a
libcurl read callback instead of `post_fields_copy`. Rejected `take_secret`
values stay under the JSON zeroizing guard. `Request` Debug and Drop do not
expose or retain signatures or redacted thinking as ordinary bytes.
Fresh restoration is `SessionReplaced` with the prepared selection and no
private-state recovery.

No shared contract, shared runtime, live Anthropic call, summarized display,
manual budget thinking, or g04 closure.

## Changed Surfaces

- `docs/research/209-anthropic-messages-adaptive-thinking-evidence.md`
- `crates/swallowtail-adapter-anthropic/**`: typed mode, request encoding, SSE
  grammar, structured pump, continuation capture/replay, fixtures, tests,
  public API
- `docs/guides/anthropic-direct-prepared-integration.md`
- `docs/guides/provider-route-matrix.md`
- `docs/guides/provider-solution-feature-matrix.csv` notes
- g04.062, cards 173-175, programme, triage, reserved closeout, Next Task

## Validation

Worker validation passed on this branch:

- `cargo fmt -p swallowtail-adapter-anthropic`
- `effigy validate:focused swallowtail-adapter-anthropic` (103 tests)
- `effigy package:verify-affected swallowtail-adapter-anthropic`
- `git diff --check`
- `git diff --check a69b3546eea09c7cf15edea0733a8301dec1e662...HEAD`

Review round 1 on `05386dd4` requested raw-frame redaction/zeroization, fail-closed
late private order, missing overflow/reorder proof, range whitespace on new SSE
fixtures, and a clean worktree. Those landed on `88bfc0a1`.

Review round 2 on `88bfc0a1` required a redacted zeroizing outbound replay
body that does not clone private fields through `serde_json::Value`, inbound
parse/failure temporaries under a zeroizing guard, and proofs at those
ownership boundaries instead of leftover-buffer test helpers. Those landed on
`9ea1cff7`.

Review round 3 on `9ea1cff7` required the replay builder to stay under
zeroizing ownership through every fallible path, a no-copy curl upload of the
redacted body, and zeroized rejection of wrong-typed secret JSON. Those are
applied on this head. The worktree no longer carries `.tmp-research-209/`.

`effigy doctor` reproduces the inherited baseline: 378 god-file findings
(332 warnings, 46 errors) plus one generated-in-src warning. Session parser
extraction replaced the former `attempt.rs` warning; new thinking proofs stay
under the warning threshold. Default QA used no credentials, account state, or
paid inference.

PR: [#61](https://github.com/inflatable-cookie/swallowtail/pull/61).
Worker branch: `t3code/review-anthropic-adaptive-thinking`.

## Unresolved

- merge remains operator-authorised
- g04 stays open; next move is review/merge then inventory reassessment
