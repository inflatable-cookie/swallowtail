# OpenAI Realtime GA Fixture

Captured from the provider-supported Realtime WebSocket and event reference on
2026-07-22. The corpus freezes one server-to-server, API-key-backed, manual
audio-turn subset for `gpt-realtime-2.1`.

This directory remains the superseded historical proof for facade
`openai-realtime-2026-07-22` / private behavior `openai.realtime-manual-pcm-v1`.
Current production uses `openai-realtime-reasoning-2026-08-27` and rejects this
facade point.

The native cancel frame omits the optional `response_id`, matching the GA
default-conversation form so a cancellation racing `response.created` remains
sendable.

It contains JSON events only. No credential, endpoint client, WebSocket,
device, paid request, raw provider capture, or live account state is present.
Fixture identifiers and PCM bytes are synthetic.

`generation-controls-session-update.json` freezes the session-scoped positive
output maximum with no `reasoning` member. That omission shape remains the
current no-selection byte baseline. Response-schema controls remain unsupported
on this selected route.
