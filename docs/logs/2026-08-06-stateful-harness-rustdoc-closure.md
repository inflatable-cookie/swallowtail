# 2026-08-06 Stateful Harness Rustdoc Closure

## Result

`swallowtail-adapter-codex`, `swallowtail-adapter-claude-agent`, and
`swallowtail-adapter-kimi` now enforce crate-root denied missing docs.

The review preserves their state and authority boundaries. Codex exec remains
separate from app-server sessions, model and thread catalogues, imports,
reconciliation, settled attachment, resume/load, and inactive-thread
management. Claude Agent ACP keeps API-key and provider-owned local auth,
consumer-mediated permissions, continuation, and lifecycle behavior separate
from native one-shot `claude -p`. Kimi keeps ACP, headless, attached
local-server, owned local-server, cross-transport import, managed recovery,
reconciliation, and lifecycle operations explicit.

No facade gains routing, credential discovery, retry, fallback, or stronger
continuation or management authority. The batch removes 493 warnings,
reducing the workspace from 1,155 to 662 without suppression. Twenty-one of
27 packages now enforce denied missing docs. The remaining warnings belong to
Gemini, OpenCode, Ollama, llama.cpp, Oh My Pi, and Pi.

## Validation

- focused validation passed 338 tests across the three adapter packages
- warnings-denied clippy passed for all three packages
- extracted package proof passed for all three archives
- crate-root denied-missing-doc Rustdoc passed for all three packages
- the 27-package semantic API baseline remained unchanged
- workspace all-feature Rustdoc completed with 662 remaining warnings

No authenticated provider, consumer, tag, push, GitHub Release, or registry
effect ran.

## Next

Continue card 126 with Gemini, OpenCode, and Ollama, then close the local RPC
and serving adapters.
