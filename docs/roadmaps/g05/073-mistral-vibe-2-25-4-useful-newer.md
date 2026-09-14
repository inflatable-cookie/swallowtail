# g05.073 Mistral Vibe 2.25.4 Useful Newer

Owner: Tom
Created: 2026-09-14
Depends on: Contract 029; Research 150, 199, 252, 308, and 320; g05.072
Vision tags: route currentness, Mistral Vibe, headless, exact pin

## Outcome

Reopen the exact `mistral-vibe.release` pin and qualify official GitHub/PyPI
`2.25.4` for `mistral-vibe.headless` only if complete provider-free tag,
distribution-tree, CLI, streaming-wire, authority, and lifecycle evidence
admits it. Preserve `QualifiedOnly` and one exact maintained point. Do not
infer a range across the published successors after `2.24.2`.

Planning rechecked both official channels on 2026-09-14. GitHub latest is
`v2.25.4`, published `2026-09-12T18:39:34Z`, at lightweight tag commit
`19b5b74faa78d0816b8d4d4c7d7543fc3520678c`; PyPI latest is matching
non-yanked `2.25.4`. GitHub has eight stable successors: `v2.24.3`,
`v2.24.4`, `v2.24.5`, and `v2.25.0..=v2.25.4`. PyPI carries seven of them;
`2.24.4` is absent there and must remain a named packaging gap unless exact
official evidence settles otherwise. `vibe` remains absent from this host.

## Ready-State Rubric

- [x] Tom authorized the complete Research 308 currentness campaign, including
      reopening exact-pin families.
- [x] g05.072 completed and advanced Kiro to exact `2.21.4`; its lifecycle
      closeout is on canonical `main`.
- [x] Official GitHub/PyPI `2.25.4`, all eight GitHub successor tags, the PyPI
      `2.24.4` gap, and absent host CLI are identified separately.
- [x] Research 150 fixes the selected streaming wire, Research 199 fixes the
      caller-decreasing maximum-turn binding, and Research 252 fixes the
      Plan-only profile authority boundary.
- [x] No prompt, setup/login, installation, host update, or provider credit is
      needed for the identity and claim decision.

## Work

1. Re-probe GitHub latest stable, all releases and exact tag commits, release
   assets, PyPI metadata/distributions, ACP registry discovery metadata, and
   installed `--version` availability. Retrieve exact tagged source trees and
   PyPI sdists/wheels for `2.24.2..=2.25.4` into `/tmp`; verify every available
   digest and freeze Research 321. Keep GitHub `2.24.4` distinct from its
   missing PyPI distribution. Do not install or execute a downloaded artifact.
2. Derive a deterministic per-hop source/distribution tree ledger and mapped-
   module closure. Classify every changed input feeding `vibe --prompt
   --output streaming --max-turns <1..=8> --trust --agent plan --workdir`:
   argument parsing, Plan profile application, headless callback denial,
   completed-history NDJSON schema/deduplication, content and activity events,
   provider/limit/malformed/failure mapping, process exit, working-resource
   authority, deadline, task ownership, session close, and joined cleanup.
   Changelogs are discovery only; commit identity before production claim edits.
3. Keep `vibe-acp`, TUI, stdin prompt omission, `--continue`/`--resume`,
   teleport, JSON/text output, `--setup`, upgrade checks, `--auto-approve`/
   `--yolo`, `--max-price`, `--max-tokens`, worktrees/additional directories,
   other agent profiles, and ACP-registry lag unmapped only with concrete
   non-effect proof. Do not turn advertised capability into an operation.
4. If the selected contract is unchanged or mechanically adaptable without a
   new public lifecycle, authority, or operation, rebind the one exact claim
   point to `2.25.4`. Keep the behavior revision only when mapped wire and
   lifecycle are unchanged. Do not retain `2.24.2` as a second exact point,
   infer a range, or qualify the GitHub-only `2.24.4` packaging gap by adjacency.
5. Stop with the smallest exact counterexample if a mapped wire, lifecycle,
   permission/profile, authentication, limit, failure, or process-authority
   boundary changed materially, the channel mismatch affects selected identity,
   or base compatibility requires a live provider session. Recheck both
   official channels immediately before identity commit and push. Provider-free
   retrieval, classification, correction, and validation may rerun normally.

## Dispatch Manifest

| Field | Task 073 |
| --- | --- |
| Readiness | ready |
| Prerequisites | Contract 029; Research 150, 199, 252, 308, and 320; completed g05.072; clean pushed `main`; official GitHub tags/releases/assets and PyPI metadata/distributions |
| Completion conditions | all eight GitHub successors and seven matching PyPI points are frozen and classified; the `2.24.4` channel gap stays explicit; identity precedes claim; one honest exact-`2.25.4` decision lands; QualifiedOnly and route-family boundaries survive; exact-head review and named gates pass |
| Owned mutable paths | Mistral Vibe selection, prepared route, command, stream decoder/driver, tests, and fixtures under `crates/swallowtail-adapter-mistral-vibe/**`; Research 321 and one research-index line; exact Mistral Vibe prepared-guide, route/activity/feature matrices, architecture ceiling, standing-lane, `[Unreleased]`, identity-log, claim-log, and task-result lines; `PAPERCUTS.md` append only |
| Reserved shared closeout surfaces | lifecycle task record and generated projections; submitted-handoff deletion belongs to the repository hook |
| Forbidden paths | other adapters; contracts; historical research/captures/releases; Vibe ACP/TUI/teleport/setup/provider routes; live provider probes; release/tag/workflow files |
| Approved concurrent siblings | none on shared currentness surfaces; this campaign is serial |
| Worker capability class | evidence-first Rust currentness worker; exact Git/PyPI distribution ledger and structured-headless boundary reconciliation; no provider credentials |
| Acceptance evidence | GitHub baseline plus eight successor tag/release identities; PyPI baseline plus seven successor distribution identities and exact `2.24.4` absence; complete per-hop tree/module ledger; mutation-sensitive selection/command/stream/authority/lifecycle fixtures; current docs/claim agreement |
| Review oracle | skipped tag or distribution, erased channel gap, inferred range, unclassified mapped delta, changelog-only inference, ACP/TUI/profile/permission flattening, advertised-operation inference, lost limit/resource/cleanup boundary, provider reliance, or unjustified feature widening |
| Oracle gate | not required — Contract 029 and Research 150/199/252 settle this provider-free exact-pin comparison and its falsifiable stop conditions; no new auth, permission, secret, or execution policy is delegated |
| Stop conditions | official-channel disagreement affecting selected identity; selected public lifecycle/authority/authentication/permission/profile/limit/failure change; new driver/facade required; provider/live evidence required for base compatibility |
| Escalation owner | operator via Chatterbox for new policy; Queue coordinator for mechanical blockers |

## Boundaries

One route family and one exact-pin decision. No prompt, inference, setup/login,
credential use, live structured run, installation, host update, downloaded-
artifact execution, ACP/TUI/teleport/provider work, release, tag, publication,
or consumer mutation. Do not infer compatibility from adjacent tags, package
semver, changelogs, registry metadata, or advertised capabilities.

## Validation

- `cargo fmt -p swallowtail-adapter-mistral-vibe -- --check`
- `effigy validate:focused swallowtail-adapter-mistral-vibe`
- `effigy package:verify-affected swallowtail-adapter-mistral-vibe`
- `effigy check:examples`
- `effigy package:api`
- `effigy qa:routes`
- `effigy qa:northstar`
- focused Mistral Vibe identity, delta-ledger, selection, command, stream,
  activity, maximum-turn, permission/profile, and cleanup selectors
- relevant research/log/roadmap/number/lifecycle/next-action checks
- `git diff --check`

## Acceptance

- [ ] official GitHub and PyPI identities reproduce with the channel gap intact
- [ ] every changed selected input is classified
- [ ] identity evidence precedes the claim decision
- [ ] exact QualifiedOnly shape and route boundaries remain truthful
- [ ] advertised siblings and capabilities remain independently gated
- [ ] both official latest channels are rechecked at both boundaries

## Next Task

Return the exact outcome to Chatterbox, then compile and dispatch Deep Agents
as the next Research 308 family without asking Tom to repeat authorization.
