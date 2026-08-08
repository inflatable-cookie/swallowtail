# 052 Shared Adapter Scaffolding

Status: done
Owner: Tom
Created: 2026-08-08
Generation: g03
Depends on: g03.051
Vision tags: maintainability, provider-neutral core, regression safety
Contract refs: 011, 029, 037
Planning state: cards 156-160 completed

## Problem

A verified deep audit measured roughly 15-18K of the 136.5K adapter lines
(11-13%) as near-identical copies across at least two adapters, concentrated
in provider-neutral module families:

| Family | Reach | Approx identical lines |
| --- | --- | --- |
| discovery probe (`--version` parse + classify + outcome) | 11 adapters | ~1,480 |
| prepared plan construction (`instance_with_capabilities`, `requirements`, `build_plan`) | 16 adapters | ~1,515 |
| turn/session/pump run loops | 36 files | ~3,509 |
| failure stage mappers | 32 files | ~493 |
| curl transport wrappers | 13 adapters | ~884 |
| ACP event-to-activity projection | 5 adapters | ~820 |
| catalogue parse/paginate | 30 files | ~2,320 |
| lifecycle locks and prepared wrappers | 18-20 files | ~1,966 |

Whole-crate pairs are up to 42% identical (pi vs oh-my-pi). Every copy
duplicates the same bug-fix surface; the 11 discovery copies currently wait
for the same fixes. The duplication is provider-neutral scaffolding that
belongs in `swallowtail-runtime` or `swallowtail-testkit` behind the
AGENTS.md provider-neutrality boundary; provider-specific parts (version
regexes, claim constants, request builders) stay adapter-local.

## Goals

- [ ] extract provider-neutral probe, plan, failure-mapping, and run-loop
      helpers into shared crates
- [ ] extract the ACP activity projector
- [ ] migrate adapters in bounded tranches with identical public behavior
- [ ] prove behavior parity through the existing focused and extracted-package
      gates

## Execution Plan

- [x] Execute card 156 (shared probe, binding-parse, and stage helpers;
      pi and oh-my-pi pilot).
- [x] Execute card 157 (prepared plan builder extraction).
- [x] Execute card 158 (run-loop scaffold and ACP activity projector).
- [x] Execute card 159 (transport and catalogue wrapper consolidation).
- [x] Execute card 160 (provider-wide migration and acceptance).

## Closeout

Measured at card 160: 12 duplicated discovery-probe machinery files reduced
to 4, all recorded as intentionally adapter-local (antigravity and cursor
staged outcome codes, codex ProcessExit-class staging with stderr
sanitization, grok stderr capture beyond the shared cap). The shared
scaffold lives in `swallowtail-runtime/src/installed_discovery.rs` (465
lines: codes macro, exact-version parse, bounded probe, outcome codes).
Remaining duplication targets (prepared plan, run loops, transport,
catalogue, projector, lifecycle locks) stay recorded as operator-level
topology decisions for the suite planning checkpoint after g03.053.

## Boundaries

- no public API, diagnostic-code, behavior, or version-range change
- no provider-neutral vocabulary acquires provider or consumer dependencies
- provider-specific parsing, claims, and request builders remain adapter-local
- no tag, release, registry publication, or live provider work

## Acceptance Criteria

- [ ] shared helpers exist in `swallowtail-runtime` or `swallowtail-testkit`
      with their own focused tests
- [ ] every migrated adapter passes focused, affected-package, and
      extracted-package proof with an unchanged public API baseline
- [ ] the adapter duplication families shrink by the measured amounts
- [ ] at least one full migration tranche lands before the next tranche starts

## Next Planning Checkpoint

The suite planning checkpoint after g03.053: decide remaining duplication
targets and whether claim and surface consistency needs contract work first.
