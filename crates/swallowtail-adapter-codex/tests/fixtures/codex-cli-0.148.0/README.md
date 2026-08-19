# Codex CLI 0.148.0 currentness corpus

Secret-free identity for host Codex `0.147.0` and official npm
`@openai/codex` `0.148.0` before Swallowtail raises the `codex.cli` ceiling.

The mapped exec JSONL flags and app-server methods remain. `exec fork`,
top-level `fork`, and `thread/fork` stay unmapped. JSONL processor and
`exec_events.rs` are byte-identical with `0.147.0`. The host install was
not replaced.

No fixture contains a credential, host path, account identity, provider
payload, or real session id.
