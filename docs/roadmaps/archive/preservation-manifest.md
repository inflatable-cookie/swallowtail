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
