# 2026-08-30 g05.003 Watcher HTTP Bridge Promotion

Status: complete
Owner: Tom
Date: 2026-08-30

## Decision

Promote Research 260's minimal HTTP candidate as a distinct provider-neutral,
operation-scoped watcher bridge. Contract 060 owns the closed listener,
private bearer authority, exact host/operation/turn correlation, completion
barrier, and joined cleanup.

The bridge is not a generic HTTP or MCP server. It does not reuse the sign-in
loopback port, serving-endpoint publication, network service, or watcher
registration as listener authority. It adds no container or sandbox
requirement.

## Sequencing

g05.003 card 016 is ready for the core/runtime/local-host/testkit bridge only.
Claude adapter configuration, watcher skill injection, Stop-hook wiring,
current version qualification, live provider work, and consumer claims remain
in planned cards 010-011.

The first worker returns one PR. No auto-continuation is authorized.

## Authority

- [Contract 060](../contracts/060-operation-scoped-watcher-http-bridge.md)
- [Research 260](../research/260-claude-code-watcher-bridge-transport.md)
- [g05.003](../roadmaps/g05/003-operation-scoped-watcher-proof.md)
- [card 016](../roadmaps/g05/batch-cards/016-operation-scoped-watcher-http-bridge-core.md)
