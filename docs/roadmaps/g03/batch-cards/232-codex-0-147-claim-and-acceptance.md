# 232 Codex 0.147.0 Claim And Acceptance

Status: completed
Owner: Tom
Created: 2026-08-17
Milestone: `../072-recurring-version-currentness-and-codex-0-147.md`
Depends on: card 231

## Goal

Apply the card 231 segment decision to the Codex claims, prove focused
package evidence, and close g03.072 without touching other 127 families.

## Scope

1. Extend or milestone `codex.exec` and `codex.app-server` through exact
   `0.147.0` only if card 231 named that shape.
2. Update matrices, Codex guide version text, and probes that still encode
   `0.146.0` as the latest qualified point.
3. Keep later stables visible unverified newer.
4. Close the milestone and return to the currentness checkpoint.

## Out Of Scope

- other Research 127 families
- moving the Codex baseline
- provider prompts, install, update, version bump, tag, or publication

## Acceptance Criteria

- [x] installed `0.147.0` classifies as qualified, or the bound stays
      `0.146.0` with an explicit stop
- [x] later stable Codex versions remain permitted and visibly unverified
- [x] existing gaps and exclusions stay intact
- [x] focused and extracted-package Codex proof pass if the claim moves
- [x] public route truth matches the claim

## Validation

- `effigy validate:focused swallowtail-adapter-codex`
- `effigy package:verify-affected swallowtail-adapter-codex`
- `effigy qa:routes` if matrices change
- `effigy qa:docs`
- no broad workspace suite

## Stop Conditions

- stop if card 231 did not name a segment shape
- stop if live provider work would be required to close the claim

## Auto-Continuation

No. After closeout, reassess remaining Research 127 families one at a time.
Do not start Grok `1.0.x` inside this card.

## Evidence

- Research 128
- `CODEX_LATEST_QUALIFIED_VERSION` = `0.147.0`
- same behavior revisions as `0.146.0`
- synthetic later-stable UnverifiedNewer is now `0.148.0`
