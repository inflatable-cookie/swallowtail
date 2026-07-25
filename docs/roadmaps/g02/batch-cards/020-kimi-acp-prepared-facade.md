# 020 Kimi ACP Prepared Facade

Status: completed
Owner: Tom
Created: 2026-07-25
Completed: 2026-07-25
Milestone: `../008-representative-cross-shape-facades.md`

## Objective

Prove the provider-wide facade against the current Kimi Code persistent ACP
route.

## Governing Refs

- Contracts 015, 017, 023, 029, 032-034, and 037
- Kimi currentness and ambient-isolation research
- card 019

## Scope

1. Prepare one exact host-approved Kimi executable and compatibility result.
2. Preserve ambient harness configuration as the current supported posture.
3. Bind typed new, load, resume, prompt, and interruption operations.
4. Keep replay, bounded write callbacks, delegated authentication, resource
   authority, and optional isolation explicit.
5. Retain unverified-newer attempts without extending guaranteed support.

## Acceptance Criteria

- [x] normal setup needs no manual descriptor, binding, plan, or request echoes
- [x] load replay remains different from resume
- [x] ambient execution makes no containment claim
- [x] host/provider isolation remains optional and explicit
- [x] callbacks and delegated credentials preserve ordered joined cleanup

## Validation

- pinned offline Kimi corpus
- persistent ACP conformance under both host identities
- exact range and unverified-newer cases
- low-level driver regression

## Execution Evidence

- `prepare_kimi` probes one host-approved executable and retains exact
  qualified or unverified-newer compatibility, target, host, access
  provenance, environment, and configured-instance evidence
- `prepare_session` derives the model-bound instance, immutable plan, ambient
  configuration, `AmbientHost` isolation, workspace access, lifecycle
  capabilities, and new-session request
- prepared new, load, and resume delegate to the existing
  `InteractiveSessionDriver`; prompt and active-turn interruption remain on
  the returned low-level handles
- deterministic local and remote-authoritative fixtures prove prompt, bounded
  write, interruption, load replay, replay-free resume, reasoning,
  failure-before-effects, and ordered process/resource/credential cleanup
- 75 focused Kimi and ACP tests pass; one live installed probe remains gated
- full repository QA passes with 658 deterministic tests and four gated live
  probes ignored
- warnings-denied Clippy passes and Doctor remains at the known 19 findings
- additive Kimi public-API drift remains visible against the held candidate;
  card 036 owns baseline replacement
- the Kimi prepared-integration guide and example compile

## Auto-Continuation

Completed. Card 021 is active.
