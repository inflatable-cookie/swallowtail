# DeepSeek Harness JSON-RPC fixture corpus

This corpus freezes the redacted machine boundary qualified by Research 124:
runtime-bin `0.1.0rc6`, route `deepseek-harness.jsonrpc`, and protocol facade
`deepseek-harness.sdk-jsonrpc-v1`.

The JSONL files contain lifecycle shape only. Prompt, reasoning, tool
argument, tool result, session, cwd, and credential-bearing values are either
empty or stable redaction markers. They are not private probe transcripts.

`session.event` notification counts are intentionally not durable JSONL
record counts. The live probe observed 4,626 SDK events while the persisted
run contained 668 records. The adapter must bound the live stream and project
only the stable lifecycle fields.

`check-deepseek-harness-corpus.py` validates every fixture and mutates the
success corpus in memory for malformed, oversized, post-terminal, and
mismatched-model rejection tests. It is package-independent so the corpus can
be validated before the Rust package exists.
