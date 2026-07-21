# Anthropic `2023-06-01` Fixture

Captured: 2026-07-20
Authority: provider-supported public Claude API
Audience: `api.anthropic.com`

Official sources:

- https://platform.claude.com/docs/en/api/overview
- https://platform.claude.com/docs/en/manage-claude/authentication
- https://platform.claude.com/docs/en/api/versioning
- https://platform.claude.com/docs/en/api/models/list
- https://platform.claude.com/docs/en/api/messages/create
- https://platform.claude.com/docs/en/build-with-claude/streaming
- https://platform.claude.com/docs/en/api/errors
- https://platform.claude.com/docs/en/api/rate-limits

Payload values are synthetic. Shapes, field meanings, header names, event
order, and error types come from the listed provider docs. No authenticated
request was made.

The subset uses a Console API key in `x-api-key`. Workload Identity Federation,
Claude subscription OAuth, cloud-platform facades, SDK retry behavior, beta
headers, tools, recovery requests, and non-streaming Messages are excluded.

Unknown top-level SSE event types are ignored as provider-directed forward
compatibility. Unknown content-block or delta semantics fail closed because
ignoring them could corrupt output. Cancellation only closes local connection
work; the API documents no cancel route for this request.
