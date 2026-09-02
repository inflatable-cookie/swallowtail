# Research 277 — Claude Subscription Dual-Route Direction

Status: promoted operator direction
Owner: Tom
Date: 2026-09-02
Authority: operator conversation; official Anthropic documentation; Swallowtail
`main` at `8fce8b63`

## Decision

Swallowtail will pursue two distinct Claude Agent routes in parallel:

1. `claude-agent.sdk` is the primary featureful Claude route. It will use the
   official TypeScript Claude Agent SDK through a bounded Node sidecar and aim
   at the session, tool, permission, control, MCP, and continuity surface used
   by applications such as Paseo and T3 Code.
2. `claude-agent.acp` remains the portable ACP route. It will expose as much of
   the qualified ACP bridge as Swallowtail can prove without pretending that
   ACP and the native SDK are one protocol or one behavior revision.

Neither route replaces or aliases the other. Claude Code headless and
response-only routes also remain separate.

## Subscription Authority

Anthropic's 2026-06-16 Help Center update says the proposed Agent SDK billing
change is paused and, for now, Agent SDK, `claude -p`, and third-party app
usage still draw from the user's Claude subscription limits. The preserved
policy explicitly names third-party apps authenticating through the Agent SDK.
Anthropic says it will announce an update before a change takes effect.

This is sufficient current first-party authority to design subscription-backed
third-party Agent SDK support. It is not a permanent entitlement. The route
must freeze the policy URL, retrieval time, and exact applicable statement as
currentness evidence.

Canonical sources:

- <https://support.claude.com/en/articles/15036540-use-the-claude-agent-sdk-with-your-claude-plan>
- <https://code.claude.com/docs/en/agent-sdk/overview>
- <https://code.claude.com/docs/en/authentication>
- <https://code.claude.com/docs/en/agent-sdk/permissions>

## Credential Boundary

The supported product shape is per-user subscription authentication owned by
the official Claude credential layer. Swallowtail and its consumer do not
extract, copy, transmit, persist, pool, or proxy Claude OAuth tokens. They may
observe typed readiness and launch or direct the official authentication flow.
Normal provider usage limits and failures remain visible and binding.

API, Bedrock, Vertex, and Foundry credentials may be supported later as
separate access profiles. They do not replace the subscription-backed target.

## Product Target

The native SDK route should eventually cover persistent read-write sessions,
streaming, `canUseTool`, dynamic permission mode, Bash under Swallowtail's
process-authority rules, interrupt, model/effort/thinking controls, resume and
fork, MCP, checkpoint/rewind, hooks, subagents, plugins, commands, account and
usage state. Provider-specific functions stay typed and route-local unless a
second provider proves a portable abstraction.

The ACP route should first investigate read-write interactive sessions,
session-scoped permissions and mode changes, terminal mediation, mid-session
model/effort changes, client MCP servers, auth readiness, packaging, session
management, attachments, slash commands, and subagent transcript projection.

## Release Effect

The operator broke the `v0.4.0` feature freeze for this work. g05.021 and card
050 are paused, not completed or discarded. The partial card-050 semantic API
generation is retained as non-accepted audit evidence. Release preparation
cannot resume until the two Claude lanes reach a reviewed boundary and the
release audit is rerun against the new exact source head.

## Parallel Boundary

The first wave is two independent evidence and contract gates. The SDK lane
owns native SDK artifact, authentication, sidecar, lifecycle, and route-shape
evidence. The ACP lane owns the current qualified bridge capability census and
safe tranche selection. Shared provider-neutral vocabulary and implementation
cards are a later orchestrator integration step after both exact-head reviews.

