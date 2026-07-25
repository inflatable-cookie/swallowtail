# Kimi Code 0.29.0 Negotiated Reasoning Fixtures

Deterministic ACP v1 evidence for the exact `0.29.0` reasoning milestone.

Pins:

- annotated `@moonshot-ai/kimi-code@0.29.0` tag and peeled source commit
- `@moonshot-ai/acp-adapter` `0.3.5`
- locked `@agentclientprotocol/sdk` `0.23.0`
- ACP wire version `1`

This corpus freezes the private `thinking` option mapping: declared effort
levels, legacy boolean fallback, always-thinking models, and exact effective
confirmation. Missing, duplicate, malformed, unsupported, rejected, or drifted
options fail without another value, model, route, or provider.

The corpus does not claim a continuous release interval from `0.28.1`, widen
ACP into a generic configuration API, mutate loaded or resumed sessions, or
require a sandbox.
