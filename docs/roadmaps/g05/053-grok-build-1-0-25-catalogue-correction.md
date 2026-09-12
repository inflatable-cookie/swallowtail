# g05.053 Grok Build 1.0.25 Catalogue Correction

Status: ready; operator-directed correction of g05.052's over-broad non-admission
Owner: Tom
Created: 2026-09-12
Depends on: Contracts 020 and 047; g05.052; PR #315; installed Grok `1.0.25`
Vision tags: model catalogue, Grok Build, provider suppression, consumer truth

## Outcome

Deliver the exact installed Grok `1.0.25` pre-session model catalogue through
a separately prepared `ModelCatalog` operation. Correct g05.052's
non-admission premise: it searched for a dedicated `remote_fetch` environment
variable but missed Grok's documented, allowlisted per-process `GROK_CONFIG`
overlay and Swallowtail host-local's ambient-environment clearing.

The catalogue process uses exact argv `--no-auto-update models` and one
catalogue-only environment binding that supplies an authorized isolated
`GROK_HOME` plus:

```json
{"features":{"remote_fetch":false,"managed_config":false}}
```

Preparation requires positive host evidence that no higher-priority
requirements/MDM layer defeats either suppression. If that evidence is absent,
conflicted, or stale, preparation fails before the catalogue process starts.

## Ready-State Rubric

- [x] The operator rejected g05.052's non-admission and explicitly instructed
      Chatterbox to fix the seam.
- [x] Exact `1.0.25` installed documentation states that `GROK_CONFIG` is an
      allowlisted inline JSON overlay covering `features`, including
      `features.remote_fetch` and `features.managed_config`.
- [x] Swallowtail host-local clears ambient environment before applying exact
      approved environment bindings.
- [x] `--no-auto-update` is accepted only before the `models` subcommand.
- [x] The enterprise-precedence caveat is explicit and fail-closed rather than
      silently ignored.
- [x] No prompt, model session, provider inference, release, tag, or Desktop
      mutation is required.

## Work

1. Freeze Research 306 from the exact `1.0.25` shipped configuration docs,
   command grammar, installed executable identity, host-local `env_clear`
   launch semantics, and PR #315 review history. Correct Research 305 and its
   ruling log narrowly: the earlier verdict remains historical, but its claim
   that no bindable suppression exists is superseded by the generic
   `GROK_CONFIG` overlay.
2. Recover the admitted implementation from PR #315's preserved Git objects
   rather than rewriting it from memory. Correct it before admission:
   `--no-auto-update models`; a distinct catalogue environment reference;
   exact provider-suppression evidence; `HarnessConfigurationPosture::ProviderSuppressed`
   in instance, requirements, plan, and driver validation; and a typed
   pre-process rejection when a requirements/MDM override is not proved absent
   or safely disabling.
3. Preserve the previously reviewed bounded parser and exact-`1.0.25`
   supplemental metadata projection: live listing owns membership, order, and
   default; the frozen embedded document supplements exact matching IDs only.
   Unknown valid IDs pass through. Missing source fields remain absent.
4. Prove with fake processes and the real local host that ambient variables,
   `GROK_CONFIG_PATH`, user config, and unrelated ACP environment state do not
   leak into the catalogue launch; the exact overlay and isolated home do;
   conflicting or unproved enterprise pins reject before spawn; malformed
   overlay evidence rejects; and success, failure, deadline, disconnect, and
   cleanup remain bounded and joined.
5. After every provider-free gate passes, run at most one exact authenticated,
   provider-suppressed `1.0.25` catalogue command. No retry. It must send no
   prompt, open no model session, perform no inference, and retain a redacted
   capsule proving exact argv, suppression evidence, ordered IDs/default,
   completion, and cleanup. Stop if the pre-process suppression proof fails.
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
| Completion conditions | Research 306 freezes the missed overlay and host launch evidence; the preserved admitted implementation is corrected to a fail-closed provider-suppressed operation; at most one authorized catalogue command proves the accepted path after provider-free gates; exact-head review and all named validation pass; Desktop receives the source-linked capsule |
| Owned mutable paths | `crates/swallowtail-adapter-grok/**`; exact Grok rows in current route/feature/activity matrices and integration guides; `CHANGELOG.md` `[Unreleased]`; current-role `release-baselines/production-routes-0.5.0.txt` and `release-baselines/public-api-0.5.0/swallowtail-adapter-grok.txt`; narrowly required route/front-door/activity validation scripts and tests; `docs/research/305-grok-1-0-25-model-catalogue-evidence.md`, new `docs/research/306-*.md`, and research index; `docs/logs/2026-09-12-g05-052-grok-1-0-25-model-catalogue-ruling.md`, one new corrective claim log, and logs index; this task's result/status; `PAPERCUTS.md` append only |
| Reserved shared closeout surfaces | `docs/roadmaps/README.md`, `docs/roadmaps/g05/README.md`, and `docs/roadmaps/generation-index.md`; queue coordinator edits these at closeout |
| Forbidden paths | `docs/releases/0.5.0.md`; Research 281 and every historical freeze ledger/baseline; Grok ACP execution selection/claim behavior; contracts; every other adapter; Desktop or consumer repositories; release/tag/workflow files |
| Approved concurrent siblings | none on Grok, current inventory, or closeout surfaces |
| Worker capability class | complex Rust/evidence correction worker; exact process-environment and configuration-precedence reasoning; one bounded provider-suppressed catalogue observation only after provider-free proof |
| Acceptance evidence | exact shipped Grok config docs and executable identity; local-host environment-clearing proof; mutation-sensitive fake-process corpus; exact environment/argv capture; one redacted live catalogue capsule; current-vs-tagged inventory regression; focused package and route gates |
| Review oracle | the smallest counterexample is omitting `--no-auto-update`, reusing an ambient ACP environment, allowing a higher-priority pin to force remote fetch, spawning before suppression evidence, trusting the embedded document for live membership, contacting inference, mutating tagged evidence, or collapsing catalogue into ACP execution |
| Stop conditions | exact `1.0.25` rejects the overlay; `features.remote_fetch=false` is not allowlisted/effective; enterprise precedence cannot be proved before spawn; the command prompts, creates a session, performs inference, or cannot close/join; provider-free gates fail before the single live allowance |
| Escalation owner | operator via Chatterbox for semantic/authority questions; queue coordinator for mechanical blockers |

## Boundaries

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

- [ ] exact per-process suppression is bound and proved before spawn
- [ ] exact argv is `--no-auto-update models`
- [ ] ambient config/environment cannot leak into the child
- [ ] enterprise precedence fails closed
- [ ] listing owns membership/order/default; supplemental metadata stays exact-ID only
- [ ] lifecycle, bounds, redaction, and cleanup pass
- [ ] tagged `v0.5.0` and historical evidence remain immutable
- [ ] Desktop receives an exact-source adoption boundary

## Next Task

Implement g05.053 through the queue now. On merge, Desktop may adopt only the exact accepted source
SHA under separate consumer authority or wait for a separately authorized
future source tag.
