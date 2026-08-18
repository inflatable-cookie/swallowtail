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
| Current deferrals | `docs/roadmaps/g03/README.md` Current Checkpoint |
| Next unused numbers | highest research, g03, and batch-card files plus their indexes |

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

Compare selected mapped help against the frozen `help.txt` / protocol
fixture. Diff the whole dump only to find additions, then classify each
addition as mapped, unmapped, or argv0 noise.

Do not re-download an official asset already extracted in this session
unless the digest does not match.

## Identity fixture

Secret-free directory, for example
`crates/swallowtail-adapter-<name>/tests/fixtures/<cli>-<official>/`:

- `identity.json` — axis, host, official, published stables since previous
  ceiling, unpublished next, keep-gap flags, claim-at-observation, and
  `identity_decision`
- `protocol.json` — selected flags still present, unmapped additions,
  decoder specimen name, no-prompt / no-live / no-host-change flags
- `README.md` — short, no secrets

`claim_at_observation` is the **before** claim. The decision records the
intended after shape. Production `selection.rs` still matches "before"
until the claim card.

## Research identity record

`docs/research/<NNN>-<family>-<version>-identity.md`

- Status: promoted
- Question: remaining rank plus compatible-extension / milestone / stop
- Remaining AllowUnverified table at observation time (this family's rank
  only needs to be honest; do not rewrite older research tables)
- Method: what was compared; no prompt; host not replaced
- Identity table: host vs official, with digests
- Selected protocol: mapped subset vs unmapped extras
- Decision: bullet the intended claim shape; name the claim card
- Sources: host, official URL, changelog, asset

Index it in `docs/research/README.md` under Research Records.

## Roadmap and cards

One g03 milestone, two batch cards. Status starts ready and is completed
on closeout.

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
- `docs/roadmaps/g03/README.md` runway row, Current Checkpoint, numbered
  Milestones list
- `docs/roadmaps/g03/batch-cards/README.md` Completed

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
Define the next g03 maintenance card now that remaining currentness
AllowUnverified families except deferred <name> sit on current official
stables.
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
- official latest moves during the run
- the shape would flatten onto another family
- a new mapped public operation is required before the pin can be named
- the family is exact-pin / qualified-only and the operator did not ask
  to reopen it
