# Lead: configuration-owned native Grok MCP mediation route

Status: lead; not promoted
Source: Research 289 (card 118 evidence continuation, PR 262) conditional alternative
Recorded: 2026-09-07

If the exact-route probe proves Grok Build rejects or ignores client-supplied
ACP `mcpServers`, the only evidence-backed path to consumer tools on Grok is a
separately qualified, configuration-owned native Grok MCP mediation route: a
new exact route id and version segment, not a repurposed `grok-build.acp`.

Why this is a lead and not a card: a new route authority is product scope
beyond g05.035, whose Batch D keeps Grok consumer tools and MCP withheld
unless the ACP surface gate passes. Promotion needs the operator's explicit
decision after the probe result exists.

Promotion conditions: the probe capsule recording the ACP result per installed
version (`1.0.4`, `1.0.5` as separate segments); the native MCP subsystem's
configuration authority and discovery behaviour frozen from vendor artifacts;
a Contract 061 row plan for the new route only.
