# 082 Claude SDK Mid-Session Model And Effort

Status: ready
Owner: Tom
Created: 2026-09-04
Updated: 2026-09-05
Milestone: `../029-claude-sdk-interactive-parity.md`
Depends on: cards 080 and 081 merged; Research 278 §4 and §8 layer 2; the Bovine requirement item 4

## Goal

Let a consumer change the model mid-session on `claude-agent.sdk` and
select effort at open, with the effective value confirmed by the SDK and
never assumed. Where the SDK offers no confirmation or no mid-session
setter, the route says so in a typed outcome rather than inventing one.

## Evidence Boundary

Research 278 §4 records `Query.setModel`, `Query.supportedModels`,
`Options.model` and `fallbackModel`, `Options.effort` with levels `low`,
`medium`, `high`, `xhigh`, `max` (`max` session-scoped, never persisted),
and `Options.thinking`; `setMaxThinkingTokens` is deprecated. No
mid-session effort setter is recorded. The sidecar today passes `model` at
open and fails `model_mismatch` if the init message reports a different
model.

## Scope

1. **Model at open** stays as is. Add `supported_models` to the open
   evidence from `Query.supportedModels`, bounded, so a consumer can offer
   a real list.
2. **Model mid-session**: a `set_model` wire command calling
   `Query.setModel`, returning a confirmed value. Confirmation means the
   SDK reports the model on the next `system` or assistant message, or
   `supportedModels`/state reflects it; if the SDK gives no confirmation
   signal, the route returns `model_change_unconfirmed` and keeps the
   previously confirmed model as effective. Reject a model not in
   `supported_models` before the SDK call. Expose it on the route-local
   session handle as an additive method.
3. **Effort at open**: add an optional `effort` to the session profile with
   the five admitted levels; pass `Options.effort`; confirm from the init
   message where the SDK reports it, else record it as requested-only in
   the open evidence with a typed flag. Never pass `fallbackModel`.
4. **Effort mid-session**: do not invent it. If the SDK still has no
   setter, the handle exposes no such method and the guide says effort is
   an open-time selection; if the worker finds a supported setter in the
   pinned `0.3.259` SDK, implement it under the same confirm-or-typed-fail
   rule and record the evidence.
5. `Options.thinking` stays out of scope for this card.
6. Fake-SDK fixture: prove confirmed model change, unconfirmed change with
   the previous model retained, rejection of an unsupported model, effort
   at open with and without confirmation, and that the default profile is
   byte-identical.
7. Guide, `claude-agent.sdk` matrix cells, `CHANGELOG.md` `[Unreleased]`,
   additive API baseline. One PR.

## Out Of Scope

Thinking control; resume and listing (card 083); MCP (084); ACP routes;
version pins; live Claude calls.

## Acceptance Criteria

- [ ] a model change reports a confirmed effective model or a typed
      unconfirmed outcome; never an assumed value
- [ ] unsupported models are rejected before the SDK call
- [ ] effort is selectable at open with the admitted levels only
- [ ] no effort setter is exposed unless the SDK evidence supports it
- [ ] default profile unchanged; API diff additive

## Validation

- `cargo fmt -p swallowtail-adapter-claude-agent -- --check`
- `effigy validate:focused swallowtail-adapter-claude-agent`
- `effigy package:verify-affected swallowtail-adapter-claude-agent`
- `effigy package:api`
- `effigy qa:routes`
- `effigy qa:guides`
- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`

## Review Oracle

Invariant: an effective model or effort value is reported only when the SDK
confirmed it. Smallest counterexample: `set_model` reporting the requested
value without a confirmation signal, or an effort setter that exists without
SDK evidence.

## Auto-Continuation

No. Stop after one reviewable PR for exact-head review.
