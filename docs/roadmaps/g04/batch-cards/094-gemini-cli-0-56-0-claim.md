# 094 Gemini CLI 0.56.0 Claim

Status: completed
Owner: Tom
Milestone: [g04.034 Gemini CLI 0.56.0 Useful Newer](../034-gemini-cli-0-56-0-useful-newer.md)
Created: 2026-08-22

## Task

Raise both Gemini CLI qualified ceilings to official `0.56.0` after card 093
proves that neither selected axis is a stop. Preserve ACP and headless as
separate claims.

## Edit Set

In `crates/swallowtail-adapter-gemini/src/selection.rs`:

- Set `GEMINI_CLI_ACP_LATEST_QUALIFIED_VERSION` to `"0.56.0"`
- Set `GEMINI_CLI_HEADLESS_LATEST_QUALIFIED_VERSION` to `"0.56.0"`
- Keep both baselines at `0.51.0`
- Keep claim ids and `AllowUnverified`
- Keep existing behavior revisions for compatible extensions; use a new
  adapter-private revision only when card 093 proves a private milestone
- Qualify published intermediates and keep named withdrawn or independently
  unqualified points incompatible

In tests and fixtures:

- Assert `0.56.0` is Qualified Maintained on both axes
- Assert host `0.53.0` and published intermediates are qualified on both axes
  when card 093 proves membership
- Keep the historical ACP activity and headless decoder corpora
- Add identity-corpus assertions for the `0.56.0` fixture
- Move the synthetic later-stable `UnverifiedNewer` point to the first
  unpublished stable after `0.56.0`
- Update retention claim evidence only where its exact qualified ceiling is a
  current claim, without inventing session management support

In current docs:

- Update `docs/guides/gemini-cli-prepared-integration.md`
- Update Gemini CLI route and lifecycle rows in
  `docs/guides/provider-route-matrix.md`
- Update the Gemini CLI row in
  `docs/guides/provider-solution-feature-matrix.csv`
- Update current Gemini CLI ceiling statements in
  `docs/architecture/system-architecture.md`
- Add a `CHANGELOG.md` Unreleased entry
- Write Gemini `0.56.0` identity and claim logs and index them
- Complete cards 093-094 and g04.034; update g04, standing-lane, batch-card,
  and roadmaps front doors
- Set the sole Next Task to define the first numbered per-route feature
  milestone, starting with Cursor headless model parameters unless the
  completed Gemini evidence changes the ranking

Contract 015 and Contract 038 retain their historical exact-source statements.
Do not rewrite them merely to mirror a moving ceiling. Stop if the new evidence
requires a contract decision.

## Validation

```sh
cargo fmt -p swallowtail-adapter-gemini
effigy validate:focused swallowtail-adapter-gemini
effigy package:verify-affected swallowtail-adapter-gemini
effigy qa:routes
effigy qa:northstar
effigy qa:docs:index:research
effigy qa:docs:index:logs
effigy qa:docs:index:roadmaps
effigy qa:docs:index:roadmaps:g04
effigy qa:docs:index:roadmaps:batch-cards
effigy qa:docs:next-action:roadmaps
git diff --check
```

## Result

Both separate Gemini CLI claims now classify published stable points through
`0.56.0` as Qualified Maintained. Host `0.53.0`, every proved published
intermediate, and official `0.56.0` are qualified on both axes. `0.56.1` is
the first unpublished stable and remains permitted `UnverifiedNewer`.
Behavior revisions, baselines, claim ids, and `AllowUnverified` remain
unchanged. Historical ACP and headless specimens remain authoritative;
transcript management remains unsupported without side-effect-free
confirmation. Gemini Live, Gemini Models, browser login, and
individual-account access are unchanged.

Do not run workspace `qa`, broad `qa:docs`, live probes, MSRV, or consumer
checks.

## Acceptance

- Official `0.56.0` classifies as Qualified Maintained on both separate axes
- Host `0.53.0` no longer classifies as UnverifiedNewer on either axis
- Every independently proved published intermediate is qualified
- The first unpublished later stable remains permitted UnverifiedNewer
- Browser login and individual-account access remain outside both routes
- Gemini Live and Gemini Models remain unchanged
- Historical decoder specimens remain
- Named gates pass
- Next Task advances to per-route feature compilation, not implementation

Auto-continuation: No.

## Out Of Scope

- Code Assist browser login or individual-account access
- Gemini Live, Gemini Models, or another route family
- Mapping unused flags, fields, auth modes, or new provider features
- Transcript import, history lookup, or management
- Provider prompt, live catalogue, live session, install, or host update
- Starting per-route feature implementation
- Workspace `qa`, broad `qa:docs`, live probes, MSRV, or consumer checks
