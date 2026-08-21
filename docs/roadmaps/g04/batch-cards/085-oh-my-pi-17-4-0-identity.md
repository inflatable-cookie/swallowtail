# 085 Oh My Pi 17.4.0 Identity

Status: completed
Owner: Tom
Milestone: [g04.031 Oh My Pi 17.4.0 Useful Newer](../031-oh-my-pi-17-4-0-useful-newer.md)
Created: 2026-08-21
Depends on: Research 178

## Task

Freeze official npm `@oh-my-pi/pi-coding-agent@17.4.0` identity evidence
against qualified `17.3.8`. Name segment shape. This is a published
minor-line step: check private-milestone before compatible-extension.
Do not edit production claims in this card.

## Method

1. Observe official `17.4.0` (npm `latest`, published 2026-08-20)
2. Extract the npm tarball in `/tmp`
3. Compare selected mapped flags and RPC commands against the `17.3.8`
   corpus
4. Compare `docs/rpc.md` and mapped RPC sources at `v17.3.8` and
   `v17.4.0`
5. Classify unmapped additions
6. Write identity fixture under
   `crates/swallowtail-adapter-oh-my-pi/tests/fixtures/oh-my-pi-17.4.0/`
7. Write research record 178
8. Name segment shape

No provider prompt. No live session. Host install not changed.

## Expected Shape

Compatible-extension if selected mapped subset and adapter-private
mapping are unchanged. Private-milestone only if mapping would change.
Stop if mapped protocol differs, a new public operation is required, a
live session is needed, or the shape would flatten onto Pi RPC.

## Acceptance

- [x] Identity fixture written
- [x] Research 178 promoted
- [x] Shape named: compatible-extension
- [x] Private-milestone checked: mapping unchanged
- [x] No production claim edit
- [x] Official latest still `17.4.0`

Auto-continue to claim card 086.

## Out Of Scope

- Gemini requalification (deferred)
- Mapping unused surfaces
- Provider work
- Decoder updates
- Next Task / g04 README edits
