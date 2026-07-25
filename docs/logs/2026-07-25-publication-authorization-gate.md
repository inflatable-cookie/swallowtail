# Publication Authorization Gate

Date: 2026-07-25
Roadmap: g02.012 closeout
Candidate: `0.1.0`

## Outcome

The package candidate was reproducible, but the publication lane entered
`strict-paused` after read-only inspection found that its synthetic source
commit was parentless. It could not be fast-forwarded onto canonical `main`.

That source shape was sufficient for isolated package proof. Publishing it
would require an orphan release tag while the public default branch remained
at the pre-candidate base. That is avoidable release debt.

Recommendation:

1. promote a final-publication rule requiring a clean source commit in
   canonical repository history
2. materialize the accepted candidate tree as a normal commit on `main`
3. rebuild and reverify the candidate from that exact commit
4. return the new checksums, crates.io owner identity, staged upload set, tag,
   and GitHub release for explicit authorization

## Read-Only Evidence

- the candidate bundle verifies as complete and names source commit
  `73c7f5b5b5611ef20bdcc1572deeb39ca50630e1`
- that commit has no parent; candidate base
  `91a0774010ee83594a4565e1b4e2b0daa998db28` is recorded separately
- the bundle and all 23 archives remain outside the dirty working tree under
  `.effigy/release-candidates/0.1.0/`
- all 23 exact crates.io package lookups returned `404` on 2026-07-25; this is
  current absence, not reservation or ownership
- origin is `git@github.com:inflatable-cookie/swallowtail.git`
- local and remote `v0.1.0` tags are absent
- origin `main` is still the candidate base
  `91a0774010ee83594a4565e1b4e2b0daa998db28`
- the working tree has 518 changed paths and must not be the publication source
- the current candidate builder always creates a new deterministic root
  snapshot, even when retaining a release candidate
- Effigy's generic release gate cannot infer a version from this virtual
  workspace; the contract-recorded three-stage package flow remains the
  authority

The crates.io account, token, owner, and team state were not inspected.
Contract 036 requires the operator to name the owner identity before the first
permanent publish.

## Failure Rules

- publish sequentially inside each stage to keep failure state obvious
- after any error or timeout, check crates.io before retrying
- do not continue to a dependent stage until every prerequisite is visible
- stop on checksum, source, owner, registry, version, or tag drift
- never rebuild from the live working tree during publication

## Decision Required

The operator accepted canonical-history provenance through the next-task
continuation.

Roadmap g02.013 tightened Contract 036 and the release topology, added a
clean-HEAD final-candidate mode, materialized the accepted tree locally on
`main`, and rebuilt the candidate. The later publication decision must name the
crates.io owner identity and explicitly authorize the desired external mutation
set.

Roadmap g02.013 is complete. The active candidate uses the exact normal local
`main` source and parent recorded in `release-candidates/0.1.0/candidate.env`
and passes final packaged acceptance. The provenance pause is resolved.

Publication remains blocked until the operator names the exact crates.io owner
and authorizes the bounded external mutations.
