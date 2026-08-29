# Swallowtail Agents

This file applies to the whole repository.

## Start Here

```sh
effigy tasks
effigy doctor
effigy test --plan
```

Prefer `effigy <task>` for supported work before raw commands. Do not add
package scripts that merely re-export Effigy tasks.

## Docs Authority

- `docs/README.md`
- `docs/vision/README.md`
- `docs/architecture/README.md`
- `docs/contracts/README.md`
- `docs/specs/README.md`
- `docs/roadmaps/README.md`
- `docs/logs/README.md`

Swallowtail is a standalone project. Nucleus, Soundcheck, Monkey, and future
consumers may provide evidence, but they do not own Swallowtail decisions.

## Project Posture

Swallowtail starts in strict Northstar posture.

- specs are provisional planning surfaces
- architecture records realized structure
- contracts hold durable rules and boundaries
- roadmaps sequence work
- logs record meaningful decisions and evidence

Do not implement runtime, provider, transport, or process behavior before the
relevant contracts are clear enough to test.

## Rust Code Shape

- keep crates and modules small and focused
- keep provider-neutral vocabulary free of consumer and provider dependencies
- make dependency direction visible and acyclic
- expose provider differences through capabilities, not silent flattening
- keep product prompts, tools, policy, workflows, and durable state downstream
- format with `cargo fmt -p <crate>`; the workspace uses edition 2024, so do not
  pass `rustfmt --edition 2021` on individual files
- wrap isolated provider probes in `scripts/run-with-isolated-home.sh
  --home-var GROK_HOME -- …`, or restore host `HOME` and unset provider-home
  variables before running `effigy`/`cargo`

## Continuation Rule

In a strict Northstar lane, a bare `continue` resumes the ready card named by
the previous closeout and `docs/roadmaps/README.md`.

Keep the active `## Next Task` pointer only in `docs/roadmaps/README.md`.

## Batch Size Rule

Work in meaningful batches. Inspect the ready card and nearby runway before
editing. Group related cards when one validation round can cover them. Stop and
re-scope if work becomes atomic churn.

## Validation Tier Rule

Use explicit package scope for normal batch feedback:

```sh
effigy validate:focused swallowtail-adapter-codex
effigy package:verify-affected swallowtail-adapter-codex
```

Both selectors accept one to four exact workspace package names. Do not infer
scope from changed files. Run broad `qa`, workspace tests, package checks,
candidate checks, consumer checks, MSRV checks, or live probes only when the
accepting card names that evidence tier.

## Roadmap Generation Rule

A roadmap generation is a long planning container, normally holding 30-50
numbered roadmap files. Batch cards do not count toward that range. Do not roll
to a new generation because a phase, contract set, or implementation layer
changes. Extend the active generation until it approaches that range or the
operator explicitly authorizes a structural rollover.

## Version Currentness Rule

Revalidate every production route family through the named Contract 029
checkpoint and `docs/guides/version-currentness-checkpoint.md`. This is a
standing lane: it is not a generation runway goal and does not keep a
generation open. The checkpoint writes research; it does not change claims.
Extend one family at a time through the Upgrade Workflow. Do not bulk-bump
from `latest`. Do not leave the current host or official stable
UnverifiedNewer without a named incompatible reason. Execute the lane
through the repo skill `version-currentness` at
`.cursor/skills/version-currentness/`. Sequencing lives in
`docs/roadmaps/standing-lanes.md`.

## Planning Ambiguity Rule

When the authority surfaces do not settle a direction, ask the operator rather
than inventing product policy.

## Reporting Rule

Use glue-light writing from `docs/policy/internal-writing-style.md`. For
meaningful closeouts: what changed, current state, failed or material
validation, next move.

<!-- northstar:rust-quality:start -->
## Northstar Rust Quality

Scope: Rust source, Cargo manifests, build files, tests, and directly related
documentation under this directory.

Use Northstar's strict everyday-authoring route for ordinary Rust work. Resolve
the repository-owned profile and deviations under `docs/contracts/`; never
assume a universal MSRV. Re-enter at task start and coherent batch closeout.
Preserve unrelated work. A quality audit, no-slop pass, or audit-and-fix request
is explicit audit intent; never route it through everyday authoring.
<!-- northstar:rust-quality:end -->
