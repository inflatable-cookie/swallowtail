# g05.047 v0.5.0 Release Candidate

Status: ready; candidate preparation authorized; tag not authorized
Owner: Tom
Created: 2026-09-10
Depends on: Contract 036; release playbook; tagged `v0.4.4`; g05.044,
g05.045, and g05.046
Vision tags: source release, compatibility, Claude route, Pi RPC

## Outcome

Prepare and merge one immutable source-only `v0.5.0` candidate carrying the
post-`v0.4.4` source. Run the one authorized Effigy preparation, all cheap
provider-free gates including the exact-source consumer, independent exact-head
review, and qualifying hosted CI. Freeze the accepted merge SHA and tree, then
stop before any tag.

## Version Decision

`v0.4.4` publicly said every admitted native tool under Claude SDK `default`
mode was offered through `canUseTool`. g05.045 proved native in-workspace
`Read` may complete without that callback and corrected the guarantee.
Contract 036 treats a guaranteed-behavior shrink as breaking before 1.0. The
same rule previously forced the `0.3.x` to `0.4.0` boundary. The next candidate
is therefore `0.5.0`, not `0.4.5`.

Tom explicitly authorized `v0.5.0` candidate preparation on 2026-09-10 after
this classification was surfaced. That authorization covers candidate
mutation and its provider-free/source-consumer/hosted gates. It does not cover
tag creation or push.

## Candidate Content

- g05.044 raises the qualified Pi RPC ceiling to official `0.85.1`; the
  `pi.sdk-sidecar` axis stays exact `0.84.2`.
- g05.045 records the exact Claude native mediation limitation from the three
  immutable Desktop g02.049 capsules. `permission_exchange=Yes` remains narrow
  to callbacks the SDK emits; universal native mediation is not claimed.
- g05.046 adds `ClaudeAgentSdkRegisteredOnlyBinding`: non-empty selected
  registered tools, zero native SDK tools, explicit `Read` or `ReadWrite`, all
  seven native tools structurally disallowed, and unchanged additive behavior.
- Documentation lifecycle and acceptance-audit changes since `v0.4.4` carry no
  extra runtime, provider, publication, or consumer authority.

## Ready-State Rubric

- [x] `v0.4.4` is immutable at `49c9e3b291609c9ebf5b35a284c08302f3b8d5e3`.
- [x] g05.044, g05.045, and g05.046 are merged and independently reviewed.
- [x] Current `main` is clean and equals `origin/main` at the g05.046 closeout
      `98894512e3042d1e2355c31d9385d7418dbe0e89` before this planning change.
- [x] Contract 036 classifies the corrected native-mediation guarantee as a
      pre-1.0 minor boundary.
- [x] Tom authorized `v0.5.0` candidate preparation and gates on 2026-09-10.
- [x] `v0.5.0` is absent locally and remotely at planning time.

## Dispatch Manifest

- **State:** ready; dispatch after this planning commit reaches pushed `main`.
- **Completion:** coordinated version `0.5.0`; changelog and release note;
  fresh `0.5.0` public API, route, and internal-dependency baselines; updated
  install examples; one successful `effigy --json release prepare --yes
  --check-gates --version 0.5.0`; exact-source consumer pass; independent
  exact-head review; qualifying hosted CI; merged immutable candidate SHA/tree;
  preparation receipt and closeout.
- **Owned mutable paths:** this task; `Cargo.toml`, workspace package manifests
  touched by coordinated versioning, `Cargo.lock`, `CHANGELOG.md`, root
  `README.md`, `docs/releases/0.5.0.md`, `docs/releases/README.md`, fresh
  `release-baselines/*-0.5.0*`, and the exact release scripts/docs corrections
  required for one valid preparation; one new log and indexes; `PAPERCUTS.md`
  append only.
- **Reserved closeout surfaces:** `docs/roadmaps/README.md`,
  `docs/roadmaps/g05/README.md`, and `docs/roadmaps/generation-index.md`.
- **Worker:** release-preparation worker; no provider access.
- **Feature freeze:** from this planning commit through the exact candidate
  disposition, no unrelated runtime or compatibility PR merges to `main`.
  Documentation-only queue closeout may advance `main` without changing the
  frozen candidate tree.
- **Excluded:** provider calls; credentials; Desktop or other consumer
  mutation; live application smoke; dependency repin; local or remote tag;
  crates.io; GitHub Release; binary, sidecar, installer, or model publication.
- **Escalation:** Chatterbox owns compatibility and candidate semantics. Tom
  owns the exact-SHA tag decision and any later consumer/live gates.

## Work

1. Recheck clean pushed `main`, canonical remote, immutable `v0.4.4`, absent
   local and remote `v0.5.0`, current coordinated version, and the complete
   `v0.4.4..HEAD` source delta. Stop on unrelated uncommitted work or a
   conflicting tag.
2. Classify the candidate honestly. Preserve the minor-version reason above;
   do not downgrade it to patch. Write `docs/releases/0.5.0.md`, move the real
   entries from `[Unreleased]` to `[0.5.0]`, update install/rollback guidance,
   and keep source-only distribution explicit.
3. Preseed fresh `0.5.0` semantic public API, production-route, and internal-
   dependency baselines from the current candidate tree. Never rewrite an
   earlier baseline. Update root version examples.
4. Run exactly one `effigy --json release prepare --yes --check-gates
   --version 0.5.0`. Preserve its receipt. Cheap gates run in repository order:
   format, docs/route QA, public docs, metadata, semantic API, security, and
   exact-source consumer. No second prepare follows a deterministic failure;
   return the exact failed gate to Chatterbox.
5. Open one candidate PR. Run independent exact-head review and qualifying
   hosted `CI` in parallel. PR-event CI alone does not qualify because its MSRV
   floor omits tests.
6. Merge only when review and every required check are green. If the merge SHA
   has a different tree from the qualifying run, dispatch one
   `workflow_dispatch` run at the merge SHA and require all jobs green.
7. Record candidate head, merge SHA, tree, base tree, version, release receipt
   digest, hosted run ID and trigger, all gate results, elapsed wall clock, and
   zero provider contact. Stop and request the separate exact-SHA tag decision.

## Review Oracle

| Invariant | Adversarial counterexample | Required proof |
| --- | --- | --- |
| Version is `0.5.0` | use `0.4.5` because runtime changes look additive | release note cites the withdrawn `v0.4.4` guarantee and Contract 036 minor rule |
| Prior release is immutable | edit a `0.4.4` baseline or move its tag | tag peel and baseline diffs show no historical mutation |
| Candidate content is exact | omit Pi qualification, limitation wording, or registered-only API | release note, changelog, API and route inventories match `v0.4.4..candidate` |
| Preparation is one-shot | rerun after a deterministic failure or hand-edit around a gate | one receipt and one prepare invocation; failure stops the task |
| Source consumer is exact | compile local paths or a moving branch | isolated consumer resolves every selected package from the candidate source identity |
| Hosted evidence qualifies | rely on PR CI with skipped floor tests or another tree | `workflow_dispatch`/push run is green at candidate SHA or an identical tree |
| Tag gate stays closed | create or push `v0.5.0` after green CI | local/remote tag remain absent; closeout asks for exact-SHA authorization |

## Stop Conditions

- Stop if the tree is dirty, canonical remote differs, `v0.5.0` exists, the
  prepared version is not strictly greater than `0.4.4`, or an earlier tag or
  baseline would need mutation.
- Stop on a deterministic prepare, semantic API, security, source-consumer,
  documentation, review, or hosted-CI failure. Do not bypass or silently retry.
- Stop if the candidate includes an unreviewed breaking API change beyond the
  documented native-mediation guarantee correction.
- Stop before provider contact, credential use, Desktop mutation, tag creation
  or push, registry publication, GitHub Release creation, or artifact upload.

## Evidence

On completion record the candidate PR/head/review/merge/closeout, exact commit
and tree identities, version classification, preparation receipt SHA-256,
provider-free gate results, source-consumer identity, qualifying hosted run and
all job conclusions, zero provider calls, local/remote tag absence, and every
retained non-claim.

## Next Task

Return the frozen `v0.5.0` candidate SHA and qualifying CI to Tom for the
separate annotated-tag decision. After an authorized tag, run the source-tag
consumer/working-application lane and resume blocked Desktop g02.051 against
the exact coordinated tag. No tag or consumer mutation follows automatically.
