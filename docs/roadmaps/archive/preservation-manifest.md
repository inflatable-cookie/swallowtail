# Preservation Manifest — g05.038 Flattened Task Switchover

Frozen before any deletion. Canonical task:
`docs/roadmaps/g05/038-flattened-task-switchover.md`.
Planning base `fac7fbc78fd030313fbf615896001782bb07b573`; HEAD == origin/main;
no open PRs; no unfinished Swallowtail old-format queue record. Desktop
Card 323 continues untouched (owns no Swallowtail path).

## Classification

- g01: safely closed. Generation-index: closed at 49 roadmaps
  (48 completed, one backlog move). Front-door status completed.
- g02: safely closed. Generation-index: closed at 49 roadmaps.
- g03: safely closed. Generation-index: closed at 106 roadmaps
  (operator-authorized rollover).
- g04: safely closed. Generation-index: closed at 91 roadmaps
  (76 completed, fifteen honest evidence stops, no ready milestone).
- g05: active. Sole expanded generation after migration.
- No unresolved generation. No explicit parallel active generation.

## Old/new ID map (active commitments)

| Former card | New Northstar task | State carried over |
| --- | --- | --- |
| 005 Qoder effective skill visibility binding | g05.039 | planned; gated on non-empty Research 256 disposition |
| 006 Qoder skill visibility acceptance | g05.040 | planned; serial after g05.039 |
| 130 Persistent permission grant admission | g05.041 | planned backlog stub; no dispatch without operator promotion |
| 134 Registered-tool adoption, remaining ACP routes | g05.042 | planned backlog stub; no dispatch without consumer requirement |

Other gated follow-ons (063 behind g05.026, 072 behind g05.027) already have
an honest stopped parent and do not become duplicate executable tasks. Stopped
cards (149, 132, 123, 087) stay stopped under their owning task; cancelled
card 123 stays cancelled under g05.036. Completed cards (150 of 155) are
closed history: outcomes live in `docs/logs/` closeouts, the generation-index
census, owning-task narratives, and git history.

## Unique authority destinations

No live rule exists only inside a removed tree. Durable rules already live in
`docs/contracts/` and `docs/architecture/`; the four roll-ups map each
generation's lasting decisions to those canonical destinations. The two
g04 per-route programme files move byte-identical into the archive (same
relative depth, so relative links behave identically). The three g01
downstream adoption handoffs (Status: prepared, Nucleus/Soundcheck-owned)
move byte-identical into the archive and are referenced from the g01 roll-up.

## Open commitments and new homes

- Cards 005/006 scope → g05.039/g05.040 (full contract preserved).
- Cards 130/134 stubs → g05.041/g05.042 (full contract preserved).
- g02.029 plus cards 097-098 → `docs/roadmaps/backlog/pi-rpc-session-continuity.md`
  and `provider-session-management-binding-persistence.md` (already there).
- g03 backlog moves (Aider, Kiro, OpenHands wiring) → backlog files
  (already there).
- g04 blocked follow-on cards → closed evidence with the generation;
  programme state re-audited in g05 (Contract 061 census).
- g04 two parked Bedrock items → closed with the generation; no backlog file
  exists and none is invented; re-audit in g05 only if a consumer requires it.

## Selected material evidence (kept in roll-ups, not re-proved)

Release tags `v0.1.0`/`v0.1.1`/`v0.2.0`/`v0.3.2`/`v0.4.0`–`v0.4.3`,
candidate `49c9e3b2`, per-generation PR/merge SHAs, and validation records
copied from the generation READMEs and census into the four roll-ups.

## Exact paths to remove

- `docs/roadmaps/g01/` — entire tree (49 numbered, README, batch-cards/),
  EXCEPT the three adoption handoffs moved to
  `docs/roadmaps/archive/g01-nucleus-adoption-handoff.md`,
  `g01-nucleus-task-execution-handoff.md`, `g01-soundcheck-adoption-handoff.md`.
- `docs/roadmaps/g02/` — entire tree (numbered, README, batch-cards/).
- `docs/roadmaps/g03/` — entire tree (numbered, README, batch-cards/).
- `docs/roadmaps/g04/` — entire tree (91 numbered, README, batch-cards/),
  EXCEPT `per-route-feature-completion.md` and
  `per-route-feature-inventory.md`, moved byte-identical to
  `docs/roadmaps/archive/`.
- `docs/roadmaps/g05/batch-cards/` — entire tree (155 cards + README).
- `docs/specs/templates/batch-card-template.md` — legacy template, replaced by
  `docs/roadmaps/templates/task-template.md`.

## Current links pointing at classified sources (all rewritten in this batch)

- `docs/roadmaps/README.md`: g01–g04 generation links, per-route g04 links,
  status-grammar note, generation-shape note.
- `docs/roadmaps/generation-index.md`: g05 census only (g01–g04 census
  paragraphs are frozen history and stay byte-identical).
- `docs/roadmaps/g05/README.md`: full rework to task index + frontier.
- `docs/roadmaps/g05/001-037*.md`: Batch Cards sections, card-dispatch verbs,
  batch-cards/ links, stale ready-card claims.
- `docs/roadmaps/backlog/grok-build-maintained-acp-range.md`: evidence links
  into the removed g01 tree (already dangling) → archive roll-up.
- `docs/guides/provider-route-matrix.md` + `provider-solution-feature-matrix.csv`:
  batch-card rule wording; cross_refs 130/134 → tasks 041/042.
- 37 `docs/research/` + `docs/logs/` files: dead link targets repointed only
  (g05 cards → owning task file; g04 paths → `archive/g04.md`); prose untouched.
- `effigy.toml` docs_policy; `scripts/check-roadmap-status-drift.py`,
  `scripts/check-roadmap-number-collision.py`,
  `scripts/provider_route_matrix/cross_classification.py` and their fixtures.
- `docs/contracts/001-working-rules.md`, `AGENTS.md`, `docs/specs/README.md`,
  `docs/handoffs/README.md`, `docs/guides/provider-route-matrix.md`,
  `docs/roadmaps/standing-lanes.md`, `docs/roadmaps/long-term-plan.md`,
  `docs/roadmaps/status-grammar.md`, `docs/roadmaps/backlog/grok-build-maintained-acp-range.md`.

## Retained exceptions (deliberate, not defects)

- Past-tense card evidence citations stay wherever they attribute frozen
  outcomes (task narratives, census evidence prose, standing-lane claim
  attributions, guides, contracts, release notes). Rewriting them to task IDs
  would falsify which unit did the work.
- `docs/logs/`, closed `docs/handoffs/`, immutable queue records, old specs,
  release notes including `docs/releases/README.md`, and archive prose are
  not modernized.
- `Card129`/`Research 290`/`290-*.tsv` cross-classification identifiers stay:
  they are frozen validator vocabulary, not planning dispatch.
- Frozen planning-history pointers (`this card's ## Result`, manifest serial-edge
  rows, resolved runway conditionals, `in the same card` tails) name retired
  card files; their destinations live in git history and the folded ledgers.
  Resolving them into new task edits would be re-planning, which this migration
  does not do.

---

# Preservation Manifest — g05.084 Roll Over To g06 And Compact g05

Frozen before any deletion. Canonical task:
`docs/roadmaps/g05/084-roll-over-to-g06-and-compact-g05.md` (its own record is
folded into [g05.md](g05.md) and `docs/logs/` before removal). Planning commit
`c59ce0bf769f64ffccfd32fda58c6e4ef5cb6c25`; dispatch base is pushed `main` at
`1d57cf28`. Operator rollover authority: Tom, 2026-09-17, "roll over and
compact" — documentation and planning surfaces only.

## Classification

- g05: safely closed. All 26 dispatched lifecycle records terminal
  (`status: complete`, including the four serial prerequisites g05.066,
  g05.069, g05.080, and g05.081); the exhausted runway is `planning_required`,
  closed only by the explicit operator rollover authority. Closure record:
  `.northstar/lifecycle/v1/generations/g05.closure.json` with tasks digest
  `sha256:cfe29cbb46d182207cc1d60b2701301c1e27a0ccc8a093ab03976b96ee69754f`
  over the 26 terminal fragments; receipt `g05.json` published on compaction.
- g06: active after the rollover; sole expanded generation.
- No unresolved generation. No explicit parallel active generation.

## Carry-forward map (open commitments)

| g05 task | g06 task | State carried over |
| --- | --- | --- |
| 035 Shared Harness Capability And Producer Boundary | g06.001 | planned; Batch A promoted; folded card evidence and held gates intact |
| 039 Qoder Effective Skill Visibility Binding | g06.002 | planned; gated on a non-empty Research 256 deliver-now disposition |
| 040 Qoder Skill Visibility Acceptance | g06.003 | planned; serial after g06.002 |
| 041 Persistent Permission Grant Admission | g06.004 | planned; operator promotion absent; no dispatch authorization |
| 042 Registered-Tool Adoption For Remaining ACP Routes | g06.005 | planned; consumer requirement and operator direction absent |

Everything else was terminal before this task ran. Gated follow-ons that stay
with their stopped parents (Kimi local-server claim behind g05.026,
Antigravity headless claim behind g05.027, Contract 061 Kimi gate behind
g05.009) keep those parents as their honest stopped record; they do not become
duplicate executable tasks. g05.066's live-tuple follow-on (bind the Desktop
runner to `0.3.270`/`2.1.270` before any further live operation) is recorded
in [g05.md](g05.md) — it requires a future separately authorized consumer
update, so it is not carried as an executable task; the claude-agent.sdk
feature-matrix cells cite the held real-route gate at g06.001.

## Unique authority destinations

No live rule exists only inside the removed tree. Durable rules already live
in `docs/contracts/` (058-063, 019 amendments) and `docs/architecture/`;
[g05.md](g05.md) maps the generation's lasting decisions onto those canonical
destinations. The lifecycle closure record and receipt keep the terminal
record set verifiable after fragment removal.

## Selected material evidence (kept in the roll-up, not re-proved)

Annotated source tags `v0.4.0`–`v0.5.1` with their CI runs, the Research
308-328 currentness campaign with PRs #321-#330, the Contract 061
767-row census, PR 196 at `493f8194`, Research 301 and 329, and the
per-task PR/review/merge identities recorded in `docs/logs/` closeouts.

## Exact paths removed

- `docs/roadmaps/g05/` — entire tree: README plus 82 numbered task files
  (001-081, 084; 082-083 were never created), after the g06 tree, the
  roll-up, the manifest section, the closure record, the receipt, and the
  front-door projections existed and validated.
- `.northstar/lifecycle/v1/tasks/g05.NNN.json` — all 26 terminal fragments,
  consumed into `.northstar/lifecycle/v1/generations/g05.json`.

## Reference rewiring

`docs/roadmaps/README.md`, `docs/roadmaps/generation-index.md`,
`docs/roadmaps/status-grammar.md`, `docs/roadmaps/standing-lanes.md`,
`docs/README.md` (generated projection), `effigy.toml` (roadmaps_g06 index
policy), `scripts/check-roadmap-status-drift.py` (exempt list),
`scripts/provider_route_matrix/cross_classification.py` (active-generation
derivation), `docs/guides/provider-route-matrix.md`,
`docs/guides/provider-solution-feature-matrix.csv` (producer-gap refs →
g06.001/g06.004/g06.005), and `docs/specs/014-shared-harness-capability-and-
producer-boundary.md` were updated. Historical `docs/logs/` and
`docs/research/` markdown links into the removed tree were retargeted to
[g05.md](g05.md); closed handoffs stay historical.
