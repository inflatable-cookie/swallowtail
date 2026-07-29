# 2026-07-29 ACP Activity Currentness And Corpus

## Changed

- Revalidated stable ACP activity, schema, SDK, and remote transport
  authority.
- Added shared stable-schema activity, malformed, stdio, and remote fixtures.
- Added exact activity corpora for Claude Agent, Gemini CLI, and Kimi Code.
- Clarified thought classification, raw-field exclusion, tool update, plan
  replacement, and session-metadata boundaries in Contract 044.
- Closed card 125 and made card 126 ready.

## Current State

- ACP v1 stable schema `v1.20.0` is the shared corpus point.
- Claude Agent `0.53.0..=0.61.0` excluding `0.58.0`, Gemini CLI `0.51.0`,
  and Kimi Code `0.28.1` plus `0.29.0..=0.29.2` retain their exact
  guarantees.
- Current newer Claude Agent, Gemini CLI, and Kimi Code releases remain
  permitted unverified newer. They do not widen those guarantees.
- ACP thoughts remain client-facing display content. Exact adapters decide
  whether they are reasoning summaries, warnings, other display activity, or
  exclusions.
- Physical stdio and remote transport identities remain separate from shared
  semantic updates.
- No production behavior, provider access, authentication, or live transport
  changed.

## Evidence

- 4 shared ACP activity corpus tests
- 2 Claude Agent activity corpus tests
- 2 Gemini CLI activity corpus tests
- 2 Kimi Code activity corpus tests
- 74 complete ACP protocol tests
- 8 remote ACP transport tests
- repository Rust, documentation, formatting, and package gates

## Next

Card 126 adds bounded shared ACP update records without importing provider or
runtime policy into the protocol crate.
