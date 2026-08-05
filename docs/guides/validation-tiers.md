# Validation Tiers

Use the smallest proof that owns the current change. Broader gates remain
mandatory at their milestone; they are not normal per-edit feedback.

All selectors in this guide are deterministic unless a command is explicitly
named as a live probe. They do not authorize authentication, provider prompts,
remote mutation, allowance spend, or destructive cleanup.

## Normal Package Feedback

Pass one to four exact workspace package names:

```sh
effigy validate:focused \
  swallowtail-adapter-pi \
  swallowtail-adapter-xai
```

This runs:

1. one nextest invocation for the selected packages
2. one warnings-denied all-target clippy invocation for the same packages

It does not infer scope from the worktree.

## Affected Archive Proof

When a card requires package evidence:

```sh
effigy package:verify-affected \
  swallowtail-adapter-pi \
  swallowtail-adapter-xai
```

Each archive is assembled and inspected independently. Selected extracted
packages then compile through one shared temporary target against local
unpublished Swallowtail dependencies. The temporary subset lock is generated
offline. The repository lock is unchanged.

## Static Gates

Run only the relevant static truth:

- `effigy qa:docs` for docs
- `effigy qa:routes` for route, lifecycle, feature, or activity matrices
- `effigy format:check` for Rust formatting
- `effigy package:api` for public Rust declarations
- `effigy package:metadata` for package topology

Use `effigy check:examples` when public examples or their linked guides
change. Use `effigy qa:docs` for guide indexes and links. Use
`effigy qa:routes` whenever a route, feature, activity, lifecycle, or coverage
map changes.

## Milestone And Release Gates

The accepting card owns broad validation:

- workspace: `check:rust`, `check:examples`, `lint:rust`, `test:rust`, `qa`
- package: `package:docs`, `package:msrv`, `package:verify-local`,
  `package:check`
- candidate and consumer: `package:candidate:*`
- installed live evidence: `probe:*`

Do not run these after every local edit. Do not weaken or skip them when the
accepting card requires their evidence.

## Failure And Scope

Focused selectors reject:

- no package names
- more than four packages
- duplicate packages
- unknown workspace packages
- option-like package names

Use exact package names. Changed-file inference is deliberately absent.

Validation output is proof for the named selector and revision only. It does
not promote a provider version, route capability, credential posture, runtime
availability, or live compatibility. Keep failed deterministic proof separate
from an optional live-probe failure; neither grants retry or fallback.

Consumers normally use the compiling example and route fixture evidence linked
from the [integration guide map](integration-guide-map.md). Adapter maintainers
also run the exact focused and package tiers owned by their card. Release
operators run the milestone and release gates only when the accepting roadmap
requires them.
