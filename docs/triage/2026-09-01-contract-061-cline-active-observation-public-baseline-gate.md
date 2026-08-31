# Contract 061 Cline Active-Observation Public-Baseline Gate

Status: operator decision required; no implementation card ready
Owner: Tom
Date: 2026-09-01
Source: Card 031 merge, Batch 9.4 lifecycle-priority sequence, Contract 061,
and `main` at `5d1f173ad0637c16c24f5134ef45dc559f67c61d`

## Purpose

Name candidate G's smallest current-main blocker without coupling Kimi,
per-turn candidates, or breadth work. This is planning evidence. It does not
approve a public API, implement Rust, change a contract, contact a provider,
or promote candidate G.

## Current-Main Evidence

Card 031 completed candidate D. Batch 9.4 now has 201 proved census rows and
566 rows remaining in candidates B, C, E-G, and I-L. The lifecycle-priority
sequence therefore returns first to F and G.

F is not the next narrow gate. Its 89 rows combine four route shapes with a
compound reasoning-and-plan acknowledgement, negotiated model options, and a
post-open provider-session catalogue on `kimi-code.acp`. That is three
unproved observation families across two adapter packages.

G is smaller at 48 rows across four complete adapter-package remainders:

- `cline.acp` 11 and `cline.headless` 8;
- `command-code.headless` 11;
- `copilot-cli.acp` 9; and
- `goose.acp` 9.

Its two no-control audits already have a proved negative-coverage pattern.
The unresolved public boundary is route-local to `cline.acp`:

- `driver/mode.rs::confirm_plan_mode` validates the exact provider mode and
  returns only `Result<(), RuntimeFailure>`; `driver.rs` discards the
  confirmation, so neither exact effective nor exact rejected Plan state is
  retained;
- `session/new` can carry a bounded model config-option snapshot, but
  production parses only the session ID and Plan-mode evidence;
  `ClineSessionHandle::negotiated_model_options` always returns `None`; and
- `ClinePreparedSession::open_session` returns only the generic interactive
  handle, so no cohesive active-observation contribution can be named.

Prepared success, an open session, or matrix documentation cannot substitute
for either post-open observation. Candidate G therefore still fails promotion
rubric item 2. No numbered implementation card is honest on current `main`.

## Narrow Operator Decision

Decide only whether `cline.acp` may use the same route-local pattern already
accepted separately for OpenAI Realtime and Claude Agent, extended to the two
exact Cline observation families:

1. retain inside `swallowtail-adapter-cline` the exact provider-effective or
   exact rejected Plan acknowledgement, plus any exact bounded current and
   advertised model-option snapshot returned while opening that session; and
2. expose those retained values through one additive adapter-owned
   open-with-projection outcome/failure while preserving
   `ClinePreparedSession::open_session` and one shared private open lifecycle.

Acceptance would authorize a later planning batch to fix exact adapter-owned
type names, signatures, state transitions, source identities, malformed and
absent-value behavior, provider-free fixtures, and then reassess the complete
candidate G package. It would not itself make candidate G ready.

Rejection keeps candidate G unpromoted and returns the lifecycle-priority
audit to candidate F. Deferral leaves g05.009 paused with no ready
implementation card.

## Fixed Boundary

Any accepted surface must remain adapter-local. It may reuse existing runtime
projection records, `NegotiatedSessionModelOptions`, active-observation source
identity, fixed bounds, and composer failures. Stop if it needs a new
runtime/testkit/core public type, generic provider payload, callback, registry,
runtime route enumeration, or amendment to Contracts 037, 047, 057, or 061.

The later gate must preserve these distinctions:

- omitted Plan creates no acknowledgement state;
- prepared Plan is requested, prepared, or pending only;
- provider-effective and rejected Plan require one exact well-formed wire
  confirmation;
- absent model options remain absent, not an inferred catalogue;
- a model option snapshot is observation, not model mutation or route-wide
  catalogue authority;
- prepared and active-session sources differ; and
- malformed, duplicate, unadvertised, unbounded, or ambiguous values produce
  runtime failure with no invented contribution.

## Exclusions

- `kimi-code.acp`, candidate F, or `EffectiveReasoningSetup`
- `cline.headless`, Command Code, Copilot CLI, or Goose public-baseline changes
- candidates B, C, E, I-L or Batch 9.5
- runtime/core public API, contracts, census, compatibility, currentness,
  watcher, skill-discovery, papercut, or generation-closeout work
- provider contact or live probes

## Review Oracle

Invariant: no candidate is promoted until exact Cline post-open truth has one
operator-approved adapter-local retention and projection boundary.

Counterexamples:

- prepared Plan presented as provider-effective — stop; no wire confirmation
- an open session presented as model-option evidence — stop; exact current and
  advertised values are required
- a model option list treated as mutation or catalogue authority — stop
- a static mismatch diagnostic presented as exact rejected Plan state — stop;
  the rejected value is not retained
- one active source borrowed by another route or operation — fail closed
- candidate G narrowed to omit Cline's post-open rows — fail the complete
  package-remainder rule
- candidate G promoted before exact adapter signatures and deterministic
  oracles are fixed — stop before implementation

## Authority

- [Contract 061](../contracts/061-consumer-route-feature-and-control-projection.md)
- [Batch 9.4 package expansion](2026-08-31-contract-061-batch-9-4-package-expansion.md)
- [card 030 acknowledgement stop](../roadmaps/g05/batch-cards/030-contract-061-acknowledgement-candidate-reassessment.md)
- [completed card 031](../roadmaps/g05/batch-cards/031-contract-061-claude-agent-package-and-acknowledgement.md)
- [reviewed census](2026-08-30-consumer-route-feature-and-option-projection-census.csv)
