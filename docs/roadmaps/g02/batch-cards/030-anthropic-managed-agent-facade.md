# 030 Anthropic Managed Agent Facade

Status: complete
Owner: Tom
Created: 2026-07-25
Milestone: `../010-hosted-direct-and-provider-state-facades.md`

## Objective

Add a prepared facade for Anthropic's provider-managed agent resource route.

## Governing Refs

- Contracts 014, 020, 022, 029, and 037
- managed-agent fixtures
- card 029

## Scope

1. Prepare one operator-owned agent, driver-owned environment/session, endpoint
   audience, credential source, and retention posture.
2. Bind create, event recovery, callback, interrupt, and ordered deletion.
3. Preserve authoritative persisted events and provider-managed recovery.
4. Grant no repository, provider filesystem, external sandbox network, or
   local-container authority.
5. Keep this route separate from Anthropic direct Messages.

## Acceptance Criteria

- [x] managed resource ownership and retention are explicit
- [x] recovery reconciles authoritative events without silent retry semantics
- [x] callbacks stay correlated and downstream-executed
- [x] remote deletion truth remains exact
- [x] hosted direct and managed-agent facades cannot substitute

## Validation

- [x] deterministic managed-agent corpus
- [x] recovery, callbacks, interrupt, usage, deletion, and cleanup cases
- [x] both host identities
- [x] Anthropic lint with warnings denied and 44 adapter tests
- [x] full repository QA and 23-crate public API declaration baseline
- [x] doctor returns only the known 19 oversized-file findings

## Auto-Continuation

No. Close g02.010 and advance to card 031.
