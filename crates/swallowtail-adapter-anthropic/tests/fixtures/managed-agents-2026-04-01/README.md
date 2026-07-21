# Claude Managed Agents Fixture Corpus

Frozen: 2026-07-21

Authority:

- https://platform.claude.com/docs/en/managed-agents/environments
- https://platform.claude.com/docs/en/managed-agents/sessions
- https://platform.claude.com/docs/en/managed-agents/events-and-streaming
- https://platform.claude.com/docs/en/api/beta/sessions/events
- https://platform.claude.com/docs/en/api/beta/sessions/create
- https://github.com/anthropics/skills/blob/main/skills/claude-api/shared/managed-agents-tools.md

The corpus is synthetic. IDs, timestamps, model names, content, request IDs,
and errors are inert fixture values. It records the documented
`managed-agents-2026-04-01` REST/SSE shapes; it was not captured from an
account and cannot authorize live access.

The subset uses a first-party API key, one exact agent version, a session
override that fixes the model and custom tools, and one cloud environment with
limited networking. `allowed_hosts` is explicitly empty. MCP servers, package
managers, built-in tools, skills, files, vaults, memory, multiagent, webhooks,
schedules, preview deltas, and external network access are excluded.

Persisted events are authoritative. `event_start` and `event_delta` are
negative drift fixtures only. The current API reference's `user.interrupt`
shape governs; an older curl example that used `interrupt` is excluded.

