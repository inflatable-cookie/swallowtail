# 2026-09-17 g05.084 Roll Over To g06 And Compact g05

Status: complete; structural rollover executed under operator authority
Owner: Tom
Task: g05.084
Planning: `c59ce0bf769f64ffccfd32fda58c6e4ef5cb6c25`

## Result

Tom authorized the structural rollover on 2026-09-17 ("roll over and
compact"). g05 is closed and g06 is the sole active generation.

The preservation oracle ran before any deletion and is recorded as the
g05.084 section of `docs/roadmaps/archive/preservation-manifest.md`: all 82
numbered g05 tasks were classified, no live rule exists only inside the
removed tree, and every open commitment has an active home. The five planned
tasks carried forward with full outcome, rubric, acceptance, evidence, and
stop conditions: g05.035 → g06.001, g05.039 → g06.002, g05.040 → g06.003,
g05.041 → g06.004, and g05.042 → g06.005. The four serial prerequisites
(g05.066, g05.069, g05.080, g05.081) had already settled, so they stayed
terminal. g05.066's live-tuple follow-on stays recorded at its stopped parent
in `docs/roadmaps/archive/g05.md`; the claude-agent.sdk feature-matrix cells
now cite the held real-route gate at g06.001.

Lifecycle state: the closure record
`.northstar/lifecycle/v1/generations/g05.closure.json` commits disposition
`closed` over tasks digest
`sha256:cfe29cbb46d182207cc1d60b2701301c1e27a0ccc8a093ab03976b96ee69754f`;
the 26 terminal fragments were consumed into the receipt
`.northstar/lifecycle/v1/generations/g05.json`;
`projection-targets.json` declares `active_generation: g06` with targets
`docs/README.md`, `docs/roadmaps/README.md`, and `docs/roadmaps/g06/README.md`;
the three generated projection blocks were regenerated with digest
`sha256:cf47685042c25fd50885a6ef905aa75e4290935b58d19805d01b59c8011e8940`
(g06 open, `planning_required`, no terminal records yet). `docs/roadmaps/`,
`generation-index.md`, `status-grammar.md`, `standing-lanes.md`, and the
`effigy.toml` index policy (`roadmaps_g06`) were rewired; the front door
names the g06 frontier as the Next Task with no dispatch-ready lane.
`scripts/provider_route_matrix/cross_classification.py` now derives the
active generation from the index instead of hard-coding g05.

`docs/roadmaps/g05/**` (README plus 82 numbered tasks) was deleted last. No
product or runtime code, contract or architecture semantics, release state,
tag, or consumer repository changed.

## Current State

g06 opens with five planned tasks and no dispatch authorization: g06.001
awaits the producer proposal's independent review and canonical contract
promotion, g06.002-g06.003 stay behind the Research 256 gate, and
g06.004-g06.005 lack consumer requirement and operator direction. Chatterbox
resumes planning on the g06 frontier and settles g06's wider focus with the
operator.

## Validation

- `effigy qa:docs` and `effigy qa:northstar` pass in full at the head
- `effigy qa:routes` passes with the g06 producer-gap references
- lifecycle `verify` passes (0 fragments, receipt intact) and `tasks-digest`
  reproduces the closure digest from the receipt
- no remaining Markdown link from front doors, `docs/logs/`, or
  `docs/research/` resolves into `docs/roadmaps/g05/`
- branch: `ns-2a98cdd4-c8e9-4814-85a9-52c6c4c1d78d`; worktree clean before
  closeout

## Next

Chatterbox reconciles the carried-forward set, compiles the next ready lane,
and settles g06's wider focus with the operator. No dispatch, release, tag,
publication, live-provider, or consumer authority follows from this
closeout.
