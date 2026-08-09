# 119 Route-Path Idioms Opt-In Evidence

Status: draft
Owner: Tom
Date: 2026-08-09

## Question

Can the idioms mechanism (Contract 055) become an opt-in route feature so
consumers stop re-wiring selection and delivery per session — and which
consumer proves it?

## Method

Seam inventory over the realized runtime and core surfaces, plus the
authority map and consumer evidence. No code changes.

## Seam Evidence

The realized runtime already carries every seam the feature needs:

1. **Fold channel** — `SessionOptions.developer_instructions` is an optional
   `OperationContent` field on interactive session options (Contract 012),
   already translated through harness configuration channels per Contract
   034 and redacted per the existing developer-instruction discipline.
2. **Registration pattern** — `DiagnosticObserver` is an optional
   `Send + Sync` trait registered on the execution-host service set
   (Contract 010); missing registration is a no-op, never a failure. The
   idioms source and recorder can follow the same shape.
3. **Plan binding** — Contract 037 prepared plans bind activity-affecting
   options as immutable prepared evidence; an idioms opt-in can bind
   source identity and maximum at preflight.
4. **Capability gating** — routes already advertise portable capabilities;
   an idioms capability lets non-advertising routes reject the opt-in
   before provider work.

## Boundary Ruling

The vision rule: hosts own system prompts and agent instructions. The
opt-in flag is the host's intent; the runtime populates the opted-in
developer-instructions field from an opted-in `IdiomSource` under one fixed,
bounded rendering rule. The runtime never invents instructions when the flag
is absent, never renders without a registered source, and never changes
provider payload shapes. This extends Contract 055's "no prompt composition"
boundary with an explicit host-gated exception and must be recorded as an
amendment, not inferred.

## Testbed Choice

Nucleus is the interactive, review-heavy consumer: persistent sessions,
typed question and plan flows, task lists, and accept/reject-style product
decisions — the exact signal flow idioms exist for. Soundcheck's bounded
structured runs generate few interaction signals and would prove only the
delivery half. Nucleus adoption is therefore the higher-information testbed;
the correction-loop proxy remains a later measurement lane.

## Recommendation

Proceed to Spec 007 and Contract 056. Sequence: runtime surface first
(ports, session option, fold rule, conformance), then prepared binding and
capability gate with a Codex app-server proof (Nucleus's primary interactive
route), then a bounded Nucleus adoption delta.
