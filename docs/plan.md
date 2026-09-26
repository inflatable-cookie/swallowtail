# Plan

Updated: 2026-09-26

What matters next for Swallowtail, and why. Status lives in Queue; this file
is intent. Items that own a feature-matrix producer gap carry a lane key the
matrix cites as `plan:<key>`; keep the key stable while the gap is open.
Earlier task text for carried items (the g06 task files) is in Git history
at `49b0d308`, before the lean cut.

## Now

1. **Consumer HTTP MCP live honouring on `claude-agent.acp`** (lane
   `claude-agent-acp-http-mcp-live`) — emission stands (Research 351). The
   one authorized live attempt on pinned exact `0.79.0` was not accepted:
   typed stop `cleanup_failed` (Research 352): the MCP connect, list, call
   and `Completed` turn happened, but session close was not `Clean`, and the
   gate record dropped the cleanup diagnostic. The cell stays No. Next: a
   provider-free diagnosis that makes the gate keep typed cleanup diagnostics
   and traces what close does with an open HTTP MCP client on `0.79.0`. A new
   live attempt needs fresh operator authority.
2. **Version currentness** (lane `version-currentness`) — standing, never
   finished. Run the Contract 029 checkpoint when official stables move or a
   consumer hits an unverified-newer point; one family at a time. Known open
   stops, each getting an adaptation task (Tom, 2026-09-26):
   `antigravity.headless` at `1.1.17` behind the `1.1.22`
   provider-managed-retry stop (Research 283, 323, 353; catalogue now at
   `1.2.11`): evidence on the `AGY_CLI_MODEL_API_MAX_RETRIES` pin first,
   accepting provider retry as the fallback (Q-003), and
   `kimi-code.local-server` failing closed above `0.39.1` since `0.40.0`
   removed the Bash workspace restriction (Research 282, 326, 354; official
   now `2.1.1`): qualify `0.40.0..=2.1.1` under `AmbientHost` without the
   check, documented, with `disabled_tools` as the consumer opt-out (Q-004). An adaptation
   that needs a Contract 023 exception or narrows a consumer-visible
   guarantee comes back to Tom as a ruling.

## Next

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
  `copilot-cli.acp`, `gemini-cli.acp` (and `gemini-cli.headless`),
  `goose.acp`, `kiro.acp` and `deepagents.acp`, each with its own surface
  evidence. Needs a consumer requirement and operator direction.
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

- **Live HTTP MCP honouring on `gemini-cli.acp`** — the gate is built and
  proven against fakes, but Tom has no Gemini access with usage
  (2026-09-26). The route takes a Gemini Developer API key. Free personal
  Google login ended on 2026-06-18 (unpaid and Google One users moved to
  Antigravity CLI); personal login now needs Google AI Pro or Ultra, so a
  personal-login access profile wouldn't help. Revisit when paid access
  exists or a consumer needs the cell.

- **HTTP MCP live gates on `copilot-cli.acp`, `goose.acp`, `kiro.acp`** —
  wired to emit the entry; held until a consumer needs them.
- **Hosted interactive OAuth, OpenHands Agent Server production wiring,
  Aider headless, Kiro headless** — parked by Tom (2026-08-21): no current
  consumer need. Revisit only on explicit operator selection; don't refresh
  them periodically or infer priority from upstream releases.
- **New route candidates** — assessed leads in
  [triage](triage/2026-08-21-new-route-candidates.md); no consumer requirement
  yet.
