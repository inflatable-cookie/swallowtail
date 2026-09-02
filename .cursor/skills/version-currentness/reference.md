# Version Currentness Reference

Read this when executing a family. Templates are shapes, not version
pins. Copy the nearest prior family in the active generation and replace
numbers.

## Finding current claims

| Need | Where |
| --- | --- |
| Axis, baseline, latest qualified, posture, claim ids, behavior revisions | `crates/swallowtail-adapter-*/src/selection.rs` |
| Production version column | `docs/guides/provider-solution-feature-matrix.csv` |
| Route + lifecycle rows | `docs/guides/provider-route-matrix.md` |
| Decoder / help specimen | adapter `tests/fixtures/<family>-<baseline>/` |
| Official channel | last currentness research row, adapter README/guide, changelog URL in prior identity research |
| Current deferrals | `docs/roadmaps/standing-lanes.md` |
| Next unused numbers | highest research, roadmap, and batch-card ids on refreshed canonical pushed main (`https://github.com/inflatable-cookie/swallowtail.git` `main`), not this worktree or `origin/main` |

Do not treat a frozen currentness table as still-true official latest.
Re-probe.

Package scope is the adapter crate name, for example
`swallowtail-adapter-antigravity`. `validate:focused` and
`package:verify-affected` take one to four exact workspace package names.
Do not infer scope from changed files.

## Official channels

Use the family's documented channel. The 127 specimen used:

- npm `latest` for published CLIs
- GitHub latest stable release or tag for some installed binaries, attached
  runtimes, and ACP adapters
- crates.io max stable for SDK pins
- ACP registry JSON as discovery metadata, not as a Swallowtail claim
- host `--version` as observation only

Ignore preview, nightly, alpha, and development channels unless the
Swallowtail pin is itself that prerelease. Ignore hosted "latest model".
Do not flatten packaging, desktop About, or unofficial launchers onto the
named axis.

A major-line reset on the same package is an identity investigation, not
an `UnverifiedNewer` default.

## Probe commands (adapt per family)

Stay in the Swallowtail repo root for `effigy` / `cargo`. Use `/tmp` for
extracted official artifacts.

Typical GitHub binary family:

```sh
command -v <cli>
<cli> --version
shasum -a 256 "$(command -v <cli>)"
codesign -dv --verbose=4 "$(command -v <cli>)"   # macOS signed binaries only
gh api repos/<owner>/<repo>/releases/latest
```

Typical npm CLI family:

```sh
npm view <package> version
npm view <package> time --json
<cli> --version
```

Intermediate published hops and artifact tree comparison:

Extract tarballs/binaries for previous ceiling, intermediate stables, and
official latest into `/tmp/<pkg>-<ver>/`.

Generate relative-path file manifests with SHA-256 digests across each
extracted root without embedding host paths:

```sh
# Inside each extracted package root (e.g. /tmp/<pkg>-<ver>/package):
find . -type f | sort | while read -r f; do
  printf "%s  %s\n" "$(shasum -a 256 "$f" | awk '{print $1}')" "${f#./}"
done > /tmp/manifest-<ver>.txt

# Compare file inventory and digest deltas between hops
diff -u /tmp/manifest-<vA>.txt /tmp/manifest-<vB>.txt
```

Compare selected mapped help against the frozen `help.txt` / protocol
fixture. Diff the whole dump only to find additions, then classify each
addition as mapped, unmapped, or argv0 noise.

For multi-file packages or refactored trees, inspect every changed file that
feeds mapped routes. Byte-identical mapped files across hops are strong
stability evidence. For non-identical files, inspect `diff -u` to bound
internal/unmapped changes vs wire/lifecycle changes.

Pre-push official latest recheck:

```sh
npm view <package> version
# or gh api repos/<owner>/<repo>/releases/latest
```

Do not re-download an official asset already extracted in this session
unless the digest does not match.

## Identity fixture

Secret-free directory, for example
`crates/swallowtail-adapter-<name>/tests/fixtures/<cli>-<official>/`:

- `identity.json` — axis, host, official, published stables since previous
  ceiling, unpublished next, keep-gap flags, claim-at-observation, and
  `identity_decision`
- `protocol.json` — selected protocol invariants, classified deltas across
  published hops, bounded unmapped key sets with explicit rationale,
  byte-identical mapped file flags, no-prompt / no-live / no-host-change flags
- `dist-inventory.json` — required when shipped files feeding mapped behavior
  are non-byte-identical or package trees refactor:
  - `compared`: array of version strings from previous ceiling through official latest
  - `package_file_counts`: map of version to total file count
  - `identical_through_<range>`: array of files unchanged across all hops
  - `from_<vA>_to_<vB>`: per-hop `added`, `removed`, `changed`, `identical` arrays
  - `hashes`: map of key files to per-version SHA-256 digests
- `README.md` — short, no secrets

The inventory proves the exact file delta across published hops; it does not
require an exhaustive semantic function catalogue for clearly internal code.
When all executable and shipped files feeding mapped behavior are byte-identical
across hops (with only package metadata or version string bumps), a compact
byte-identity ledger in `protocol.json` and `identity.json` suffices without
authoring a full `dist-inventory.json`.

Delta ledger tests (`<family>_<ver>_delta_ledger.rs` or integration test):

- assert exact string sets and key objects (`assert_exact_string_set`, `assert_true_object`, exact arrays)
- assert package file counts and exact `added`/`changed`/`identical` sets when a full inventory is used
- assert byte-identical mapped file hashes or compact byte-identity assertions across hops
- ensure test fails if unmapped keys or file sets mutate independently of self-authored booleans

`claim_at_observation` is the **before** claim. The decision records the
intended after shape. Production `selection.rs` still matches "before"
until the claim card.

## Research identity record

`docs/research/<NNN>-<family>-<version>-identity.md`

- Status: promoted
- Question: remaining rank plus compatible-extension / milestone / stop
- Remaining AllowUnverified table at observation time (this family's rank
  only needs to be honest; do not rewrite older research tables)
- Method: what was compared; versions extracted in `/tmp`; file counts or
  byte-identity digests per hop; reference to frozen `dist-inventory.json`
  when generated; no prompt; host not replaced
- Identity table: host vs official, with digests
- Selected protocol: mapped subset vs unmapped extras; byte-identical mapped
  files; classified delta categories; unmapped boundaries with reasons
- Decision: bullet the intended claim shape; name the claim card
- Sources: host, official URL, changelog, asset

Index it in `docs/research/README.md` under Research Records.

## Roadmap and cards

One milestone in the then-active generation, two batch cards. Status
starts ready and is completed on closeout. If no generation is active,
stop and ask. Do not invent a generation to house currentness.

Canonical pushed-main authority is
`https://github.com/inflatable-cookie/swallowtail.git` `refs/heads/main`.
`origin/main` is not authority (forks, stale tracking refs). Immediately
before allocating ids, and again immediately before push:

```sh
effigy qa:docs:roadmaps:numbers
```

That selector fetches the canonical ref into
`refs/swallowtail/roadmap-authority` and fails closed if the fetch cannot
refresh it. Next unused is the max numbered
`docs/roadmaps/<active>/*.md` and `batch-cards/*.md` on that refreshed
tree. A number already assigned to a path on canonical main cannot move
to another path; take a new unused number. Same-path content edits are
allowed. Forks that need an explicit base pass `--authority` at that URL;
do not point it at the fork's origin.

Milestone name shape: `<NNN> <Family> <version> Useful Newer`.

Identity card: freeze evidence, name segment shape, **no production claim
edit**. Auto-continue to the claim card.

Claim card: raise the bound, refresh tests/docs/indexes, named validation.
Auto-continuation: No.

Both cards list Gemini (or whatever is currently deferred) as out of
scope unless the operator lifted that gate.

## Claim edits

In `selection.rs`:

- keep claim ids unless a new window is actually a new claim
- keep `AllowUnverified` unless the family is exact-pin / qualified-only
- `InterfaceVersionSegment::new(baseline, latest, behavior, Maintained)`
  replacing `exact` or extending the old ceiling
- unit tests: baseline qualified, official qualified, published
  intermediates qualified, gap rejected, synthetic later
  `UnverifiedNewer`
- discovery/foundation table: host qualified, official qualified,
  synthetic later unverified

Multiple claims on one axis (catalogue + headless, ACP + HTTP, …) stay
distinct behavior revisions. Raise both, or name why one stays behind.

Synthetic `UnverifiedNewer` in tests must not imply that unmapped flags,
sibling auth, or `--continue` / dangerous-bypass flags are selected.

## Public docs checklist

Update current surfaces that name the ceiling:

- family prepared-integration guide
- `docs/guides/provider-route-matrix.md` capability row and lifecycle row
- `docs/guides/provider-solution-feature-matrix.csv`
- `docs/architecture/system-architecture.md` only if it names this
  ceiling
- `CHANGELOG.md` Unreleased
- `docs/logs/<date>-<family>-<version>-identity.md`
- `docs/logs/<date>-<family>-<version>-claim.md`
- `docs/logs/README.md`
- `docs/roadmaps/README.md` Next Task
- then-active generation README runway, Current Checkpoint, numbered
  Milestones list
- then-active generation `batch-cards/README.md` Completed

Leave historical research, old logs, and immutable release docs alone.
`v0.3.2` (or whichever tagged release) keeps its recorded package/route
counts; current source may already differ.

Contracts name rules, not the moving ceiling. Do not edit a contract
unless it actually states the current bound.

`crates/swallowtail-testkit/tests/fixtures/provider-wide-harness-activity.json`
often still shows an older `qualified` string. Leave it unless `qa:routes`
or another named gate fails.

## Next Task examples

Verb must be one of: Define, Implement, Validate, Extract, Adopt,
Reassess, Close, Map, Prove.

Family remaining:

```
Implement <Family> useful-newer qualification for the current official
stable. Host already sits on a qualified bound. Do not leave the official
point UnverifiedNewer without a named incompatible reason.
```

Sweep complete except current deferrals:

```
Reassess the active generation now that remaining currentness
AllowUnverified families except deferred <name> sit on current official
stables. Do not keep the generation open for currentness.
```

`generation-index.md` and `long-term-plan.md` keep their pointer to the
roadmaps front door. Do not duplicate the batch pointer there.

## Checkpoint classification

The checkpoint research row may still say `visible unverified-newer`.
That is a research classification, not permission to skip the family.
After the record, compile one-family Upgrade Workflow work for every
current official/host stable that is newer than the qualified ceiling
and is not a named deferral or named incompatible gap.

## Stops

Stop the family and ask, leaving claims unchanged, when:

- exact artifact identity cannot be corroborated from official sources
- selected mapped help or protocol differs from recorded evidence
- qualifying the official point needs a provider prompt or live session
- official latest moves during the run or before final push
- the shape would flatten onto another family
- a new mapped public operation is required before the pin can be named
- the family is exact-pin / qualified-only and the operator did not ask
  to reopen it
- shipped tree inventory reveals unclassified changes to mapped wire format,
  lifecycle, or failure behavior
