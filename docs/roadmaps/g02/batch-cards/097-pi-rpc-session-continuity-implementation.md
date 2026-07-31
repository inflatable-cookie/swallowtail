# 097 Pi RPC Session Continuity Implementation

Status: moved to backlog
Owner: Tom
Created: 2026-07-28
Milestone: `../029-pi-rpc-session-continuity.md`
Depends on: card 096

Backlog gate: Research 053 found no public Pi attachment surface that preserves
the exact host-leased working resource. Resume only after its stated upstream
unpause condition is met.

## Objective

Implement Pi RPC load and resume through a separate persistent-session
prepared profile.

## Scope

1. Implement only the exact load and resume paths qualified by card 096.
2. Keep the current ephemeral interactive and `--no-session` structured-run
   profiles unchanged.
3. Bind the durable session to exact configured instance, execution host,
   working resource, access, model, provider-state posture, and qualified
   version evidence.
4. Return bounded ordered replay before load readiness.
5. Return no replay phase for resume.
6. Preserve extension cancellation, RPC correlation, prompt lifecycle,
   callback policy, deadlines, disconnect truth, credential-last release, and
   joined owned-process cleanup.
7. Expose public prepared operations and inspectable safe evidence.
8. Change matrix cells only after focused conformance passes.

## Acceptance Criteria

- [ ] Pi load and resume map to separate public prepared operations
- [ ] copied session paths or ids fail without a matching opaque binding
- [ ] load replay completes before a usable handle exists
- [ ] resume emits no historical replay
- [ ] ephemeral profiles still prohibit provider state
- [ ] every guaranteed compatibility segment executes
- [ ] cancellation, overflow, disconnect, and cleanup are joined
- [ ] no native-close, archive, restore, delete, or containment claim widens

## Auto-Continuation

Continue to card 098 only after both cells have deterministic production and
prepared-facade evidence.
