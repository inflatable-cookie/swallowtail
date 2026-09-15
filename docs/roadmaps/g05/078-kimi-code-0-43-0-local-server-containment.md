# g05.078 Kimi Code 0.43.0 Local-Server Containment

Owner: Tom
Created: 2026-09-15
Depends on: Contracts 017, 023, and 029; Research 270, 282, 308, and 325;
g05.017, g05.026, and g05.077
Vision tags: route currentness, Kimi Code, local server, process authority,
fail closed

## Outcome

Ready. Requalify the separate `kimi-code.local-server` claim through official
stable `0.43.0` under the Research 308 containment-or-fail-closed rule. Land
the maximal safe prefix already evidenced through `0.39.1`. At the first
uncontained authority change, make every later point fail closed rather than
leaving unsafe releases available through `AllowUnverified`. A later segment
may reopen only where exact shipped source proves containment was restored.

Research 282 already froze the selected REST/WebSocket v2 route through
`0.41.0`: `0.39.0` and `0.39.1` preserve the Bash workspace assertion, while
`0.40.0` removes it and `0.41.0` also removes the auto-permission-mode
dangerous-command guard. Research 325 freezes exact npm/GitHub identities for
`0.39.1`, `0.40.0`, `0.40.1`, `0.41.0`, `0.42.0`, and `0.43.0`. Planning
rechecked npm on 2026-09-15: latest remains `0.43.0`; installed `kimi` remains
`0.34.0` at
`sha256:9f4337e10da47843f6b550474012a53ba8b30dd665f83b176a5cd479c5f7e859`.

## Ready-State Rubric

- [x] Tom authorized every Research 308 family, explicitly including renewed
      Kimi containment work.
- [x] g05.077 completed; canonical `main` is clean and synchronized.
- [x] Research 282 identifies the exact safe-prefix and authority-change
      boundary; Research 325 freezes every later official identity.
- [x] Research 308 settles the product rule: contain authority or fail closed.
- [x] No prompt, login, provider session, local-server execution, installation,
      or host update is needed.

## Work

1. Re-probe npm/GitHub latest and installed `kimi --version`. Reuse Research
   282 and Research 325 identities and retained official artifacts where their
   digests reproduce; retrieve missing tagged source or npm tarballs into
   `/tmp` without executing them. Freeze Research 326 with one
   mutation-sensitive local-server selected-file ledger spanning
   `0.38.0..=0.43.0` before changing the claim.
2. Use official release notes only to discover candidates. Inspect the exact
   source files feeding `kimi web --no-open --host 127.0.0.1`, REST/WS v2,
   authentication, model catalogue, session operations, approval/question,
   Bash and terminal execution, permission policy, retry/failure, cancellation,
   process ownership, and cleanup. Reproduce the known `RuntimeWorkspaceView`
   / Bash-tool assertion boundary and classify only the `0.41.0→0.42.0` and
   `0.42.0→0.43.0` deltas not already settled by Research 282. Use one
   deterministic selected-file inventory; no binary archaeology or repeated
   whole-package scans.
3. Prove the maximal safe segment. If `0.39.0` and `0.39.1` preserve the
   selected wire and workspace assertion, extend the maintained heartbeat-ping
   segment through `0.39.1`. At `0.40.0`, trace every possible containing
   boundary: provider assertion, explicit Swallowtail mediation, or
   `ProviderEnforced`/`HostEnforced` isolation. Ambient cwd, loopback binding,
   process ownership, tool allowlists, leases, and prompts are not containment.
4. If the uncontained Bash `cwd` widening persists, change the local-server
   claim to `QualifiedOnly` at the maximal safe ceiling so `0.40.0+` cannot be
   admitted as unverified newer. Revise the claim identity if Contract 029
   requires it; preserve historical fixtures and name the published rejected
   gap without manufacturing redundant exclusions. If an exact later release
   restores containment, introduce only the smallest private milestone/segment
   supported by that evidence. Do not paper over `0.41.0`'s dangerous-command
   guard removal.
5. Keep installed ACP/headless, Python `kimi-cli`, Kimi Platform, Remote
   Control/web UI, Tower/subagents, MCP dynamic tools, skills/hooks, and model
   service behavior separate unless exact dependency tracing reaches the
   selected local-server route. Do not widen public operations or infer
   provider features from release notes.
6. Update local-server selection/tests/fixtures, prepared guide,
   route/activity/feature matrices, architecture ceiling where current,
   `[Unreleased]`, Research 326, identity/claim logs, standing lane, this task,
   g05 index/census, and the sole Next Task pointer. Recheck official latest
   immediately before push; follow Contract 029's in-run movement rule.
   Ordinary provider-free retrieval, comparison, correction, and validation
   may rerun normally.

## Dispatch Manifest

| Field | Task 078 |
| --- | --- |
| Readiness | ready |
| Prerequisites | Contracts 017, 023, and 029; Research 270, 282, 308, and 325; completed g05.017, g05.026, and g05.077; clean pushed `main`; official npm/GitHub channels; installed official `kimi 0.34.0` |
| Completion conditions | Research 326 freezes the selected local-server ledger before claim edits; safe prefix is maximized; every point from `0.40.0` through current latest has an exact containment classification; uncontained points fail closed in production; current docs and claim agree; exact-head review and named gates pass |
| Owned mutable paths | Kimi local-server selection, prepared route, drivers/protocol/activity, tests, and fixtures under `crates/swallowtail-adapter-kimi/**`; Research 326 and one research-index line; exact Kimi local-server prepared guide, route/activity/feature matrices, architecture ceiling, standing-lane, `[Unreleased]`, identity-log, claim-log, task-result, g05 index/census, and Next Task lines; `PAPERCUTS.md` append only |
| Reserved shared closeout surfaces | lifecycle task record and generated projections; submitted-handoff deletion belongs to the repository hook |
| Forbidden paths | Kimi installed ACP/headless behavior, claims, guide rows, or fixtures; Python `kimi-cli`; other adapters; contracts; historical research/captures/releases; provider prompts/live probes; local-server execution; host install/update; release/tag/workflow files |
| Approved concurrent siblings | none on shared currentness surfaces; this campaign is serial |
| Worker capability class | evidence-first Rust currentness worker; npm/source artifact comparison and authority tracing; no provider credentials |
| Acceptance evidence | reproduced Research 282/325 identities; mutation-sensitive local-server selected-file ledger; exact per-hop wire and authority classification; production proof that unsafe later points fail closed |
| Review oracle | claim edit before evidence; skipped stable hop; failure to land the evidenced safe prefix; `AllowUnverified` still admitting an uncontained point; containment inferred from cwd, loopback, process ownership, leases, allowlists, or prompt policy; installed-route flattening; provider reliance; repeated broad or binary scanning; unjustified public operation widening |
| Oracle gate | not required — Research 308 records Tom's contain-or-fail-closed authority; Research 282 fixes the known boundary; Contracts 017/023/029 settle containment and claim mechanics, while new product policy still returns to Chatterbox |
| Stop conditions | official identity disagreement; containment result depends on live/provider evidence; selected public lifecycle requires a new driver/facade; exact later restoration is ambiguous; operator policy needed beyond the recorded fail-closed rule |
| Escalation owner | operator via Chatterbox for new policy; Queue coordinator for mechanical blockers |

## Boundaries

One existing local-server family only. Provider-free exact-source evidence and
the smallest private mapping/claim change required to maximize the safe prefix
and fail closed afterward. No prompt, inference, catalogue call, login,
credential access, local-server execution, installation, downloaded-binary
execution, host update, installed ACP/headless mutation, release, tag,
publication, or consumer mutation.

## Validation

- `cargo fmt -p swallowtail-adapter-kimi -- --check`
- `effigy validate:focused swallowtail-adapter-kimi`
- `effigy package:verify-affected swallowtail-adapter-kimi`
- `effigy check:examples`
- `effigy package:api`
- `effigy qa:routes`
- `effigy qa:northstar`
- focused local-server identity-ledger, selection, REST/WS, Bash/terminal,
  permission, failure/retry, cancellation, retention, activity, and cleanup tests
- relevant research/log/roadmap/number/lifecycle/next-action checks
- `git diff --check`

## Acceptance

- [ ] official identities reproduce from the frozen evidence and current channels
- [ ] the selected local-server ledger covers every published hop through latest
- [ ] `0.39.0..=0.39.1` lands when its safe-prefix evidence reproduces
- [ ] every uncontained point from `0.40.0` fails closed in production
- [ ] any reopened later segment has exact restored-containment evidence
- [ ] installed ACP/headless and unrelated Kimi surfaces remain unchanged
- [ ] official latest is rechecked at the push boundary

## Next Task

Return the exact outcome to Chatterbox, then compile and dispatch the Oh My Pi
RPC major-line identity family from fresh canonical `main` without asking Tom
to repeat authorization.
