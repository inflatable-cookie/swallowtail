# 2026-09-01 Kimi Code 0.39.1 Claim

Card: g05.016 batch 042
Research: [270](../research/270-kimi-code-0-39-1-identity.md)

## What changed

A split outcome on the two `kimi-code.executable` installed-harness axes, plus
a same-family claim correction that card 041 forced.

**Headless — corrected and extended.**

- `kimi.headless.stream-json.v1` corrects down from `0.29.0..=0.37.2` to
  `0.29.0..=0.32.0`.
- `kimi.headless.stream-json.v2` corrects down and extends from exact
  `0.38.0` to `0.33.0..=0.39.1`.
- `KIMI_HEADLESS_LATEST_QUALIFIED_VERSION` becomes `0.39.1`. Unpublished
  `0.39.2` is the synthetic later `UnverifiedNewer` point on that axis.
- Host `0.34.0` reclassifies from a broken v1 point to qualified Maintained
  v2.

**ACP — stopped.**

- `KIMI_CODE_LATEST_QUALIFIED_VERSION` stays `0.38.0`.
- Exact `0.39.0` and `0.39.1` join the claim's exclusions and classify
  `Incompatible`.
- Unpublished `0.38.1` remains the first admissible unverified-newer point
  above the `0.38.0` ceiling; unpublished `0.39.2` is a later admissible one.
- No new ACP behavior revision.

Selection tests, conformance, installed discovery, plan-mode and reasoning
dispatch, prepared-facade and import newer-version fixtures, the local-server
corpus ceiling assertions, the prepared integration guide, the route and
lifecycle matrices, the feature-matrix CSV, the architecture ceiling, the
changelog, the generation index census, and the standing lane follow.

## Current state

The headless split point is exact and evidence-backed rather than inherited.
Through `0.32.0` the print engine is gated on `KIMI_CODE_EXPERIMENTAL_FLAG`
and defaults to v1; from `0.33.0` it is gated on `KIMI_CODE_LEGACY_FLAG` and
defaults to v2, and the adapter never sets that flag. The two segments are
adjacent published points, so the correction opens no gap. Since g04.064 the
claim had carried `0.33.0..=0.37.2` as qualified v1 while those releases emit
the `system.version` preamble the v1 decoder answers with `malformed_stream`.

The ACP stop is an authority decision, not a version-label decision. The
containment trace found no adapter or runtime control over the local spawn the
`0.39.0` terminal runner introduces, and the route declares
`HarnessIsolation::AmbientHost` with no isolation claim. Exclusions are
assessed before the `AllowUnverified` path, so both exact points are refused
rather than silently attempted. `AllowUnverified` still holds above them, so
this is an exact stop and not a posture change.

Exact negative points survive: `0.28.0` and `0.28.2` stay outside ACP and
`0.28.1` stays outside headless. `kimi-code.local-server` stays exact `0.28.1`
plus `0.29.0..=0.38.0`, still reports `0.38.1` as `UnverifiedNewer`, and still
treats `0.39.0` and `0.39.1` as unverified-newer on its own axis. Kimi Platform
Chat, Python `kimi-cli`, Gemini's deferral, and the g05.009
provider-operation observation gate are untouched; card 034 stays planned and
candidate F stays unpromoted at 249/518.

Public API surface is unchanged. Proof modules live under
`crates/swallowtail-adapter-kimi/tests/kimi_code_0_39_1_identity/`, now
including routing and authority modules, behind the `executable_identity` test
target. That target also adopts `tests/kimi_code_executable_identity.rs`,
which `autotests = false` had left orphaned and never compiled. A new in-crate
unit test binds the recorded headless argv to the private
`headless_command::arguments` constructor without widening public API.

Falsification: moving the boundary off `0.33.0`, inverting the legacy-flag
polarity, treating `system.version` as a v1 line, leaving a historical corpus
asserting the superseded `0.38.0` boundary, making either excluded ACP point
admissible, flipping the containment trace without evidence, or leaking the
ACP stop onto headless or local-server each fail the suite.

## Next move

Define the containment or mediation gate ACP `0.39.x` needs before
requalification, then run a fresh all-route currentness checkpoint to select
the next single family.
