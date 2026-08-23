# 2026-08-23 g04.047 Gemini Live Output-Token Maximum Closeout

Status: merged through PR 46 at `c2878262`; shared closeout applied
Owner: Tom
Milestone: g04.047
Cards: 130-132 complete
Research: 194
PR: https://github.com/inflatable-cookie/swallowtail/pull/46
Review: changes requested on `ab850d3f`; all four findings applied
Merge: fast-forwarded to `main` at `c2878262` on 2026-08-23
Implementation and reviewed head: `c2878262c8d1bdc34810c5edc8f84e6235938a43`

## Outcome

Promoted Research 194 and bound exact Gemini Live output-token maximum
dispatch for model `gemini-3.1-flash-live-preview`.

Delivered:

- domain `1..=65_536` → setup `generationConfig.maxOutputTokens`
- optional prepared input
  `GeminiLiveSessionProfileInput::with_maximum_output_tokens`
- existing shared carrier `OpenRealtimeMediaSessionRequest`
- capability `OutputTokenLimit` + exact `OutputTokenMaximum`
- facade
  `...BidiGenerateContent.thinking-output-max-2026-08-23`
- private behavior
  `gemini.live-preview-manual-pcm-rollover-thinking-output-max-v3`
- claim `gemini.live-preview-window-3`
- model-route revision `prepared-3`
- pre-thinking historical point retained unchanged as
  `GEMINI_LIVE_SUPERSEDED_FACADE_REVISION` (`...BidiGenerateContent`)
- thinking-capable historical point retained as
  `GEMINI_LIVE_THINKING_SUPERSEDED_FACADE_REVISION`
  (`...thinking-2026-08-23`)

Omission preserves current initial and resume setup bytes and claims no
output-limit capability. Selected maxima remain immutable across initial
setup, one planned rollover, and fresh restoration, and compose with every
admitted thinking level. Docs claim dispatch only.

## Validation

Worker evidence:

- `cargo fmt -p swallowtail-adapter-gemini`
- `effigy validate:focused swallowtail-adapter-gemini`
- `effigy package:verify-affected swallowtail-adapter-gemini`
- `effigy check:examples`
- `effigy package:api` after updating the unreleased Gemini baseline
- remaining card-132 index and route gates recorded in the PR body
- Effigy doctor god-file baseline restored to the inherited 371 findings
  (326 warnings, 45 errors) by keeping `plan_with_maximum` off
  `tests/live_support/fixture.rs`
- independent review validation: 80 focused tests, affected-package proof,
  semantic API, diff check, and inherited doctor-baseline confirmation
- all five required GitHub CI jobs passed on exact merged head `c2878262`; the
  stable job needed reruns after two unrelated cancellation-fixture flakes,
  each of which passed in isolation

No live Gemini call, credential, account inspection, or paid work.

## Shared-Surface Closeout

The worker correctly restored shared indexes to `origin/main`. The orchestrator
applied the reserved delta after fast-forward merge.

- Research 194, the log index, cards 130-132, milestone g04.047, the programme,
  and generation checkpoint now record the promoted and merged result.
- Architecture, the provider route matrix, and the generated solution-feature
  matrix now record the optional positive maximum and current opaque facade.
  Only the `gemini.live` output-token-limit cell changed from `No` to `Yes`.
- Contract 029 records the new current opaque point and preserves both earlier
  points as frozen, non-executable evidence. Standing currentness checkpoint
  records remain unchanged until their next named run.
- The changelog records the additive route-local control and its dispatch-only
  evidence boundary. No release, tag, or workspace-version mutation was
  selected.
- The sole Next Task now points to g04.048 compilation from the remaining
  promoted per-route feature inventory. No route or control is preselected.

## Worktree / Branch

- worktree: `/Users/tom/.t3/worktrees/swallowtail/t3code-3e5e8041`
- branch: `t3code/review-gemini-live-output-maximum`
- planning base ancestor: `c51e3e9898c6ea08e217d0d981d2b982e0a5590b`
- evidence commit: `8d3717a010ea496c7f231fd6f4e576dfd4d05c9f`
- reviewed and merged head: `c2878262c8d1bdc34810c5edc8f84e6235938a43`
- worker does not merge
