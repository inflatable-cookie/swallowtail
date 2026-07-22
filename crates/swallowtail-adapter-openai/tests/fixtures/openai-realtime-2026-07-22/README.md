# OpenAI Realtime GA Fixture

Captured from the provider-supported Realtime WebSocket and event reference on
2026-07-22. The corpus freezes one server-to-server, API-key-backed, manual
audio-turn subset for `gpt-realtime-2.1`.

The native cancel frame omits the optional `response_id`, matching the GA
default-conversation form so a cancellation racing `response.created` remains
sendable.

It contains JSON events only. No credential, endpoint client, WebSocket,
device, paid request, raw provider capture, or live account state is present.
Fixture identifiers and PCM bytes are synthetic.
