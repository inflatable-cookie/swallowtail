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

This candidate is the local post-hardening baseline. Nucleus vertical and
sustained evidence plus Soundcheck's distinct structured-run integration are
accepted. Publication remains operator-held for continued consumer soak.

`effigy package:candidate:verify` rebuilds the package family from the retained
source bundle and compares archive and file-list hashes.

The candidate source is a clean non-root commit in local canonical history.
Its bundle preserves complete history. Generated evidence remains outside that
source commit.

The exact source is
`f63a9a3653c7e17e795b06935b280a8fbbcf87cb`, with parent
`5326e6f4b24d7d05978b9bd4dc8407ccb3b9a565`. The package-manifest digest is
`e2037b0c931b0ecf37a18491f5c233324f02417987baea00c9b322802edfbfad`.

Provider evidence runs every production route without credentials, installed
providers, or provider calls. Consumer evidence records deterministic source-
snapshot commits for both consumers, the candidate package-set checksum, and
Soundcheck's locked compile-time asset input. It does not mutate either
consumer.

The pre-hardening `f142d927767f` candidate remains under
`.effigy/release-candidates/superseded/0.1.0-f142d927767f/`. The technically
passing `5326e6f4b24d` rebuild remains under
`.effigy/release-candidates/superseded/0.1.0-5326e6f4b24d/`; its packaged
README still described the refresh as pending, so it was not retained as the
active soak baseline.
