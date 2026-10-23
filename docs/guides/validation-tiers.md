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

Tom expanded this approval on 2026-10-23: “Yes to both - let’s get all
those "no" rows transitioned”, relayed by the Oxide owner through Supervisor
`73f7c3b4-8e0e-4b67-a1d0-7a09f1f6b364`. Every entry point without a genuine
Cargo requirement moves through scoped implementation and independent review:

- Ordinary builds, checks and tests use Oxide. Metadata, locate-project,
  fetch, update, install, package, publish, tree, fmt, deny and audit retain
  their arguments through Oxide. Textual switching grants no authority to
  execute network, installation, publication or secret-bearing operations.
- Preserve toolchain pins with `RUSTUP_TOOLCHAIN=<same> oxide ...` or a
  suitable rust-toolchain file. Cargo-running host tools use supported
  `oxide exec -- <tool>` after project qualification. External commands
  currently take a brief slot lease; that is existing behaviour.
- Repair sibling-binary discovery with `CARGO_BIN_EXE_<name>` where
  applicable and prove unchanged test semantics before switching. An example
  target must not be assumed to expose a binary-target environment variable.
- Private `CARGO_TARGET_DIR` and temporary-workspace one-shot builds remain
  **pending Oxide one-use workspace support**. This includes release/consumer
  clones and private probes such as `run-with-owned-cargo-target.sh`. Do not
  switch them or drop their target ownership before qualified support ships.
- CI, container/VM guest builds, Tarpaulin with its own target, and tests
  whose subject is Cargo remain genuine Cargo exceptions. Preserve explicit
  `:cargo` fallbacks at least until formal Effigy v0.15; remove them only
  through owned work afterward. Other exceptions do not expire with v0.15.

Adoption permits the generated agent-document block and supported routing
changes, not global configuration changes, live provider probes or output
cleanup. Nucleus and consumer repositories are outside this project's scope.
A target symlink needs an exact non-directory ignore rule such as `/target`;
`/target/` does not suffice. Build-slot placement on Builds is separate from
final-output profile links. Profile links and one-use seeding are not shipped.

### Switched routes and their plain-Cargo escapes

A switched route runs `bash scripts/oxide-rust.sh <same arguments>`. The helper
passes those arguments to Oxide unchanged and replaces itself with the tool, so
argv, exit status, signals, stdout and stderr are the tool's own. That covers
ordinary builds, checks, clippy, tests and nextest, and the non-building
metadata, fmt, deny, doc and public-api routes. A route whose tool runs Cargo
itself uses the helper's `--host-tool` mode for one `oxide exec --` lease.

Every switched selector keeps a `<selector>:cargo` sibling. For a direct route
the sibling is the same arguments on plain `cargo`; for a maintained-script
route it runs the same script with `SWALLOWTAIL_RUST_RUNNER=cargo`. Neither
sibling puts Oxide anywhere on its route.

- **Hosted CI.** `GITHUB_ACTIONS=true` selects the plain-Cargo branch, because
  hosted runners have no `oxide`. Workflow files do not change.
- **Missing Oxide.** Locally, an absent `oxide` exits 127 and names both
  escapes. It never falls back to Cargo silently.
- **One-run escape.** `SWALLOWTAIL_RUST_RUNNER=cargo` runs a single route on
  plain Cargo without changing its definition.
- **Toolchain pins.** `cargo +<name>` becomes `RUSTUP_TOOLCHAIN=<name> oxide …`.
  The pin, compiler, features, filters, nextest profile and package scope are
  unchanged; the plain branch passes the same variable to Cargo.
- **Guard.** `effigy validate:oxide-routing` owns the inventory. It fails on a
  selector that spells a Cargo command without being switched or listed here,
  on a missing, changed, reordered or Oxide-tainted `:cargo` sibling, on a stale
  entry, on a switched route that selects a whole test binary containing
  private-target tests, on a switched script that runs a workspace-wide test
  lane, on a maintained script that disagrees with its classification, and on
  a changed release include, target ignore rule or agent block.
  `effigy check:oxide-routing` is its syntax and compile companion.

These routes stay on plain Cargo, and their reasons are durable:

- **Pending one-use workspace support.** A route that owns a private
  `CARGO_TARGET_DIR` or a temporary workspace keeps it until qualified Oxide
  one-use support ships: `run-with-owned-cargo-target.sh`
  (`check:sdk-courier-timeout`, `validate:sdk-courier-timeout`,
  `validate:sdk-courier-idle`), the Claude SDK and Grok registered-tool tests
  that build a courier target of their own
  (`validate:sdk-registered-lifetime`, `validate:sdk-unsolicited-close`,
  `validate:current-grok-build-acp`), the node SDK route whose filter selects
  the whole `claude_agent_sdk_driver` binary
  (`validate:current-claude-agent-sdk-node`), the whole-workspace test lane
  (`test:rust`, `package:release-floor`, the commented local-heavy release
  `test` recipe), whole-package
  `validate:focused`, `package:source-consumer`, `check:lease-api-fixture`,
  `package:verify-affected`, and the `sdk-v0.5.2` backport runner selectors.
- **Genuine exceptions.** Hosted CI keeps its unchanged raw-Cargo workflows.
  `observe:copilot-acp-private-assessment` reviews the pinned original entry
  invocation, and its subject is that invocation. Tests whose subject is the
  Cargo invocation force the helper's plain branch:
  `validate:format-selectors:test` asserts the exact `cargo fmt` argv.
  Container/VM guest builds and Tarpaulin with its own target are named
  exceptions, and this repository has neither.
- **Query precheck.** `scripts/validation/package-scope.sh` reads workspace
  metadata for the scope of pending routes and stays a plain query so those
  routes remain wholly off Oxide.
- **Historical evidence.** `release-candidates/0.1.0/` scripts and
  `docs/research/` command transcripts are frozen evidence, not maintained
  entry points.

A route that changes a test result stays on Cargo and is reported as a
dogfood finding for Oxide, never hidden by changing runtime or test semantics.
The switch is qualified against the installed binary receipt and hash for that
run; the repository records no installation pin of its own.
