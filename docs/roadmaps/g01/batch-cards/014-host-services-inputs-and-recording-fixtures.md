# Host Services, Inputs, And Recording Fixtures

Status: completed
Owner: Tom
Roadmap: 005 Async Runtime and Conformance
Updated: 2026-07-19

## Goal

Implement Contract 010 host-service traits, portable input records, and
recording fixtures without real host side effects.

## Scope

- scoped task, blocking-work, and monotonic-time services
- process, network-policy, credential, working-resource, attachment, and
  diagnostic-observer services
- opaque executable, endpoint, credential, resource, attachment, and schema
  references
- secret lease redaction and scope behavior
- attachment and structured-output descriptors
- recording service fixtures with call counters and scripted outcomes

## Out Of Scope

- OS process implementation
- HTTP client or provider SDK
- filesystem materialization
- real credential stores or sign-in
- schema validation

## Acceptance Criteria

- drivers can require only the services they use
- fixture services expose every attempted side effect
- secrets and raw host paths are absent from formatting and public records
- attachment/schema limits participate in requirements
- owned and external cleanup authority remains distinct

## Validation

- focused runtime and testkit tests
- `effigy qa`
- `git diff --check`

## Closeout

- Runtime host ports cover scoped tasks, blocking work, monotonic time,
  processes, network policy, credentials, working resources, attachments, and
  restricted diagnostics without a mandatory god trait.
- Executable, endpoint, credential, resource, attachment, and schema
  references are host-owned and opaque in default formatting.
- Secret leases redact, remain audience-bound, clear their byte buffer, and
  invoke their release hook on drop.
- Attachment and structured-output records carry safe metadata; inline schemas
  are bounded, and attachment/schema constraints participate in preflight.
- Testkit recording services expose every attempted host call and support
  scripted success or failure without performing real I/O.
