# Alibaba Model Studio fixture corpus

Observed: 2026-07-22. Offline only.

Retained-conversation retrieval and replay evidence revalidated: 2026-08-05.

Authority:

- https://help.aliyun.com/en/model-studio/regions/
- https://help.aliyun.com/en/model-studio/permission-management-overview
- https://help.aliyun.com/en/model-studio/qwen-api-via-openai-responses
- https://help.aliyun.com/en/model-studio/openai-compatible-conversations
- https://help.aliyun.com/en/model-studio/model-pricing
- https://help.aliyun.com/en/model-studio/coding-plan-faq

Frozen route: Singapore workspace-dedicated Conversations and Responses,
general Model Studio API key, pay-as-you-go, exact
`qwen3.7-plus-2026-05-26`.

Excluded: Coding Plan, Token Plan, legacy or other-region domains, aliases,
catalogue discovery, response storage, previous-response continuation,
session cache, tools, files, multimodal input, reasoning output, background,
retry, reattachment, resume, and fallback.

The 2026-08-05 retained additions freeze a separate, not-yet-production
profile: exact conversation retrieval, ascending item pagination, strict
completed user/assistant replay, hard page/item/byte bounds, preservation on
ordinary close, and separate explicit cleanup authority. The operation-owned
profile above remains delete-on-close. Replay-free resume remains excluded.

`conversation-retrieved.json`, `items-page-1.json`, and `items-page-2.json`
are synthetic protocol shapes based on the current documented Conversations
surface. `retained-recovery-cases.json` freezes Swallowtail dispositions, not
provider error prose or undocumented error codes. Missing, deleted, foreign,
stale, malformed, oversized, and uncertain cases never produce readiness or
fallback.

The request builder constructs only `model`, `input`, `conversation`,
`stream=true`, `store=false`, and `reasoning.effort=none`. It rejects model
substitution, non-streaming mode, response storage, other reasoning effort,
tools, the cache header, background mode, retry, `previous_response_id`, output
bounds, and fallback before a wire request exists. It never passes through
provider-ignored fields. Metadata mutation, direct item creation, stored-
response retrieval/deletion, files, multimodal content, search, MCP, and code
execution have no input surface.

Provider identifiers and content are synthetic. No account, workspace, key,
conversation, request, or paid inference produced this corpus.
