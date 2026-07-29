# Pi RPC 0.80.10 Corpus

Frozen deterministic evidence for `@earendil-works/pi-coding-agent@0.80.10`.
Accessed 2026-07-22; usage and activity evidence refreshed 2026-07-29:

- https://www.npmjs.com/package/@earendil-works/pi-coding-agent
- https://github.com/badlogic/pi-mono/blob/main/packages/coding-agent/docs/rpc.md
- https://github.com/badlogic/pi-mono/blob/main/packages/coding-agent/README.md
- https://github.com/badlogic/pi-mono/blob/main/packages/coding-agent/docs/settings.md
- https://github.com/earendil-works/pi/blob/v0.80.10/packages/coding-agent/docs/rpc.md

The corpus binds one exact Contract 029 qualification point. It is not a
semver-range claim. Records use strict LF delimiters. No fixture installs Pi,
loads ambient configuration, authenticates, or contacts a model provider.

`protocol.json` fixes the first invocation shape. Provider, model, executable,
working directory, and delegated authentication remain opaque runtime inputs;
the fixture placeholders are not defaults.

`usage-events.jsonl` freezes two disjoint assistant-message usage records and
terminal settlement. Cost is retained only to prove that token usage and cost
are separate fields.

`activity.jsonl` freezes exact agent, turn, message, thinking, tool,
compaction, retry, and settled lifecycle. Tool execution is harness-owned;
extension UI remains a callback. Current `0.82.1` adds bash updates and
summarization-retry events, so those are absent from this exact profile.

`input-callback-corpus.json` freezes exact `prompt.images` transport for one
bounded `image/png` attachment. Its bytes are synthetic. It also freezes
unsupported media, count, size, request-plan, cancellation, and cleanup
outcomes. Pi extensions and skills remain ambient process configuration, not
portable consumer tools or provider-owned search.
