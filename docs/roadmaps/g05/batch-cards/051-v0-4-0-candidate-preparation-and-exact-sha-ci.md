# 051 v0.4.0 Candidate Preparation And Exact-SHA CI

Status: ready; card 050 audit accepted via PR 198 exact head `2f4923b8`, merged as `835fe9ff`; stops before the mutating prepare path for separate explicit operator authorization
Owner: Tom
Created: 2026-09-02
Milestone: `../021-v0-4-0-release-readiness.md`
Depends on: completed card 050 with accepted exact-head audit and freeze census; consume its frozen 49-route candidate boundary, including `pi.sdk-sidecar` and `claude-agent.sdk`

## Goal

Prepare, review, and freeze one coordinated `v0.4.0` source candidate, pass all
11 local gates, land it on canonical `main`, and require CI at that exact SHA.

## Progress 2026-09-03

The first explicit one-shot prepare authorization was consumed. Effigy applied
the workspace-version and changelog mutations, then rolled both back when
`lint` could not resolve internal `^0.4.0` requirements against workspace
packages still recorded as `0.3.3` in `Cargo.lock`. No prepared state, release
commit, or tag was created.

The supported repair is `release.sync-files = ["Cargo.lock"]` plus the
package-identity hardening accepted in Effigy PR 89 exact head `7182e753`,
merged as `4c554135`. That Effigy applies the version, changelog, and lockfile
mutations before gates; lock sync uses `cargo update --workspace --quiet`,
obtains the post-mutation workspace-member name and version map from Cargo
metadata, and rejects and restores any change outside those exact package
identities and versions. This Swallowtail repair must receive exact-head review
and merge before the preserved candidate edits are replayed. A fresh operator
authorization is required for the later mutating prepare; the consumed
authorization does not carry forward.

## Scope

1. Start from the accepted card 050 head in a clean candidate worktree. Fetch
   canonical GitHub and prove exact local base, `origin/main`, tag absence, and
   no open mergeable feature/currentness PR inside the freeze.
2. Audit and restructure release prose before promotion. Consolidate the
   current `[Unreleased]` content so each present structural heading occurs
   once, including one `### Added`, one `### Changed`, and one `### Breaking`.
   Put the removed OpenAI Background `minimal` guaranteed value under
   `### Breaking` with an entry that names the removal explicitly; Effigy
   v0.12.1 infers the pre-1.0 minor only from the `Breaking` category, while
   `Removed` alone resolves to patch. Do not rename `[Unreleased]`, create its
   dated `0.4.0` heading, edit release comparison links, or change
   `workspace.package.version`, or change coordinated internal requirements;
   Effigy owns those preparation mutations.
3. Complete the candidate edits outside Effigy's preparation ownership. Set
   write audited release notes and upgrade/rollback prose, and preserve
   `publish = false`, Rust `1.95.0`, Apple Silicon macOS support, and the
   source-only distribution boundary. Do not manually apply any Effigy-owned
   mutation and then run prepare over it. The version-file mutation owns both
   `workspace.package.version` and all coordinated internal requirements.
4. Create distinct `v0.4.0` package, dependency, 49-route, and semantic API
   baselines from the accepted card 050 census. The 49-route candidate
   includes `pi.sdk-sidecar` and `claude-agent.sdk`; this is frozen audit input,
   not a candidate-inclusion decision. Never edit or regenerate a prior release
   baseline.
5. Ensure the unpromoted changelog and release notes match the audited source.
   Name every classified break, including the removed OpenAI Background
   `minimal` value, corrected opaque facade point, fail-before-effects
   behavior, consumer impact, coordinated upgrade, and exact rollback to
   immutable `v0.3.3`. The required Pi route-add entry must name the exact
   `0.84.2` SDK, Node `22.23.2`, source-tagged sidecar, private
   `swallowtail-pi-sdk-jsonl-v1` wire, exact host-leased cwd, bounded load
   replay, durable app-owned session state, and no archive/restore/delete.
   It must say that new consumers provision those exact axes and session
   directory, existing v0.3.3 consumers have no action unless opting in, and
   rollback omits the route and sidecar calls without mixing workspace versions
   or aliasing `pi.rpc`.
6. Record the 40-package source inventory, 49 current integration/route rows, release package
   order, dependency graph, API baseline, target/floor, source contents, and the
   required known limits from the milestone.
7. Prove the source tree contains no build output, generated cache, local
   release bundle, host absolute path, credential, auth state, private endpoint,
   mutable provider payload, or unreviewed live capture. Remove only
   candidate-owned generated local state before the clean-tree proof.
8. Run the read-only `effigy --json release status`. Require its changelog
   inference to report `suggested_bump: minor`, `next_version: 0.4.0`, and tag
   `v0.4.0`. Status has no version override, so this is an independent
   structural check. Then run the read-only
   `effigy release prepare --plan --version 0.4.0` and review the exact planned
   Cargo.toml coordinated-version, changelog-promotion, and Cargo.lock sync
   mutations. The first mutation must include the workspace package version and
   all eight internal compatible requirements. The explicit version is
   mandatory even though status now infers the same result.
9. Stop and request separate explicit operator authorization for the single
   mutating preparation command. Only after that authorization, run exactly
   `effigy release prepare --yes --check-gates --version 0.4.0` once. Effigy is
   the sole mutation owner for `workspace.package.version`, coordinated
   internal requirements, `[Unreleased]` promotion into the dated `0.4.0`
   section, and synchronization of workspace member versions in `Cargo.lock`.
   Require all configured gates
   together on the resulting complete candidate: `fmt`, `lint`,
   `lint:no-features`, `test`, `qa`, `docs`, `metadata`, `api`, `security`,
   `floor`, and `source`. The earlier 2,825-test exploratory run and any partial
   rerun are not evidence.
10. Immediately extract the promoted release with
    `effigy changelog extract CHANGELOG.md --version 0.4.0`. Freeze the exact
    output and its digest in candidate evidence. Prove the extracted section
    has the single deduplicated headings and the structural `Breaking` entry,
    then make no further candidate edit.
11. Open one candidate PR, receive exact-head review, and land only the accepted
   head on canonical `main`. Record the resulting candidate SHA; no later
   closeout commit may be presented as that candidate.
12. Dispatch canonical `CI` against `main`. Select the `workflow_dispatch` run
   whose `headSha` equals the candidate SHA and require every configured job to
   pass. Reconfirm local `HEAD`, `origin/main`, and CI head identity.
13. Leave the candidate clean and immutable for card 052. Do not create a tag.

## Out Of Scope

Tag creation or push, release execution, crates.io, GitHub Release, binary or
sidecar publication, installer, provider call, live probe, consumer-repo
mutation, working-application smoke, feature/currentness work, claims beyond
the accepted audit, CI workflow edits, or unrelated papercut repair.

## Acceptance Criteria

- all 40 packages and internal requirements use coordinated `0.4.0`, applied
  together by Effigy's version-file mutation
- the frozen 49-route candidate includes both `pi.sdk-sidecar` and
  `claude-agent.sdk`; Pi release-note, consumer, and rollback treatment is a
  required Card051 action, not an unresolved inclusion choice
- new `v0.4.0` baselines match the accepted audit; every older baseline is
  byte-for-byte unchanged
- `[Unreleased]` is deduplicated before promotion, its structural `Breaking`
  section names the `minimal` guaranteed-value removal, and read-only release
  status selects minor `0.4.0`
- Effigy alone applies the Cargo.toml coordinated-version,
  changelog-promotion, and package-identity-checked Cargo.lock workspace-member
  sync mutations after separate explicit operator authorization; no manual
  replay or fallback substitutes for that path
- the frozen exact `0.4.0` changelog extraction, release notes, source
  inventory, upgrade, rollback, route count, known limits, and actual source
  agree
- candidate worktree is clean and free of forbidden source contents
- all 11 local gates pass on one complete tree after the final synchronized
  candidate changes
- exact-head review accepts the candidate before canonical landing
- canonical CI passes at the exact candidate SHA and not a merely recent SHA
- no local or remote `v0.4.0` tag is created

## Validation

- `effigy --json release status`
- `effigy release prepare --plan --version 0.4.0`
- `effigy release prepare --yes --check-gates --version 0.4.0`
- `effigy changelog extract CHANGELOG.md --version 0.4.0`
- exact-SHA `CI` workflow query and watch from the Effigy release protocol
- `git status --porcelain`
- local `HEAD` / `origin/main` / workflow `headSha` equality
- local and remote absence of `v0.4.0`

## Review Oracle

Invariant: the reviewed candidate tree is the tree that passed every local gate
and canonical CI, and all release copy describes that exact tree.

Smallest counterexample: one dependency still requires `^0.3.3`, an internal
requirement is pre-applied outside Effigy's version-file mutation, Cargo.lock
is not a planned sync mutation, lock sync admits a package not identified by
post-mutation Cargo metadata as a workspace member at its own version, the
`minimal` removal lacks a structural `### Breaking` entry or sits only in a
non-breaking category, duplicate headings enter the promoted section, a person
pre-applies an Effigy-owned mutation, one prior baseline changes, a gate
passes before the last edit, CI points at another commit, or release notes
omit one audit-ledger break.

Required proof: pre-mutation status and explicit-version plan, separate
operator authorization, the single mutating prepare transcript, exact
three-mutation plan, workspace-only lockfile diff,
metadata/dependency diff, frozen changelog extraction and digest, old-baseline
checksums, new baseline generation record, complete source inventory, 11 named
gate results, accepted PR head, canonical candidate SHA, exact-SHA CI run,
clean-tree output, and tag-absence proof.

## Auto-Continuation

No. Stop at the read-only preparation plan for separate operator authorization.
After candidate completion, card 052 also requires explicit operator authority
for its authenticated working-application path.

## Stop Conditions

Stop if read-only status does not infer minor `0.4.0`, if the explicit-version
plan does not own the intended coordinated Cargo.toml version, changelog, and
Cargo.lock sync mutations, if the active Effigy lacks accepted merge
`4c554135`, if lock sync would move any package or metadata outside exact
post-mutation workspace identity and version authority, or if prepare cannot
operate on the intended tree.
Return to planning; do not apply a
manual fallback, bypass, or second mutation path. Also stop on missing operator
authorization, candidate drift, any failing or skipped gate, a modified
historical baseline, source contamination, overlapping mergeable
feature/currentness PR, review mismatch, non-canonical base/remote, CI from
another SHA, tag presence, or a required workflow edit.
