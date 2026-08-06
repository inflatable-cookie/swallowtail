# Tagged CI Anthropic Cancellation Repair

Date: 2026-08-06
Roadmap: g03.043
Card: 131

## Outcome

The first manually dispatched `v0.1.0` workflow exposed an Anthropic Managed
Agents cancellation test race under full runner contention. Cancellation and
deadline use the same remote interruption and cleanup mechanics, but Contract
009 requires accepted consumer cancellation to remain `Cancelled`.

The runtime now wakes the attachment pump directly when cancellation is
accepted, polls cancellation before stream and deadline readiness, and checks
accepted cancellation again when a deadline signal is classified. A direct
regression makes simultaneous readiness deterministic.

The first repair CI run proved a second boundary: the test's five-second
operation deadline included managed environment and session setup. Under full
workspace contention, setup could consume the budget before the test requested
cancellation. The two cancellation tests now use an explicit 30-second
deadline. Other managed fixtures retain their five-second bound; the direct
arbitration regression retains coverage of the production race.

## Remote Evidence

- failed tag run `31101523861` at release commit
  `a8bef72b718d3d9e503da48b3af05da4b674d4ec`
- failed first-repair run `31102764319` at
  `3b960b30b3e1bad01df2c06e00f067e361093eab`
- passing repair run `31103299936` at
  `4ffbd8f8a5302b9ce31ee37687876fcab8661f58`
- passing jobs: stable format/lint/test/guides, Rust 1.90, Bedrock Rust 1.94.1,
  documentation/API, supply-chain, and external source consumer
- `v0.1.0` remains an annotated immutable tag at the original release commit

## Local Validation

- deterministic simultaneous cancellation/deadline regression: passed
- former driver and prepared-facade failures: passed in ten repeated
  concurrent runs
- `effigy validate:focused swallowtail-adapter-anthropic`: 58 passed
- `effigy package:verify-affected swallowtail-adapter-anthropic`: passed
- workspace nextest: 1,464 passed; 11 skipped
- complete Rust 1.90 non-Bedrock workspace tests: passed
- focused clippy, formatting, docs QA, and diff checks: passed

## Boundaries

No tag moved or was recreated. No new release, registry publication, GitHub
Release, consumer mutation, live provider call, or authenticated provider work
ran. A future `0.1.1` release remains a separate operator decision.
