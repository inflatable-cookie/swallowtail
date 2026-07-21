# 040 OpenCode HTTP Harness Driver

Status: completed
Owner: Tom
Updated: 2026-07-20
Milestone: `../012-opencode-http-harness-proof.md`

## Objective

Implement attached OpenCode discovery, catalogue, and interactive sessions over
the approved HTTP/SSE subset.

## Governing References

- Research 004
- Contracts 005, 006, 008-010, and 012-014
- Roadmap 012
- card 039 and `swallowtail-adapter-opencode` fixture `opencode-1.14.48`

## Scope

- new OpenCode adapter crate
- attached endpoint, delegated provider auth, model catalogue, sessions, turns,
  SSE normalization, abort, deadlines, and close
- exact preflight and route binding
- blocking libcurl HTTP/SSE work behind scoped `BlockingWorkService` jobs
- bounded async delivery through host-owned, joined `ScopedTaskService` work

## Out Of Scope

- owned server process
- provider credential injection
- config mutation, sharing, shell, or experimental tool endpoints
- server basic-auth credentials, resume, bounded-write access, dynamic tools,
  non-default session options, or implicit provider/model selection

## Implementation Steps

1. Replace the fixture-only crate stub with private version-bound protocol and
   bounded HTTP/SSE parsing modules.
2. Register model-catalogue and interactive-session roles with only the
   observed read-only capabilities and exact host-service requirements.
3. Acquire the endpoint, delegated credential, and read-only working-resource
   leases under one preflight-bound host and audience.
4. Run blocking libcurl operations only through the host blocking-work port;
   route the SSE reader through bounded channels and a joined scoped task.
5. Observe health/version, translate provider and model ids without selecting
   defaults, create the deny-first session, subscribe before prompt, filter the
   bound session, and normalize ordered terminal state.
6. Abort on cancellation, deadline, permission, question, provider failure, or
   protocol failure; join local work before releasing leases.
7. Reject resume, write access, tools, unsupported options, secret credentials,
   and foreign events before widening provider behavior.

## Acceptance Criteria

- [x] adapter claims only observed roles and capabilities
- [x] close does not stop the external server
- [x] abort, disconnect, timeout, and provider failure stay distinct
- [x] no OpenCode type leaks into core/runtime

## Evidence Required

- fixture-driven route and request-shape assertions
- catalogue identity/default and limit translation tests
- read-only permission and unsupported-input rejection before network work
- ordered output, duplicate-idle, abort, provider failure, disconnect, unknown
  event, foreign-session, deadline, and joined-cleanup tests
- dependency proof showing no Tokio/global executor and no concrete HTTP type in
  core or runtime

## Validation

- focused adapter tests pass with 19 tests
- dependency scan shows libcurl only in the OpenCode adapter; no Tokio or
  global executor is present
- full repository QA passes with 150 tests
- `git diff --check` passes

## Outcome

- implemented exact host-approved HTTP endpoint and delegated-credential
  acquisition with awaited release
- implemented version-bound health, provider catalogue, deny-first session,
  prompt, event, and abort routes
- kept blocking libcurl work behind `BlockingWorkService`; one host-owned
  joined task owns each turn and its bounded SSE reader
- normalized output, cancellation, timeout, provider failure, disconnect,
  unknown events, foreign sessions, duplicate idle, and cleanup without raw
  provider diagnostics

## Stop Conditions

- required behavior lies outside card 039 fixtures
- provider selection or permission policy becomes implicit

## Auto-Continuation

Yes, after card 041 is ready and focused validation passes.
