# Qwen Code 0.22.3 currentness corpus

This secret-free identity corpus freezes official npm
`@qwen-code/qwen-code` `0.22.3` before Swallowtail widens the
`qwen-code.package` claim. Host `qwen` was not installed.

Exact npm tarball and selected git-blob identities live in
`identity.json`. Official `latest` is `0.22.3` on the same 0.22 segment
as maintained `0.22.0..=0.22.1`. Published intermediate is `0.22.2`.
Selected flags, exact `--resume`, image-only catalogue filter, and
`set_effort` remain. `cli-entry.js` and `reasoning-effort.ts` are
byte-identical from `0.21.15` through `0.22.3`. Stream types, catalogue
controller, and dashscope match `0.22.2`. Plan-mode blobs match
`0.22.2`. `config.ts` deltas are the unmapped GeminiMd-to-Memory rename
and unmapped `tools.eager`. `session.ts` deltas are comment renames
(`gemini.tsx` to `llm.tsx`, `getGeminiClient` to `getLlmClient`).
Reasoning and budgets stay exact `0.21.15`.

Unpublished stables `0.20.2` and `0.21.16` stay gaps. Dist-tag `preview`
`0.22.2-preview.1` and nightly are ignored. First unpublished later
stable is `0.22.4`. No provider prompt. No live catalogue. No live
headless session. The host install was not replaced. The frozen
`0.21.15`, `0.22.1`, and `0.22.2` corpora stay.

No fixture contains a credential, host path, account identity, provider
payload, or real session id.
