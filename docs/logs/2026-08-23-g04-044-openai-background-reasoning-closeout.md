# 2026-08-23 g04.044 OpenAI Background Reasoning Correction Closeout

Status: complete; review and merge remain operator-owned
Owner: Tom
Milestone: g04.044
Cards: 122-123
Branch: `t3code/openai-background-reasoning-correction`
Worktree: `/Users/tom/.t3/worktrees/swallowtail/t3code-4946f4f4`
Base: `1b3b5bb243a14eeacc9475cc9efa595f4e009321`
Implementation commit: `a8cad66`
PR: https://github.com/inflatable-cookie/swallowtail/pull/43
Review: awaiting orchestrator
Merge: none; worker must not merge

## Outcome

Cards 122-123 complete. Exact GPT-5.6 `openai.background` reasoning now
admits `none|low|medium|high|xhigh|max` and rejects `minimal` before endpoint,
credential, request, or provider work. The corrected route binds
`openai-responses-background-2026-08-23` and private behavior revision
`openai.responses-background-v2`. The July point remains historical and is
not silently rewritten or retained as a supported route claim.

## Route-Local Surfaces

- `crates/swallowtail-adapter-openai/src/prepared_profile/background.rs` now
  validates the exact six-value set.
- `crates/swallowtail-adapter-openai/src/selection.rs` publishes the new exact
  facade point through the configured instance, facade binding, operation
  requirements, claim, prepared evidence, and activity basis. The model-route
  record retains model/provider identity and capability truth; it does not bind
  an interface version.
- Driver validation rejects stale facade plans before endpoint or credential
  acquisition. Deterministic route tests cover absent reasoning, every
  admitted value through plan/evidence/policy/driver/wire, explicit
  `minimal` and foreign rejection, and historical-point drift.
- The prepared guide states dispatch qualification only and does not claim
  provider-effective reasoning depth.

## Shared-Surface Closeout Delta

The worker did not edit shared surfaces outside the named route lane.

- `docs/architecture/system-architecture.md`: no architecture change; the
  existing realized OpenAI background route shape remains accurate.
- Route/feature/activity matrices and shared matrix assertions: the feature
  posture remains reasoning selection available, but
  `crates/swallowtail-testkit/tests/fixtures/direct-activity-applicability.json`
  still names the historical July facade and needs an orchestrator-side
  current-point update after review/merge. No shared matrix file was edited.
- Contract 036 / changelog / release notes / workspace versions: the removal
  of previously guaranteed `minimal` is a breaking route guarantee shrink and
  requires an explicit next-minor source-release and release-note delta. No
  release or version mutation belongs to this lane.
- Programme, front doors, and indexes: after merge, the orchestrator must move
  cards 122-123 from Planned to Completed in
  `docs/roadmaps/g04/batch-cards/README.md`; change the g04.044 entries from
  ready to complete in `docs/roadmaps/g04/README.md` and
  `docs/roadmaps/g04/per-route-feature-completion.md`; record the merged
  PR/head in the g04 checkpoint; remove `reserved` from this closeout's entry in
  `docs/logs/README.md`; refresh the g04 status in
  `docs/roadmaps/generation-index.md`; and replace the execution pointer in
  `docs/roadmaps/README.md` and the programme's Next Planning Boundary with
  the orchestrator-selected next task. The worker leaves those shared status,
  index, and front-door surfaces unchanged.
- `docs/roadmaps/README.md`: leave the sole Next Task pointer in place until
  the orchestrator performs merge closeout and selects the next lane.

## Validation

- Card 122 focused validation passed: `cargo fmt -p
  swallowtail-adapter-openai`, `effigy validate:focused
  swallowtail-adapter-openai`, `effigy package:verify-affected
  swallowtail-adapter-openai`, `effigy package:api`, `effigy qa:northstar`,
  and `git diff --check`.
- Card 123 final gates passed: `effigy check:examples`, `effigy qa:routes`,
  `effigy qa:northstar`, all six named research/log/roadmap/batch-card and
  next-action index selectors, `effigy package:api`, and `git diff --check`.
- The inherited doctor baseline remains 371 god-file findings (326 warnings,
  45 errors) plus one generated-in-src warning; no lane-created doctor
  finding was introduced.
- Package verification retained existing unused-patch warnings; API
  documentation retained the inherited Ollama broken-link warning. Both
  selectors passed, and no OpenAI warning was added.

## Unresolved

- Orchestrator review is pending; merge remains unauthorized.
- Shared activity-inventory correction and the Contract 036 next-minor
  release-facing delta remain outside this worker branch.
- No planning decision is needed for the route-local correction. Release
  selection, publication, and merge remain separate operator decisions.
