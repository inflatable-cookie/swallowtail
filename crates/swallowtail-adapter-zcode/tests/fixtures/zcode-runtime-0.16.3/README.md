# ZCode app-server fixture corpus

This corpus freezes the redacted machine boundary qualified by Research 126:
runtime `0.16.3`, route `zcode.app-server`, and protocol facade
`zcode.protocol-stdio-v1`.

The JSONL files contain lifecycle shape only. Prompt, reasoning, tool
input, session, cwd, and credential-bearing values are either empty or
stable redaction markers. They are not private probe transcripts.

Framing is line-delimited JSON without a `jsonrpc` field. `session/create`
blocks on server→client `session/requestRuntimePreferences`. The create
result is a snapshot (`messages`, `projection`, `session`, …) and is not
the event stream. Live events are `session/event` notifications after
`session/subscribe` and `session/send`.

Handshake create-plus-preferences is probe-proven. Prompt, tool, and
terminal sequences are reconstructed from the documented app-server
protocol so the driver can bind send, idle, and terminal before a live
model run exists. Live cardinality is not claimed.

Launcher `zcode-app-cli@3.7.7-13` and desktop About `3.7.7` are recorded
and are not the compatibility axis.

`check-zcode-app-server-corpus.py` validates every fixture and mutates the
success corpus in memory for malformed, oversized, post-terminal, and
mismatched-runtime rejection tests. It is package-independent so the corpus
can be validated before the Rust package exists.
