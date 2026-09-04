# Preflight Tool Exclusion Keyed On Boundary Claim

Date: 2026-09-04
Contract: `../contracts/013-interactive-session-access-policy.md`
Roadmap: `../roadmaps/g05/029-claude-sdk-interactive-parity.md`

## Ruling

Card 080's PR 221 stopped because shared preflight refused `ReadWrite` with
`Capability::ToolCalls` for every session policy. Contract 013 excludes
consumer tools only in the Bounded Workspace Task profile, where they could
bypass the one writable root; the Ambient Harness profile claims no boundary
and already allows read-write. The operator ruled to narrow the guard to the
boundary claim. Contract 013 gains one clarifying paragraph; no dimension or
profile changes; the Codex profiles are unaffected.

## Runway

Card 089 changes core and testkit. PR 221 merges as it stands. Card 080's
worker then lifts the typed write refusal in a second PR. All three ride
`v0.4.1`.
