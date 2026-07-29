# 101 Provider Retention Implementation Tranche

Status: completed
Owner: Tom
Created: 2026-07-28
Milestone: `../030-provider-retention-feature-closure.md`
Depends on: card 100

## Objective

Implement only the provider-retention cells qualified by card 100.

## Scope

1. Add public prepared operations for the selected exact routes.
2. Preserve binding, capability, version, topology, access, effect, and
   deletion-strength truth.
3. Keep archive, restore, delete, close, and cleanup separate.
4. Join transport and host work before releasing access.
5. Change matrix cells only after focused conformance passes.

## Acceptance Criteria

- [x] every converted cell has a public prepared path
- [x] raw provider ids cannot mint authority
- [x] no action or outcome is strengthened
- [x] cancellation and uncertainty remain visible
- [x] focused exact-range conformance passes offline

## Evidence

- Gemini CLI exposes exact prepared stored-transcript deletion and separate
  durable versus temporary-cleanup structured profiles. A durable run returns
  its management binding only after successful terminal completion.
- Claude Agent exposes an opt-in temporary structured profile that performs
  native close then exact operation-private session deletion. Its durable
  profile remains unchanged.
- OpenAI background Responses perform one terminal deletion attempt and report
  exact `Response` cleanup truth without changing inference status.
- Gemini, Claude Agent, and OpenAI focused fixtures cover success,
  cancellation, lost or contradictory acknowledgements, reconciliation, and
  release ordering.
- `cargo fmt --all -- --check` and `cargo check --workspace --lib -j2` pass.

## Auto-Continuation

Continue to card 102 only after every selected cell has deterministic
production evidence.
