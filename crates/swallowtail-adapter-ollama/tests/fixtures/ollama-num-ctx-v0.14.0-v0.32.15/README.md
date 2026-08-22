# Ollama Num Ctx V0.14.0 Through V0.32.15

This secret-free corpus freezes exact tagged-source and current official
specimens for Ollama native `options.num_ctx` on the qualified
`ollama.runtime` window. It does not start a runtime, send a prompt, or claim
effective context allocation.

Every listed tag retains `ChatRequest.Options` as a JSON object map,
`Runner.NumCtx` as JSON `num_ctx`, and `/api/chat` as the native request
surface. Zero, negative, and absent values remain server-default behavior
outside Swallowtail dispatch.

No fixture contains credentials, model files, endpoint values, prompts, or live
inference responses.
