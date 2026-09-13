# g05.054 v0.5.1 Release Candidate

Status: complete; PR #317 merged candidate 755a2669 at e9140b46 after independent exact-head review 5649975685
Owner: Tom
Created: 2026-09-13
Depends on: Contract 036; g05.051; g05.053; Desktop g02.089 Phase A
Vision tags: source release, patch, Grok catalogue, OpenCode currentness

## Outcome

Prepare and merge one source-only `v0.5.1` candidate containing the exact
Desktop-qualified g05.053 implementation. The candidate may change coordinated
version and release evidence only. Every path under `crates/` must remain
byte-identical to qualified Swallowtail main
`0209dd7ffc19c457ca56d55683576e09f049f7ee`.

This task stops before tag creation. Contract 036 requires the later annotated
tag authorization to name the exact candidate SHA. The operator's point-release
request authorizes candidate preparation and its gates; it does not erase that
exact-SHA checkpoint.

## Ready-State Rubric

- [x] Desktop Phase A passed against exact Swallowtail source `0209dd7f` and
      returned an independently reviewed consumer capsule.
- [x] g05.053 merged at `3e9f29e7`; its documentation-only closeout at
      `0209dd7f` did not change `crates/`.
- [x] Qualified `crates/` tree is
      `186f3ba42a4aa896c03a3bddcacb007b1afbd8cc`; qualified
      `swallowtail-adapter-grok` tree is
      `1a6f777f84f9aef80c146badf6944811568e23cd`.
- [x] Push CI run `34724808090` passed all 11 jobs at exact `0209dd7f`.
- [x] Read-only Effigy release status selects patch `0.5.1`; all seven local
      gates pass; local and remote `v0.5.1` are absent.
- [x] `[Unreleased]` contains only the compatible OpenCode `1.18.30`
      qualification and additive Grok `1.0.25` catalogue.

## Work

1. Start from clean pushed `main` at this planning commit. Verify `v0.5.0`
   remains immutable, `v0.5.1` remains absent locally and remotely, workspace
   version is `0.5.0`, the qualified source is an ancestor, and the frozen
   `crates/` and Grok adapter tree identities above still resolve.
2. Prepare patch `0.5.1` through
   `effigy --json release prepare --yes --check-gates --version 0.5.1`.
   Retain its receipt. A stale ignored release-state file from another release
   is not candidate evidence; a fresh queue workspace must not reuse it. A
   failed provider-free prepare does not consume release authority: retain its
   exact diagnostic, repair only within this manifest, and rerun in the same
   isolated workspace. Do not create a rehearsal copy solely to protect a
   one-shot allowance.
3. Promote the two real `[Unreleased]` entries into `0.5.1`. Add
   `docs/releases/0.5.1.md`, release index/install guidance, and fresh `0.5.1`
   package, route, dependency, and semantic API baselines required by current
   release gates. Before the prepare, restore only
   `release-baselines/production-routes-0.5.0.txt` and
   `release-baselines/public-api-0.5.0/swallowtail-adapter-grok.txt`
   byte-for-byte from immutable tag `v0.5.0`: g05.053 temporarily placed its
   post-tag route and API additions in those version-labelled files while the
   workspace still named `0.5.0`. Prove the restored blobs match the tag, then
   carry the additions into the fresh `0.5.1` baselines. Do not alter any other
   prior release note or baseline.
   Repair `scripts/check-consumer-front-door.py` so the current release note's
   Production Routes inventory is compared with the fresh current-version
   route baseline, not the previous tag's baseline. Permit the inherited
   unchanged-set shorthand only when the current and previous route sets are
   equal. `0.5.1` must truthfully list all 50 routes, including
   `grok-build.catalogue`; do not hide the addition in prose to satisfy the old
   assertion.
4. Prove `git diff --exit-code 0209dd7f..<candidate> -- crates/` and exact
   subtree equality. Version, lock, changelog, release note, baselines, and
   directly required release scripts/docs are the only candidate changes.
5. Run the complete provider-free candidate board, external exact-source
   consumer, and `effigy release execute --plan`. Open one PR and obtain
   independent exact-head review. Run qualifying hosted CI at the exact
   candidate or an identical tree; after merge, require exact merge-SHA CI
   when the merge changes identity.
6. Record candidate head, merge SHA/tree, qualified-source ancestry, frozen
   `crates/` and Grok subtree equality, version and package identities,
   preparation receipt digest, source-consumer result, hosted run and all job
   conclusions, and absence of local/remote `v0.5.1`, registry publication,
   GitHub Release, or other artifact. Return that exact SHA to Chatterbox for
   the separate annotated-tag authorization.

## Dispatch Manifest

| Field | Task 054 |
| --- | --- |
| Readiness | ready |
| Prerequisites | Contract 036; g05.051 and g05.053 complete; Desktop g02.089 Phase A green; clean pushed `main` |
| Completion conditions | successful `0.5.1` prepare receipt; release metadata and fresh baselines; zero `crates/**` diff from `0209dd7f`; provider-free/source-consumer gates; exact-head review; qualifying hosted CI; merged immutable candidate; exact identity returned for tag authorization |
| Owned mutable paths | `Cargo.toml`; `Cargo.lock`; `CHANGELOG.md`; root `README.md`; new `docs/releases/0.5.1.md`; `docs/releases/README.md`; fresh `release-baselines/*0.5.1*`; exact tag restoration of `release-baselines/production-routes-0.5.0.txt` and `release-baselines/public-api-0.5.0/swallowtail-adapter-grok.txt`; directly required release validation docs/scripts; this task and one candidate evidence log; `PAPERCUTS.md` append only |
| Reserved shared closeout surfaces | `docs/roadmaps/README.md`; `docs/roadmaps/g05/README.md`; `docs/roadmaps/generation-index.md`; logs index |
| Forbidden paths | every `crates/**` path; prior release notes and baselines except the two exact tag restorations above; `.github/workflows/**`; tags; registry/GitHub Release/artifact publication; provider or Desktop repositories |
| Approved concurrent siblings | none that changes source, release inputs, baselines, or closeout surfaces |
| Worker capability class | release-candidate worker; provider-free; ordinary automatic pool |
| Acceptance evidence | Desktop Phase A capsule; frozen source/subtree identities; one Effigy prepare receipt; exact-source consumer; semantic API and route baselines; independent review; exact-SHA hosted CI |
| Review oracle | reject any `crates/**` change, non-patch version, reused release state, prior-version change other than byte equality with the two named `v0.5.0` tag blobs, release note route inventory unequal to the fresh `0.5.1` baseline, missing source consumer, different-SHA CI, tag/publication mutation, or inferred registry availability |
| Stop conditions | dirty or divergent base; existing `v0.5.1`; qualified subtree mismatch; either restored blob differs from tag `v0.5.0`; deterministic prepare/gate/review/CI failure; required runtime change; any other historical mutation; tag or publication side effect |
| Escalation owner | Chatterbox for compatibility/tree/release semantics; queue coordinator for mechanical blockers; Tom for exact-SHA tag authorization |

## Boundaries

No provider call, Grok command, prompt, session, inference, tool, update, retry,
Desktop mutation, dependency upgrade, runtime change, workflow edit, tag,
crates.io publication, GitHub Release, binary, sidecar, installer, or model
artifact is authorized. Swallowtail remains `publish = false`; registry proof
for this source-only line is the verified absence of a registry publication,
not an invented package release.

## Validation

- `effigy --json release simulate`
- `effigy --json release status --check-gates`
- `effigy --json release prepare --yes --check-gates --version 0.5.1`
- `effigy release execute --plan`
- complete repository-owned candidate and source-consumer selectors
- `git diff --exit-code 0209dd7f..<candidate> -- crates/`
- exact candidate/merge hosted CI
- `git diff --check`

## Acceptance

- [x] coordinated version is `0.5.1`
- [x] qualified `crates/` and Grok adapter trees are unchanged
- [x] the two named `0.5.0` baselines match immutable tag `v0.5.0` exactly
- [x] release note, changelog, and fresh baselines describe the actual source
- [x] successful prepare receipt and exact-source consumer pass
- [x] independent review and exact-SHA hosted CI pass
- [x] candidate merges without tag or publication
- [x] exact candidate identity returns for the tag gate

## Next Task

None — merged. PR #317 landed candidate `755a2669` at `e9140b46` after
independent exact-head review `5649975685`; exact-head CI green at the
candidate (runs `34730275986` and `34730278564`); merge tree identical. The
immutable merged identity returns to Tom for exact-SHA authorization of the
`v0.5.1` annotated source tag. Desktop g02.089 remains held until that real
tag exists.
