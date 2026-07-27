# 055 OpenCode Deletion Range Corpus

Status: completed
Owner: Tom
Created: 2026-07-26
Updated: 2026-07-27
Milestone: `../018-opencode-session-deletion-proof.md`

## Objective

Add OpenCode session deletion to the exact recursively frozen selected surface
across `1.14.48..=1.18.4`.

## Governing Refs

- Research 036
- Contracts 014, 029, and 038
- Research 027
- tagged OpenCode OpenAPI schemas

## Scope

1. Follow the delete operation and every local schema reference at all 45
   published versions in the maintained range.
2. Add deletion-specific behavior revisions only where the selected closure
   changes.
3. Freeze success body, missing target, active target, child-session,
   authentication, and server-error behavior available from primary evidence.
4. Preserve unpublished gaps, prerelease rejection, latest-qualified, and
   unverified-newer posture.
5. Record exact commits, dates, full and selected hashes, and exclusions.

## Acceptance Criteria

- [x] every qualified point has complete selected delete-schema closure
- [x] unpublished versions stay outside all segments
- [x] provider-declared data deletion is not upgraded to hard erasure
- [x] child or descendant semantics are explicit
- [x] current documentation is not projected backward without tagged evidence
- [x] the existing six-route behaviors remain unchanged

## Validation

- deterministic schema extraction and hash check
- compatibility corpus tests
- selected-surface uniqueness checks
- `git diff --check`

## Stop Conditions

- a required tagged schema is missing or contradictory
- runtime behavior cannot be inferred safely enough for a deletion claim
- adding delete would shrink the existing range

## Auto-Continuation

No until range evidence is reviewed. Then make card 056 ready.

## Outcome

Research 039 and the separate deletion corpus freeze all 45 exact tagged
releases without changing the six-route production claim.

- two delete-schema closures split at `1.15.6`
- eight exact deletion segments preserve unpublished and cross-minor gaps
- two runtime evidence revisions record background-job cancellation from
  `1.14.51` without inventing a provider busy guard
- success is `ProviderDataDeleted` with
  `ProviderDefinedDescendants`, never hard erasure
- missing targets reject with 404; 5xx or transport loss after dispatch leaves
  provider truth unconfirmed
- Basic authentication and raw-body diagnostic exclusions remain explicit
- `1.18.5` stays visible unverified-newer rather than entering the guaranteed
  range

Card 056 is ready.

## Validation Evidence

- exact tagged extraction: 45 full OpenAPI digests matched; two delete
  closures
- OpenCode adapter: 44 tests passed; one installed probe skipped
- Rust check, format check, docs QA, Northstar QA, and diff check passed
- Effigy doctor retained the known 25-findings baseline
