# Claude Agent ACP Structured Projection

Date: 2026-07-27
Card: `../roadmaps/g02/batch-cards/074-acp-structured-projection-and-claude-agent.md`

## Changed

- Added a provider-neutral ACP single-turn projection assertion pack to
  `swallowtail-testkit`.
- Added independent `StructuredRun` registration, validation, lifecycle, and
  prepared operation to Claude Agent ACP.
- One run creates one private session, executes one prompt, reaches one
  terminal outcome, closes natively at qualified versions, and joins all
  process, turn, deadline, resource, and credential work.
- Bound durable transcript retention explicitly. Native close and process exit
  do not claim transcript deletion.
- Preserved Claude's existing filesystem callback and permission-stop
  behavior without inventing a consumer approval exchange.
- Kept stdio and explicit remote ACP composition separate.
- Changed the Claude Agent solution-matrix structured-run cell from `No` to
  `Yes`.

## Current State

The harness projection lane now has a reusable ACP assertion pack and its first
production proof. The unchanged Claude range remains `0.53.0..=0.61.0`,
excluding `0.58.0`; newer stable versions remain visible and unverified.

Card 075 is ready for Pi RPC and attached OpenCode HTTP. Kimi thread deletion
remains unsupported.

## Validation

- all Claude Agent adapter tests pass, including five structured-run tests
- all testkit tests pass with the new assertion pack
- strict all-target Clippy passes for both crates
- explicit remote-ACP portability and full Claude lifecycle regression pass
- docs and provider-route checks pass
- no live provider access, credential, account, consumer edit, or release
  mutation occurred
