# g05.050 v0.5.0 Tagged-Source Consumer Proof

Date: 2026-09-10
Status: complete; provider-free release evidence recorded for Desktop g02.051 relay
Task: `../roadmaps/g05/050-v0-5-0-tagged-source-consumer-proof.md`
Handoff: `../handoffs/20260910-132615-g05-050-v0-5-0-tagged-source-consumer-proof.md`

## Result

The canonical remote annotated tag `v0.5.0` is consumable as one exact
source. The repository source-consumer selector passed from a clean detached
checkout of the tag at exact peel
`582d01d6b6890eed5195a1fcbee0ae985304a6c7`. One isolated external Cargo
application selected three direct Swallowtail dependencies through the
canonical HTTPS Git URL with only `tag = "v0.5.0"`, resolved every selected
Swallowtail package to that exact peel with no path, branch, revision, patch,
or mixed-source leak, and compiled the public registered-only Claude Agent
SDK binding with explicit `ReadWrite` access under pinned MSRV `1.95.0`. No
session opened and no provider was contacted. The temporary consumer and
detached worktree were removed; the tag and canonical `main` were unchanged
throughout.

## Tag Capsule

Verified before, during, and after the proofs:

- annotated tag object `c772c5839806b6cbf1c9d3b495049b362e4c0c52`, type
  `tag`, identical locally and on
  `https://github.com/inflatable-cookie/swallowtail`;
- peels to exact source `582d01d6b6890eed5195a1fcbee0ae985304a6c7`
  (tree `eebb6978be365f79ae41436912240886d0aecf78`); and
- SHA-256 of the complete tag object bytes `0949222cf5df2a72e1ac87dc37d7cbcb`
  `8adaa8a610a37772b2e70b85b0c41ba1`; SHA-256 of the annotation body alone
  `eb99cdf8a1e63fcf7fd45b619102790874053858eaaa2bef758563dccddf76a3`,
  matching the approved three-paragraph text recorded by the tag closeout.

Tag-triggered CI run 34467974791 was green 11/11 at that peel; the tag was
never moved, recreated, or force-updated during this task.

## Repository Source Consumer

A detached checkout of the tag at a temporary path sat at HEAD
`582d01d6b6890eed5195a1fcbee0ae985304a6c7` (tree
`eebb6978be365f79ae41436912240886d0aecf78`) with a clean status. From that
checkout exactly this command ran:

```text
effigy package:source-consumer
```

It exited 0 and closed with:

```text
external source consumer passed at exact commit 582d01d6b6890eed5195a1fcbee0ae985304a6c7
```

The complete 53-line transcript has SHA-256
`2d198fe13725c81a908b6dd8f92e57cab8f3398cc8249337c783120ad625846a`.

## External Remote-Tag Consumer

In a fresh temporary directory outside the repository, a minimal Cargo
application declared exactly three direct Swallowtail dependencies, each with
`git = "https://github.com/inflatable-cookie/swallowtail"` and only
`tag = "v0.5.0"`:

- `swallowtail-adapter-claude-agent` — the public registered-only Claude SDK
  binding;
- `swallowtail-runtime` — `RegisteredToolPreparation` and
  `PreparationFailure`;
- `swallowtail-core` — `ResourceAccess`.

No path, branch, `rev`, patch, or workspace override appeared anywhere. The
retained manifest has SHA-256
`74e9214453a137d2018cb63ae888fc5710ace622449a23f03d6f2f2c82cb5d9c`; the
retained consumer source has SHA-256
`49a50e7c144d791b4cfcf4f242f43bd62c5330330d7277f6ee06db139d75b662`.

### Locked Source Identity

`cargo +1.95.0 generate-lockfile` locked 47 packages; the lockfile SHA-256 is
`3691591cd46a33c2b7c18e5a5f7780ba51d24b2f71773bc3fcf369e0001588e8`. Every
Swallowtail package reachable from the three direct dependencies — six
packages — carries one identical Git source:

```text
swallowtail-adapter-claude-agent 0.5.0
swallowtail-core 0.5.0
swallowtail-host-local 0.5.0
swallowtail-idioms 0.5.0
swallowtail-protocol-acp 0.5.0
swallowtail-runtime 0.5.0
```

with source
`git+https://github.com/inflatable-cookie/swallowtail?tag=v0.5.0#582d01d6b6890eed5195a1fcbee0ae985304a6c7`
in each case. Locked metadata (SHA-256
`1f24aa1eb44619dc12100a3c21f9e8471f260a4dd81458c0789d0903cc2e7b2d`) passed
machine-checked assertions requiring at least three Swallowtail packages,
every Swallowtail source on the canonical HTTPS URL with the tag query and
the exact peel, and zero `file://` or path sources anywhere. One tag, one
peel, no mixed-source leak.

### Compile-Time Registered-Only Proof

`cargo +1.95.0 check --locked` (rustc 1.95.0, cargo 1.95.0 — the repository
MSRV from `release-baselines/rust-toolchains-0.2.0.env`) finished cleanly;
the 47-line transcript has SHA-256
`567c946ee8a9a04dc890bb2a1e5ed5b28e2f5114630bcdba13f47557c6c48f6f`.

The consumer source type-checks exactly the surface Desktop g02.051 needs,
at compile time only:

- `ClaudeAgentSdkRegisteredOnlyBinding::new` called with a
  `RegisteredToolPreparation`, `ResourceAccess::ReadWrite`, and
  `ClaudeAgentSdkPermissionMode::AcceptEdits` — the registered-only
  constructor is the only public surface that admits an empty native tool
  set, so the zero-native session with an explicit `ReadWrite` lease is
  representable from the tagged source;
- the carried profile is asserted `is_registered_only()` and reports the
  explicit `ReadWrite` access through `resource_access()`, both compiled
  through the public API; and
- the function is coerced to an exact function-pointer type in `main`, so the
  type checker cannot elide the call shape.

The binding was never constructed with a live preparation, no session
opened, no process launched, and no provider, credential, or network path
beyond Cargo source retrieval and Git object transfer was exercised.

## Cleanup And Immutability

The detached worktree, the external consumer, its lockfile and target
artifacts, and the temporary root were removed. Canonical Swallowtail
remained clean on the task branch; the tag object, peel, tree, and annotation
digests were re-read after cleanup and match the capsule above; remote
`refs/tags/v0.5.0` still points at tag object `c772c583`. No Swallowtail or
consumer-repository mutation occurred, no dependency was repinned, and no
provider was contacted at any point.

## Non-Claims

This proof is API-consumability evidence, not a working-route acceptance. It
does not open a Claude session, does not exercise the Card 318 exact live
qualification tuple, does not pass or fail any Desktop route, and does not
authorize a publication, tag movement, or provider call. Desktop task
`1e7b9baf-9b97-4e49-a6d4-17aa24bbcb5c` remained blocked throughout. Chatterbox
owns the relay of this capsule into the narrow Desktop `v0.5.0` pin amendment
and the same-task resume.

## Merge And Closeout

Documentation PR #313 (head `f1b301af07d308680e7c1a8bac167abd3cdd0440`, one
commit over handoff `ac56130d`) passed independent exact-head review
`5618788611` with no findings and merged to canonical `main` at
`fa2e4b942785a288befd3fc76f15e04638f17eaa`. Validation at review:
`effigy qa:docs` pass, `effigy qa:northstar` pass, `git diff --check` clean.
No provider was contacted and no consumer was mutated.
