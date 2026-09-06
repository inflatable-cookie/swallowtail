# 085 Grok ACP Answerable Permissions

Status: ready
Owner: Tom
Created: 2026-09-04
Updated: 2026-09-06
Milestone: `../029-claude-sdk-interactive-parity.md`
Depends on: the current Grok permission observation in `crates/swallowtail-adapter-grok/src/turn.rs`; the `claude-agent.acp` `CallbackHub` precedent; Contract 015

## Goal

Answer `grok-build.acp` `session/request_permission` requests from the consumer through the runtime callback exchange, on the `claude-agent.acp` precedent, or publish an explicit labelled activity-only posture if Grok Build's ACP cannot be answered.

## Scope

1. Today `connection/dispatch.rs` routes `session/request_permission` to `reject_permission`, which observes, cancels the turn, and answers `cancelled`. Port the `claude-agent.acp` pattern: a `CallbackHub` with `exchanges_permissions` gating, `enqueue_permission` into the runtime `CallbackExchange`, and a responder that writes the selected option back with the provider request id.
2. Profile flag on the Grok prepared session selecting exchange versus the current reject-and-cancel; default unchanged (reject).
3. Bound: at most 32 options, `toolCallId` required, one outstanding request per turn, deadline from the turn; abandonment resolves as rejection and is recorded.
4. Fake ACP fixture proofs: request answered with each option kind, timeout, abandonment, malformed request, request without an active turn.
5. If Grok Build's live ACP identity (the frozen `grok-build.acp` research) shows it never honours the answered option, stop and publish the labelled activity-only posture in the matrix instead; that is a valid outcome.

## Out Of Scope

Grok discovery or install guidance (card 088); any Claude route; new ACP methods.

## Acceptance Criteria

- [ ] exchange path implemented on the precedent, default unchanged
- [ ] bounds and abandonment recorded typed
- [ ] fixture proofs as listed, or the labelled activity-only posture with evidence
- [ ] guide, matrix, changelog `[Unreleased]`, additive baseline; one PR

## Validation

- `cargo fmt -p swallowtail-adapter-grok -- --check`
- `effigy validate:focused swallowtail-adapter-grok`
- `effigy package:verify-affected swallowtail-adapter-grok`
- `effigy package:api`
- `effigy qa:northstar`
- `git diff --check`

## Review Oracle

Invariant: a permission answer reaches the provider only from a consumer decision on the runtime exchange, and silence never grants. Smallest counterexample: an option selected by the adapter.

## Stop Conditions

Grok Build's ACP requires a method or field outside Contract 015 (record; return to Chatterbox).

## Auto-Continuation

No. Stop for exact-head review.

## Result

Implemented the opt-in Grok Build ACP permission exchange on the Claude ACP
callback precedent. The default reject-and-cancel path is unchanged; the
consumer path exposes only bounded one-shot choices, carries turn deadlines,
and abandons pending requests on cancellation, timeout, failure, or close.
Provider-free ACP fixtures cover both one-shot options, persistent-option
withholding, malformed input, bounds, abandonment, timeout, and no active turn.
Ready for exact-head independent review after push.

The stable solution feature matrix intentionally retains `permission_exchange`
as `No`: the shared route-contract inventory still classifies the Grok cell as
`upstream_unsupported`. The route guide and prepared facade document the
explicit opt-in exchange without changing that stable solution disposition.
