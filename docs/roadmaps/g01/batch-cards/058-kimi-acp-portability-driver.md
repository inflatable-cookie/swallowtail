# 058 Kimi ACP Portability Driver

Status: complete
Owner: Tom
Updated: 2026-07-21
Milestone: `../018-kimi-code-acp-portability-proof.md`

## Objective

Implement the explicit ambient Kimi Code mapping over the existing ACP wire
layer and persistent-session runtime.

## Governing References

- `../../../contracts/017-provider-owned-session-load-replay-and-host-containment.md`
- `../../../research/009-native-macos-kimi-containment-and-successor-delta.md`
- `../../../research/010-kimi-code-successor-and-artifact-currentness-repair.md`
- `../../../research/011-kimi-macos-app-sandbox-runtime-compatibility.md`
- `../../../research/012-local-harness-orchestrator-isolation-posture.md`
- `057-harness-process-filesystem-containment.md`
- `065-kimi-code-successor-currentness-and-fixture-delta.md`
- `066-optional-harness-isolation-rebaseline.md`

## Scope

- pinned current successor process and isolated app-owned delegated-auth state
- explicit `AmbientHost` isolation and ambient process/network authority
- negotiated new/load/resume, replay, prompt/update, cancellation, and text
  write-callback subset exactly as retained by card 065
- working-resource location and callback scope without a filesystem-boundary
  claim
- exact session, resource, access, isolation, topology, and cleanup binding
- joined task/process/resource/credential cleanup

## Out Of Scope

- credential extraction, login mutation, terminal callbacks, MCP injection,
  permission approval, model-routing policy, or any bounded filesystem claim
- configured non-OAuth provider credentials
- provider- or host-enforced Kimi isolation

## Acceptance Criteria

- [x] no Kimi branch enters provider-neutral ACP framing
- [x] unsupported capabilities fail before provider or host effects
- [x] only a prior exact binding may load or resume
- [x] host-mediated writes stay inside one authorized callback resource
- [x] the harness working directory is never described as containment
- [x] resume cannot change the ambient isolation posture
- [x] disconnect and cancellation leave no detached work

## Validation

- focused driver and fixture tests
- focused clippy
- `git diff --check`

## Evidence

- `swallowtail-adapter-kimi` pins Kimi Code `0.28.1`, ACP wire `1`, one
  host-approved `acp` process argument, isolated provider state, and an opaque
  delegated membership credential lease.
- runtime load and replay records keep load separate from resume; durable
  bindings now include working resource and expanded access policy.
- local host text replacement rejects read-only leases, traversal, symlinks,
  non-regular targets, and writes beyond the one MiB bound.
- deterministic production fixtures cover new, load replay, resume without
  replay, prompt output, text write callback, active-turn cancellation, exact
  binding rejection before effects, and joined resource/credential cleanup.
- focused adapter, ACP, local-host, runtime, Codex, and OpenCode tests pass;
  warnings-denied focused clippy and `git diff --check` pass.
- full repository QA passes with 269 tests; installed probes remain gated.
- doctor remains at the inherited 19 findings: 12 warnings and 7 errors.

## Stop Conditions

- cards 057, 065, or 066 remain incomplete
- implementation needs behavior excluded or unsettled by the successor corpus
- implementation would require a bounded filesystem or network claim

## Auto-Continuation

No. Mark card 059 ready only after focused driver validation passes.
