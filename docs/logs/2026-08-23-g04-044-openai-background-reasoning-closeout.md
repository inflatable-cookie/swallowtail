# 2026-08-23 g04.044 OpenAI Background Reasoning Correction Closeout

Status: complete and merged
Owner: Tom
Milestone: g04.044
Cards: 122-123
Branch: `t3code/openai-background-reasoning-correction`
Worktree: `/Users/tom/.t3/worktrees/swallowtail/t3code-4946f4f4`
Base: `1b3b5bb243a14eeacc9475cc9efa595f4e009321`
Implementation commit: `a8cad66`
PR: https://github.com/inflatable-cookie/swallowtail/pull/43
Review: accepted at exact head `bdb7ea88d850ef6610ade2d581a58fc076f980f2`
Merge: PR 43 fast-forwarded to `main` at `bdb7ea88d850ef6610ade2d581a58fc076f980f2`

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

The worker did not edit shared surfaces outside the named route lane. Merge
closeout applied the following shared delta on `main`.

- `docs/architecture/system-architecture.md`: no architecture change; the
  existing realized OpenAI background route shape remains accurate.
- Route/feature/activity matrices and shared matrix assertions: feature posture
  remains reasoning selection available. The direct-activity fixture now names
  the corrected August facade point; no capability cell changed.
- Contract 036 and `CHANGELOG.md` classify removal of previously guaranteed
  `minimal` as a breaking guarantee shrink. The next source release must
  advance the coordinated pre-1.0 minor and carry the named release-note
  delta. No release, tag, or workspace-version mutation was selected.
- Programme, front doors, and indexes record cards 122-123 and g04.044 complete,
  PR 43 at `bdb7ea88`, and g04 still active at 44 numbered roadmaps.
- The sole Next Task is now a planning boundary: reassess the remaining
  promoted per-route feature gaps, select one coherent route-local control
  family, and compile g04.045. No implementation card is ready yet.

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
- Exact PR head `bdb7ea88d850ef6610ade2d581a58fc076f980f2` passed all five
  CI jobs. The first MSRV attempt hit an unrelated OpenCode deadline assertion;
  its one rerun passed before merge.
- Merge closeout passed focused `swallowtail-testkit` validation (93 tests),
  affected package proof, route/activity matrices, Northstar QA, log/roadmap/
  g04/batch-card indexes, the sole-next-action gate, and `git diff --check`.

## Unresolved

- The next g04 route/control family is intentionally unselected until the
  g04.045 planning pass rechecks current route and contract truth.
- Release selection, tagging, publication, and workspace-version mutation
  remain operator-owned and unselected.
