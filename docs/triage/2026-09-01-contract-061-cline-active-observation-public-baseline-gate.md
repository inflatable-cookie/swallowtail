# Contract 061 Cline Active-Observation Public-Baseline Gate

Status: complete; strict-ready; card 032 ready
Owner: Tom
Date: 2026-09-01
Source: operator decision, Card 031 merge, Contract 061, and `main` at
`153e3e43e9f3c52c3819d0f55b104dbfae6d3058`

## Purpose

Close candidate G's route-local public blocker without transferring Cline
authority to Kimi or changing a shared runtime contract. This is planning
evidence. It fixes the adapter-owned surface and proof boundary for card 032;
it does not implement Rust, contact a provider, or authorize another candidate.

## Operator Decision

The operator approved both Cline points:

1. retain inside `swallowtail-adapter-cline` exact provider-effective or exact
   rejected Plan acknowledgement plus exact bounded negotiated model options;
   and
2. expose that truth through one additive adapter-owned projected-open seam
   while preserving `ClinePreparedSession::open_session`.

The decision is route-local to `cline.acp`. It adds no runtime/testkit/core
public type and grants no authority to Kimi, another candidate G route, or
Batch 9.5.

## Exact Public Surface

`ClinePreparedSession::open_session` keeps its current signature, handle,
failure codes, and cleanup behavior. Both open methods use one private open
lifecycle. Card 032 adds this public Cline family:

```rust
pub type ClineProjectionOpenFuture = BoxFuture<
    'static,
    Result<ClineProjectionOpenOutcome, ClineProjectionOpenFailure>,
>;

pub struct ClineProjectionOpenOutcome { /* private fields */ }

impl ClineProjectionOpenOutcome {
    pub fn session(&self) -> &dyn InteractiveSessionHandle;
    pub const fn contribution(&self) -> &ConsumerRouteProjectionContribution;
    pub fn negotiated_model_options(&self) -> Option<&NegotiatedSessionModelOptions>;
    pub fn into_parts(
        self,
    ) -> (
        Box<dyn InteractiveSessionHandle>,
        ConsumerRouteProjectionContribution,
    );
}

pub enum ClineProjectionOpenFailure {
    Runtime(RuntimeFailure),
    Rejected {
        failure: RuntimeFailure,
        contribution: ConsumerRouteProjectionContribution,
    },
}

impl ClineProjectionOpenFailure {
    pub const fn failure(&self) -> &RuntimeFailure;
    pub const fn rejected_contribution(
        &self,
    ) -> Option<&ConsumerRouteProjectionContribution>;
    pub fn into_parts(
        self,
    ) -> (RuntimeFailure, Option<ConsumerRouteProjectionContribution>);
}

impl ClinePreparedSession {
    pub fn open_session_with_projection(
        &self,
        prepared_source_id: ConsumerRouteProjectionSourceId,
        active_session_source_id: ConsumerRouteProjectionSourceId,
        services: HostServices,
    ) -> ClineProjectionOpenFuture;
}
```

The outcome's direct model-option accessor returns the same snapshot as
`session().negotiated_model_options()`. The generic handle remains the exact
bounded value carrier, including current value, provider order, and optional
display names. The contribution names the observation without flattening that
typed snapshot into an ambiguous string domain.

The six prepared facades add the established exact method:

```rust
pub fn consumer_route_projection_contribution(
    &self,
    source_id: ConsumerRouteProjectionSourceId,
) -> Result<
    ConsumerRouteProjectionContribution,
    ConsumerRouteProjectionFailure,
>;
```

The facades are `ClinePreparedSession`, `ClineHeadlessPreparedRun`,
`CommandCodePreparedRun`, `CommandCodePreparedSession`,
`CopilotCliPreparedSession`, and `GoosePreparedSession`. Only the Cline adapter
exports the projected-open family. The Cline, Command Code, Copilot CLI, and
Goose adapter semantic API baselines may change; runtime, testkit, core, and all
other adapter baselines may not.

## Retained Cline State

One private Cline open lifecycle returns a handle plus route-local observation
state or one typed internal rejection. The exact internal shape is:

- Plan omitted: `NotRequested`;
- exact confirmation `currentValue = "plan"`: `Effective("plan")`;
- exact well-formed confirmation `currentValue = "act"`, with the frozen
  unique provider domain `["plan", "act"]`: rejected `"act"`; and
- missing, malformed, duplicate, transport, setup, or otherwise ambiguous Plan
  confirmation: ordinary runtime failure with no contribution.

The projected failure uses `Rejected` only for the exact admitted `"act"`
mismatch and preserves the existing
`swallowtail.cline.acp.harness_mode_mismatch` diagnostic. Every other failure
uses `Runtime`. The preserved method unwraps the same internal rejection to its
original `RuntimeFailure`, so both methods keep route code and cleanup parity.

The shared lifecycle classifies model evidence as absent, exact, or invalid.
An exact snapshot comes only from one `configOptions` row with `id = "model"`,
`type = "select"`, `category = "model"`, one non-blank current value, a
non-empty unique option list containing that value, and optional bounded
display names. `NegotiatedSessionModelOptions::new` remains the count, text,
uniqueness, and current-membership authority.

Absent model evidence is `None` on either path. Exact evidence is retained on
the session handle. Invalid, duplicate, ambiguous, unadvertised, or unbounded
model evidence preserves legacy `open_session` success with no snapshot. The
projected path instead closes the opened session and returns `Runtime` with
`swallowtail.negotiated_model_options.invalid`; it publishes no contribution.
This split preserves the existing open contract and the accepted fail-closed
projection rule. Plan confirmation remains route admission and retains its
existing failures.

## Projection Semantics

Prepared and active sources are caller-supplied and must differ. Equal IDs fail
before provider work with
`swallowtail.cline.projection_source_identity_invalid`.

Prepared rows use `AdapterContribution(prepared_source_id)`. Post-open Plan
acknowledgement and model observation use
`ActiveSessionObservation(active_session_source_id)`. The active source is
omitted when neither row exists.

Both Cline-only active identities use bounded namespaced extensions qualified
by exact route and protocol-facade revision:

- `feature.active-session-plan-ack` carries `AcknowledgementState`, exact
  `"plan"` effective or exact `"act"` rejected, `NotSelectable`,
  wire-acknowledgement evidence, observation-only actor posture, and
  acknowledged mutation authority from the active source;
- `feature.negotiated-model-options-observation` carries `Observation`, a
  descriptor domain, `NotSelectable`, observed state, wire evidence,
  observation-only actor posture, and no mutation authority. Its exact typed
  values remain on the generic handle and outcome accessor.

The prepared `control.harness-mode` is also a bounded namespaced identity. On
`cline.acp`, exact `"plan"` is requested, prepared, and pending at session
start. On `cline.headless`, it is requested and prepared only. Omission creates
no row and no Swallowtail default. Provider-effective or rejected state never
appears on the prepared control.

A successful projected open carries the prepared contribution plus whichever
exact active rows exist. Exact rejected Plan carries the rejected
acknowledgement row and no session or model-option row. Invalid model evidence
or later projection admission failure closes the opened session and returns
`Runtime`.

## Candidate G Readiness

The gate closes rubric item 2 for the complete four-package candidate. Card
032 owns 48 exact census tuples with these maximal dispositions:

| Route | Census | Emitted | Withheld |
| --- | ---: | ---: | ---: |
| `cline.acp` | 11 | 9 | 2 |
| `cline.headless` | 8 | 7 | 1 |
| `command-code.headless` | 11 | 10 | 1 |
| `copilot-cli.acp` | 9 | 6 | 3 |
| `goose.acp` | 9 | 6 | 3 |
| **Total** | **48** | **38** | **10** |

Model-catalogue rows are withheld on all five routes. Cline ACP, Copilot CLI,
and Goose persistence rows are withheld because their exact prepared session
policy is `Prohibited`; documentation cannot override it. Copilot CLI and
Goose no-control audits remain construction-time negative coverage. No Command
Code capability absent from the census may leak into the ledger.

Prepared capability, model-selection, harness-mode, activity, and active
observation rows come only from their exact facades. Activity remains
descriptor-only. Command Code model selection stays distinct for structured
run and interactive session. Cline Plan and model-observation rows are proved
present in maximal projected opens and absent from omitted or observation-free
opens.

No shared public decision, provider contact, live probe, callback, registry,
runtime enumeration, generic provider payload, or contract amendment is
needed. Four exact packages fit the normal focused-validation maximum.
Candidate G therefore passes the promotion rubric as card 032.

## Review Oracle

Invariant: only exact prepared evidence or one exact Cline active observation
may publish a candidate G row. Session existence, documentation, another route,
or malformed optional data cannot substitute.

Counterexamples and required proof:

- prepared Plan marked provider-effective — fail; it remains pending until the
  exact confirmation
- static mismatch failure presented as rejected without exact `"act"` in the
  unique `["plan", "act"]` response — fail as `Runtime` with no contribution
- model options inferred from session existence — fail; exact wire evidence is
  required
- malformed, duplicate, ambiguous, unadvertised, or unbounded model evidence
  accepted by projected open — fail; close and return the exact runtime error
  with no contribution; preserved open remains successful with no snapshot
- model options presented as selectable, acknowledged mutation, or catalogue
  authority — fail the observation-only posture
- a rejected Plan contribution retaining model options or a session — fail;
  the open did not complete
- equal prepared and active source IDs — fail before process or resource work
- active source attached to prepared rows, or retained when both observations
  are absent — fail source admission
- preserved and projected opens differing in route failure code or cleanup for
  the same Plan fixture — fail shared-lifecycle proof
- Cline ACP observation published on Cline headless, Command Code, Copilot CLI,
  or Goose — fail route and operation applicability
- persistence inferred on Cline, Copilot CLI, or Goose from documentation —
  fail against exact prepared provider-state policy
- either no-control audit emitted as a selectable row — fail negative coverage
- 48 rows reached through a filter, exception list, duplicate semantic ID, or
  borrowed route identity — fail exact tuple reconciliation
- matching-source cross-route, cross-operation, cross-instance, stale-revision,
  or cross-access mixture accepted — fail closed

## Validation Boundary

Card 032 names exactly these packages:

- `swallowtail-adapter-cline`
- `swallowtail-adapter-command-code`
- `swallowtail-adapter-copilot-cli`
- `swallowtail-adapter-goose`

It adds package-scoped formatting, focused validation, extracted-package,
semantic API, route, docs, Northstar, god-file, and diff checks. No provider
contact or live probe belongs to the card.

## Authority

- [Contract 061](../contracts/061-consumer-route-feature-and-control-projection.md)
- [Batch 9.4 package expansion](2026-08-31-contract-061-batch-9-4-package-expansion.md)
- [completed card 031](../roadmaps/g05/batch-cards/031-contract-061-claude-agent-package-and-acknowledgement.md)
- [reviewed census](2026-08-30-consumer-route-feature-and-option-projection-census.csv)
- [Batch 9.1 public baseline](2026-08-31-contract-061-batch-9-1-public-baseline-gate.md)
