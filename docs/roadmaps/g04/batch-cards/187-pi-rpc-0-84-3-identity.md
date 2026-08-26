# 187 Pi RPC 0.84.3 Identity

Status: completed
Owner: Tom
Milestone: [g04.067 Pi RPC 0.84.3 Useful Newer](../067-pi-rpc-0-84-3-useful-newer.md)
Created: 2026-08-26

## Task

Freeze official npm `@earendil-works/pi-coding-agent` `0.84.3` identity
evidence. Name segment shape. Do not edit production claims in this card.

## Method

1. Observe official `0.84.3` (npm `latest`, published 2026-08-24)
2. Compare extracted official tarball, selected git blobs, and changelog
   against the frozen `0.84.2` corpus
3. Classify unmapped additions
4. Write identity fixture under
   `crates/swallowtail-adapter-pi/tests/fixtures/pi-rpc-0.84.3/`
5. Write research record 214
6. Name segment shape

No provider prompt. No live session. Host install not present and not
changed.

## Expected Shape

Compatible-extension: selected RPC types, framing, session-cwd, and mapped
argv flags match `0.84.2`. `toolcall_start` extras, `--`, `powershell`,
and bundled bin path stay unmapped.

## Acceptance

- Identity fixture written
- Research 214 promoted
- Shape named: compatible-extension
- No production claim edit
- Passes `effigy validate:focused swallowtail-adapter-pi` at the claim
  card

Auto-continue to claim card 188.

## Out Of Scope

- SDK sidecar
- Oh My Pi
- Mapping unused surfaces
- Provider work
- Decoder updates
- Moving `docs/roadmaps/README.md` or g04 generation status
