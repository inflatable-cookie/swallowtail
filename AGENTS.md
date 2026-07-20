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

## Continuation Rule

In a strict Northstar lane, a bare `continue` resumes the ready card named by
the previous closeout and `docs/roadmaps/README.md`.

Keep the active `## Next Task` pointer only in `docs/roadmaps/README.md`.

## Batch Size Rule

Work in meaningful batches. Inspect the ready card and nearby runway before
editing. Group related cards when one validation round can cover them. Stop and
re-scope if work becomes atomic churn.

## Roadmap Generation Rule

A roadmap generation is a long planning container, normally holding 30-50
numbered roadmap files. Batch cards do not count toward that range. Do not roll
to a new generation because a phase, contract set, or implementation layer
changes. Extend the active generation until it approaches that range or the
operator explicitly authorizes a structural rollover.

## Planning Ambiguity Rule

When the authority surfaces do not settle a direction, ask the operator rather
than inventing product policy.

## Reporting Rule

Use glue-light writing from `docs/policy/internal-writing-style.md`. For
meaningful closeouts: what changed, current state, failed or material
validation, next move.
