# DeepSeek Harness Web `/api` fixture corpus

This corpus freezes the redacted machine boundary qualified by Research 125:
`@deepseek-ai/dsh@0.1.0-rc.6`, launcher command `dsh web`, route
`deepseek-harness.local-server`, and compatibility axis
`deepseek-harness.web`.

The fixtures cover the allowlisted POST `/api/<method>` calls, the two
server-only event downlinks, paged history inspection, loopback and browser
trust fences, carrier failures, and denied methods. Prompt, reasoning, tool,
credential, account, private-path, and export content is absent or represented
only by a stable redaction marker. The identities are deterministic fixture
tokens so request/response correlation remains testable without preserving
upstream random identifiers.

`session.history` is an inspection-only proof. Each page records zero agent
starts, prompt dispatches, provider-work calls, and interactive-handle
creation. The pages are bounded and move backwards with `beforeSeq`.

`artifact.json` records the npm/CLI pin. It deliberately does not use the
launcher version output, `host.describe`, or a JSON-RPC payload digest as the
compatibility axis. The upstream revision is source-shape evidence only; no
browser, account, credential, or live model is required to validate this
corpus.
