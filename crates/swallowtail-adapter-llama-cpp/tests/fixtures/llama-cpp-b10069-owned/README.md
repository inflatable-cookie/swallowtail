# llama.cpp b10069 Owned Lifecycle Fixture

Deterministic lifecycle evidence for the pinned `b10069` owned-serving driver.
The fixture freezes the tagged `listening on <address>` startup record and
malformed, duplicate, and non-loopback rejection cases. HTTP readiness reuses
the bounded health, properties, and single-model catalogue fixture with build
`b10069-178a6c449`.

No llama.cpp binary or model is installed or executed by default tests.

Source boundary:

- <https://github.com/ggml-org/llama.cpp/releases/tag/b10069>
- <https://github.com/ggml-org/llama.cpp/blob/b10069/tools/server/README.md>
- <https://github.com/ggml-org/llama.cpp/blob/b10069/tools/server/server.cpp>
- <https://github.com/ggml-org/llama.cpp/blob/b10069/tools/server/server-http.cpp>
