# 2026-07-25 Claude And Gemini ACP Prepared Facades

Status: complete

## Changed

`swallowtail-adapter-claude-agent` and `swallowtail-adapter-gemini` now expose
adapter-local prepared normal paths for stdio ACP:

- exact host-approved executable discovery
- retained compatibility and access evidence
- configured instance and immutable preflight derivation
- plan-derived new-session requests
- typed prepared session execution
- low-level driver escape hatches

Claude Agent keeps the consumer-selected model route explicit. Gemini CLI's
current ACP model remains provider observation and does not become a fabricated
route.

Gemini also gains exact semantic version discovery. `0.51.0` is qualified.
Later stable releases remain executable as visible unverified-newer evidence;
older releases reject.

## Authority

Both routes use explicit ambient configuration, `AmbientHost`, read-only
workspace access, and prohibited provider-owned durable state. These are not
containment claims.

Both prepared facades select `acp-v1-stdio`. They do not accept or infer a
remote endpoint, switch transports, or fall back. Remote ACP composition stays
separate for card 026.

## Validation

- Claude Agent all-target suite: 16 deterministic tests pass
- Gemini all-target suite: 32 deterministic tests pass; one live installed
  probe remains ignored without its explicit gate
- prepared facade tests pass on local and remote-authoritative host identities
- unsupported session options reject before session process effects
- examples compile and focused warnings-denied lint passes
- full Effigy QA passes across the workspace
- Doctor remains at the known 19 oversized-file findings: 7 errors and 12
  warnings

## Next

Card 025 adds separate Pi RPC and Qwen headless prepared facades.
