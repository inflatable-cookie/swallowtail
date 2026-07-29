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

`generation-controls-request.json` freezes exact reasoning effort and
provider-native JSON Schema fields beside the existing positive output bound.
It is contract evidence for card 085, not a realized adapter claim.

`response-delete.json` freezes one terminal
`DELETE /v1/responses/{response_id}` attempt. Confirmed cleanup requires the
exact id and `deleted=true`. Missing identity, active or unconfirmed remote
state, 404, transport loss, mismatched id, or malformed acknowledgement remains
unconfirmed. Deletion joins before credential release and never substitutes
for native cancellation.
