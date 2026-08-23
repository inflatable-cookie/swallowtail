# 2026-08-23 g04.047 Gemini Live Output-Token Maximum Closeout

Status: worker complete; awaiting review and merge
Owner: Tom
Milestone: g04.047
Cards: 130-132
Research: 194
PR: pending

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
- superseded thinking-capable point retained as
  `GEMINI_LIVE_SUPERSEDED_FACADE_REVISION`

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

No live Gemini call, credential, account inspection, or paid work.

## Shared-Surface Delta

Worker branch did not edit shared front doors. After review and merge, the
orchestrator should apply:

| Surface | Required delta |
| --- | --- |
| `docs/architecture/system-architecture.md` | note Gemini Live optional output-token maximum on the current facade point |
| `docs/guides/provider-route-matrix.md` | update `gemini.live` row facade point and mention optional `1..=65536` maximum |
| route/feature matrices / programme / indexes | if they still say thinking-only facade or omit output maximum |
| `CHANGELOG.md` | unreleased note for Gemini Live output-token maximum |
| Contract 029 currentness | no widening; record exact new opaque point only if the checkpoint inventory tracks it |
| `docs/roadmaps/README.md` Next Task | advance after merge closeout; do not leave g04.047 as sole next task |
| matrix assertions / shared package lists | only if they pin the previous facade literal |

## Worktree / Branch

- worktree: `/Users/tom/.t3/worktrees/swallowtail/t3code-3e5e8041`
- branch: `t3code/review-gemini-live-output-maximum`
- planning base ancestor: `c51e3e9898c6ea08e217d0d981d2b982e0a5590b`
- worker does not merge
