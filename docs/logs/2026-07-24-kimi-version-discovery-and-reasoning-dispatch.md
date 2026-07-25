# Kimi Version Discovery And Reasoning Dispatch

Date: 2026-07-24
Card: `../roadmaps/g01/batch-cards/130-kimi-version-discovery-range-and-reasoning-dispatch.md`

## Outcome

The production Kimi ACP descriptor now publishes discovery and the
`kimi-code.executable` compatibility claim. Exact `0.28.1` and `0.29.0`
singleton segments are maintained. Exact stable releases above `0.29.0`
remain permitted but visibly unverified through the latest qualified private
behavior. Gaps, prereleases, malformed versions, and older points reject.

Installed observation runs only the host-approved executable with
`--version`. It grants no installation, update, authentication, configuration,
working-resource, or session authority. Output is bounded and parsed as one
exact semantic version. Local and remote-authoritative fixtures preserve host
identity, target identity, classification, cancellation, safe diagnostics,
and joined process cleanup.

Interactive preflight retains the same exact executable binding. ACP
initialization must report that exact version before session allocation.
`0.28.1` dispatches the legacy boolean reasoning shape. `0.29.0` and permitted
stable newer versions dispatch the declared-effort shape.

One new-session reasoning request produces one correlated
`session/set_config_option` call. Provider option ids and snapshots remain
adapter-private. Readiness follows exact effective confirmation. Missing,
duplicate, malformed, unsupported, rejected, unconfirmed, and drifted options
fail without changing model, route, driver, or provider. Load and resume
selection fail before host effects.

## Preserved Boundaries

- no container, sandbox, or implicit containment claim
- no executable search, install, update, downgrade, or ambient fallback
- no provider configuration bag in core or runtime
- no reasoning mutation of an existing provider session
- no live authentication in default tests
- no Nucleus or Soundcheck changes

## Validation

- `cargo test -p swallowtail-adapter-kimi`
- workspace all-target check
- workspace warnings-denied clippy
- `effigy format:check`
- `effigy qa:docs`
- `git diff --check`

All passed. Final Rust validation used a fresh temporary Cargo target because
the shared local target directory contains 65,535 dependency entries and its
artifact lookup stalled. No repository output path changed. The live Kimi
executable probe remains separately gated.

The doctor delta was reviewed directly. The god-file scan briefly rose to 24
findings during fixture construction, then returned to the inherited 19 after
the runtime, driver, agent, host-service, and reasoning tests were split into
focused modules. The remaining count is 12 warnings and seven errors.

## Continuation

Card 131 is ready: run the shared Contract 034 and persistent ACP assertion
packs across both authoritative topologies, complete full QA, and close
roadmap 043.
