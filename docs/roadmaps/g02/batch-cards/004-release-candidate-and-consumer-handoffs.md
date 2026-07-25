# 004 Release Candidate And Consumer Handoffs

Status: completed
Owner: Tom
Created: 2026-07-24
Milestone: `../001-release-boundary-and-package-readiness.md`

## Governing Refs

- Contract 036
- release and package topology architecture
- completed card 003 package evidence

## Objective

Prepare one non-published release candidate and exact consumer upgrade
handoffs, then return publication and tagging to the operator.

## Scope

1. Freeze the selected package versions, contents, dependency order, checksums,
   changelog, and release notes.
2. Prove clean consumer builds against the candidate through isolated fixtures
   or separately authorized consumer validation.
3. Write exact Nucleus and Soundcheck upgrade, rollback, and compatibility
   handoffs without editing either repository by default.
4. Produce an Effigy release plan only if the manifest and contract support it.
5. Stop before registry upload, tag creation, push, or release execution.
6. Treat crates.io account, owner, team, token, and final name availability as
   external release-gate state, not package-preparation prerequisites.

## Acceptance Criteria

- [x] candidate package evidence is immutable and reproducible
- [x] consumer handoffs name exact versions, validation, rollback, and known
      compatibility limits
- [x] release notes distinguish crate compatibility from provider ranges
- [x] no live credential or registry mutation occurs
- [x] one explicit operator release decision remains
- [x] Apple Silicon macOS is reported as verified and other targets as
      unverified, not prohibited

## Validation

- full contract-selected release gates
- candidate package checksum and content audit
- isolated consumer compatibility evidence
- `effigy qa`
- `effigy doctor` delta review
- `git diff --check`

## Stop Conditions

- candidate evidence changes after consumer validation
- a consumer needs an unplanned compatibility shim
- package ownership or release credentials are unresolved
- publication would require an unauthorized external mutation

## Auto-Continuation

No. Registry publication, tags, pushes, and release execution require explicit
operator approval.

## Outcome

The first coordinated `0.1.0` candidate freezes all 23 packages in the
Contract 036 publication order. A retained Git bundle, archive checksums,
audited file-list checksums, and an independent rebuild prove the local
candidate from one deterministic clean source commit.

Temporary copies of Nucleus and Soundcheck resolve exact `=0.1.0`
requirements through the four candidate archives they consume. Their original
repositories and lockfiles remain untouched. Separate handoffs name upgrade,
validation, rollback, target, toolchain, and Codex-interface limits.

Effigy's generic release planner cannot resolve a release version from this
virtual staged workspace. The candidate therefore uses explicit
Contract 036 package stages rather than forcing a single-package release
model. This is a known planning limit, not publication authority.

No registry, credential, owner, team, tag, push, release, workflow, or consumer
mutation occurred. The operator must review the frozen evidence and explicitly
authorize any registry preflight or publication work.
