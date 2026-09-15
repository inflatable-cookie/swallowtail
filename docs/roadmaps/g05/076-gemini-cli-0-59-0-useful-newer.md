# g05.076 Gemini CLI 0.59.0 Useful Newer

Owner: Tom
Created: 2026-09-15
Depends on: Contracts 017, 023, and 029; Research 182, 230, 235, 239, 244,
255, 308, and 323; g05.075
Vision tags: route currentness, Gemini CLI, ACP, headless, exact artifacts

## Outcome

Ready. Requalify the separate Gemini CLI ACP and headless claims from their
shared `0.56.0` ceiling through official stable `0.59.0`, or stop either claim
at its first exact incompatible hop. Tom's 2026-09-14 campaign direction
explicitly lifts the former Gemini deferral.

Planning rechecked the official channels on 2026-09-15. npm latest is
`@google/gemini-cli@0.59.0`, published `2026-09-08T21:19:17.301Z`, with
integrity
`sha512-RHcjpQEMwVkrWz75mEFsuMuM0JRVJgzLQzkGhEY6SE0KjwOWFA1wYKTUqpzDUHprcE2mgcTActyd4ihVdC2dpg==`.
GitHub latest is `v0.59.0`, published `2026-09-08T21:13:43Z`, at commit
`fb0d535af931b27c51e87e5e6ade72905b1e8390`; the darwin-arm64 unsigned asset
digest is
`sha256:0c8938c68df7e46fd1de63583e4c0a1d7a452e1ecb1d33065f9ac12b8814876a`.
The published stable hops after `0.56.0` are exactly `0.57.0`, `0.58.0`, and
`0.59.0`. The installed host remains `gemini 0.53.0` at the Research 182
digest `4a8f99947eae4e1ff501269ba8b9ca2d1216db044fb75e01f4ee86fd1d8f175e`.

## Ready-State Rubric

- [x] Tom authorized every Research 308 family and explicitly lifted the
      Gemini deferral.
- [x] g05.075 completed; canonical `main` is clean and synchronized.
- [x] Research 182 freezes the `0.56.0` ACP/headless ceiling and prior stable
      points; later option evidence remains independently bounded.
- [x] Official npm/GitHub `0.59.0`, its three-hop stable sequence, and the
      unchanged installed-host observation are known separately.
- [x] No prompt, login, provider session, installation, or host update is
      needed.

## Work

1. Re-probe npm and GitHub latest plus installed `gemini --version`. Retrieve
   official npm packages and tagged source for `0.56.0`, `0.57.0`, `0.58.0`,
   and `0.59.0` into `/tmp`; corroborate package/tag identity and freeze
   Research 324 plus a mutation-sensitive fixture before changing claims.
2. Compare only files that contain or feed the selected mapped routes. ACP
   owns `--acp`, initialize, session/new, prompt, cancel, session/update,
   filesystem callbacks, selected modes, authentication advertisement,
   failure, process ownership, and cleanup. Headless owns prompt input,
   `--output-format stream-json`, explicit model/approval/config inputs,
   selected stream events and terminal result, cancellation, retention,
   failure, process ownership, and cleanup. Release notes are discovery; the
   shipped npm/source tree is identity and behavior evidence. Use one
   deterministic changed-file inventory, not repeated whole-tree scans or
   binary-string archaeology.
3. Classify `0.56.0..0.57.0`, `0.57.0..0.58.0`, and
   `0.58.0..0.59.0` independently per claim. Prove a compatible extension,
   introduce the smallest adapter-private milestone when selected mapping
   changed without changing the public lifecycle, or stop at the first exact
   selected incompatibility. Do not force ACP and headless to share an
   outcome.
4. Keep Gemini Live, Gemini Models HTTP catalogue, browser/individual-account
   login, Vertex, gateway, transcript management, advertised-but-unselected
   ACP methods, ambient extension/MCP mutation, dangerous approval bypass,
   sandbox and thinking feature gates, and provider-private metadata separate
   unless a selected-path dependency makes one unavoidable. Do not transfer
   exact `0.56.0` option evidence to later points without its own proof.
5. Land the maximal honest segments while preserving baseline `0.51.0`, claim
   ids, known gaps, and `AllowUnverified` unless evidence requires a narrower
   result. Update selection tests, the prepared guide, route/lifecycle/feature
   matrices, architecture ceiling where current, `[Unreleased]`, Research 324,
   identity/claim logs, standing lane, this task, the g05 index/census, and the
   sole Next Task pointer.
6. Recheck official latest before the identity commit and immediately before
   push. Follow Contract 029's in-run movement rule. Stop on identity
   disagreement, a selected public lifecycle change, provider/live evidence,
   or an operator policy question. Ordinary provider-free retrieval,
   comparison, correction, and validation may rerun normally.

## Dispatch Manifest

| Field | Task 076 |
| --- | --- |
| Readiness | ready |
| Prerequisites | Contracts 017, 023, and 029; Research 182, 230, 235, 239, 244, 255, 308, and 323; completed g05.075; clean pushed `main`; official npm/GitHub channels; installed official `gemini 0.53.0` |
| Completion conditions | Research 324 freezes `0.56.0..=0.59.0` identity before claims; all three stable hops receive independent ACP/headless selected-path classifications; maximal honest segments or exact stops land; current docs and claims agree; exact-head review and named gates pass |
| Owned mutable paths | Gemini CLI selection, prepared ACP/headless, commands, drivers/decoders, tests, and fixtures under `crates/swallowtail-adapter-gemini/**`; Research 324 and one research-index line; exact Gemini prepared-guide, route/activity/feature matrices, architecture ceiling, standing-lane, `[Unreleased]`, identity-log, claim-log, task-result, g05 index/census, and Next Task lines; `PAPERCUTS.md` append only |
| Reserved shared closeout surfaces | lifecycle task record and generated projections; submitted-handoff deletion belongs to the repository hook |
| Forbidden paths | Gemini Live and Models catalogue behavior; other adapters; contracts; historical research/captures/releases; provider prompts/live probes; host install/update; release/tag/workflow files |
| Approved concurrent siblings | none on shared currentness surfaces; this campaign is serial |
| Worker capability class | evidence-first Rust currentness worker; npm/source artifact comparison; no provider credentials |
| Acceptance evidence | npm/GitHub/package/tag identities for all four compared points; mutation-sensitive selected-file ledger; exact ACP/headless per-hop classification; current docs/claim agreement |
| Review oracle | claim edit before identity; skipped stable hop; all-or-nothing ACP/headless coupling; unclassified changed mapped file; option-proof transfer from `0.56.0`; inferred range; Gemini Live/Models/auth flattening; provider reliance; repeated whole-tree or binary scanning; unjustified operation/authority widening |
| Oracle gate | not required — Research 308 records Tom's lifted deferral and one-family campaign authority; Contracts 017/023/029 settle identity-first qualification, while new product policy still returns to Chatterbox |
| Stop conditions | official identity disagreement; selected wire/auth/permission/failure/resource/lifecycle change without exact mapping; new public operation/driver/facade required; provider/live evidence required; operator policy needed |
| Escalation owner | operator via Chatterbox for new policy; Queue coordinator for mechanical blockers |

## Boundaries

One package family with two existing claims. Provider-free exact-artifact
evidence and the smallest necessary private mapping/claim change only. No
prompt, inference, catalogue call, login, credential access, installation,
downloaded-binary execution, host update, live provider work, release, tag,
publication, or consumer mutation.

## Validation

- `cargo fmt -p swallowtail-adapter-gemini -- --check`
- `effigy validate:focused swallowtail-adapter-gemini`
- `effigy package:verify-affected swallowtail-adapter-gemini`
- `effigy check:examples`
- `effigy package:api`
- `effigy qa:routes`
- `effigy qa:northstar`
- focused Gemini identity-ledger, ACP, headless, selection, failure,
  cancellation, retention, resource, activity, terminal, and cleanup selectors
- relevant research/log/roadmap/number/lifecycle/next-action checks
- `git diff --check`

## Acceptance

- [ ] official identities reproduce for all compared releases and channels
- [ ] every published selected-path change is classified independently per claim
- [ ] identity evidence precedes any claim or mapping change
- [ ] maintained segments and incompatible gaps are maximally honest
- [ ] existing sibling routes and exact option evidence remain independently bounded
- [ ] official latest is rechecked at identity and push boundaries

## Next Task

Return the exact outcome to Chatterbox, then compile and dispatch the Kimi Code
installed ACP/headless family from fresh canonical `main` without asking Tom to
repeat authorization.
