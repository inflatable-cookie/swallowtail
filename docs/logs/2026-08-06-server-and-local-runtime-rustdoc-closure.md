# 2026-08-06 Server And Local Runtime Rustdoc Closure

## Result

`swallowtail-adapter-gemini`, `swallowtail-adapter-opencode`, and
`swallowtail-adapter-ollama` now enforce crate-root denied missing docs.

The review keeps unlike routes and authorities visible. Gemini's installed
CLI ACP and headless routes remain separate from hosted Models and Live
media. OpenCode keeps server observation, model and session catalogues,
interactive and structured execution, import, reconciliation, replay load,
resume, and inactive-session deletion distinct. Ollama keeps attached local
runtime and model-artifact observation separate from inventory, structured
inference, resource-free sessions, and context-losing fresh replacement.

No facade gains routing, credential discovery, retry, fallback, durable
continuation, or stronger management authority. The batch removes 407
warnings, reducing the workspace from 662 to 255 without suppression.
Twenty-four of 27 packages now enforce denied missing docs. The remaining
warnings belong to llama.cpp, Oh My Pi, and Pi.

## Validation

- focused validation passed 200 tests across the three adapter packages
- warnings-denied clippy passed for all three packages
- extracted package proof passed for all three archives
- crate-root denied-missing-doc Rustdoc passed for all three packages
- the 27-package semantic API baseline remained unchanged
- workspace all-feature Rustdoc completed with 255 remaining warnings

No authenticated provider, consumer, tag, push, GitHub Release, or registry
effect ran.

## Next

Continue card 126 with llama.cpp, Oh My Pi, and Pi, then run the card-level
workspace documentation and QA gate.
