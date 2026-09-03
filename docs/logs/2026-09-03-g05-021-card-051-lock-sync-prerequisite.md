# g05.021 Card 051 Lock Sync Prerequisite

Date: 2026-09-03
Status: review required

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

This prerequisite does not run prepare, refresh Cargo.lock manually, weaken
`--locked`, tag, publish, or contact a provider. The preserved candidate edits
resume only after exact-head review and merge. A new explicit operator
authorization is required before one later mutating prepare attempt.
