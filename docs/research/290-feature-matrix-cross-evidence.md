# Research 290 — Card129 Feature Matrix Cross Evidence

Status: complete; evidence ledger; no live/provider claim
Owner: Tom
Date: 2026-09-07
Card: g05.035 / 129

## Purpose

This is the frozen evidence ledger for Card129's `provider_limitation` cells.
It does not add runtime support or promote a provider claim. Each TSV row
names one matrix row and unavailable feature, records the unavailable finding,
and points to an anchored line in the accepted Research 281 route-behavior
ledger. Research 281 is the frozen route and feature-matrix census; Card129
keeps the cell-level cross reference explicit so a guide-only or unanchored
citation cannot pass the route check.

## Machine contract

The route-matrix checker requires every provider limitation to reference one
`#L<line>` row in [the TSV ledger](290-feature-matrix-cross-evidence.tsv). The
row must match the matrix `route_id`, feature, and kind, and its `finding` must
start with `qualified route unavailable:`. The ledger's `basis` must itself be
an anchored line in Research 281 under `docs/research` or `docs/contracts`.
Prepared integration guides are not evidence for this disposition: the docs
front door classifies guides as non-authoritative integration usage.

Producer gaps use an existing non-complete g05 batch card. The matrix notes
carry a `Card129 producer-gap reasons:` marker with one non-empty reason for
each producer-gap feature. A withheld value follows the same producer-gap
rule; it is never a third classification.

No live provider, credential, consumer, server, tag, release, or runtime
operation was used to produce this ledger.
