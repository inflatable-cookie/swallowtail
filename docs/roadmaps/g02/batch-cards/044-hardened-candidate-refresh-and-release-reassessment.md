# 044 Hardened Candidate Refresh And Release Reassessment

Status: completed
Owner: Tom
Created: 2026-07-25
Completed: 2026-07-26
Milestone: `../014-consumer-scale-application-proof-and-hardening.md`

## Objective

Freeze a corrected local candidate after application-scale evidence, then
reassess release readiness without publishing.

## Entry Gates

- cards 041-042 complete
- card 043 explicitly superseded with its Soundcheck integration, readiness,
  and defect evidence retained
- all accepted Swallowtail defects closed or explicitly held
- candidate source clean and in normal local history

## Scope

1. Classify public API and guaranteed-behavior changes from the current
   `0.1.0` baseline.
2. Rebuild all 23 packages and all 22 packaged route proofs.
3. Re-run deterministic packaged Nucleus and Soundcheck consumer proofs;
   reconcile adapter, consumer-backend, and vertical-smoke evidence without
   repeating UI or provider workloads.
4. Supersede the current candidate without deleting it.
5. Update release notes, compatibility risks, rollback, and currentness.
6. Stop before push, publication, tag, GitHub release, workflow, or owner
   mutation.

## Acceptance Criteria

- [x] exact corrected source, parent, archives, and evidence reproduce
- [x] package, provider-route, isolated-consumer, scenario-harness, and
      vertical-smoke evidence reconcile
- [x] remaining provider-auth, protocol, capability, topology, and scale risks
      are explicit
- [x] only one local candidate is active
- [x] publication remains a fresh bounded operator decision

## Change Classification

The immediate `f142d927` baseline and corrected `5326e6f` candidate have
identical public declaration hashes across all 23 crates. No crate version or
provider-interface claim changes.

The guaranteed-behavior delta is corrective:

- Codex prepared tool bounds derive from actual bounded declarations
- Codex prepared sessions include the promised time service
- non-zero Codex version probes expose safe exit detail under the same stable
  diagnostic code

## Evidence

The exact source, parent, archive, bundle, provider, and consumer digests are
retained in `release-candidates/0.1.0/` after the source freeze.
- 23 packages reproduce from the complete source bundle
- 20 packaged facade suites cover all 22 production routes
- Nucleus passes 15 deterministic tests; 2 installed/live probes stay ignored
- Soundcheck passes 6 deterministic tests; 1 installed/live probe stays
  ignored
- the packaged Codex suite passes 93 tests
- no credential, provider call, consumer edit, push, tag, registry, workflow,
  owner, or release mutation occurred

The pre-hardening binary candidate is retained at
`.effigy/release-candidates/superseded/0.1.0-f142d927767f/`.

## Risks And Reassessment

- hosted authentication and installed-runtime probes remain gated
- only Apple Silicon macOS and the declared/current Rust toolchains are
  verified
- unverified-newer provider interfaces remain mileage-may-vary
- isolated consumer proofs bind synthetic snapshots rather than canonical
  consumer release commits
- consumer command-boundary harness coverage is not yet uniform

The candidate passes the current technical release gates. Publication is not
recommended yet because the operator has chosen continued working-application
soak before crates.io. This candidate is the exact baseline for that soak.

## Auto-Continuation

No. Return the evidence and recommendation. Do not infer publication authority.
