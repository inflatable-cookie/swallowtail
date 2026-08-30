# 2026-08-31 g05.007 Card 020 Linux Envelope

Status: ready
Owner: Tom
Milestone: g05.007
Card: 020
Contracts: 044, 059, 060

## Decision

The operator selected the `linux-x86_64` host for the still-unconsumed card 020
turn. The exact installed Claude Code version remains `2.1.251`, the exact model
remains `claude-haiku-4-5`, and the frozen official `linux-x64` native SHA-256
is
`fd5f10ff0eb58daec04900466b143ea98aab50abf208a422bc008eaec13f61f7`.
No Darwin dispatch is authorized.

The first card 020 worker stopped before contact because the probe and original
envelope froze Research 261's `darwin-arm64` digest. No request reached Claude,
so the one-turn budget was not consumed. PR 127 is evidence of that pre-contact
finding, but the operator withheld its merge as a terminal stop.

## Repair Boundary

Card 020 now authorizes one bounded pre-contact probe repair. The worker must
select the frozen native digest by the actual target platform, preserve the
existing `darwin-arm64` value, add the frozen `linux-x64` value, and reject
unsupported targets. Credential-free proof must show that neither supported
platform accepts the other platform's digest and that no fallback exists.

The live run starts only from the clean committed repair head after every named
pre-contact validation row passes. This decision does not authorize production
changes, a prompt or lifecycle-oracle change, another model, API-key billing,
fallback, a second turn, or an automatic rerun.

## Prerequisites Cleared

- `Cargo.lock` moves `chacha20` from yanked `0.10.1` to non-yanked `0.10.2`.
- The exact required `cargo-public-api 0.52.0`, pinned
  `nightly-2026-08-05`, and Debian `pkg-config` discovery are present on the
  selected host.
- `effigy package:verify-affected swallowtail-runtime swallowtail-host-local
  swallowtail-adapter-claude-agent swallowtail-testkit` passes for four
  packages.
- `effigy package:api` passes for all 40 packages and preserves the immutable
  public-API baselines.

These setup actions did not contact Claude or consume the turn.

## Dispatch And Review

g05.007 remains a serial single-owner lane. One fresh manual worker handoff may
execute card 020. No overlapping worker may edit the watcher probe, card,
milestone, outcome log, or claim surfaces.

The worker returns one reviewable PR and never merges it. Under the operator's
standing instruction, Helm may merge that later PR only after the orchestrator
reviews the exact head and posts a merge-authorized verdict after required
checks pass. A withheld verdict means no merge. PR 127 remains excluded.
