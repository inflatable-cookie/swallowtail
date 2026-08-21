# g04.025 Codex 0.149.0 Useful Newer

Status: completed
Owner: Tom
Created: 2026-08-21

## Purpose

Qualify official npm `@openai/codex` `0.149.0` (published 2026-08-20) on
the `codex.cli` axis (`codex.exec` + `codex.app-server` routes).

This is Contract 029 currentness work. It does not keep the generation open.

## Acceptance

Identity:
- Freeze official `0.149.0` evidence
- Name segment shape (compatible-extension / milestone / stop)
- No production claim edit in identity card

Claim:
- If compatible-extension: raise `CODEX_LATEST_QUALIFIED_VERSION` to `0.149.0`
- Update tests, guides, feature matrix, architecture if it names the ceiling
- Pass `effigy validate:focused swallowtail-adapter-codex`
- Pass `effigy package:verify-affected swallowtail-adapter-codex`
- Pass `effigy qa:routes`
- Pass `effigy qa:northstar`
- Pass docs index gates
- Pass next-action gate

Do not run workspace `qa`, broad `qa:docs`, live probes, MSRV, or consumer
checks.

## Out Of Scope

- Gemini requalification (deferred)
- Provider prompt, live catalogue, live session, install, or host update
- Mapping unused surfaces (exec fork, thread fork, top-level fork, builtin providers, tui export, async hooks)
- Flattening families
- Decoder updates unless adapter mapping changed

## Batch Cards

- [068-codex-0-149-0-identity.md](batch-cards/068-codex-0-149-0-identity.md) — completed
- [069-codex-0-149-0-claim.md](batch-cards/069-codex-0-149-0-claim.md) — completed

## References

- [Research 172 Codex 0.149.0 Identity](../../research/172-codex-0-149-0-identity.md)
- [Contract 029 Interface Version Qualification And Compatibility](../../contracts/029-interface-version-qualification-and-compatibility.md)
- [Version Currentness Checkpoint](../../guides/version-currentness-checkpoint.md)
- [Standing Lanes](../standing-lanes.md)
