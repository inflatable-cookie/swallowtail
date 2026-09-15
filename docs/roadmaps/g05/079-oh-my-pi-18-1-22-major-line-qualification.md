# g05.079 Oh My Pi 18.1.22 Major-Line Qualification

Owner: Tom
Created: 2026-09-15
Depends on: Contracts 017, 023, and 029; Research 217 and 308; g05.078
Vision tags: route currentness, Oh My Pi, RPC, major-line mapping

## Outcome

Qualify or stop the `oh-my-pi.rpc` route across every published npm stable
after `17.4.0` through current official `18.1.22`. Settle `17.4.1` and
`17.4.2` independently, then treat `18.x` as a new adapter-private major-line
mapping segment rather than inferring that the 17.x compatibility window
continues.

Official npm and GitHub stable are `18.1.22` at planning. The installed host
reports `omp/18.1.16`; that is an observation inside the assigned family, not
qualification or update authority. Research 217 found selected RPC argv,
commands, framing, and decoded fields compatible through observed `18.0.6`,
but stopped because official latest moved during the run. This task reuses
that evidence only where exact identities reproduce and completes the whole
published ledger through `18.1.22`.

## Ready-State Rubric

- [x] Tom authorized every Research 308 family, explicitly including Oh My Pi.
- [x] g05.078 completed; canonical `main` is clean and synchronized.
- [x] Research 217 identifies the prior 17-to-18 mapping evidence and stop.
- [x] npm and GitHub agree on official stable `18.1.22`; installed
      `omp/18.1.16` is observed separately.
- [x] No prompt, provider contact, login, installation, host update, or
      downloaded-artifact execution is required.

## Work

1. Re-probe official npm/GitHub latest and installed `omp --version`. Retrieve
   the official npm packages and matching tagged source into `/tmp` without
   executing downloaded code. Freeze Research 327 with exact npm metadata,
   tarball digests, GitHub tag/commit/tree identity, the installed executable
   digest, every published stable after `17.4.0`, and every npm-unpublished
   gap. Reproduce Research 217's retained identities before relying on them.
2. Use official release notes as the behavioral discovery authority. Build
   one deterministic changed-file inventory across the full published ledger,
   then inspect only shipped source that contains or feeds the selected RPC
   route: `--mode rpc`, RPC v2 negotiation and framing, model/reasoning
   selection, catalogue, prompt/session lifecycle, typed questions, tool and
   activity records, usage, terminal outcome, retry/failure, retention,
   cancellation, process ownership, and cleanup. Do not perform repetitive
   whole-package or binary scans.
3. Classify `17.4.0 -> 17.4.1` and `17.4.1 -> 17.4.2` independently. Extend
   the existing 17.x segment only through the maximal consecutive prefix whose
   selected behavior is proved. Preserve any exact stopped hop or unpublished
   gap visibly.
4. Classify every published 18.x hop in order. The 17-to-18 transition must
   compile as a distinct adapter-private behavior revision and compatibility
   segment if admitted, even when the public operation remains unchanged.
   Trace every changed selected file; decoder tolerance may admit additive
   unknown fields only when production tests prove they cannot change mapped
   semantics. Stop at the first unresolved selected wire, authority, failure,
   resource, or lifecycle change. Do not flatten `oh-my-pi.package` onto
   `pi.package`.
5. Land the maximal honest independent segments. Preserve the baseline,
   historical specimens, claim posture, and claim id unless Contract 029
   requires the smallest revision. Add mutation-sensitive identity and
   selection tests/fixtures. Update the Oh My Pi prepared guide, route and
   feature matrices, current architecture ceiling, `[Unreleased]`, Research
   327, identity/claim logs, standing lane, this task, g05 index/census, and
   the sole Next Task pointer.
6. Recheck official latest immediately before the identity commit and before
   push. Apply Contract 029's in-run movement rule. Ordinary provider-free
   retrieval, comparison, correction, and validation may rerun normally; no
   artificial one-shot budget applies.

## Dispatch Manifest

| Field | Task 079 |
| --- | --- |
| Readiness | ready |
| Prerequisites | Contracts 017, 023, and 029; Research 217 and 308; completed g05.078; clean pushed `main`; official npm/GitHub channels; installed official `omp/18.1.16` |
| Completion conditions | Research 327 freezes the complete published successor ledger and selected-source inventory before claim edits; both 17.x hops and every 18.x hop are classified; an admitted 18.x range has a distinct adapter-private behavior segment; maximal honest segments or exact stops land; docs and claims agree; exact-head review and named gates pass |
| Owned mutable paths | Oh My Pi selection, prepared profiles, commands, drivers/decoders, tests, and fixtures under `crates/swallowtail-adapter-oh-my-pi/**`; Research 327 and one research-index line; exact Oh My Pi prepared-guide, route/activity/feature matrices, architecture ceiling, standing-lane, `[Unreleased]`, identity-log, claim-log, task-result, g05 index/census, and Next Task lines; `PAPERCUTS.md` append only |
| Reserved shared closeout surfaces | lifecycle task record and generated projections; submitted-handoff deletion belongs to the repository hook |
| Forbidden paths | `pi.package` and the Pi adapter; other adapters; contracts; historical research/captures/releases; provider prompts/live probes; credential access; host install/update; release/tag/workflow files |
| Approved concurrent siblings | none on shared currentness surfaces; this campaign is serial |
| Worker capability class | evidence-first Rust currentness worker; npm/source artifact comparison; no provider credentials |
| Acceptance evidence | exact npm/GitHub/host identities; complete published and unpublished-gap ledger; one deterministic changed-file inventory; mutation-sensitive selected-source ledger; per-hop 17.x and 18.x mapping classifications; production segment and doc agreement |
| Review oracle | claim edit before identity; skipped published stable; inferred major-line range; admitted 18.x on the 17.x behavior revision; unclassified changed mapped file; changelog-only claim without source corroboration; Oh My Pi/Pi flattening; provider reliance; repetitive broad or binary scanning; unjustified public operation or authority widening |
| Oracle gate | not required — Research 308 records Tom's Oh My Pi authority; Research 217 supplies reproducible prior evidence; Contracts 017/023/029 settle identity-first private mapping, while a new public operation or product policy returns to Chatterbox |
| Stop conditions | official identity disagreement; selected public operation/driver/facade required; mapped wire/auth/permission/failure/resource/lifecycle change lacks an exact private mapping; provider/live evidence required; operator policy needed |
| Escalation owner | operator via Chatterbox for new policy; Queue coordinator for mechanical blockers |

## Boundaries

One existing Oh My Pi RPC family only. Provider-free exact-artifact evidence
and the smallest private mapping/claim changes needed for honest 17.x and 18.x
segments. No prompt, inference, catalogue call against a provider, login,
credential access, installation, downloaded-artifact execution, host update,
Pi adapter mutation, release, tag, publication, or consumer mutation.

## Validation

- `cargo fmt -p swallowtail-adapter-oh-my-pi -- --check`
- `effigy validate:focused swallowtail-adapter-oh-my-pi`
- `effigy package:verify-affected swallowtail-adapter-oh-my-pi`
- `effigy check:examples`
- `effigy package:api`
- `effigy qa:routes`
- `effigy qa:northstar`
- focused identity-ledger, selection, negotiation, framing, model/reasoning,
  question, tool/activity, usage, terminal, failure/retry, retention,
  cancellation, process, and cleanup tests
- relevant research/log/roadmap/number/lifecycle/next-action checks
- `git diff --check`

## Acceptance

- [ ] official identities reproduce for every published stable through latest
- [ ] npm-unpublished gaps remain explicit and are never inferred compatible
- [ ] identity evidence lands before any claim or mapping change
- [ ] both 17.x hops and every 18.x hop receive an exact selected-path ruling
- [ ] any admitted 18.x range uses a distinct private behavior segment
- [ ] historical evidence, `pi.package`, and unrelated surfaces remain separate
- [ ] official latest is rechecked at identity and push boundaries

## Next Task

Return the exact outcome to Chatterbox and report the Research 308 campaign
complete. Do not infer release, tag, publication, live-provider, or consumer
authority.
