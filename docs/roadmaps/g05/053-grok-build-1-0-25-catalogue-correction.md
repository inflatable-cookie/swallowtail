# g05.053 Grok Build 1.0.25 Catalogue Correction

Status: complete; PR #316 merged at 3e9f29e7 after independent exact-head review 5649291554
Owner: Tom
Created: 2026-09-12
Depends on: Contracts 020 and 047; g05.052; PR #315; installed Grok `1.0.25`
Vision tags: model catalogue, Grok Build, authenticated metadata, consumer truth

## Outcome

Deliver the exact installed Grok `1.0.25` pre-session model catalogue through
a separately prepared `ModelCatalog` operation. Classify exact argv
`--no-auto-update models` honestly as an authenticated, non-inference metadata
command. It may refresh authentication or catalogue metadata. It must not send
a prompt, open a model session, invoke inference, dispatch a tool, update the
harness, retry, or retain provider state beyond the bounded operation.

The prior `ProviderSuppressed` requirement was not traced to the Desktop
consumer need. It incorrectly turned Grok's internal auth-refresh watcher into
a product failure even though the final observation exited zero, parsed the
requested catalogue, opened no model session, performed no inference, joined
cleanup, and succeeded with a zero-credit account. Remove that requirement and
the private suppression machinery rather than adding an operating-system
network sandbox.

## Ready-State Rubric

- [x] The operator rejected g05.052's non-admission and explicitly instructed
      Chatterbox to fix the seam.
- [x] The operator confirmed that authenticated catalogue metadata traffic is
      acceptable; the required boundary is no prompt, session, tool, or
      inference.
- [x] Swallowtail host-local clears ambient environment before applying exact
      approved environment bindings.
- [x] `--no-auto-update` is accepted only before the `models` subcommand.
- [x] The previous enterprise-precedence and remote-fetch checks are removed
      because neither protects the accepted non-inference boundary.
- [x] No prompt, model session, provider inference, release, tag, or Desktop
      mutation is required.

## Work

1. Freeze Research 306 from the exact `1.0.25` command grammar, installed
   executable identity, host-local launch semantics, PR #315 review history,
   and all observations. Correct Research 305 and its ruling log narrowly: the
   earlier verdict remains historical, while the final successful capsule and
   corrected authenticated-metadata boundary supersede its non-admission.
2. Recover the admitted implementation from PR #315's preserved Git objects
   rather than rewriting it from memory. Keep exact `--no-auto-update models`,
   bounded capture, redaction, lifecycle, and the shipped bullet grammar.
   Remove the operation-private suppression file, enterprise-precedence gate,
   `ProviderSuppressed` binding, and suppression-only public types/tests. Use
   `HarnessConfigurationPosture::Ambient`; catalogue authority remains
   separate from inference authority.
3. Preserve the previously reviewed bounded parser and exact-`1.0.25`
   supplemental metadata projection: live listing owns membership, order, and
   default; the frozen embedded document supplements exact matching IDs only.
   Unknown valid IDs pass through. Missing source fields remain absent.
4. Prove with fake processes and the real local host that exact argv,
   executable, bounded output, redaction, success, failure, deadline,
   disconnect, and cleanup remain deterministic and joined. Do not add tests
   for the removed suppression machinery.
5. Preserve the earlier observations as evidence. The replacement
   process exited zero with 113 stdout bytes and zero stderr bytes; exact binary
   format pieces prove the parser rejected Grok's shipped `*`/`-` bullet rows,
   not the catalogue membership. Its unwind guard removed the private home
   before the runtime suppression proof was retained. After the corrected
   bullet parser, persistent redacted capsule, and revised provider-free gates
   pass. The final observation is already sufficient and must not be repeated:
   it parsed ordered `grok-4.6` default then `grok-4.5`, exited zero, opened no
   session, performed no inference, joined cleanup, and persisted redacted
   counts/digests. Its auth-refresh watcher and absence of a fresh remote-origin
   cache are descriptive metadata-path evidence, not acceptance gates.
6. Restore the additive post-`v0.5.0` route-50 current-source inventory using
   g05.052's 2026-09-12 blocker ruling: current working baselines may advance;
   tagged `v0.5.0`, Research 281, historical ledgers, and release notes remain
   byte-immutable. Deliver the exact merged-source adoption capsule to Desktop
   Chatterbox.

## Dispatch Manifest

| Field | Task 053 |
| --- | --- |
| Readiness | ready |
| Prerequisites | Contracts 020 and 047; g05.052/PR #315 evidence; exact installed Grok `1.0.25`; clean pushed `main` |
| Completion conditions | Research 306 freezes the observations and corrected metadata boundary; the preserved implementation has the shipped bullet grammar and failure-safe redacted evidence; the existing final capsule is accepted without another live run; exact-head review and all named validation pass; Desktop receives the source-linked capsule |
| Owned mutable paths | `crates/swallowtail-adapter-grok/**`; exact Grok rows in current route/feature/activity matrices and integration guides; `CHANGELOG.md` `[Unreleased]`; current-role `release-baselines/production-routes-0.5.0.txt` and `release-baselines/public-api-0.5.0/swallowtail-adapter-grok.txt`; narrowly required route/front-door/activity validation scripts and tests; `docs/research/305-grok-1-0-25-model-catalogue-evidence.md`, new `docs/research/306-*.md`, and research index; `docs/logs/2026-09-12-g05-052-grok-1-0-25-model-catalogue-ruling.md`, one new corrective claim log, and logs index; this task's result/status; `PAPERCUTS.md` append only |
| Reserved shared closeout surfaces | `docs/roadmaps/README.md`, `docs/roadmaps/g05/README.md`, and `docs/roadmaps/generation-index.md`; queue coordinator edits these at closeout |
| Forbidden paths | `docs/releases/0.5.0.md`; Research 281 and every historical freeze ledger/baseline; Grok ACP execution selection/claim behavior; contracts; every other adapter; Desktop or consumer repositories; release/tag/workflow files |
| Approved concurrent siblings | none on Grok, current inventory, or closeout surfaces |
| Worker capability class | Rust/evidence correction worker; simplify the retained catalogue implementation to the authenticated non-inference boundary; no further live observation |
| Acceptance evidence | exact shipped Grok command grammar and executable identity; local-host launch proof; mutation-sensitive fake-process corpus; exact argv capture; one redacted live catalogue capsule; current-vs-tagged inventory regression; focused package and route gates |
| Review oracle | the smallest counterexample is omitting `--no-auto-update`, sending a prompt, opening a model session, invoking inference or a tool, trusting the embedded document for live membership, mutating tagged evidence, retaining raw account data, or collapsing catalogue into ACP execution |
| Stop conditions | the command prompts, creates a model session, invokes inference or a tool, updates the harness, cannot close/join, exceeds bounds, leaks raw account data, or the provider-free gates fail |
| Escalation owner | operator via Chatterbox for semantic/authority questions; queue coordinator for mechanical blockers |

## Boundaries

Tom accepted the authenticated non-inference metadata boundary on 2026-09-12.
The existing final observation is the acceptance evidence; no further Grok
run is authorized or needed. Its zero-credit success supports the observed
non-inference classification but does not prove entitlement or future billing
state.

This corrects the catalogue seam only. It does not widen Grok ACP execution,
change `grok_build_model_for_version`, infer entitlement from listing, hard-code
models in Desktop, create a generic router, install/update Grok, edit host or
fleet configuration, create a release/tag, publish, or mutate a consumer.
Credentials and raw host configuration remain private. The catalogue listing
is not a model prompt and grants no inference authority.

## Validation

- `cargo fmt -p swallowtail-adapter-grok -- --check`
- `effigy validate:focused swallowtail-adapter-grok`
- `effigy package:verify-affected swallowtail-adapter-grok`
- `effigy check:examples`
- `effigy package:api`
- `effigy qa:routes`
- `effigy qa:consumer-docs`
- `effigy qa:northstar`
- focused local-host environment-clearing and catalogue selectors
- research, logs, roadmaps, g05, roadmap-number, status, next-action, and link checks
- `git diff --check`

## Acceptance

- [x] exact argv is `--no-auto-update models`
- [x] no prompt, model session, tool dispatch, inference, update, or retry occurs
- [x] listing owns membership/order/default; supplemental metadata stays exact-ID only
- [x] lifecycle, bounds, redaction, and cleanup pass
- [x] tagged `v0.5.0` and historical evidence remain immutable
- [x] Desktop receives an exact-source adoption boundary

## Result

Worker evidence. The route is admitted as an authenticated, non-inference
metadata operation on exact installed `grok 1.0.25`
(`f7e67d6988e2`, SHA-256
`9ef4a40ad60c6a5178a65caf39c2a148e6a98d0d2d350b10329dee34d9195d9c`) under
`HarnessConfigurationPosture::Ambient`. One bounded `--no-auto-update models`
process with stdin closed unwritten reports ordered exact ids and the source
default; it may refresh authentication or bounded catalogue metadata but
sends no prompt, opens no model session, dispatches no tool, invokes no
inference, updates nothing, retries nothing, and retains no provider state
beyond the operation.

The shipped exact-`1.0.25` output grammar was recovered from the installed
binary: an authentication preamble followed by `  * <id> (default)` and
`  - <id>` bullet rows. The parser requires that grammar and the exact default
marker, tolerates the preamble, and fails closed on a bare row, duplicate or
ambiguous default, malformed, empty, or over-limit document. The
operation-private suppression file, enterprise-precedence gate,
`ProviderSuppressed` binding, and suppression-only public types/tests were
removed rather than replaced with network isolation.

The accepted final observation is recorded in
`crates/swallowtail-adapter-grok/tests/fixtures/grok-1.0.25-model-catalogue/live-capsule.json`:
exact argv, zero exit, stdout 113 bytes with zero stderr, bullet-grammar parse
to ordered `grok-4.6` default then `grok-4.5`, no prompt, no model session, no
inference, and joined cleanup. Its descriptive auth-refresh and cache fields
are metadata-path evidence, not acceptance gates. Both earlier observations
are preserved as evidence and were not retried. Research 306 freezes the
corrected boundary and the capsule; Research 305 and the g05.052 ruling log
keep narrow correction notes.

`grok_build_acp_claim`, `grok_build_model_for_version`, released `v0.5.0`, and
every tagged or historical ledger and release note are unchanged. Route 50
(`grok-build.catalogue`) is additive post-`v0.5.0`: current source and the
working `0.5.0` baselines validate at 50 production routes while tagged
`v0.5.0` and Research 281 stay frozen at 49.

## Next Task

None — merged. PR #316 landed the authenticated non-inference catalogue at
`3e9f29e7` after independent exact-head review `5649291554`. Desktop may adopt
only the exact accepted source SHA under separate consumer authority. No
further Grok run, inference, release, tag, Desktop mutation, or consumer pin
follows from this task.
