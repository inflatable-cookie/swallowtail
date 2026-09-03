# 050 v0.3.3 To Candidate Compatibility And Freeze Audit

Status: ready; unpaused after PR 196 merged at `493f8194`; sole Next Task
Owner: Tom
Created: 2026-09-02
Updated: 2026-09-03
Milestone: `../021-v0-4-0-release-readiness.md`
Depends on: Contract 036; Research 276; immutable `v0.3.3`

## Goal

Freeze the exact release census and classify every semantic API and
guaranteed-behavior delta from `v0.3.3` to one reviewed current-source head.

## Scope

1. Fetch canonical GitHub without changing source. Record canonical remote,
   branch, reviewed head, `v0.3.3` tag object and peel, merge base, commit count,
   changed-file count, and full `v0.3.3..HEAD` inventory.
2. Confirm the freeze has no open mergeable feature or currentness PR. Ignore
   only this planning/implementation release lane; stop on an overlapping head.
3. Reconcile the exact 40-package graph and every production release route at
   the resumed reviewed head with the immutable `v0.3.3` baselines. Immutable
   release inventory remains 47; wider current integration/route gates are 49
   due to `pi.sdk-sidecar` and `claude-agent.sdk`.
4. Generate current semantic Rust API evidence with the Contract 036 toolchain
   into temporary audit output. Compare all 40 packages with the immutable
   `public-api-0.3.3` baseline and the current unreleased candidate evidence.
5. Audit the complete source, changelog, contracts, architecture, guides,
   matrices, logs, and tests for guaranteed-behavior changes. Classify every
   removal, signature change, range shrink, route-identity change, verified-
   target change, MSRV change, lifecycle/cleanup/access/isolation/evidence
   weakening, and additive compatible surface. Do not limit the audit to the
   known OpenAI Background `minimal` removal or the caller-bounded interactive
   session close signature.
6. Produce a package-by-package semantic API ledger and route/guaranteed-
   behavior ledger. Each changed item names evidence, compatibility class,
   consumer effect, changelog/release-note coverage, and required upgrade or
   rollback text. Each unchanged package/route has explicit negative evidence.
7. Confirm coordinated minor `0.4.0`, 40 packages, the exact reviewed-head
   release-route count, Rust `1.95.0`, Apple Silicon macOS, and source-only
   annotated-tag intent. Immutable release inventory remains 47; wider current
   integration/route gates are 49 due to `pi.sdk-sidecar` and `claude-agent.sdk`.
   Freeze the exact candidate inputs for card 051.
8. Return the authenticated working-application smoke as an explicit operator
   question. Require the later authority packet to name repository/application,
   route, command, exact candidate SHA/tag consumption, credential/provider
   authority, permitted mutations, evidence/redaction, cleanup, and retry
   budget. Do not select an application or contact a provider.

## Out Of Scope

Cargo version or requirement changes, release notes, changelog promotion,
candidate or historical release baselines, code, claims, fixtures, CI
workflows, release preparation, tags, pushes, provider calls, consumer-repo
mutation, application smoke, feature/currentness work, or papercut repair.

## Acceptance Criteria

- exact base/head identity and the full large delta are recorded
- all 40 package APIs and every reviewed-head release route appear exactly once
  in the frozen census; immutable release inventory remains 47, while wider
  integration/route gates are 49 due to `pi.sdk-sidecar` and `claude-agent.sdk`
- every API and guaranteed-behavior break is explicit; no `changelog-only`
  inference substitutes for semantic or behavioral evidence
- the known `minimal` removal is classified as breaking and no other break is
  silently assumed away
- the caller-bounded `InteractiveSessionHandle::close` signature and cleanup
  guarantee are classified as a coordinated `v0.4.0` API and behavior break
- unreleased semantic API evidence names the removed zero-argument close
  exactly; the guard still rejects every removal not present in that approved
  `v0.4.0` evidence and no immutable tagged baseline changes
- immutable `v0.1.x`, `v0.2.0`, `v0.3.0`, `v0.3.2`, and `v0.3.3` baselines are
  unchanged
- feature/currentness freeze and deferred limits match Research 276 and current
  g05 authority
- card 051 has one exact reviewed audit head and no unresolved release-version,
  package, route, target, or compatibility choice

## Validation

- `effigy package:metadata`
- `effigy package:api`
- `effigy qa:routes`
- `effigy qa:docs:index:roadmaps`
- `effigy qa:docs:index:roadmaps:g05`
- `effigy qa:docs:index:roadmaps:batch-cards`
- `effigy qa:docs:roadmaps:status`
- `effigy qa:docs:next-action:roadmaps`
- `effigy qa:docs:links`
- `effigy qa:docs:roadmaps:numbers`
- `effigy qa:northstar`
- `git diff --check`

These are audit and planning gates, not the 11 candidate release gates.

## Review Oracle

Invariant: the ledgers partition the full `v0.3.3` to reviewed-head semantic
and guaranteed-behavior delta without rewriting prior evidence.

Smallest counterexample: a removed public item absent from the API ledger, a
route behavior changed only in tests but absent from the behavior ledger, or a
current item compared against an overwritten historical baseline.

Required proof: exact Git identities and counts, generated semantic diff, full
package and route partitions, behavior-source cross-reference, changelog
coverage map, immutable-baseline diff, and explicit compatibility result per
changed item.

## Auto-Continuation

No. Exact-head review must accept the audit and freeze census before card 051
can become ready.

## Stop Conditions

Stop on remote/base ambiguity, a missing package/route, an unclassified break,
historical baseline drift, an open mergeable feature/currentness PR, or any
need to choose product compatibility policy not fixed by Contract 036 and the
operator decisions.
