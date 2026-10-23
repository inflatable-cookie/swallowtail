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
profile and output assumptions are inventoried. Preserve explicit Cargo
escapes and CI's plain-Cargo route until formal Effigy v0.15. Private-target,
container, metadata, deny and packaging routes remain on Cargo. In particular,
`run-with-owned-cargo-target.sh` owns its target and cleanup; Grok's MCP probe
discovers a sibling example binary. Neither is an automatic switch candidate.

Adoption permits the agent-document block and supported selector changes, not
global configuration changes, live provider probes or output cleanup. A target
symlink needs an exact non-directory ignore rule such as `/target`; `/target/`
does not suffice. Build-slot placement on Builds is separate from final-output
profile links and does not prove that those links are implemented.

### Switched selectors and their plain-Cargo escapes

A switched selector runs `bash scripts/oxide-rust.sh <cargo arguments>`. The
helper passes those arguments to Oxide unchanged and replaces itself with the
tool, so argv, exit status, signals, stdout and stderr are the tool's own. Every
switched selector keeps a `<selector>:cargo` sibling whose command is the same
arguments with plain `cargo` and no Oxide anywhere on its route. Use the
sibling when Oxide itself is broken.

- **Hosted CI.** `GITHUB_ACTIONS=true` selects the plain-Cargo branch, because
  hosted runners have no `oxide`. Workflow files do not change.
- **Missing Oxide.** Locally, an absent `oxide` exits 127 and names both
  escapes. It never falls back to Cargo silently.
- **One-run escape.** `SWALLOWTAIL_RUST_RUNNER=cargo` runs a single selector on
  plain Cargo without changing its definition.
- **Guard.** `effigy validate:oxide-routing` owns the inventory. It fails on a
  selector that spells a Cargo command without being switched or listed here,
  on a missing, edited, reordered or Oxide-tainted `:cargo` sibling, on a stale
  exception, on a script wrapper that acquired the helper, and on a changed
  release include, target ignore rule or agent block.
  `effigy check:oxide-routing` is its syntax and compile companion.

These routes stay on plain Cargo, and their reasons are durable:

- **Formatting (`format:*`).** `cargo fmt` is plain passthrough in Oxide, so
  routing it adds a process, not storage. `cargo fmt` is not a build route.
- **Live provider probes (`probe:*`).** They need explicit operator authority
  and spend, which an ordinary local build route must not carry.
- **Nested Cargo and private targets.** `run-with-owned-cargo-target.sh` owns
  its private `CARGO_TARGET_DIR` and cleanup, and Oxide replaces an inherited
  target with `<workspace>/target`: `check:sdk-courier-timeout`,
  `validate:sdk-courier-timeout`, `validate:sdk-courier-idle`.
  `validate:sdk-registered-lifetime` and `validate:sdk-unsolicited-close` select
  Claude SDK registered-tool tests whose nested courier build requires that
  run-owned target. `validate:current-grok-build-acp` selects Grok registered
  tool tests that build a courier into their own target and would inherit
  Oxide's slot build directory.
- **Whole-workspace and whole-package proof.** `test:rust` and
  `validate:focused` cover those same nested-Cargo test binaries, so they keep
  the milestone shape on Cargo.
- **Pinned toolchains.** `lint:current-pi-sdk-sidecar` and
  `lint:current-copilot-cli-acp` select a compiler with a leading
  `cargo +1.95.0`. Oxide refuses a leading `+toolchain`, and the MSRV proof
  keeps its exact pin.
- **Metadata, deny, docs and packaging.** `security:dependencies`,
  `package:docs`, `package:metadata`, `package:api`, `package:msrv`,
  `package:release-floor`, `package:source-consumer`,
  `package:verify-affected`, `format:package` and the
  `scripts/release/backports/...` runner keep plain Cargo: they are release or
  CI gates, metadata tools or archive builders, not ordinary build routes.
  `config/release.toml` gates stay plain Cargo for the same reason.
- **Mixed static proof clusters.** `check:copilot-acp-offline-proof`,
  `validate:copilot-acp-offline-proof`, `check:current-copilot-cli-acp`,
  `validate:current-copilot-cli-acp`, `validate:copilot-acp-private-assessment`,
  `check:copilot-acp-authenticated-proof` and
  `validate:copilot-acp-authenticated-proof` interleave Python validation with
  one Cargo step and are not ordinary local build entry points.
- **Sibling-example discovery.** Grok's MCP probe test finds a sibling example
  binary from `current_exe()` and no selector runs it, so it stays a Cargo-only
  test route; it is not an automatic switch candidate.

A route that changes a test result stays on Cargo and is reported as a
dogfood finding for Oxide, never hidden by changing runtime or test semantics.
The switch is qualified against the installed binary receipt and hash for that
run; the repository records no installation pin of its own.
