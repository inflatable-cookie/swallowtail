# 2026-08-29 g05.003 Claude Watcher Bridge Readiness

Status: complete
Owner: Tom
Milestone: g05.003
Cards: 010, 015
Contracts: 010, 041, 059
Research: 257

## Result

Card 010 is not implementation-ready after PR 118. Cards 009 and 014 now
supply the shared watcher registry and ordinary host-process supervision, but
the Claude route still lacks a provider-to-host transport.

Claude's private MCP configuration can name HTTP or stdio servers. Swallowtail
has no production MCP server, inbound loopback-listener lease, or helper RPC
boundary that can connect those calls to the in-process
`WatcherHostService`. The sign-in loopback callback port is purpose-limited,
and `ServingEndpointService` validates an endpoint already observed from an
owned child; neither grants a driver authority to bind a watcher server.

The installed Claude Code is `2.1.251`, above Research 257's admitted
`2.1.220..=2.1.241` window. The current binary still advertises `--bare`,
`--mcp-config`, `--strict-mcp-config`, `--settings`, `--add-dir`, and
`--include-hook-events`, but the live same-turn Stop re-entry proof remains
unrun.

## Decision

Card 015 is the sole ready lane. It must settle the host-owned MCP/IPC binding,
current-version segment, authentication and correlation, cleanup, and live
acceptance gate in Research 260. Cards 010-011 remain planned. No production
wiring starts from the incomplete transport assumption.

## Evidence

- PR 118 merged at `1f0ce87c`; post-merge planning tip began at
  `eb014a9b`.
- `HostServices` registers `WatcherHostService`, but no MCP listener or
  broker service.
- Claude Code `2.1.251` help confirms HTTP/stdio MCP configuration and the
  Research 257 headless flags.
- `effigy doctor` retains the inherited repository baseline: 384 god-file
  findings, one stale graph warning repaired during assessment, and one
  generated-in-source warning.

## Next

Run card 015 from its committed manual worker handoff. Do not start cards
010-011.
