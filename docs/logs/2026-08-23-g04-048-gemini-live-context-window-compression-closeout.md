# 2026-08-23 g04.048 Gemini Live Context-Window Compression Closeout

Status: complete; merged through PR 47 at `47848056`
Owner: Tom
Milestone: g04.048
Cards: 133-135 complete
Research: 195 promoted
PR: https://github.com/inflatable-cookie/swallowtail/pull/47
Worker implementation commit: `35b1b282`
Reviewed and merged head: `47848056`
Worker branch: `t3code/gemini-live-context-compression`

## Outcome

Bound the non-empty Gemini Live context-compression subset for exact model
`gemini-3.1-flash-live-preview`:

- omission, preserving the prior initial and resume setup bytes
- typed `GeminiLiveContextWindowCompression::sliding_window()` selection
- exact `contextWindowCompression.slidingWindow = {}` dispatch on initial and
  rollover/resume setup
- immutable selected state through one planned rollover and fresh realtime
  restoration
- composition with every admitted thinking level and omitted/selected output
  maximum
- current facade
  `...BidiGenerateContent.thinking-output-max-context-compression-2026-08-23`
- private behavior `gemini.live-preview-manual-pcm-rollover-thinking-output-max-context-compression-v4`
- claim `gemini.live-preview-window-4`
- model-route revision `prepared-4`

Explicit `triggerTokens` and `targetTokens` remain withheld. Official evidence
closes their int64 JSON string representation but not the exact model-specific
numeric domain, ordering, or rejection behavior. Documentation claims setup
dispatch only: no provider acceptance, effective compression, retained history,
duration, savings, or semantic continuity guarantee.

## Validation

- `cargo fmt -p swallowtail-adapter-gemini`
- `effigy validate:focused swallowtail-adapter-gemini` — 85 tests passed
- `effigy package:verify-affected swallowtail-adapter-gemini`
- `effigy check:examples`
- `effigy qa:routes`
- `effigy qa:northstar`
- research, logs, roadmaps, g04, batch-card, and next-action index gates
- `effigy package:api`
- `git diff --check`

No credential, account inspection, provider request, paid work, or live Gemini
call was performed.

## Result

PR 47 fast-forwarded to `main` at exact reviewed head `47848056`. GitHub records
that head as the merge commit. The route-local implementation and reserved
shared closeout delta are complete.

## Shared Closeout Delta

The orchestrator applied the reserved shared delta after merge:

- architecture: record the Gemini adapter-local typed prepared selection; keep
  portable capabilities and the shared realtime carrier unchanged
- route/feature matrices: mark only `gemini.live` default-only context-window
  compression as available/dispatch-only; leave sibling routes unchanged
- programme/front door: g04.048 is closed and g04.049 compilation is next;
  route and control selection remain open to inventory reassessment
- indexes: Research 195 is promoted, cards 133-135 are complete, and the sole
  `docs/roadmaps/README.md` Next Task pointer now names g04.049 compilation
- changelog: record the additive route-local default-only dispatch surface and
  its withheld numeric forms; do not claim effective compression
- Contract 029: record the current exact facade/private behavior/claim point
  and retain the three earlier points as frozen, non-executable proof; no
  contract rule change
- currentness: do not widen the standing Contract 029 checkpoint or infer a
  newer provider version
