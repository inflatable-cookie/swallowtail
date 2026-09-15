# g05.075 Antigravity 1.2.2 Useful Newer Requalification

Owner: Tom
Created: 2026-09-14
Depends on: Contracts 017, 023, and 029; Research 177, 283, 308, and 322; g05.074
Vision tags: route currentness, Antigravity, retry containment, exact artifacts

## Outcome

Reopen `antigravity-cli.release` against official GitHub `1.2.2`. Qualify the
catalogue and headless claims independently only across exact contiguous
segments whose selected behavior and authority are proved. Preserve the
`1.1.22` provider-managed-retry stop unless a later artifact exposes a finite,
deterministic bound or disable mechanism that Contract 023 can admit without
new operator policy. Fail closed at the smallest unproved boundary instead of
treating the entire family as all-or-nothing.

Planning rechecked the official channel on 2026-09-14. Latest stable is
`1.2.2`, published `2026-09-12T03:51:08Z`, at tag commit
`ba985e6b5de2ac8aa09860a154a102831eb7722b`. Since Research 283, GitHub has
published `1.1.27`, `1.1.28`, `1.2.0`, `1.2.1`, and `1.2.2`. The installed
host remains official `agy 1.1.19`, SHA-256
`96fae3fccfb444c7fb2c6d8d70426e5c978e4f21cfc4507a541f612a8b8ffeef`.

Operator ruling, 2026-09-15: use the official release notes as the behavioural
authority for this requalification. Stop exhaustive binary-string scanning and
do not require unpublished implementation detail to finish the task. Official
release/tag/asset identity plus the published notes are sufficient evidence;
absence of a selected-path change in those notes may be treated as unchanged
for this bounded compatibility decision. Preserve already-collected hashes,
but do not make further binary forensics an acceptance condition.

The notes say `1.1.28` retries transient model errors “for much longer” with
exponential backoff, and `1.2.1` retries 502/503/504, per-minute 429, and
mid-stream interruptions. `1.1.28` also changes `--print-timeout` expiry to
partial output plus successful exit; `1.2.0` adds a content-filter stop reason.
Classify those published changes directly. Unknown retry remains incompatible;
no inferred implementation bound is required before recording that stop.

## Ready-State Rubric

- [x] Tom authorized all Research 308 families, explicitly reopened
      Antigravity, and retained the existing bounded-mechanism-or-fail-closed
      rule.
- [x] g05.074 completed and advanced Deep Agents to exact `0.1.30`; canonical
      `main` is clean and synchronized.
- [x] Research 283 freezes `1.1.17..=1.1.26` and the exact `1.1.22` retry stop.
- [x] Official `1.2.2`, every new stable hop, current release assets, tag
      identity, and installed host identity are known separately.
- [x] No prompt, login, installation, downloaded-binary execution, or provider
      credit is needed for this requalification.

## Work

1. Re-probe official releases/tags and installed `agy` identity. Retrieve the
   official linux-x64 and mac-arm64 assets for `1.1.27`, `1.1.28`, `1.2.0`,
   `1.2.1`, and `1.2.2` into `/tmp`; verify GitHub asset digests, tag commits,
   extracted binary digests/sizes/build identities already collected, and the
   boundary against frozen `1.1.26`. Freeze Research 323 and a
   mutation-sensitive extension of the existing distribution ledger. Do not
   execute downloaded binaries or continue exhaustive string scanning.
2. Classify every new hop for each selected claim. Catalogue owns `agy models`
   authentication, stdout grammar, ordering/default semantics, failure,
   process/task deadline, and cleanup. Headless owns `--print`,
   `--output-format stream-json`, explicit model/mode/sandbox/effort/schema,
   exact conversation continuation, event/content/activity/usage decoding,
   failure and stop reasons, resource authority, cancellation, provider-native
   limits/retry, process/task ownership, retained state, and joined cleanup.
   Treat the official notes as complete enough for this bounded decision under
   Tom's 2026-09-15 ruling. Do not require full shipped-binary inventories,
   decompilation, or hidden retry implementation details.
3. Reconcile the retry boundary from the published notes. A host deadline is
   not a provider retry policy. Vague “much longer,” exponential backoff, or a
   timeout that only waits after final output does not satisfy Contract 023.
   Keep that exact point incompatible without further reverse engineering; do
   not infer a finite retry count from observed timing.
4. Record the maximal honest result per claim. Catalogue may advance even if
   headless cannot. Headless may extend only through the last contiguous safe
   point before `1.1.22`, and may add a later maintained segment only from the
   first exact point with a proved admissible retry boundary. Preserve an
   explicit incompatible gap. Keep behavior revisions only where wire,
   terminal, authority, and lifecycle are unchanged; use an adapter-private
   milestone only for a mechanically selected bound that needs no new public
   operation or policy.
5. Keep interactive `/model`, ambient `--continue`, `--agent`, Remote Control
   services, dangerous permission bypass, MCP/plugin management, voice,
   custom agents/skills, Gemini API-key sign-in, conversation delete, and
   provider kill/worktree cleanup unmapped unless a selected-path dependency
   makes one unavoidable. Do not flatten `antigravity-acp`, Gemini CLI, or
   Gemini Live onto this family.
6. Stop with the smallest exact counterexample if selected identity, wire,
   auth, permission, retry, timeout, failure, content-filter, resource,
   retained-state, or cleanup behavior cannot be bounded provider-free, or a
   new public driver/facade or operator policy is required. Recheck official
   latest immediately before identity commit and push. Provider-free download,
   extraction, scanning, classification, correction, and validation may rerun
   normally.

## Dispatch Manifest

| Field | Task 075 |
| --- | --- |
| Readiness | ready |
| Prerequisites | Contracts 017, 023, and 029; Research 177, 283, 308, and 322; completed g05.074; clean pushed `main`; official GitHub releases/tags/assets; installed official `agy 1.1.19` |
| Completion conditions | all five new releases and both platform assets are identified; existing collected hashes are retained; catalogue and headless selected deltas are independently classified from official release notes; retry/timeout semantics receive a named stop where the notes expose no admissible bound; maximal honest exact segments land without erasing gaps; exact-head review and named gates pass |
| Owned mutable paths | Antigravity selection, commands, prepared route, catalogue/headless/session drivers and decoders, tests, and fixtures under `crates/swallowtail-adapter-antigravity/**`; Research 323 and one research-index line; exact Antigravity prepared-guide, route/activity/feature matrices, architecture ceiling, standing-lane, `[Unreleased]`, identity-log, claim-log, and task-result lines; `PAPERCUTS.md` append only |
| Reserved shared closeout surfaces | lifecycle task record and generated projections; submitted-handoff deletion belongs to the repository hook |
| Forbidden paths | other adapters; contracts; historical research/captures/releases; ACP/Gemini families; provider prompts/live probes; downloaded-binary execution; host install/update; release/tag/workflow files |
| Approved concurrent siblings | none on shared currentness surfaces; this campaign is serial |
| Worker capability class | evidence-first Rust currentness worker; official-release-note classification; no provider credentials |
| Acceptance evidence | GitHub release/tag/asset identities for five new points; retained collected hashes; official release notes; explicit per-claim segment/gap decision; current docs/claim agreement |
| Review oracle | skipped official release note, inferred retry bound, host-deadline substitution, catalogue/headless all-or-nothing coupling, erased incompatible gap, inferred range, unclassified published selected-path delta, ACP/Gemini flattening, provider reliance, further exhaustive binary forensics, or unjustified operation/authority widening |
| Oracle gate | not required — Tom's Research 308 direction and Contracts 017/023/029 settle bounded-mechanism-or-fail-closed; the worker may prove or reject a mechanism but cannot accept unknown retry policy or create new operator policy |
| Stop conditions | official identity disagreement; selected retry/timeout/failure/resource/lifecycle change without exact containment; new public operation/driver/facade required; provider/live evidence required; operator policy needed |
| Escalation owner | operator via Chatterbox for new policy; Queue coordinator for mechanical blockers |

## Boundaries

One release family with two existing claims. Provider-free evidence and the
smallest necessary claim/mapping change only. No prompt, inference, catalogue
call, login, credential access, package installation, downloaded-binary
execution, host update, live provider work, release, tag, publication, or
consumer mutation. No generic authorization for provider-managed retry.

## Validation

- `cargo fmt -p swallowtail-adapter-antigravity -- --check`
- `effigy validate:focused swallowtail-adapter-antigravity`
- `effigy package:verify-affected swallowtail-adapter-antigravity`
- `effigy check:examples`
- `effigy package:api`
- `effigy qa:routes`
- `effigy qa:northstar`
- focused Antigravity identity, distribution-ledger, catalogue, headless,
  continuation, failure, timeout/retry, resource, activity, terminal, and
  cleanup selectors
- relevant research/log/roadmap/number/lifecycle/next-action checks
- `git diff --check`

## Acceptance

- [ ] official identities reproduce for all new releases and named assets
- [ ] every published selected-path change is classified independently per claim
- [ ] retry and print-timeout boundaries use the published evidence and fail closed where no admissible bound is stated
- [ ] identity evidence precedes any claim or mapping change
- [ ] maintained segments and incompatible gaps are maximally honest
- [ ] both claims retain their authority, lifecycle, and sibling boundaries
- [ ] official latest is rechecked at both identity and push boundaries

## Next Task

Return the exact outcome to Chatterbox, then compile and dispatch Gemini as the
next Research 308 family without asking Tom to repeat authorization.
