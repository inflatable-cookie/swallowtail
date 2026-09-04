# g05.021 Card 051 Lock Sync Prerequisite

Date: 2026-09-03
Status: accepted

## Failure

The first explicitly authorized
`effigy release prepare --yes --check-gates --version 0.4.0` ran once. `fmt`
passed. `lint` failed because internal `^0.4.0` requirements could not resolve
workspace members still recorded as `0.3.3` in `Cargo.lock`. Remaining gates
did not run. Effigy rolled back its workspace-version and changelog mutations,
wrote no `.release-prepared.json`, and created no commit or tag.

That authorization is consumed. It does not authorize a retry.

## Repair

Swallowtail now configures `release.sync-files = ["Cargo.lock"]`. Effigy PR 89
exact head `7182e7538134bbffc820f9680b81df7e04eea1dc` was independently accepted
and merged as `4c5541354a0e86c5a4ee67092523858e915440f1`. That exact merge applies the
workspace version, coordinated internal requirements, changelog promotion, and
lock sync before gates. Cargo.lock sync runs
`cargo update --workspace --quiet`, obtains the post-mutation member names and
versions from Cargo metadata, and rejects and restores any changed package or
lock metadata outside that exact authority. This closes the prior value-only
hole where a third-party package moving to the selected workspace version
could evade rejection. The old Swallowtail comment claiming lock refresh
happened before preparation was false for a pre-1.0 minor.

Card 051's read-only plan must therefore show exactly three mutations:

1. `Cargo.toml` workspace package version and all eight coordinated internal
   requirements to `0.4.0`.
2. `[Unreleased]` promotion to the dated `0.4.0` changelog section.
3. `Cargo.lock` workspace-member version synchronization to `0.4.0`.

The exact merge checkout produced `v0.12.1+local.4c55413` and the Swallowtail
read-only plan was ready with those three paths and all eight coordinated
internal requirements. Effigy's focused unit proof rejected third-party,
same-as-workspace, value-swap, structural, and mixed-workspace counterexamples.
Its CLI proof forced a third-party move to the selected workspace version and
confirmed failure plus restoration of `Cargo.toml`, `CHANGELOG.md`, and
`Cargo.lock`, with no prepared state.

## Boundary

Swallowtail PR 200 exact head `e55021d2` passed independent review and merged
as `20b937ee`. The candidate then resumed from that canonical base without the
obsolete manual manifest or papercut edits. Every source authority surface was
written before preparation to describe the eventual frozen state.

A second fresh operator authorization covered exactly one Effigy prepare. The
first nine gates passed, then `floor` failed in
`no_compared_isolation_candidate_satisfies_the_review_oracle` because a
parallel watcher test reused another local host's released temporary path.
Effigy rolled back `Cargo.toml`, `CHANGELOG.md`, and `Cargo.lock` and wrote no
prepared state. That authorization is consumed. The watcher fixture now gives
each local host a process-and-sequence-unique temporary root and keeps the
path-removal assertion behind a deterministic reuse regression. The floor
selector's obsolete in-gate `cargo update --workspace` and value-only lock
validator are removed; accepted Effigy merge `4c554135` is the sole lock-sync
owner before the floor's pinned-Rust Clippy and tests.

A third fresh operator authorization covered exactly one Effigy prepare.
`fmt`, `lint`, and `lint:no-features` passed, then `test` failed
`deadline_after_dispatch_is_joined_unconfirmed_and_releases_access`: the
OpenCode fixture's independent 20-millisecond deadline and 100-millisecond
response timers let the response win under scheduler delay and produced
`Applied` instead of `UnconfirmedAfterEffect`. Effigy rolled back all three
owned mutations and wrote no prepared state. That authorization is consumed.
The repaired fixture holds the response after observed DELETE dispatch,
explicitly fires and observes the deadline, and only then releases the response
for joined cleanup. The exact test held 100 default-toolchain runs and 25 Rust
`1.95` runs; the focused OpenCode suite and Clippy passed.

One later final operator authorization covered exactly one Effigy prepare. The
transaction owned `Cargo.toml`, `CHANGELOG.md`, and `Cargo.lock`, passed all 11
configured gates, and was followed immediately by read-only changelog
extraction. The source then froze for exact-head review. Exact fingerprints,
gate timings, and extraction digest are recorded in the candidate PR evidence,
not a post-gate source edit.

This prerequisite does not refresh Cargo.lock manually, weaken `--locked`,
execute a release, tag, publish, contact a provider, or mutate a consumer.
Independent review later accepted PR 201 exact head `10d9b7a0`; it merged as
immutable candidate `56f3913a`. Canonical workflow-dispatch run 33853812785
used that exact `headSha` and passed all six jobs, completing Card 051. Card 052
remains planned and blocked on its complete operator-authenticated application
authority packet. Every release-execution or tag action remains separately
gated.
