# Validation Tiers

Use the smallest proof that owns the current change. Broader gates remain
mandatory at their milestone; they are not normal per-edit feedback.
New to the shared vocabulary? Read [Key Concepts](key-concepts.md).

All selectors in this guide are deterministic unless a command is explicitly
named as a live probe. They do not authorize authentication, provider prompts,
remote mutation, allowance spend, or destructive cleanup.

## Task Proof

Tom's directive (2026-09-30): targeted checks per task, full QA at milestones.
Briefs name Effigy selectors for tests of the changed behaviour and a compile
of the touched code, plus relevant static checks below. Use a test target or
filter that proves the change; package selection alone is not a test filter.
If no selector covers that scope, add a narrow selector as part of the task.

Workers run those checks once, open the PR and report. Reviewers read the diff,
run the same checks and exercise the behaviour. Neither runs whole suites or
repeat passes. Queue runs no separate per-task validation command; GitHub CI
gates merges.

## Whole-Package Proof

Pass one to four exact workspace package names:

```sh
effigy validate:focused \
  swallowtail-adapter-pi \
  swallowtail-adapter-xai
```

This runs:

1. one nextest invocation for the selected packages
2. one warnings-denied all-target clippy invocation for the same packages

It does not infer scope from the worktree or filter tests within a package.
Despite its name, `validate:focused` runs whole package suites. It is not a
per-task default; reserve it for milestone package proof.

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

The planner runs `effigy qa` on `main` at release points and after major chunks
of work, then briefs fixes for what it finds. Release acceptance owns additional
release gates:

- workspace: `check:rust`, `check:examples`, `lint:rust`, `test:rust`, `qa`
- package: `package:docs`, `package:msrv`, `package:verify-local`,
  `package:check`
- candidate and consumer: `package:candidate:*`
- installed live evidence: `probe:*`

Do not run broad gates per task. Do not weaken or skip release evidence.

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
also run the targeted checks their task brief names. Release
operators run the milestone and release gates only when the accepting brief
requires them.
