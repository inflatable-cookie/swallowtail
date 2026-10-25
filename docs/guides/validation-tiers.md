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

Extracted packages keep the `crates/<package>` position, so a test that embeds
workspace evidence through `../../../` resolves against that temporary root;
the run copies only the read-only workspace lock and retained research records
there, never into a package. A test that embeds a sibling package's fixtures
(`../swallowtail-<package>/tests/...`) needs that sibling in the same group,
because the extracted tree contains only the selected packages.

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

### Tom's 2026-10-25 ruling

Tom's Oxide Supervisor `d5260e79-e0f9-47d6-b854-123a59e0b1d3` relayed Tom's
2026-10-25 ruling: every ordinary local Rust route uses `oxide <cmd>` or
`oxide exec -- <tool>` when a tool runs Cargo itself or needs its own target.
It supersedes the earlier one-use deferral and per-selector escape retention:
there are no `<selector>:cargo` sibling selectors and no project escape
variable. The only local escapes are Effigy's `EFFIGY_OXIDE_ESCAPE=cargo` for
one run and running Cargo by hand. Missing Oxide never silently selects Cargo.

Run `oxide init` for the managed guidance. The switch preserved private-target
ownership, cleanup, toolchain pins, filters, profiles and test semantics. It
grants no authority to change target custody, remove outputs, install tools or
run release or provider commands. `.github` CI and product code or fixtures
whose subject is Cargo retain their recipes. The installed Oxide workflow
contract owns supported storage and process behaviour; a routed wrapper is not
proof of cache reuse or containment, and this repository claims no
fingerprint-equivalence or saving from routing.

Tom's 2026-10-23 standing approval covers supported Oxide adoption in Rust
projects with Paseo workspaces (decisions
`3b041cf4-df3d-4e17-9886-5af51428ecf3` and
`e1c1d3b5-9ff2-4f42-81b6-7e2a3f6e3a14`). The
[Oxide workflow contract](https://github.com/inflatable-cookie/oxide/blob/main/docs/knowledge/contracts/workflow.md#switching-an-existing-project)
owns switching semantics. Effigy still owns task routing and host admission.

Adoption permits the generated agent-document block and supported routing
changes, not global configuration changes, live provider probes or output
cleanup. Nucleus and consumer repositories are outside this project's scope.
A target symlink needs an exact non-directory ignore rule such as `/target`;
`/target/` does not suffice. Build-slot placement on Builds is separate from
final-output profile links. Profile links and one-use seeding are not shipped.

### Routes and their single escape

`scripts/oxide-rust.sh` is the one place that spells a plain Cargo command. A
route runs `bash scripts/oxide-rust.sh <cargo command> <same arguments>`; the
helper passes those arguments to Oxide unchanged and replaces itself with the
tool, so argv, exit status, signals, stdout and stderr are the tool's own. That
covers ordinary builds, checks, clippy, tests and nextest, and the
non-building metadata, fmt, deny, doc and public-api routes. The maintained
scripts that own a validation or release proof route their Cargo commands
through the same helper.

- **Hosted CI.** `GITHUB_ACTIONS=true` selects the plain-Cargo branch, because
  hosted runners have no `oxide`. `.github/workflows` stays byte-identical and
  keeps its raw Cargo commands; the helper branch is the only CI-specific step.
- **One-run escape.** `EFFIGY_OXIDE_ESCAPE=cargo effigy <selector>` is Effigy's
  escape, honoured by the helper for that run and never persisted in the
  manifest or the environment. Any other value refuses before a tool starts.
  Running `cargo` by hand is the only other plain route.
- **Missing Oxide.** Locally, an absent `oxide` exits 127 and names the escape.
  It never falls back to Cargo silently.
- **Toolchain pins.** `cargo +<name>` becomes `RUSTUP_TOOLCHAIN=<name> oxide …`.
  The pin, compiler, features, filters, nextest profile and package scope are
  unchanged; the plain branch passes the same variable to Cargo.
- **Guard.** `effigy validate:oxide-routing` owns the inventory. It fails on a
  `:cargo` escape selector, on the retired project escape variable, on a
  selector that spells a Cargo command without being routed or listed here, on
  a selector that takes `--host-tool` itself instead of letting the tool own its
  lease, on a leading `+toolchain` argument, on a changed command list for the
  routes this guide transitioned, on a maintained script that disagrees with its
  classification, on a private target without its marker and cleanup, on a
  foreign-workspace driver without its own build directory, and on a changed
  release include, workflow, target ignore rule or agent block.
  `effigy check:oxide-routing` is its syntax and compile companion.

### Tools, private targets and temporary workspaces

A maintained script that runs Cargo itself re-executes once through the
helper's `--host-tool` mode, which is one `oxide exec --` lease. Its own
`cargo` words are the tool's, so no route takes two leases:

- `run-with-owned-cargo-target.sh` runs the courier and SDK validation groups
  under one fresh run-owned `CARGO_TARGET_DIR`. The outer build and the nested
  courier build share that warm target, which this invocation alone owns; the
  directory is created under the system temp root, marked
  `.swallowtail-run-owned`, and removed on exit. `CARGO_TARGET_DIR` keeps every
  final output for the run private while the intermediates go to the held slot,
  which is this checkout's own intermediate state.
- `run-grok-acp-client-mcp-probe.sh` builds and runs its echo example inside the
  lease; final outputs stay in `<checkout>/target/debug/`.
- `validate-focused-packages.sh`, `check-release-floor.sh`, `check-msrv.sh`,
  `check-package-metadata.sh`, `generate-public-api-baseline.sh`,
  `format-packages.sh` and `scripts/validation/package-scope.sh` are ordinary
  routes: they call the helper, and the whole-workspace and package lanes keep
  their own scope, filters and pins. Their nested courier builds take the
  task-owned target the test helper creates when `CARGO_TARGET_DIR` is the
  checkout's own target.

### Named proof exceptions: foreign workspaces

These proofs build a workspace that is not this checkout, so each one keeps
both its own target directory and its own build directory inside its run-owned
scratch root. Another workspace's finals and intermediates therefore never
enter this checkout's `target` or this project's shared slot, and the run
removes only the directory it created:

- `verify-affected-packages.sh` (`package:verify-affected`) archives packages
  and compiles the extracted subset in a temporary root;
- `verify-source-consumer.sh` (`package:source-consumer`) generates a consumer
  workspace and lock outside this checkout;
- `verify-lease-api-fixture.sh` (`check:lease-api-fixture`) compiles the
  released lease API from a temporary external consumer;
- the `sdk-v0.5.2` backport runner (`format:sdk-patch-backport`,
  `check:sdk-patch-backport`, `validate:sdk-patch-backport`) stages a private
  candidate and consumer clone with their own targets, build directories and
  pinned toolchains.

Each still takes exactly one `oxide exec --` lease, so the route, admission and
refusal wording are Oxide's; the private build directory is the contract's
named exception for a proof that must build outside the shared project state.

### Genuine Cargo exceptions

These stay on plain Cargo, and their reasons are durable:

- **Hosted CI.** `.github/workflows` keeps its raw-Cargo commands; hosted
  runners have no `oxide`. The release gates that hosted CI calls stay on
  Effigy selectors, which route as described above.
- **The pinned original invocation.**
  `observe:copilot-acp-private-assessment` reviews
  `scripts/copilot-acp-original-entry.sh`, whose subject is that exact
  invocation and which needs original authority.
- **A test whose subject is the Cargo invocation.**
  `validate:format-selectors:test` asserts the exact `cargo fmt` argv of
  `format:package`, so it takes Effigy's one-run escape.
- **Product code and fixtures whose subject is Cargo.** Crate test fixtures
  that run Cargo as their behaviour, and historical research transcripts, keep
  their literal recipes. They are evidence, not entry points.
- **Container/VM guest builds and Tarpaulin with its own target** are named
  exceptions, and this repository has neither.
- **Historical evidence.** `release-candidates/0.1.0/` scripts and
  `docs/research/` transcripts are frozen, not maintained entry points.

A route that changes a test result stays on Cargo and is reported as a dogfood
finding for Oxide, never hidden by changing runtime or test semantics. The
switch is qualified against the installed binary receipt and hash for that run;
the repository records no installation pin of its own.
