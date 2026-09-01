# 041 Kimi Code 0.39.1 Identity

Status: completed
Owner: Tom
Created: 2026-09-01
Updated: 2026-09-01
Milestone: `../016-kimi-code-0-39-1-useful-newer.md`
Depends on: Contract 029; Research 269; official stable `0.39.1`

## Goal

Freeze exact official Kimi Code `0.39.1` identity and classify the selected
ACP, headless v1, and headless v2 surfaces per axis without changing a claim.

## Scope

1. Recheck npm, GitHub tag/commit, tarball integrity, platform archives and
   their sidecars and manifest, extracted artifacts, and selected git blobs.
   Do not infer identity from registry `latest` alone.
2. Record publication adjacency through `0.39.0` and the first unpublished
   later stable.
3. Keep host `0.34.0` observation-only. Do not install, update, replace, or
   run it. Corroborate it against the official `0.34.0` artifact.
4. Recompute rather than trust the frozen `0.38.0` corpus digests.
5. Name the executing bundle and path actually launched, including which
   agent-core implementation the naked ACP and headless argv reach.
6. Compare selected ACP and headless surfaces against `0.38.0` and both newer
   stables. Derive mapped and unmapped ledgers from production authority, not
   changelog prose alone.
6a. Re-test the inherited engine-routing premise rather than assuming it. Walk
   every published point for the engine gate, the print dispatch site, the
   renderer, the v2 runner, and both ACP entry points, and prove the exact
   routing boundary from shipped artifacts as well as source.
6b. Treat any capability, failure, or process-authority change as an
   authority-invariant question before it is a revision-label question. Trace
   whether an adapter or runtime control actually contains it, from code and
   contracts.
7. Add mutation-sensitive cross-corpus oracles that detect fabricated or
   self-consistent fixture drift where practical.
8. Record local-server-only deltas as observations for that separate family.
   Do not widen, edit, or flatten its claim, fixtures, route, guide, matrix
   cell, or conclusions.
9. Add Research 270 and one secret-free `0.39.1` identity corpus.
10. Commit identity evidence before any selection, matrix, guide, changelog,
    or standing-lane claim edit.
11. Record compatible extension, private milestone, new revision, or stop,
    separately per axis.

## Out Of Scope

Production claim edits, local-server claim change, Kimi Platform Chat, another
family, Gemini, provider contact, model request, authentication, catalogue or
session work, install, host update, live probe, projection, skill, papercut,
g05.009 card 034, release, or execution of downloaded official binaries.

## Acceptance Criteria

- official identity is corroborated through independent official channels
- mapped and material unmapped additions are explicit per axis
- current production claims are byte-for-byte unchanged in this commit
- fixture provenance, digests, and negative boundaries are load-bearing
- `kimi-code.local-server` and g05.009 are provably unaffected
- card 042 continues only for axes with an admitted segment

## Validation

- `effigy validate:focused swallowtail-adapter-kimi`
- `effigy qa:northstar`
- `git diff --check`

## Auto-Continuation

Yes, to card 042 only for the axes whose segment is admitted.

## Result

Official stable remained exact `0.39.1` through the run. Published stables above
the previous `0.38.0` ceiling are exactly `0.39.0` and `0.39.1`; `0.38.1`,
`0.39.2`, and `0.40.0` are unpublished. Registry integrity, release `.sha256`
sidecars, and `manifest.json` agree for every downloaded artifact, and every
recomputed `0.38.0` digest matches the frozen corpus. Host `0.34.0` is
byte-identical to the official `0.34.0` darwin-arm64 extracted artifact and was
not installed, updated, replaced, or executed. No downloaded binary was
executed.

Research 270 and the frozen corpus landed in identity-only commit `d931d9a9`;
production claims were unchanged in that commit. The executing path is
agent-core-v2 on both routes because the adapter never sets
`KIMI_CODE_LEGACY_FLAG`; Research 270 corrects the ACP evidence surface
Research 179 named without moving a qualified point. Cross-corpus oracles hold
across the npm bundle and both extracted archives at all three versions.

Re-testing the inherited routing premise changed the outcome. The default
`kimi -p` engine flips to agent-core-v2 at `0.33.0`, not `0.38.0`:
`experimental-v2.ts` redefines `isKimiV2Enabled()` from
`KIMI_CODE_EXPERIMENTAL_FLAG` truthy to `!isLegacyEnabled()` there, and the
string `KIMI_CODE_LEGACY_FLAG` is absent from the `0.32.0` bundle entirely.
Production had claimed `0.33.0..=0.37.2` as qualified
`kimi.headless.stream-json.v1` while the adapter's v1 decoder rejects the v2
`system.version` preamble those releases emit. Host `0.34.0` sat inside that
broken span. Research 179 and 211 carry errata; the `0.37.2` and `0.38.0`
corpora carry errata; corpus `kimi-code-0.33.0-headless-routing` freezes the
boundary from both npm and platform-archive artifacts.

The `0.39.0` ACP terminal-runner change was traced as an authority question.
Swallowtail always advertises `terminal: false`, so the new
`local.spawn` branch is always taken; the route declares
`HarnessIsolation::AmbientHost` with no isolation claim, Contract 015 denies
containment from process ownership and treats terminal requests from a
terminal-less client as scope-stopping, and no adapter or runtime control
mediates the spawn. Containment is absent. Corpus
`kimi-code-0.39.0-acp-authority` records the trace.

Segment shape is therefore split: headless v1 corrects down to
`0.29.0..=0.32.0`, headless v2 corrects down and extends to
`0.33.0..=0.39.1`, and `kimi-code.acp` stops at `0.38.0` with exact `0.39.0`
and `0.39.1` excluded. `kimi-code.local-server` and g05.009 are unaffected, so
card 042 continued.
