# llama.cpp b9910 OpenAI Chat Fixture

Synthetic protocol evidence for one attached, single-model llama.cpp
deployment. Default QA reads these files only. It does not start a server,
download a model, use a credential, or make a network request.

## Frozen Identities

- server: `llama-server` release `b9910`, commit
  `f5525f7e7a7e7cbecd386144299493ea40499bd3`
- artifact: operator-supplied `stories260K-f32.gguf` from
  `ggml-org/test-model-stories260K` revision
  `479896ec924af6d40fd419ab8f4d1eb2101de00d`
- artifact SHA-256:
  `270cba1bd5109f42d03350f60406024560464db173c0e387d91f0426d3bd256d`
- deployment: external, attached, single model, loopback HTTP
- facade: bounded llama.cpp OpenAI-compatible Chat Completions subset
- model route alias: `swallowtail-fixture-stories260k`

The artifact is 1,185,376 bytes. Swallowtail does not redistribute it and
makes no license or provenance decision for the operator.

## Optional Live Fixture

The operator supplies the exact artifact outside this repository and starts
the exact installed server. The port and full endpoint remain host-approved
inputs; no default port is part of the fixture.

```sh
llama-server \
  --model /operator/owned/stories260K-f32.gguf \
  --alias swallowtail-fixture-stories260k \
  --host 127.0.0.1 \
  --port HOST_APPROVED_PORT \
  --ctx-size 512 \
  --batch-size 32 \
  --parallel 1 \
  --seed 42 \
  --jinja \
  --chat-template chatml \
  --reasoning-format none \
  --no-webui
```

The operator owns server start, stop, logs, model placement, and endpoint
exposure. An attached Swallowtail driver may call only `/health`, `/props`,
`/v1/models`, and `/v1/chat/completions`. Closing the driver closes owned
connections and joins owned work; it never stops the server or changes the
artifact.

## Capability Boundary

This fixture proves text chat, ordered SSE output, final token usage, and the
observed ChatML system-role/string-content template shape. Tools, parsed
reasoning, structured output, and multimodal input remain unsupported in this
route even if another llama.cpp deployment can expose them.

## Sources

- <https://github.com/ggml-org/llama.cpp/releases/tag/b9910>
- <https://github.com/ggml-org/llama.cpp/blob/b9910/tools/server/README.md>
- <https://github.com/ggml-org/llama.cpp/blob/b9910/tools/server/tests/utils.py>
- <https://github.com/ggml-org/llama.cpp/blob/b9910/tools/server/tests/unit/test_chat_completion.py>
- <https://huggingface.co/ggml-org/test-model-stories260K/tree/479896ec924af6d40fd419ab8f4d1eb2101de00d>
