# Contract 061 Claude Agent Acknowledgement Public-Baseline Gate

Status: complete; strict-ready; card 031 ready
Owner: Tom
Date: 2026-08-31
Source: operator decision, Card 030 current-main stop, Contract 061, and
`main` at `074580eadef9d0a96c4c0c385da9a5db08786051`

## Purpose

Close candidate D's sole public-baseline blocker without transferring a
route-local decision to Kimi or Cline. This is planning evidence. It does not
implement Rust, change a contract, contact a provider, or authorize any route
outside the Claude Agent adapter package.

## Operator Decision

The operator approved both points Card 030 left open for
`claude-agent.acp`:

1. retain the exact provider-confirmed effective reasoning value or exact
   rejected reasoning value inside `swallowtail-adapter-claude-agent`; and
2. expose that retained truth through one additive adapter-owned
   open-with-projection outcome and failure while preserving
   `ClaudeAgentPreparedSession::open_session`.

The decision is route-local. It adds no runtime/core public type and makes no
decision for `kimi-code.acp`, `cline.acp`, `EffectiveReasoningSetup`, negotiated
model-option observation, or provider-session catalogue observation.

## Exact Adapter Surface

The existing `ClaudeAgentPreparedSession::open_session` signature, returned
handle, failure behavior, and cleanup behavior remain public and unchanged.
Both public open methods use one private low-level open lifecycle so they cannot
drift in setup, acknowledgement validation, handle wrapping, failure codes, or
cleanup.

Card 031 adds this adapter-owned public family:

```rust
pub type ClaudeAgentProjectionOpenFuture = BoxFuture<
    'static,
    Result<ClaudeAgentProjectionOpenOutcome, ClaudeAgentProjectionOpenFailure>,
>;

pub struct ClaudeAgentProjectionOpenOutcome { /* private fields */ }

impl ClaudeAgentProjectionOpenOutcome {
    pub fn session(&self) -> &dyn InteractiveSessionHandle;
    pub const fn contribution(&self) -> &ConsumerRouteProjectionContribution;
    pub fn into_parts(
        self,
    ) -> (
        Box<dyn InteractiveSessionHandle>,
        ConsumerRouteProjectionContribution,
    );
}

pub enum ClaudeAgentProjectionOpenFailure {
    Runtime(RuntimeFailure),
    Rejected {
        failure: RuntimeFailure,
        contribution: ConsumerRouteProjectionContribution,
    },
}

impl ClaudeAgentProjectionOpenFailure {
    pub const fn failure(&self) -> &RuntimeFailure;
    pub const fn rejected_contribution(
        &self,
    ) -> Option<&ConsumerRouteProjectionContribution>;
    pub fn into_parts(
        self,
    ) -> (RuntimeFailure, Option<ConsumerRouteProjectionContribution>);
}

impl ClaudeAgentPreparedSession {
    pub fn open_session_with_projection(
        &self,
        prepared_source_id: ConsumerRouteProjectionSourceId,
        active_session_source_id: ConsumerRouteProjectionSourceId,
        services: HostServices,
    ) -> ClaudeAgentProjectionOpenFuture;
}
```

The exact prepared contribution method already established by cards 022-024
is added to the candidate D prepared facades. It has no callback or provider
payload:

```rust
pub fn consumer_route_projection_contribution(
    &self,
    source_id: ConsumerRouteProjectionSourceId,
) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure>;
```

The contributing facades are `ClaudeAgentPreparedRun`,
`ClaudeAgentPreparedSession`, `ClaudeAgentPreparedDelete`,
`ClaudeCodePreparedRun`, and `ClaudeCodeResponsePreparedRun`. Each emits only
the exact operation truth its own prepared evidence binds. Lifecycle actions
reachable from the prepared session remain descriptors; they do not become
provider-effective state merely because the method exists.

## Acknowledgement Semantics

The private Claude Agent open lifecycle retains one exact route-local result:

- no requested reasoning produces no reasoning acknowledgement state;
- a returned `effort.currentValue` exactly equal to the requested
  `ReasoningMode` produces provider-effective state;
- an exact different value produces rejected state only when the confirmation
  is otherwise well formed, the value is one of that exact response's
  advertised effort options, the qualified prepared route admits it as a
  reasoning mode, and it satisfies the projection's bounded value admission;
  and
- a missing, malformed, duplicated, unadvertised, unbounded, out-of-order, or
  otherwise ambiguous value remains `Runtime` failure with no contribution.

The adapter retains only the bounded reasoning value needed for normalized
projection. It never exposes the ACP response, config-option object, command,
path, credential, or arbitrary provider payload.

The two caller-supplied source IDs must differ and are admitted before provider
work. Prepared selection and session-start rows use
`AdapterContribution(prepared_source_id)`. Post-open acknowledgement,
provider-effective, and rejected truth use
`ActiveSessionObservation(active_session_source_id)`. The latter never becomes
the source of prepared truth.

A prepared contribution may carry requested, prepared, and pending reasoning.
It never carries provider-effective or rejected state. A successful projected
open carries provider-effective state only after exact matching confirmation.
An exact well-formed mismatch returns `Rejected` with the rejected
contribution and no open session. Omitted reasoning creates neither an
acknowledgement row nor an unused active-observation source.

## Candidate D Readiness

Candidate D now passes the Batch 9.4 promotion rubric as one package-coherent
card:

- its complete adapter-package remainder is exactly 53 census tuples: 30
  `claude-agent.acp`, 12 `claude-code.headless`, and 11
  `claude-code.response-only`;
- all prepared facades and the sole active-observation facade are named;
- the only new public API is the adapter-owned additive result above;
- the existing runtime composer, source kinds, acknowledgement authority,
  feature/control identities, fixed maxima, and failure types are sufficient;
- provider-free fixtures can prove matching, rejected, absent, and unknown
  acknowledgement outcomes plus exact ledger coverage; and
- one adapter package fits the focused validation boundary.

Rows without exact prepared or acknowledgement evidence stay withheld at
construction. The model-catalogue row is not inferred from the route matrix.
Activity stays descriptor-only. Structured-run, headless, response-only,
session-management, and interactive-session applicability remain distinct.

## Review Oracle

Invariant: only the exact `claude-agent.acp` open acknowledgement may publish
provider-effective or rejected reasoning, and it must not change the existing
open path or widen another route.

Smallest counterexamples and required stops:

- prepared success presented as provider-effective reasoning — fail before
  publication; only the exact post-open confirmation may prove it
- a different `currentValue` returned as rejected without appearing in both
  the exact response's advertised effort options and the qualified route's
  reasoning modes — fail as `Runtime` with no contribution
- malformed or missing confirmation turned into rejected state — fail; exact
  rejected value is required
- equal prepared and active source IDs — fail before provider work
- an active-observation source attached to prepared rows — fail contribution
  admission
- omitted reasoning names an acknowledgement row or unused observation source
  — fail
- the preserved and projected open methods return different route failure
  codes or cleanup outcomes for the same fixture — fail
- a Claude Code headless or response-only run gains active-session
  acknowledgement from the ACP session path — fail on route/operation
  applicability
- the 53-row ledger reaches its total through a filter, exception list, or
  borrowed route identity — fail exact tuple reconciliation

## Validation Boundary

Card 031 uses only `swallowtail-adapter-claude-agent` plus the repository-owned
semantic API, route/docs/Northstar, god-file, and diff checks. It changes the
adapter public API baseline. It does not change runtime, testkit, core,
Contracts 037/047/057/061, the census, or any provider claim. No live probe or
provider contact belongs to the card.

## Authority

- [Contract 061](../contracts/061-consumer-route-feature-and-control-projection.md)
- [Card 030 stop](../roadmaps/g05/batch-cards/030-contract-061-acknowledgement-candidate-reassessment.md)
- [Batch 9.4 package expansion](2026-08-31-contract-061-batch-9-4-package-expansion.md)
- [reviewed census](2026-08-30-consumer-route-feature-and-option-projection-census.csv)
- [Batch 9.1 public baseline](2026-08-31-contract-061-batch-9-1-public-baseline-gate.md)
