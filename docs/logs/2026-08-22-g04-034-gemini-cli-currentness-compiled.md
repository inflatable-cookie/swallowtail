# 2026-08-22 g04.034 Gemini CLI Currentness Compiled

## Change

- re-probed installed Gemini CLI `0.53.0` and official npm/GitHub `0.56.0`
- compiled g04.034 and cards 093-094 for one-family identity-before-claim work
- kept ACP and headless on separate axes inside the same family run
- fixed the g04 batch-card front door to record completed Pi cards 089-092

## Decision

The selected access posture is an enterprise-owned API key through Gemini
CLI's existing provider-supported API-key profile. Code Assist browser login,
individual-account service, hosted OAuth, Gemini Live, and Gemini Models are
not part of the run.

The worker must qualify both selected CLI axes deterministically. If either
axis needs live authentication, a provider prompt, a new public operation, or
a materially changed lifecycle, both claims stay unchanged and the family
returns for an explicit keep-or-remove decision.

## Evidence

- host: `gemini 0.53.0`, executable SHA-256
  `4a8f99947eae4e1ff501269ba8b9ca2d1216db044fb75e01f4ee86fd1d8f175e`
- npm `latest`: `@google/gemini-cli@0.56.0`
- GitHub release `v0.56.0`, published 2026-08-19
- official darwin-arm64 unsigned asset SHA-256
  `be0c20ccf8b6be6ce01654736847168a9328e92db4db4c0d0b776de70703fb8f`

## Validation

- `effigy qa:northstar`
- `effigy qa:docs:index:logs`
- `effigy qa:docs:index:roadmaps`
- `effigy qa:docs:index:roadmaps:g04`
- `effigy qa:docs:index:roadmaps:batch-cards`
- `effigy qa:docs:next-action:roadmaps`
- `git diff --check`
