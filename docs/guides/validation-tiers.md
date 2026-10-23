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
- `effigy format:check` for Rust formatting across the workspace
- `effigy format:package <pkg>...` for Rust formatting of one to four exact
  workspace packages; runs `cargo fmt` then a check pass for that scope only
- `effigy check:copilot-acp-host-permission-format` for the Python formatting
  of the host-permission proof runner; check-only, needs the pinned Ruff on
  `PATH` (see [Python Format Prerequisite](#python-format-prerequisite)).
  Expected negative baseline: it exits nonzero and names
  `scripts/copilot-acp-host-permission-proof.py`, because the frozen runner
  carries historical formatting debt. The runner is not reformatted, exempted
  or suppressed. Its hash bindings stay intact. The selector is not part of
  mandatory QA. Open question Q-008 records the unmade operator decision.
- `effigy check:python-format <file>...` for any other Python file, same rules
- `effigy package:api` for public Rust declarations
- `effigy package:metadata` for package topology

Use `effigy check:examples` when public examples or their linked guides
change. Use `effigy qa:docs` for guide indexes and links. Use
`effigy qa:routes` whenever a route, feature, activity, lifecycle, or coverage
map changes.

## Python Format Prerequisite

`effigy check:python-format` and `effigy check:copilot-acp-host-permission-format`
need Ruff `0.14.1` on `PATH`. The check refuses any other version. It never
rewrites files. Install the pinned formatter in a task-owned environment, not
globally:

```sh
python3 -m venv "$HOME/.local/share/swallowtail/ruff-0.14.1"
"$HOME/.local/share/swallowtail/ruff-0.14.1/bin/pip" install ruff==0.14.1
PATH="$HOME/.local/share/swallowtail/ruff-0.14.1/bin:$PATH" \
  effigy check:copilot-acp-host-permission-format
```

Prefix `PATH` on the one validation invocation only. Do not edit shell startup
files. `pipx install ruff==0.14.1` or `uv tool install ruff==0.14.1` also
work. No workflow or manifest installs Ruff.

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

## Oxide Adoption Boundary

Tom's 2026-10-23 standing approval covers supported Oxide adoption in Rust
projects with Paseo workspaces (decisions `3b041cf4-df3d-4e17-9886-5af51428ecf3`
and `e1c1d3b5-9ff2-4f42-81b6-7e2a3f6e3a14`). The
[Oxide workflow contract](https://github.com/inflatable-cookie/oxide/blob/main/docs/knowledge/contracts/workflow.md#switching-an-existing-project)
owns switching semantics. Effigy still owns task routing and host admission.

Ordinary local build/check/test entry points may switch after their command,
profile and output assumptions are inventoried. Keep explicit `:cargo`
fallbacks at least until formal Effigy v0.15. CI, private-target, container,
metadata, deny and packaging routes remain on Cargo unless separately
qualified and scoped for switching, regardless of Effigy's version. In particular,
`run-with-owned-cargo-target.sh` owns its target and cleanup; Grok's MCP probe
discovers a sibling example binary. Neither is an automatic switch candidate.

Adoption permits the agent-document block and supported selector changes, not
global configuration changes, live provider probes or output cleanup. A target
symlink needs an exact non-directory ignore rule such as `/target`; `/target/`
does not suffice. Build-slot placement on Builds is separate from final-output
profile links and does not prove that those links are implemented.
