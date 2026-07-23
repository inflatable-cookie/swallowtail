# Ollama Native Text V1 Fixture

Offline corpus for the attach-only native API subset qualified from `v0.14.0`
through `v0.32.1`.

Tagged source evidence:

- [`v0.14.0`](https://github.com/ollama/ollama/tree/v0.14.0) —
  `02a24015968d612b418448b73cffaa1b0652d161`
- [`v0.18.0`](https://github.com/ollama/ollama/tree/v0.18.0) —
  `3980c0217d27e05a441808a446e7ee5ea7e04256`
- [`v0.30.0`](https://github.com/ollama/ollama/tree/v0.30.0) —
  `2c71d8d7ca6edbc9bdc1a312f71ce3b079c0fe56`
- [`v0.32.1`](https://github.com/ollama/ollama/tree/v0.32.1) —
  `30c390384e20333b67cadab60da5bcb669407f01`

For every tag, `api/types.go` retains the selected `ChatRequest`,
`ChatResponse`, `ListResponse`, `ProcessResponse`, and `ShowResponse` fields.
`server/routes.go` retains `/api/version`, `/api/tags`, `/api/ps`, `/api/show`,
and `/api/chat`.

Current official endpoint documentation was checked on 2026-07-23:

- https://docs.ollama.com/api-reference/get-version
- https://docs.ollama.com/api/tags
- https://docs.ollama.com/api/ps
- https://docs.ollama.com/api-reference/show-model-details
- https://docs.ollama.com/api/chat
- https://docs.ollama.com/api/errors

The corpus is synthetic and contains no model, manifest, path, prompt from a
user, credential, endpoint, account, or live inference response.
