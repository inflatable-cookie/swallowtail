# Route Readiness Decisions And g04 Initial Run

Date: 2026-08-19

## Decision

The connection facade may observe a provider-disclosed authenticated subject
(email, login, or plan) as a restricted, redacted-by-default claim. Reveal is
consumer presentation. Contract 047 still forbids account identifiers in the
selection snapshot.

Sign-in is library-max: start, poll, complete, cancel through host ports.
Persistence is a Swallowtail store port with an optional simple adapter.
Model hide, reorder, consumer-default, and favourite are a bound overlay on
exact catalogue identity.

Facade implementation waits for a current-source Git tag. Contract 036's
planning hypothesis is patch `v0.3.3` unless the release inventory finds a
break.

## Sequence

1. g04.001 inventory against Spec 011
2. g04.002 spec closeout and contract targets
3. g04.003 source tag
4. contract and implementation only after that tag

## Next

Map existing Swallowtail instance, access, discovery, and catalogue records
against the consumer connection-lifecycle surfaces.
