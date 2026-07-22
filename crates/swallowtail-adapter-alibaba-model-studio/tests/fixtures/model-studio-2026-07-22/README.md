# Alibaba Model Studio fixture corpus

Observed: 2026-07-22. Offline only.

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
