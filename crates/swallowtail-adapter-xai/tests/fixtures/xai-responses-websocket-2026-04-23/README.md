# xAI Responses WebSocket fixture

Evidence snapshot: 2026-07-20. The directory suffix records the current xAI
WebSocket guide update date, not an xAI API version.

Official sources checked on 2026-07-20:

- <https://docs.x.ai/developers/advanced-api-usage/websocket-mode>
- <https://docs.x.ai/developers/rest-api-reference/inference/chat>
- <https://docs.x.ai/developers/rest-api-reference/management/auth>
- <https://docs.x.ai/developers/cost-tracking>

The supported frame subset is text-only `response.create`,
`response.created`, `response.in_progress`, `response.output_text.delta`,
`response.output_text.done`, `response.completed`, and `error`. The event names
and order follow xAI's documented Responses-stream equivalence; this fixture is
not a captured provider transcript. It uses one exact model, `store=false`, no
tools, and serial first/follow-up turns.
Usage and billed cost are taken from the terminal response. The parser also
implements replacement, rather than summation, if an earlier cumulative usage
snapshot is present.

The excluded set is deliberate: HTTP model catalogue, warmup, background,
provider or client tools, search, attachments, structured output, storage,
retry, reconnect, resume, branching, and live authentication. Unknown event
types fail closed. Cancellation is a WebSocket close; the fixture invents no
text protocol cancellation message.

All ids, model names, prompts, outputs, costs, and credentials are synthetic.
Tests use a loopback endpoint and `fixture-secret`; no xAI credential or
external request is used.
