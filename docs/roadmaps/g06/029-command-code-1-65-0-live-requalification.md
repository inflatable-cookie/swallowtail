# g06.029 Command Code 1.65.0 Live Requalification

Owner: Tom
Created: 2026-09-24
Depends on: Contract 029; Contract 043; Research 330, 339; completed g06.006 and g06.021
Vision tags: route currentness, Command Code, live evidence, exact pin

## Outcome

Run one operator-authorized live gate on exact `command-code.headless`
`1.65.0` with `deepseek/deepseek-v4-flash`, repeating the g06.006 gate, and
move the live-derived cells from `1.54.0` to `1.65.0`.

## Why It Matters

g06.021 moved the exact point to `1.65.0`, but Research 330's live evidence
stays bound to `1.54.0`, so completion, tool lifecycle, usage and private
continuation cells are gated at the qualified point. Tom authorized the gate
on 2026-09-24.

## Ready-State Rubric

- [x] Research 339 froze `1.65.0` identity and digests.
- [x] The g06.006 probe (`tests/live_installed_probe.rs`, `live-probes`
      feature) already ran this gate shape on `1.54.0`.
- [x] Tom authorized one attempt and the pinned install it needs.
- [x] Scope, acceptance, validation, evidence and stop conditions are explicit.

## Decisions

- **Pinned install.** Host `command-code` is `1.54.0`. Install exactly
  `command-code@1.65.0` from the official npm registry, then verify
  `command-code --version` reports `1.65.0` and `dist/index.mjs` SHA-256
  matches Research 339. On any mismatch, stop. Never run `command-code
  update`, `/update` or `--list-models`; the adapter passes
  `--no-auto-update`.
- **Harness proof.** Rerun the probe's fake-provider tests on the same code
  path before the attempt; if the harness changed for `1.65.0`, commit the
  record-shape test first.
- Model `deepseek/deepseek-v4-flash`, the same model as g06.006, with the
  stored `~/.commandcode` account state. One attempt: one structured turn plus
  the Contract 043 two-turn private exact-id continuation. No rerun or
  substitution.
- Success moves the live-derived cells to exact `1.65.0`; a typed stop keeps
  them gated and names the model and failure.

## Dispatch manifest

- **State:** ready; provider-operation lane with one authorized gate.
- **Completion:** the gate runs once and is accepted or typed-stopped;
  matrices and guide agree; independent exact-head review accepts the head.
- **Owned mutable paths:** `crates/swallowtail-adapter-command-code/**` for
  probe and fixture changes; the Command Code prepared guide; its feature and
  activity matrix rows; one new research record and its index line;
  `CHANGELOG.md` `[Unreleased]`; `PAPERCUTS.md` append only.
- **Reserved closeout surfaces:** lifecycle task record and generated
  projections; submitted-handoff deletion belongs to the repository hook.
- **Excluded:** this card's prose; contracts; other families; any second
  attempt or model substitution; updates beyond the pinned install; release,
  tag, publication.
- **Worker evidence:** the authenticated run result and the research record.
  Retain no raw provider stream, account identifier, prompt, session id or
  private path.
- **Escalation:** operator via Chatterbox for credit, budget or model
  questions; Queue coordinator for mechanical blockers.

## Acceptance and review oracle

| Invariant | Adversarial counterexample | Required proof |
| --- | --- | --- |
| Exact executable | The gate runs on a self-updated or unverified binary | version and digest check recorded before the attempt |
| One attempt | A retry or model swap | run result shows one attempt |
| Cells follow evidence | A cell moves without live coverage | each moved cell cites the new record |
| Research 330 immutable | The `1.54.0` record is rewritten | no diff to Research 330 |

## Stop conditions

Stop before the attempt if the install or digest check fails, or if the
harness proof fails.

## Evidence

Research 330 and 339; the new research record. Focused and affected-package
validation for `swallowtail-adapter-command-code`, route and docs QA.

## Next Task

Chatterbox reconciles the live cells after closeout.
