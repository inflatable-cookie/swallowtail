# 019 Local llama.cpp Attached

Status: completed
Owner: Tom
Created: 2026-08-20
Depends on: completed g04.018
Vision tags: consumer integration, route readiness, explicit selection
Contract refs: 011, 014, 029, 037, 047, 057
Planning state: cards 053-055 completed
Research: 170

## Problem

llama.cpp attached already has a prepared facade and an exact opaque
runtime claim. A consumer still cannot list it as an addable local-runtime
route or admit an instance through Contract 057. There is no credential.
Swallowtail does not start or stop the operator-owned server. Owned
ephemeral serving stays a different route.

## Generation Runway Goal

Expand addable-route coverage on the proved local-runtime shape, then reuse
`prepare_llama_cpp_attached`.

## Goals

- [x] expose an adapter-local local-runtime addable descriptor for
      `llama-cpp.attached`
- [x] admit through the 057 store with no credential field
- [x] reuse `prepare_llama_cpp_attached` after admission
- [x] refresh access status, project 029 update observation, and keep
      subject Absent

## Non-Goals

- hosted interactive OAuth
- advertising `llama-cpp.owned` from this row
- starting, stopping, or installing the attached server
- inventing a catalogue `provider_id` so overlay can mark rows
- live `/health` probes in the addable row
- OpenHands production wiring
- rewriting `public-api-0.3.3`

## Execution Plan

### Batch 19.1 — Addable Descriptor

- [x] Execute card 053.
- [x] ship `AddableRouteDescriptor` from `swallowtail-adapter-llama-cpp`
- [x] topology local-runtime; config field API endpoint; no credential
      field

### Batch 19.2 — Admission And Prepare

- [x] Execute card 054 after card 053.
- [x] admit through the 057 store with an opaque endpoint config ref
- [x] no sign-in loop; reuse `llama_cpp_attached_access_profile`
- [x] `prepare_llama_cpp_attached` still prepares after admission with
      host `InstanceTargetRef`

### Batch 19.3 — Refresh, Update, And Subject

- [x] Execute card 055 after card 054.
- [x] refresh host-supplied `AccessStatus`; subject stays Absent
- [x] `observe_instance_update` reuses `llama_cpp_attached_runtime_claim`;
      032 stays unobserved unless an executable is supplied
- [x] unmarked catalogue rows stay unmarked; do not invent a provider id

## Acceptance Criteria

- [x] a consumer can assemble a catalog that includes llama.cpp attached by
      linking the adapter
- [x] admission writes no secret bytes and no credential refs
- [x] `prepare_llama_cpp_attached` still runs after admission
- [x] update observation reuses 029
- [x] no live server start, stop, or health probe from the addable row
- [x] `public-api-0.3.3` stays immutable

## Lane Runway

- previous: g04.018 installed Claude Agent ACP
- this milestone: local llama.cpp attached
- later: hosted OAuth gate

## Decision Gates

- Stop if Swallowtail starts or stops the attached server.
- Stop if `llama-cpp.owned` is advertised from this row.
- Stop if overlay invents a catalogue provider id.
- Stop if 047 `Ready` / `NotReady` changes.
- Stop if OpenHands gains a production route.
- Stop if this route starts a sign-in loop.
