# 144 Grok ACP Driver And Prepared Facade

Status: completed
Owner: Tom
Created: 2026-07-30
Milestone: `../043-grok-build-maintained-acp-route.md`

## Goal

Implement one exact ambient Grok ACP v1 interactive route through the normal
prepared facade.

## Governing Refs

- Research 070
- Contracts 009-015, 023, 029, 033-034, 037, and 044
- roadmap g02.043
- cards 142-143

## Scope

1. Add the exact `--no-auto-update agent stdio` process request.
2. Validate ACP wire, agent version, model, capabilities, and `cached_token`.
3. Activate the existing delegated credential once and discard response
   metadata.
4. Allocate one durable local session and run bounded text turns.
5. Map native cancellation, terminal outcomes, ACP activity, negotiated model
   options, provider requests, disconnect, and explicit attachment close.
6. Add a typed prepared input, evidence, integration, and session operation.
7. Join process, protocol pump, turn task, working resource, and delegated
   credential work.

## Acceptance Criteria

- [x] no authentication method fallback exists
- [x] access, configuration, isolation, retention, and model drift fail early
- [x] raw auth metadata, stderr, prompts, and provider payloads stay private
- [x] activity fidelity matches Contract 044
- [x] close claims attachment cleanup, not session deletion
- [x] cancellation and every exercised failure path join owned work
- [x] low-level and prepared paths remain available

## Evidence

- exact `--no-auto-update agent stdio` request
- exact ACP v1, `0.2.114`, `cached_token`, headless activation, and
  `grok-4.5` validation
- one delegated credential acquisition and release per attachment
- durable provider-session preservation with ambient read-write harness
  authority and no deletion claim
- bounded assistant, reasoning-summary, plan, provider-tool, unknown-event,
  provider-request, cancellation, and terminal projection
- 12 focused Grok tests and warnings-denied clippy pass
- no live provider prompt or account mutation

## Validation

- focused Grok driver, prepared facade, activity, and failure tests
- shared ACP activity assertions
- focused warnings-denied clippy
- `git diff --check`

## Stop Conditions

- Stop if exact activation can open login or select another mechanism.
- Stop if a required callback lacks host authority.
- Do not add load, resume, lifecycle management, or sandbox claims.

## Auto-Continuation

Yes. Continue to card 145 after focused validation.
