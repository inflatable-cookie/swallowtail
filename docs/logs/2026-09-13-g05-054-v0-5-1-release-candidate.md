# 2026-09-13 g05.054 v0.5.1 Release Candidate

Date: 2026-09-13
Task: `../roadmaps/g05/054-v0-5-1-release-candidate.md`
No provider call, credential access, Desktop edit, tag creation or push,
registry publication, or live-acceptance inference.

## Result

Source-only `v0.5.1` candidate prepared over immutable `v0.5.0` from amended
planning main `02bd284a`. The coordinated version is `0.5.1`, the patch bump
Contract 036 selects for compatible additions: the OpenCode HTTP `1.18.30`
qualification (g05.051, Research 304) and the exact Grok Build `1.0.25`
authenticated non-inference model catalogue with additive route
`grok-build.catalogue` (g05.052/053, Research 305/306). Every path under
`crates/` is byte-identical to Desktop-qualified source
`0209dd7ffc19c457ca56d55683576e09f049f7ee`; frozen `crates/` tree
`186f3ba42a4aa896c03a3bddcacb007b1afbd8cc` and `swallowtail-adapter-grok`
subtree `1a6f777f84f9aef80c146badf6944811568e23cd` are unchanged.

One authorized
`effigy --json release prepare --yes --check-gates --version 0.5.1` completed
with all seven cheap gates green in repository order (fmt 2.0s, qa 11.4s,
docs 9.3s, metadata 0.1s, api 39.8s, security 0.8s, source 14.2s). Successful
JSON capture SHA-256
`146d95d7bf0aeef8419bb3a73fc2e80fff2324f6ba59e1f5eebc8e0cb836eca7`; state
receipt `.release-prepared.json` (`prepared_at`
`2026-09-13T01:11:22.728017+00:00`, previous `0.5.0`, tag `v0.5.1`,
`version_override_used: true` for the explicit `--version` flag while
suggestion and request both equal `0.5.1`) SHA-256
`2da0fc03de3053c214375c1d543390124f3e944202563eb78772aa9f587644a8`.

## Recoverable Prepare Attempt History

Per the amended Contract 036, provider-free local prepare is recoverable, not
one-shot. The first prepare attempt failed fast at the `qa` gate (9.9s):
`consumer front-door check failed: missing
release-baselines/production-routes-0.5.1.txt` — the fresh route and
dependency baselines existed only in a since-removed rehearsal scratch, not
in this workspace. Failed capture SHA-256
`4aedd0fd04adbd3beb97e95c9d3aec708363bc2ff0eec8553fcffd2cbe8945a4`. The
attempt rolled back cleanly (workspace still `0.5.0`, no state file). The
repair added `release-baselines/production-routes-0.5.1.txt` (50 routes) and
`release-baselines/internal-dependencies-0.5.1.tsv` (91 edges at `^0.5.1`)
inside the manifest, and the same isolated workspace rerun passed all gates.
The disposable rehearsal checkout was abandoned per the amendment.

## Tagged Baseline Restoration

Exactly the two named `v0.5.0` baselines were restored byte-for-byte from
immutable tag `v0.5.0` before the fresh `0.5.1` baselines were generated;
g05.053 had temporarily advanced them while the manifest still named `0.5.0`:

- `release-baselines/production-routes-0.5.0.txt` — working blob
  `c01a87d231d1918b5bab9a507e3a08214f678ab5` equals tag blob (49 routes)
- `release-baselines/public-api-0.5.0/swallowtail-adapter-grok.txt` — working
  blob `0dba826fefb1b59a3d8541196fb518057ba969ca` equals tag blob

The post-tag additions carried into fresh `0.5.1` baselines:
`release-baselines/production-routes-0.5.1.txt` (50 routes including
`grok-build.catalogue`), `release-baselines/public-api-0.5.1/` (40 packages,
pinned generator `cargo-public-api 0.52.0` + `nightly-2026-08-05`), and
`release-baselines/internal-dependencies-0.5.1.tsv` (91 edges, edge set
identical to `0.5.0`). No other prior release note or baseline changed;
`docs/releases/0.5.0.md` is untouched.

## Front-Door Repair

`scripts/check-consumer-front-door.py` now compares the current release
note's Production Routes inventory with the fresh current-version route
baseline instead of the previous tag's baseline, and admits the inherited
unchanged-set shorthand only when the current and previous route sets are
equal. `docs/releases/0.5.1.md` truthfully lists all 50 routes including
`grok-build.catalogue`.

## Validation

Post-prepare proofs: `git diff --exit-code 0209dd7f..working -- crates/`
empty; `git diff --check` clean; coordinated version `0.5.1` across the
workspace manifest and lock. `effigy qa` full candidate board green
(docs/index/status/guide checks, northstar spine, route/lifecycle/feature/
activity matrices at the 50-route inventory, fmt, clippy, and 3520 nextest
tests across 195 binaries). `effigy --json release status --check-gates`
re-passed all seven gates post-prepare; its `ready: false` reflects only the
correctly emptied `[Unreleased]` section. `effigy release execute --plan`
previewed the execute step and correctly blocked on uncommitted prepared
files; execution and any tag mutation remain outside this task's authority.
The `source` gate passed the isolated external Git-source consumer at the
prepared tree. Zero provider contact.

## Surfaces

`Cargo.toml` / `Cargo.lock` / `CHANGELOG.md` / root `README.md`,
`docs/releases/0.5.1.md`, `docs/releases/README.md`,
`scripts/check-consumer-front-door.py`, the two restored `v0.5.0` baselines,
fresh `release-baselines/*0.5.1*`, this log and its index line. `v0.5.1`
remains absent locally and remotely; `v0.5.0` remains immutable at
`582d01d6b6890eed5195a1fcbee0ae985304a6c7` (tag object `c772c583`). No
crates.io publication, GitHub Release object, binary, sidecar, installer, or
model artifact exists or follows from this task.

## Next

Independent exact-head review and qualifying hosted `CI` at the exact
candidate head; merge is queue-owned. The exact merged candidate identity
returns to Tom for the separate annotated-tag authorization. Desktop g02.089
stays held until the real `v0.5.1` tag exists.
