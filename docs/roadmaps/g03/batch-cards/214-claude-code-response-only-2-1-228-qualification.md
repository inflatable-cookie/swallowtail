# 214 Claude Code Response-Only 2.1.228 Qualification

Status: superseded by card 216
Owner: Tom
Created: 2026-08-12
Milestone: `../068-claude-code-response-only-protocol-compatibility.md`

The exact-patch policy proved operationally wrong before Figmatic adoption.
Card 216 replaces it with the operator-approved protocol-compatible boundary.

## Goal

Replace exact `2.1.227` with exact `2.1.228` as the response-only route's sole
qualified Claude Code release without changing the prepared integration.

## Acceptance

- [x] exact discovery accepts `2.1.228` and rejects `2.1.227`, `2.1.229`,
      malformed output, and every unqualified release
- [x] exact corpus preserves empty tools and MCP plus bounded private-thinking
      validation and discard
- [x] ordinary output remains one untrusted text response
- [x] the live prepared facade passes normal, medium-progress, and cancellation
      paths through local Max/OAuth with no `ANTHROPIC_API_KEY`
- [x] focused, affected-package, guide, route, and docs checks pass

## Evidence

- pre-change live probe failed closed during discovery at `VersionParse`
- exact installed payload: `/Users/tom/.local/share/claude/versions/2.1.228`
- focused validation: 80 tests passed
- affected-package archive, dependency closure, and extracted compilation passed
- guide coverage passed for 36 routes, 35 examples, and 44 portable features
- route, lifecycle, feature, and 70-operation activity matrices passed
- the full docs selector passed links, all indexes, next action, agent defaults,
  consumer front door, guides, and literal version expectations
- live validation: one gated test passed in 18.59 seconds
- implementation commit: `6a3fe2aaeb0ccae8fc53598d90509b0280412182`

## Stop Conditions

- stop on non-empty tools or MCP, visible private thought, a second response,
  API-key dependence, prepared-API drift, command drift, or any need for a
  version range
