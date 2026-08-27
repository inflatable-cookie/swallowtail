# OpenAI Realtime Reasoning-Effort Evidence

Captured from official OpenAI Realtime documentation and API reference on
2026-08-27. The corpus freezes session-scoped `reasoning.effort` dispatch for
exact model `gpt-realtime-2.1` on the GA Realtime WebSocket surface.

It contains JSON specimens only. No credential, endpoint client, WebSocket,
device, paid request, raw provider capture, or live account state is present.
Fixture identifiers and event bodies are synthetic.

`reasoning-effort-session-update.json` freezes one positive session-scoped
effort selection beside the dated route's manual PCM session shape. Production
encoding uses event id `session-config-1` with the same session body members.

`reasoning-effort-session-updated.json` freezes the matching provider
acknowledgement shape documented for `session.updated`.

`reasoning-effort-response-create-override.json` freezes the documented
per-response override surface. It remains withheld lead evidence; the current
encoder still emits bare `response.create`.

These files are contract evidence for Research 236 and g04.084 cards 236-237.
Facade `openai-realtime-reasoning-2026-08-27` binds the five exact session
values with matching acknowledgement. Effective reasoning depth and
reasoning-token usage remain unclaimed.

Authority:

- [GPT-Realtime-2.1 model page](https://developers.openai.com/api/docs/models/gpt-realtime-2.1)
- [Realtime and audio guide](https://developers.openai.com/api/docs/guides/realtime)
- [Realtime prompting guide](https://developers.openai.com/api/docs/guides/realtime-models-prompting)
- [Realtime client events reference](https://developers.openai.com/api/reference/resources/realtime/client-events)
- [Realtime server events reference](https://developers.openai.com/api/reference/resources/realtime/server-events)
