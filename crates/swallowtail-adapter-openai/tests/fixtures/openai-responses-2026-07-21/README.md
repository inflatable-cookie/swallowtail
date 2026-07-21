# OpenAI Responses Background Fixtures

Captured: 2026-07-21

Authority:

- [OpenAI background mode guide](https://developers.openai.com/api/docs/guides/background)
- [OpenAI Responses reference](https://developers.openai.com/api/reference/resources/responses)
- generated OpenAI OpenAPI 3.1 document, API version 2.3.0

The corpus fixes the first Swallowtail subset: OpenAI public API, API-key
access, API billing, `background=true`, `stream=true`, `store=false`, explicit
temporary retention, one exact model, one positive output bound, and at most
one `starting_after` stream reattachment.

Response ids, request ids, content, usage, and rate values are synthetic.
Default tests use no OpenAI credential, account, external request, or paid
inference. ChatGPT, Codex, subscription OAuth, community OAuth, tools, search,
files, conversations, webhooks, Batch API, retry, and fallback are excluded.
