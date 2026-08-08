# 054 Remaining Duplication Tranches

Status: done
Owner: Tom
Created: 2026-08-08
Generation: g03
Depends on: g03.053
Vision tags: maintainability, provider-neutral core, regression safety
Contract refs: 011, 029, 037, 039, 043-045
Planning state: cards 165-167 done; card 168 cancelled

## Problem

Cards 156-160 extracted the probe, plan, run-loop, transport, and catalogue
wrapper scaffolds and cut ~1,700 duplicated lines, but the measured families
still hold adapter-local copies that fall under the shared helpers:

| Family | Reach | 052 audit estimate | Shared scaffold |
| --- | --- | --- | --- |
| discovery failure stage mappers | 4 adapters (codex, cursor, antigravity, muse) | ~493 lines | `installed_probe_codes!` + `probe_outcome_failure` |
| catalogue parse/paginate | ~19 adapters | measured: ~60-line validation slice | disposed by card 159; shared slice not net-positive |
| ACP event-to-activity projection | 5 adapters | ~820 lines | projector scaffold (card 158) |

The g03.053 suite planning checkpoint committed to this order (operator,
2026-08-08): failure stage mappers, then catalogue parse/paginate, then ACP
event projection with a contract card first.

## Goals

- [x] migrate the discovery failure-stage mappers onto the shared code table
      or record them as intentional
- [x] record the catalogue parse/paginate disposition (card 159 measured
      evidence, confirmed by card 166)
- [x] record the ACP projection boundary (card 167); no migration
- [x] each tranche passes with an unchanged public API baseline

## Execution Plan

- [x] Execute card 165 (discovery failure stage mapper consolidation).
- [x] Execute card 166 (catalogue parse/paginate consolidation) —
      measured and disposed by card 159; re-measured and confirmed.
- [x] Execute card 167 (ACP activity projection contract) — operator
      decided the projections stay adapter-local; boundary recorded.
- [x] Execute card 168 (ACP activity projection migration) — cancelled by
      the card-167 decision.

## Closeout

Two of three checkpoint tranches measured-and-disposed rather than migrated,
because the 054 checkpoint ran on the 052 audit estimates and the measured
completion evidence from cards 158-159 already covered them:

- failure stage mappers: migrated (card 165) — the codex and cursor code
  tables now reference the shared `installed_probe_codes!` table
- catalogue parse/paginate: confirmed disposed (card 166) — the shared
  slice is the ~60-line validation family; extraction needs a `serde_json`
  dependency in runtime or per-call code plumbing, net-negative
- ACP event projection: operator chose adapter-local (card 167) — no shared
  home exists under the recorded topology; card 168 cancelled

The generation returns to its evidence gate with the remaining duplication
families measured and their dispositions recorded.

## Boundaries

- no public API, diagnostic-code, or behavior change
- provider-specific parsing and payload decoding stay adapter-local
- no tag, release, registry publication, or live provider work

## Acceptance Criteria

- [ ] every mapped or recorded stage family lands with identical outcomes
- [ ] catalogue parse/paginate shares one core with adapter-local decoders
- [ ] the ACP projections share the pinned contract
- [ ] the generation returns to its evidence gate after the tranches

## Next Planning Checkpoint

The g03 evidence gate after the tranches: decide whether the generation rolls
or extends before the v0.3.0 release step.
