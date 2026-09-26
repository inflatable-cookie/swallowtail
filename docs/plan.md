# Plan

Updated: 2026-09-26

What matters next for Swallowtail, and why. Status lives in Queue; this file
is intent. Items that own a feature-matrix producer gap carry a lane key the
matrix cites as `plan:<key>`; keep the key stable while the gap is open.
Earlier task text for carried items (the g06 task files) is in Git history
at `49b0d308`, before the lean cut.

## Now

1. **Version currentness** (lane `version-currentness`) — standing, never
   finished. Run the Contract 029 checkpoint when official stables move or a
   consumer hits an unverified-newer point; one family at a time. Every stop
   gets an adaptation task (Tom, 2026-09-26); an adaptation that needs a
   Contract 023 exception or narrows a consumer-visible guarantee comes back
   to Tom as a ruling. No open stop: `antigravity.headless` qualifies current
   official `1.2.11` with `AGY_CLI_MODEL_API_MAX_RETRIES=0` pinned
   (Research 359). Official `claude-agent.acp` `0.81.2` is now the qualified
   ceiling (Research 363).

## Next

- **Carry `claude-agent.acp` HTTP MCP honouring to current** (lane
  `claude-agent-acp-http-mcp-live`) — `client_mcp_servers` is Yes on exact
  `0.79.0` only (Research 361). The route now qualifies through `0.81.2`
  (Research 363): the ACP `mcpServers` mapping and `session/close` path are
  byte-identical, but the bundled Agent SDK, which owns the MCP client, moved
  `0.3.274` → `0.3.280`. Extending the claim needs one live gate on `0.81.2`
  with the proven harness; awaiting Tom.
- **Shared harness capability and producer boundary** (lane
  `shared-harness-producer-boundary`) — one provider-neutral registered
  tool/server boundary with route-exact Claude, Codex and Grok adoption on the
  merged kernel. Needs Spec 014's independent review and canonical contract
  promotion before a brief.
- **Persistent permission grants** (lane `persistent-permission-grants`) —
  producer seam for persistent permission grants on routes with a durable
  permission policy (Contract 041). Needs operator promotion.
- **Registered-tool adoption for remaining ACP routes** (lane
  `registered-tool-remaining-acp`) — `client_mcp_servers` on `cline.acp`,
  `copilot-cli.acp`, `goose.acp`, `kiro.acp` and `deepagents.acp`, each with
  its own surface evidence. Needs a consumer requirement and operator
  direction.
- **Qoder effective skill visibility** (lane `qoder-skill-visibility`) — bind
  the exact Qoder roster through Contract 058, then close the proof with
  fixtures and guide coverage. Gated on a non-empty Research 256 deliver-now
  disposition (currently empty).
- **Rulings sweep of removed records** — the lean cut removed roadmaps,
  handoffs and about a thousand logs. Most rulings already live in contracts
  and research, but some may exist only in removed logs or cards. Sweep Git
  history at `49b0d308` and land any missing ruling in its owning knowledge
  file.
- **Pre-merge validation command** — add `validation` to `.paseo/queue.json`
  once Queue runs plain pre-merge validation; the command is in `AGENTS.md`
  (Q-002).

## Not now

- **`antigravity.headless` backfill of `1.1.18..=1.2.10`** — current official
  is qualified with the retry pin (Research 359); the interior points would
  each need their own pin evidence, and consumers track current. Revisit only
  if a consumer is pinned to one of them.
- **HTTP MCP live gates on `copilot-cli.acp`, `goose.acp`, `kiro.acp`** —
  wired to emit the entry; held until a consumer needs them.
- **Further HTTP MCP live attempts on `gemini-cli.acp`** — exact `0.61.0`
  is the honouring point (Research 362). Other window points need their
  own gate.
- **Hosted interactive OAuth, OpenHands Agent Server production wiring,
  Aider headless, Kiro headless** — parked by Tom (2026-08-21): no current
  consumer need. Revisit only on explicit operator selection; don't refresh
  them periodically or infer priority from upstream releases.
- **New route candidates** — assessed leads in
  [triage](triage/2026-08-21-new-route-candidates.md); no consumer requirement
  yet.
