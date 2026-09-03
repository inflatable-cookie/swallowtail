# g05.021 v0.4.0 Release Readiness

Status: planned; card 050 complete at `b7f804b5` pending exact-head review; cards 051-052 planned
Owner: Tom
Created: 2026-09-02
Updated: 2026-09-03
Depends on: Contract 036; Research 276; immutable `v0.3.3`
Vision tags: source release, compatibility, release readiness

## Purpose

Produce one reviewed, frozen `v0.4.0` source candidate and the evidence needed
for a later operator tag decision. Current `main` removes the previously
guaranteed OpenAI Background `minimal` reasoning value, so Contract 036
requires a coordinated pre-1.0 minor rather than `v0.3.4`.

The milestone is planning for a source-only annotated Git tag on canonical
GitHub. It does not authorize a tag, tag push, crates.io publication, GitHub
Release object, binary, sidecar, installer, provider call, or consumer-repo
mutation.

## Current Evidence

- Immutable `v0.3.3` is 40 packages, 47 production routes, Rust `1.95.0`, and
  Apple Silicon macOS as the verified target.
- Current source remains 40 packages. The immutable `v0.3.3` release inventory
  is 47 routes and the consumer front door still proves exactly that number.
  The wider integration and route gates prove 49, because current source
  carries two post-release routes beyond the release inventory:
  `pi.sdk-sidecar` and the restored `claude-agent.sdk`. Neither is in the
  release inventory. Historical package, dependency, route, API, and
  release-note baselines stay immutable.
- The audited head is 826 commits and 2,694 changed files beyond `v0.3.3`.
  Card 050's complete ledger audits the full semantic delta, rather than
  assuming the known `minimal` removal is the only break.
- The current `[Unreleased]` changelog records a large public and guaranteed-
  behavior delta. It is input to the audit, not proof that every change is
  classified or release-ready.
- Card 058 adds a second known coordinated `v0.4.0` break: interactive-session
  close now requires exact host services and a caller-selected absolute cleanup
  deadline, with one hard boundary over post-expiry cleanup. Card 050 must
  classify its package and consumer effects separately from the OpenAI
  Background `minimal` removal.
- An exploratory run passed formatting, both configured Clippy shapes, and
  2,825 tests, then stopped only because `qa:docs` found the checkpoint-held
  lock. That partial run is not release evidence. All 11 configured local gates
  must rerun on the final frozen candidate.

## Freeze Reactivation

The operator broke the feature freeze on 2026-09-02 for g05.022's native Claude
Agent SDK route and independent Claude Agent ACP expansion. Following the
acceptance and merge of PR 196 at `493f8194` completing g05.022, release
readiness is unpaused. Card 050's partial prior semantic API generation is
retained as non-accepted audit evidence. Card 050 is ready to restart its fresh
exact-head audit against canonical `main` `b7f804b5`. Card 050 is now locally
complete at that exact head and stops for independent review. Cards 051-052
remain planned and dependent. No release mutation, candidate preparation, tag,
or push is authorized.

## Runway

1. Card 050 audits exact `v0.3.3` to reviewed-current-source package,
   dependency, route, semantic API, and guaranteed-behavior deltas; freezes the
   release census; and classifies every break and compatibility claim.
2. Card 051 first restructures release prose and completes non-Effigy candidate
   edits, then requires read-only release status to infer minor `0.4.0` and an
   explicit-version preparation plan. After separate operator authorization,
   one Effigy prepare mutation owns workspace-version and changelog promotion,
   reruns all 11 local gates, and freezes the exact extracted `0.4.0`
   changelog. The accepted candidate then lands on canonical `main` and requires
   CI for that exact SHA.
3. Card 052 reruns the current external source consumer against that exact
   candidate, performs one operator-authorized current authenticated normal
   working-application path, compiles final release evidence, and stops for an
   explicit operator tag decision.

The cards are serial. Card 051 consumes card 050's frozen classifications and
baseline set. Card 052 consumes card 051's immutable candidate SHA and also
requires the operator to name the working application and complete authenticated
smoke authority, including exact source consumption and retry budget.

## Release Boundary

No card creates or pushes a tag. The final card must stop with the exact
candidate SHA and ask the operator to authorize the source commit, canonical
branch and remote, exact tag name, annotated tag message, local tag creation,
and tag push. The request must confirm that crates.io publication and a GitHub
Release object are absent, along with binaries, sidecars, and installers.

Any candidate change after the 11 local gates, exact-head review, canonical
merge, or exact-SHA CI invalidates later evidence and returns to card 051. A
failed gate stops the lane; no bypass, partial rerun, retag, or evidence from a
different SHA is accepted.

## Known Limits Required In Release Notes

- the watcher route is not live-ready and remains exact `2.1.251`
- bounded skill-inventory implementation is unplanned
- Contract 061 consumer projection is partial: 249 of 767 rows proved, 518
  remaining; g05.009/card 034 stay deferred
- Gemini requalification remains deferred
- `kimi-code.acp` remains QualifiedOnly at the A2 `0.38.0` cap
- source-only distribution includes no crates.io publication, GitHub Release,
  binaries, sidecars, installers, or model artifacts

## Batch Cards

- [050 v0.3.3 To Candidate Compatibility And Freeze Audit](batch-cards/050-v0-3-3-to-candidate-compatibility-and-freeze-audit.md) — complete at exact `b7f804b5`; independent exact-head review required before card 051
- [051 v0.4.0 Candidate Preparation And Exact-SHA CI](batch-cards/051-v0-4-0-candidate-preparation-and-exact-sha-ci.md) — planned; depends on completed card 050; mutating prepare requires separate operator authorization
- [052 v0.4.0 Consumer Proof And Operator Tag Gate](batch-cards/052-v0-4-0-consumer-proof-and-operator-tag-gate.md) — planned; awaits completed card 051 and complete operator authority for an authenticated application smoke

## Acceptance

- every semantic API and guaranteed-behavior delta from `v0.3.3` is inventoried
  and classified across all 40 packages and every release route at the resumed
  reviewed head; the immutable release count is 47
- immutable prior release baselines remain byte-for-byte unchanged; a distinct
  40-package `v0.4.0` semantic baseline and route candidate inventory are
  created only during candidate preparation; the candidate route inventory
  grows past 47 only for routes the audit accepts; PR 196 merged at `493f8194`
  with `claude-agent.sdk` restored before the audit
- all 11 local gates pass together on the frozen candidate, followed by
  canonical exact-SHA CI on that same source
- the clean candidate contains no generated cache, build output, host path,
  secret, auth state, private endpoint, or unreviewed live capture
- no mergeable feature/currentness PR remains open inside the freeze
- read-only release status selects minor `0.4.0`; Effigy alone applies the
  authorized workspace-version and changelog-promotion mutations; the promoted
  changelog has deduplicated headings and a structural `Breaking` entry for the
  OpenAI Background `minimal` removal and the caller-bounded interactive-close
  signature
- current external source-consumer and one operator-authorized working
  application pass against the exact candidate through a current normal
  authenticated product path; provider-free substitutes do not count
- final evidence names the exact SHA and stops before tag creation or push

## Review Oracle

Invariant: one exact source tree supports every `v0.4.0` compatibility,
package, route, gate, CI, source-consumer, and application-smoke statement.

Smallest counterexample: one public item or guaranteed value removed since
`v0.3.3` but omitted from the audit, one historical baseline rewritten, one
gate or smoke result from another tree, or one post-gate commit presented as
the candidate.

Required proof: complete semantic and behavior ledgers keyed to the
`v0.3.3` peel and candidate SHA; immutable-baseline diff; 40-package and exact
reviewed-head release-route inventories; release-status and explicit-version
prepare-plan output;
separate prepare authorization; frozen exact changelog extraction; all 11 local
gate results; exact-SHA CI identity; clean source inventory; exact-revision
source consumer; selected authenticated application smoke; and an operator
decision request that grants no release mutation by itself.

## Stop Conditions

Stop on an unclassified break, package or route mismatch, modified historical
baseline, open mergeable feature/currentness PR, dirty or identity-ambiguous
candidate, red or mismatched gate/CI evidence, an Effigy prepare path that
cannot operate on the intended tree, missing prepare authorization, unnamed or
unauthenticated application, incomplete retry/mutation/credential/provider
authority, or any request to create or push a tag before the final explicit
operator authorization.
