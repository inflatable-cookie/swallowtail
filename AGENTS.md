# Swallowtail Agents

Swallowtail is a Rust workspace of provider-neutral agent-runtime crates: one
core vocabulary, a runtime, host support, and opt-in adapters that expose each
provider or harness through explicit capabilities and typed prepared paths.
Consumers such as Nucleus select an exact route; Swallowtail never flattens
providers into a generic router. Compatibility is claimed per exact
interface version from frozen artifacts, never from `latest`. Releases are
source-only annotated tags.

Swallowtail is standalone. Nucleus, Soundcheck, Monkey, Longhorn and future
consumers may provide evidence; they do not own Swallowtail decisions.

## Where things live

- Current state: `docs/README.md`
- Knowledge (one owner per fact): `docs/knowledge/README.md`
- Retired concepts, which must not come back: `docs/knowledge/retired.toml`
- Open questions: `docs/knowledge/questions.md`
- What's next: `docs/plan.md`
- Unresolved leads: `docs/triage/`
- Consumer guides, route and feature matrices: `docs/guides/`
- Retained research evidence: `docs/research/`
- Release compatibility notes: `docs/releases/`

Tasks, briefs, status and papercuts live in Queue, never in this repository.
File small recurring friction as a Queue papercut and carry on: from
`~/Dev/projects/paseo-northstar-queue`, run
`node bin/queue-cli.mjs papercut.add payload.json` with `repository`
(`origin: inflatable-cookie/swallowtail`, `path`), `title`, `happened` and
`impact`, plus optional `area` and `fix`.

## Commands

```sh
effigy tasks
effigy doctor
effigy test --plan
```

- `effigy validate:focused <pkg>...` — nextest and clippy for one to four
  exact workspace packages
- `effigy package:verify-affected <pkg>...` — package archive assembly for the
  same scope
- `effigy qa:docs` — docs, link, index and guide checks
- `effigy skill run northstar-lean/retired-concepts` — retired-concept check;
  kept out of repository QA, so run it before pushing docs changes
- `effigy qa:routes` — route, feature and activity matrix checks
- `effigy qa` — the full board

Prefer `effigy <task>` over raw commands. `effigy doctor` is orientation, not
validation. Do not add package scripts that merely re-export Effigy tasks.
Details: [validation tiers](docs/guides/validation-tiers.md).

## Product rules

- Do not implement runtime, provider, transport, or process behavior before
  the relevant contract is clear enough to test.
- Keep crates and modules small and focused. Keep provider-neutral vocabulary
  free of consumer and provider dependencies. Make dependency direction
  visible and acyclic.
- Expose provider differences through capabilities, not silent flattening.
- Keep product prompts, tools, policy, workflows, and durable state
  downstream.
- Format with `cargo fmt -p <crate>`. The workspace uses edition 2024; do not
  pass `rustfmt --edition 2021` on individual files.
- Wrap isolated provider probes in `scripts/run-with-isolated-home.sh
  --home-var GROK_HOME -- …`, or restore host `HOME` and unset provider-home
  variables before running `effigy` or `cargo`.
- **Version currentness.** Revalidate route families through the Contract 029
  checkpoint ([procedure](docs/knowledge/operations/version-currentness-checkpoint.md),
  repo skill `.cursor/skills/version-currentness/`). A checkpoint writes
  research; it does not change claims. Extend one family at a time; never
  bulk-bump from `latest`. A named incompatible reason is never an endpoint:
  it gets an adaptation task that qualifies the current release (Contract 029,
  No Terminal Stop).
- **Feature matrix.** An unavailable cell is exactly one of: a provider
  limitation citing frozen evidence; a producer gap naming the `docs/plan.md`
  item that builds it (`plan:<key>`); or evidence pending, naming an open
  question in `docs/knowledge/questions.md` that states the gate owner, the
  decision tree into the other two kinds, and the cells it covers. A cross
  with none of the three is a matrix defect. "Withheld" is a producer gap with
  a reason, never a finished result. Reconcile consumer requirements against
  required cells, not against truthful reporting. Rule detail:
  [route matrix](docs/guides/provider-route-matrix.md).

## Guardrails

- Releases are source-only annotated tags ([release](docs/knowledge/contracts/release.md),
  Contract 036). No gate, changelog or merge authorizes a tag, tag push,
  publication, GitHub Release, or consumer/provider mutation; each needs
  explicit operator authority naming the exact SHA. Never move or recreate a
  tag.
- Live provider turns, paid-model spend and credentials need operator
  authority. Prove the probe harness against fakes before spending a live
  attempt.
- When the knowledge files don't settle a direction, ask the operator rather
  than inventing product policy.
- When a change alters what is true, update the owning knowledge file in the
  same PR.
- An operator ruling given in conversation goes into its owning file before
  the thread ends.

## Shell snippets

Snippets run under the default interactive shell (zsh), not the bash shebangs
of `scripts/`. Never assign zsh special parameters as ordinary variables:
`path` is tied to `PATH`, and `status` is read-only. Use task-specific names
(`output_path`, `exit_status`) instead.

## Writing

Artifacts and PR descriptions use glue-light style:
[writing style](docs/knowledge/contracts/writing-style.md).

## Validate

`effigy validate:focused <pkg>...` for the packages you touched, plus
`effigy qa:docs` and `effigy qa:routes` when docs, guides or matrices change,
before opening a PR. Broader tiers only when the brief names them.

<!-- northstar:rust-quality:start -->
## Northstar Rust Quality

Scope: Rust source, Cargo manifests, build files, tests, and directly related
documentation under this directory.

Use Northstar's strict everyday-authoring route for ordinary Rust work. Resolve
the repository-owned profile and deviations under `docs/knowledge/contracts/`; never
assume a universal MSRV. Re-enter at task start and coherent batch closeout.
Preserve unrelated work. A quality audit, no-slop pass, or audit-and-fix request
is explicit audit intent; never route it through everyday authoring.
<!-- northstar:rust-quality:end -->
