# 2026-08-23 g04.046 Gemini Live Thinking Levels Closeout

Status: delivered on the worker branch; review round 1 changes applied
Owner: Tom
Milestone: g04.046
Cards: 127-129 complete
Branch: `t3code/read-gemini-live-thinking-handoff`
Worktree: `/Users/tom/.t3/worktrees/swallowtail/t3code-5d9a1a44`
Base: `382ab3863046bbb6bb4e3dbe048fd3369aeee0c9` (`origin/main` at dispatch)
PR: https://github.com/inflatable-cookie/swallowtail/pull/45
Review: changes requested on `86d6178d`; findings 1-3 applied
Implementation head: recorded in the operator report and the PR body at each
push; the final commit on this branch carries only this record

## Outcome

Cards 127-129 complete. Research 193 admits `minimal|low|medium|high` for
exact model `gemini-3.1-flash-live-preview` at the new exact opaque facade
point
`google.generativelanguage.v1beta.GenerativeService.BidiGenerateContent.thinking-2026-08-23`,
and the route now binds them through typed prepared input, capability
constraint, immutable plan and evidence, the realtime request, driver
validation, and both setup frames.

Omission is unchanged: the fixed route still dispatches the exact `MINIMAL`
initial and resume bytes and claims no reasoning capability. Explicit
`minimal` serializes identically but is a planned `ReasoningSelection`.

Claims stop at qualified dispatch. `BidiGenerateContentSetupComplete` has no
fields, so provider acceptance, effective reasoning depth, and thought-summary
disclosure remain unclaimed.

## Evidence

Card 127 froze five current official pages on 2026-08-23 with their page dates
and specimen digests, plus exact route source and fixture truth. It made no
live Gemini call and used no credential, account inspection, or paid work.

- exact model page (2026-08-18) and Live capabilities guide (2026-08-05) both
  name `minimal`, `low`, `medium`, `high` for this exact model, default
  `minimal`
- the generate-content reference (2026-08-17) closes the `ThinkingLevel` enum
  at `THINKING_LEVEL_UNSPECIFIED|MINIMAL|LOW|MEDIUM|HIGH`
- the Live WebSocket reference lists the `GenerationConfig` fields the setup
  message does not support; `thinkingConfig` is not among them
- the Thinking guide shows per-model level variance, so no neighbouring
  model's set is inherited

## Route-Local Surfaces

- `crates/swallowtail-runtime/src/realtime_media/request.rs` carries one
  optional portable `ReasoningMode`. It is a narrow field, not a generic
  realtime settings map.
- `crates/swallowtail-adapter-gemini/src/live_reasoning.rs` holds the only
  mapping: `minimal|low|medium|high` to `MINIMAL|LOW|MEDIUM|HIGH`, plus the
  omission level. Nothing else maps.
- `crates/swallowtail-adapter-gemini/src/live_selection.rs` mints the exact new
  facade point, advances the adapter-private behavior revision to
  `gemini.live-preview-manual-pcm-rollover-thinking-v2` and the claim to
  `gemini.live-preview-window-2`, retains the former point as
  `GEMINI_LIVE_SUPERSEDED_FACADE_REVISION`, and adds additive
  capability-bearing requirement and base-capability helpers. The adapter-owned
  model-route revision advances to `prepared-2`.
- `crates/swallowtail-adapter-gemini/src/prepared_live_profile/*` validates the
  admitted value set before any effect and threads the selection into the
  capability profile, instance, model route, plan, evidence, and request.
- `crates/swallowtail-adapter-gemini/src/live.rs` rejects request/plan
  reasoning drift before endpoint, credential, or socket work.
- `crates/swallowtail-adapter-gemini/src/live_protocol/client.rs` and
  `src/live/session*.rs` encode the same level on the initial and rollover
  setup frames; restoration reuses the same plan and request.
- `crates/swallowtail-adapter-openai/src/realtime.rs` rejects an unsupported
  shared-request reasoning field before endpoint, credential, or socket work.
  Its ordinary absent path is unchanged and still covered by the existing
  realtime suite.
- Deterministic coverage: four new corpus setup frames, protocol byte tests for
  every admitted and rejected value, and prepared/driver/wire tests for
  preparation, omission, rejection, drift, one planned rollover, and fresh
  restoration.
- `docs/guides/realtime-prepared-integration.md` states dispatch qualification
  only and names the rejected values.

## Shared-Surface Delta

The worker did not edit shared surfaces. The following delta is required at
merge closeout.

- `docs/architecture/system-architecture.md`: no architecture change; the
  realized Gemini Live route shape is unchanged.
- Route/feature matrices and shared matrix assertions: `gemini.live` gains
  reasoning selection as an available per-route feature. No other cell changes.
- Contract 029 records: the `gemini.live-facade` axis now qualifies the exact
  point `...BidiGenerateContent.thinking-2026-08-23` with private behavior
  `gemini.live-preview-manual-pcm-rollover-thinking-v2` and claim
  `gemini.live-preview-window-2`. The former point and its `-v1` behavior are
  retained verbatim as frozen evidence, are not a supported claim, and are
  proven non-executable.
- `docs/research/127-all-route-version-currentness-checkpoint.md` and
  `docs/research/159-post-harness-expansion-version-currentness-checkpoint.md`
  name the former Gemini Live facade point in the standing currentness lane.
  Both are other lanes' records and were not edited here; the currentness lane
  needs the new point at its next checkpoint.
- `CHANGELOG.md`: additive public API on `swallowtail-runtime` and
  `swallowtail-adapter-gemini`, plus the OpenAI Realtime fail-closed rejection
  of a previously nonexistent field. No guarantee shrinks. No release, tag, or
  workspace-version mutation was selected.
- `release-baselines/public-api-unreleased/`: this branch already adds
  `swallowtail-adapter-gemini` and refreshes `swallowtail-runtime`. No released
  baseline was edited.
- `docs/research/README.md` and `docs/logs/README.md`: Research 193's entry
  still reads "reserved for g04.046 card 127" and the closeout entry still
  reads "reserved". Both need the delivered text at merge closeout. The worker
  reverted its edits to these shared indexes after review finding 2.
- Programme, front doors, and indexes: record cards 127-129 and g04.046
  complete once the PR merges.
- The sole Next Task advances only after orchestrator review and merge.

## Validation

- Card 127 gates passed: `effigy validate:focused swallowtail-adapter-gemini`,
  `effigy qa:northstar`, `effigy qa:docs:index:research`, and
  `git diff --check`.
- Card 128 gates passed: `cargo fmt -p swallowtail-runtime -p
  swallowtail-adapter-gemini -p swallowtail-adapter-openai`, `effigy
  validate:focused` and `effigy package:verify-affected` for the same three
  packages, `effigy package:api`, `effigy qa:northstar`, and
  `git diff --check`.
- Card 129 gates passed: `effigy check:examples`, `effigy qa:routes`, `effigy
  qa:northstar`, all six named research/log/roadmap/batch-card and next-action
  index selectors, `effigy package:api`, and `git diff --check`.
- Two lane-created doctor warnings appeared when the new coverage landed in one
  file each; splitting `live_reasoning_wire.rs` and
  `live_protocol/reasoning_tests.rs` returned the scan to the inherited
  baseline exactly.
- The inherited doctor baseline remains 371 god-file findings (326 warnings,
  45 errors) plus one generated-in-src warning. No lane-created doctor finding
  was introduced.
- Default QA used no credential, account inspection, provider request, live
  Gemini call, or paid work.

## Review Round 1

The orchestrator requested three changes on `86d6178d`. All are applied.

1. Facade point minted rather than replaced in place. The thinking-capable
   behavior is now qualified at its own exact opaque point and the former point
   and proof are retained. One part of the finding could not be taken
   literally: the former point cannot also remain a live claim segment, because
   `swallowtail-core` enforces "Opaque version windows permit one exact segment
   only" and a driver descriptor holds one claim per axis. Keeping both as
   concurrent qualified segments would require weakening Contract 029's
   opaque-axis rule, which this lane's decision gates forbid. The proof is
   preserved instead as a named public constant, unchanged frozen corpus
   frames, an explicit Research 193 disposition, and a deterministic test that
   a plan on the superseded point is rejected before endpoint, credential, or
   socket work. This is the same shape g04.044 used.
2. The worker edits to `docs/logs/README.md` and `docs/research/README.md` are
   reverted. Both shared indexes match `origin/main` again, and their required
   delta is recorded above for orchestrator merge closeout.
3. This record now states the review head and the head-recording convention
   instead of a stale implementation hash.

The red `Pinned MSRV floor` job was the already-recorded OpenCode cancellation
fixture `BrokenPipe` panic, not a lane regression. It was rerun after this
push.

## Unresolved

- Provider acceptance and effective reasoning depth are unobservable on this
  surface. Any future claim needs a separately gated live probe.
- `thinkingBudget`, `includeThoughts`, thought summaries, and context
  compression remain out of scope for this route.
- Merge, release selection, tagging, and publication remain operator-owned.
