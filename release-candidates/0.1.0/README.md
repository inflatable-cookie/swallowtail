# Swallowtail 0.1.0 Candidate Evidence

The files beside this README are generated from the retained candidate:

- `candidate.env` — version, registry, canonical source commit, exact parent,
  source scope, target, and toolchains
- `publication-order.tsv` — exact three-stage package DAG
- `packages.sha256` — all 23 archive checksums
- `package-files.sha256` — audited archive-list checksums
- `evidence.sha256` — source-bundle and evidence checksums
- `provider-validation.env` — all-route packaged prepared-facade proof
- `provider-validation.sha256` — provider proof checksum
- `consumer-validation.env` — isolated Nucleus and Soundcheck proof
- `consumer-validation.sha256` — consumer proof checksum

Binary artifacts remain in `.effigy/release-candidates/0.1.0/`.

This candidate is held for application-scale Nucleus and Soundcheck evidence.
It must not be published from package and isolated-consumer proof alone.

`effigy package:candidate:verify` rebuilds the package family from the retained
source bundle and compares archive and file-list hashes.

The candidate source is a clean non-root commit in local canonical history.
Its bundle preserves complete history. Generated evidence remains outside that
source commit.

The exact source is
`f142d927767f49fe86f2737d822fecf182f52591`, with parent
`e9ead4d35fb7754962053417bf8328e646839b32`. The package-manifest digest is
`59f9541cffc97467bb0c7e39e005fcea1cb9c0ace485856f3f9cffd4440da6d4`.

Provider evidence runs every production route without credentials, installed
providers, or provider calls. Consumer evidence records deterministic source-
snapshot commits for both consumers, the candidate package-set checksum, and
Soundcheck's locked compile-time asset input. It does not mutate either
consumer.
