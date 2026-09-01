# 043 Kimi Code ACP QualifiedOnly Cap

Status: completed
Owner: Tom
Created: 2026-09-01
Updated: 2026-09-01
Milestone: `../017-kimi-code-acp-0-39-containment-gate.md`
Depends on: recorded operator answer A2 on g05.017; completed g05.016 cards 041-042

## Goal

Cap `kimi-code.acp` above `0.38.0` under `QualifiedOnly` with one artifact-level
reopen trigger, without raising a ceiling or creating a new behavior revision.

## Scope

1. Move only `kimi-code.acp` `InterfaceNewerVersionPosture` from
   `AllowUnverified` to `QualifiedOnly`. Keep qualified segments exact `0.28.1`
   plus `0.29.0..=0.38.0`. Keep exact exclusions `0.39.0` and `0.39.1`. Revise
   only the ACP claim id from `kimi.acp.executable-window-2` to
   `kimi.acp.executable-window-5` (Contract 029). Frozen historical
   `window-3` (`kimi-code-0.30.0-0.31.0`) and `window-4` (`kimi-code-0.31.1`)
   stay reserved and are not reused.
2. Prove every exact point above the ACP ceiling fails closed, including
   unpublished `0.38.1`, excluded `0.39.0`/`0.39.1`, unpublished `0.39.2`, and
   farther `0.40.0`. Distinguish exclusion membership from posture-driven
   newer rejection using public claim accessors; do not add assessment
   variants.
3. Keep headless v1/v2, `kimi-code.local-server`, Kimi Platform Chat, and every
   second family byte-unchanged. Local-server stays `AllowUnverified`.
4. Record exactly one standing-lane reopen trigger: a shipped-artifact identity
   run may reopen planning only if every invocation path fails closed again for
   a terminal-less client, or upstream supplies a `ProviderEnforced` boundary
   satisfying Contracts 017/023. The trigger authorizes a fresh identity/claim
   decision, never automatic admission and never restoration of
   `AllowUnverified` by itself.
5. Update the guide, matrix, architecture, and changelog only where current ACP
   newer-version posture is named. Reconcile g05.017, batch-card index,
   generation/front doors, standing lane, log/index, and sole Next Task.
6. Add mutation-sensitive tests: flip ACP back to `AllowUnverified` and show
   the newer-point assertions fail; remove an exact exclusion and show the
   exact classification proof fails; change local-server posture and show the
   isolation proof fails. Restore each mutation.
7. Hold public API and god-file baselines. No public type, contract amendment,
   runtime/host change, containment implementation, provider contact, auth,
   install, host mutation, live probe, or downloaded-binary execution.

## Out Of Scope

Another family, local-server claim change, Kimi Platform Chat, Gemini, skill,
projection, papercut, g05.009 card 034, release, containment, negotiated
terminal execution, or restoring `AllowUnverified`.

## Acceptance Criteria

- [x] operator A2 is recorded; this card is the only follow-on claim change
- [x] ACP newer-version posture is `QualifiedOnly`; segments, ceiling,
      exclusions, and behavior revisions are unchanged; claim id is
      `kimi.acp.executable-window-5`
- [x] window-2 installed-executable evidence fails closed against window-5 at
      `observe_instance_update` before projection
- [x] live ACP claim id differs from pre-A2 `window-2` and from frozen
      historical `window-3`/`window-4`; `window-5` is absent from `origin/main`
      corpora and pickaxe
- [x] every named point above `0.38.0` assesses `Incompatible` and is not
      permitted
- [x] local-server, headless, and second families are unchanged
- [x] one standing-lane trigger is recorded with the no-automatic-admission rule
- [x] mutation proofs fail the named assertions and restore the original claim
- [x] public API surface is unchanged
- [x] god-file count does not rise
- [x] Next Task is the fresh all-route Contract 029 checkpoint; card 034 stays
      planned and not ready

## Validation

```sh
cargo fmt -p swallowtail-adapter-kimi
effigy validate:focused swallowtail-adapter-kimi
effigy package:verify-affected swallowtail-adapter-kimi
effigy package:api
effigy qa:routes
effigy qa:docs:roadmaps:status
effigy qa:northstar
effigy qa:docs:index:research
effigy qa:docs:index:logs
effigy qa:docs:index:roadmaps
effigy qa:docs:index:roadmaps:g05
effigy qa:docs:index:roadmaps:batch-cards
effigy qa:docs:next-action:roadmaps
effigy --json scan god-files
git diff --check
```

Do not run workspace `qa`, broad `qa:docs`, live probes, MSRV, or consumer
checks.

## Stop Conditions

- current claim types cannot express `QualifiedOnly` without changing unrelated
  axes
- applying A2 needs a new contract, public type, or behavior revision
- the change would widen `kimi-code.local-server` or flatten onto it
- the change would answer the g05.009 provider-operation observation gate
- a ceiling is raised or an exclusion is dropped

## Auto-Continuation

No. Review and merge. Next Task is the fresh all-route Contract 029 checkpoint.

## Result

Operator A2 landed. `kimi_acp_claim()` binds
`InterfaceNewerVersionPosture::QualifiedOnly` under claim id
`kimi.acp.executable-window-5`. Segments stay exact `0.28.1`
plus `0.29.0..=0.38.0`. Exact `0.39.0` and `0.39.1` stay excluded.
`KIMI_CODE_LATEST_QUALIFIED_VERSION` stays `0.38.0`. Unpublished `0.38.1`,
excluded `0.39.0`/`0.39.1`, unpublished `0.39.2`, and `0.40.0` all assess
`Incompatible`. Public assessment is one `Incompatible` variant; exclusion
membership versus posture is distinguished through `exclusions()` and
`newer_version_posture()`. Headless and local-server claims are unchanged.
The standing lane holds one artifact-level reopen trigger that never admits a
point and never restores `AllowUnverified` by itself. Evidence minted under
`kimi.acp.executable-window-2` fails closed at `observe_instance_update`.
Frozen `kimi.acp.executable-window-3` (`kimi-code-0.30.0-0.31.0/installed-range.json`)
and `kimi.acp.executable-window-4` (`kimi-code-0.31.1/release.json`) stay
historical; they are not live. `git grep` on `origin/main` and
`git log -S kimi.acp.executable-window-5 origin/main` found no prior live or
frozen use of `window-5`.
