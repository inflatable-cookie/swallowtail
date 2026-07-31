# Grok Build 0.2.114 To 0.2.117 Compatibility Corpus

This fixture freezes the exact stable Grok Build interval selected by Research
085 before the production compatibility claim is widened.

Sources:

- official npm metadata for `@xai-official/grok` and
  `@xai-official/grok-darwin-arm64`
- exact signed darwin-arm64 platform artifacts
- bounded `--no-auto-update --version` output
- one unauthenticated ACP `initialize` exchange per exact executable
- official xAI changelogs and CLI documentation

The source records were reduced to identity, selected capability, catalogue,
access, and lifecycle facts. They contain no tokens, account metadata, host
paths, raw provider payloads, prompts, or session identities.

`candidate_behavior_revision` was frozen before card 031 widened the production
claim. The corpus remains the exact evidence behind both maintained segments.
