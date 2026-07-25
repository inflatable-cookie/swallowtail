# 033 llama.cpp Attached And Owned Facades

Status: complete
Owner: Tom
Created: 2026-07-25
Milestone: `../011-specialized-runtime-facades.md`

## Objective

Add separate prepared paths for attached llama.cpp inference and owned
ephemeral serving.

## Governing Refs

- Contracts 007, 014, 018, 020, 024, 029, 031, and 037
- llama.cpp attached and owned fixtures
- card 032

## Scope

1. Prepare attached runtime endpoint, exact build, model inventory, and route
   without serving authority.
2. Prepare owned serving from one approved executable, GGUF artifact lease,
   alias, loopback policy, and readiness bound.
3. Bind attached inference and owned start/stop through different types.
4. Preserve health, build, route readiness, stderr supervision, and ordered
   cleanup.
5. Keep model acquisition, persistent serving, and Monkey ownership out.

## Acceptance Criteria

- [x] attached preparation cannot stop the external server
- [x] owned preparation cannot use an unapproved artifact or endpoint
- [x] readiness completes before a serving handle is returned
- [x] teardown invalidates endpoint authority before artifact release
- [x] route and build drift fail safely

## Evidence

- separate `prepare_llama_cpp_attached` and `prepare_llama_cpp_owned`
  constructors expose different prepared and bound-operation types
- attached catalogue and inference retain exact b9910/f5525f7e7 evidence and
  the unchanged external-server lifecycle
- owned selection couples the approved GGUF artifact and exact route; its
  derived start request repeats the same artifact binding
- owned start returns the low-level handle only after stderr endpoint
  observation, loopback publication, health, properties, and catalogue checks
- deterministic local and remote-authoritative tests preserve
  endpoint-before-artifact release and build-drift closure
- 32 focused llama.cpp tests pass; full repository QA and package API evidence
  are recorded in the closeout log

## Validation

- attached and owned deterministic suites
- artifact, serving, native-runtime, and hosted-direct conformance
- both host identities
- low-level regression tests

## Auto-Continuation

No. Close g02.011 and advance to card 034.
