# 088 Claude Agent Standalone Range And Gemini Disposition

Status: promoted
Owner: Tom
Date: 2026-07-31

## Question

Should Claude Agent `0.62.0..=0.64.0` resume independently, and should Gemini's
unscheduled range extension remain described as paused?

## Current Evidence

`@agentclientprotocol/claude-agent-acp` latest remains `0.64.0` at signed tag
commit `e56f344691a56c07e5dae2ebeb6ad2a6416f8c9d`. The local and managed wrapper
remain exact `0.63.0`. Claude Code is separately installed at `2.1.220`.

Npm package metadata and exact tarballs confirm:

| Wrapper | Commit | ACP SDK | Agent SDK | Selected delta |
| --- | --- | --- | --- | --- |
| `0.62.0` | `53a0c36ce3b0b76929d11d8b9565e319da745608` | `1.3.0` | `0.3.219` | selected output byte-identical to `0.61.0` |
| `0.63.0` | `15979bba7907484ee22111cdc33b79b0bdcd452d` | `1.3.0` | `0.3.220` | tool-progress and denial correlation, optional nested transcript metadata, fast-mode state detail |
| `0.64.0` | `e56f344691a56c07e5dae2ebeb6ad2a6416f8c9d` | `1.3.0` | `0.3.220` | opt-in host-owned steering fallback and custom-answer form marker |

The current Swallowtail form mapper already accepts the exact `0.64.0`
custom-answer marker. Swallowtail does not advertise nested transcript support
and does not opt into host-owned steering fallback, so those upstream additions
grant no new authority.

## Access Currentness

Anthropic paused its announced Agent SDK billing split. For now, Agent SDK,
`claude -p`, and third-party Agent SDK apps continue to draw from Claude plan
usage limits. The existing explicit local-subscription and public-API-key
profiles therefore remain honest and separate.

This access rule is temporally unstable. A later Anthropic change must update
credential mechanism, entitlement, metering, and support authority without
silently relabelling one route as another.

## Decision

1. Supersede coupled roadmap g03.002.
2. Compile standalone Claude roadmap g03.015 through exact `0.64.0`.
3. Move Gemini's proposed extension to deferred backlog with no implied revisit
   date. Existing Gemini support remains production truth.
4. Keep Claude Code headless separate from Claude Agent ACP.

## Contract Result

No new shared contract is required. Existing compatibility, ACP negotiation,
access, facade, lifecycle, callback, activity, and topology contracts represent
all selected behavior and exclusions.

## Sources

- [Claude Agent ACP 0.64.0](https://github.com/agentclientprotocol/claude-agent-acp/releases/tag/v0.64.0)
- [Claude Agent ACP 0.61.0 to 0.64.0 comparison](https://github.com/agentclientprotocol/claude-agent-acp/compare/v0.61.0...v0.64.0)
- [Anthropic Agent SDK subscription update](https://support.claude.com/en/articles/15036540-use-the-claude-agent-sdk-with-your-claude-plan)
