# 2026-09-01 Kimi Code 0.39.1 Identity

Card: g05.016 batch 041
Research: [270](../research/270-kimi-code-0-39-1-identity.md)

## What changed

Froze official `@moonshot-ai/kimi-code` `0.39.1` identity, the publication
adjacency through `0.39.0`, the exact headless engine-routing boundary, and
the `0.39.0` ACP process-authority delta as Research 270 plus three
secret-free corpora under
`crates/swallowtail-adapter-kimi/tests/fixtures/`. No production claim,
selection, matrix, guide, or route changed in this commit.

## Current state

Official npm and GitHub stable is `0.39.1`, published 2026-08-28. Published
stables above the previous `0.38.0` ceiling are exactly `0.39.0` and `0.39.1`.
`0.38.1`, `0.39.2`, and `0.40.0` are unpublished. npm integrity, registry
shasum, release `.sha256` sidecars, and `manifest.json` agree for every
downloaded artifact, and the recomputed `0.38.0` digests match the frozen
corpus.

Host `kimi 0.34.0` is byte-identical to the official `0.34.0` darwin-arm64
extracted artifact. It was not installed, updated, replaced, or executed.

**Routing boundary.** Re-testing the premise inherited from Research 179 and
211 changed the result. `experimental-v2.ts` defines `isKimiV2Enabled()` as
`KIMI_CODE_EXPERIMENTAL_FLAG` truthy through `0.32.0` and as
`!isLegacyEnabled()` from `0.33.0`, frozen through `0.39.1`. The string
`KIMI_CODE_LEGACY_FLAG` is absent from the `0.32.0` bundle entirely. So the
default `kimi -p` engine flips to agent-core-v2 at `0.33.0`, not `0.38.0`, and
the same release flips naked `kimi acp` to `packages/acp-server`. Production
has claimed `0.33.0..=0.37.2` as qualified `kimi.headless.stream-json.v1`
since g04.064 while the adapter's v1 decoder answers the v2 `system.version`
preamble with `malformed_stream`. Host `0.34.0` sat inside that broken span.
This is a pre-existing defect surfaced here, not a `0.39.x` regression.

**ACP authority.** The `0.39.0` `acpTerminalRunner.ts` change replaces two
fail-closed errors with a local host-process spawn in the leased cwd.
Swallowtail always advertises `terminal: false`, so that branch is always
taken. The containment trace found none: the route declares
`HarnessIsolation::AmbientHost` ("without an isolation claim"), Contract 015
states process ownership implies neither callback authority nor filesystem
containment and treats a terminal request from a terminal-less client as
scope-stopping, and no adapter or runtime control mediates a process the
harness spawns for itself.

Segment shape is therefore split: headless v1 corrects down to
`0.29.0..=0.32.0`, headless v2 corrects down and extends to
`0.33.0..=0.39.1`, and `kimi-code.acp` stops at `0.38.0` with exact `0.39.0`
and `0.39.1` to be excluded. Card 042 owns the claim edit.

Research 179 and 211 and the `0.37.2` and `0.38.0` corpora carry errata for
the false `0.38.0` boundary and the mis-named ACP implementation.

`kimi-code.local-server` deltas, including the removed
`--allow-remote-terminals` flag and the new Remote Control surface, are
recorded as observations for that separate family only. Kimi Platform Chat,
Python `kimi-cli`, Gemini's deferral, and the g05.009 provider-operation
observation gate are untouched; card 034 stays planned and candidate F stays
unpromoted at 249/518.

## Next move

Card 042 corrects the headless segmentation, stops the ACP axis with
exclusions, refreshes tests, docs, matrices, and the standing lane, and stops
for exact-head review.
