# g04.027 Ollama 0.32.15 Useful Newer

Status: completed
Owner: Tom
Created: 2026-08-21

## Purpose

Qualify official GitHub `ollama/ollama` `v0.32.15` (published 2026-08-19)
on the `ollama.runtime` axis (`ollama.attached` route).

This is Contract 029 currentness work. It does not keep the generation
open.

## Acceptance

Identity:

- Freeze official `0.32.15` evidence
- Name segment shape (compatible-extension / milestone / stop)
- No production claim edit in identity card

Claim:

- If compatible-extension: raise `OLLAMA_LATEST_QUALIFIED_VERSION` to
  `0.32.15`
- Keep exclusions `0.32.2` and `0.32.10`
- Update tests, guides, feature matrix, architecture if it names the
  ceiling
- Pass `effigy validate:focused swallowtail-adapter-ollama`
- Pass `effigy package:verify-affected swallowtail-adapter-ollama`
- Pass `effigy qa:routes`
- Pass `effigy qa:northstar`
- Pass docs index gates
- Pass next-action gate

Do not run workspace `qa`, broad `qa:docs`, live probes, MSRV, or
consumer checks.

## Out Of Scope

- Gemini requalification (deferred)
- Codex (PR 19 in flight)
- Qwen (PR 21 in flight)
- Provider prompt, live catalogue, live session, install, or host update
- Mapping unused surfaces (desktop onboarding, metadata cache, Qwen 3.8
  system-message normalize, MLX/llama.cpp deps)
- Flattening families, including llama.cpp attached/owned
- Decoder updates unless adapter mapping changed

## Batch Cards

- [074-ollama-0-32-15-identity.md](batch-cards/074-ollama-0-32-15-identity.md) — completed
- [075-ollama-0-32-15-claim.md](batch-cards/075-ollama-0-32-15-claim.md) — completed

## References

- [Research 174 Ollama 0.32.15 Identity](../../research/174-ollama-0-32-15-identity.md)
- [Contract 029 Interface Version Qualification And Compatibility](../../contracts/029-interface-version-qualification-and-compatibility.md)
- [Version Currentness Checkpoint](../../guides/version-currentness-checkpoint.md)
- [Standing Lanes](../standing-lanes.md)
