# Publication Lane Closure And Paused-Item Disposition

Date: 2026-07-30
Status: complete

## Decision

Registry publication is removed from Swallowtail's active and backlog runway.
The operator wants months of working-application usage evidence before
publication is considered. No recurring publication decision, registry
preflight, retained-candidate refresh, tag, push, or release task remains.

The existing `0.1.0` artifact is retained as a local compatibility snapshot.
Package assembly, extracted-package execution, API checks, and consumer
compatibility remain useful development gates. They do not imply a pending
release and should not force a retained-candidate rebuild after every additive
change.

Contract 036 and the release topology remain durable future safety rules. If
publication returns, it needs a new roadmap and fresh source, registry,
ownership, package, compatibility, and usage evidence.

## Paused-Item Audit

Card 059 no longer needs a canonical-source wait. Its transient lifecycle
proof remains valid, and card 136 later supplied broader extracted-package
evidence: all 23 packages assembled, the workspace compiled, and 14 lifecycle
suites covered all five management adapters. The remaining candidate-refresh
tail served the now-closed publication lane. Card 059 is superseded and
roadmap g02.019 is complete.

Pi RPC cards 097-098 remain paused on an upstream interface fact, not an
operator decision. Pi cannot currently attach durable provider state while
preserving and corroborating the exact host-leased working resource. Resume
only when a maintained public Pi surface closes that gap. Weakening resource
binding is not recommended.

Grok remains backlog because the operator has no account for its exact
activation-only delegated-authentication probe. It needs authorized account
state or new maintained documentation before currentness and implementation
can resume.

The Python Kimi CLI route is declined. Installed native Kimi Code `0.31.0`
subsequently passed authenticated headless and ACP probes, so the separate
distribution adds no current practical capability. It must not widen the Kimi
Code identity if a concrete missing capability later reopens the proposal.

Provider-session management binding persistence is consumer-demand gated. It
needs a real requirement to execute provider archive, restore, or delete after
application restart before a versioned export/import contract is justified.
The operator confirmed the deferral.

## Current State

Swallowtail has no ready implementation card. Paused and backlog items do not
block a new g02 stabilization lane. The active decision remains consumer
observable-activity adoption or selection of another contract-backed
stabilization target.
