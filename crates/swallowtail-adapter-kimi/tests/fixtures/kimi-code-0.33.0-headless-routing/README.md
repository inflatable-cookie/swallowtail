# Kimi Code headless agent-core-v2 routing boundary

This corpus answers one question with official evidence: at which exact
published point does a naked
`kimi --prompt … --output-format stream-json` stop running the legacy
agent-core v1 print body and start running agent-core-v2 `runV2Print`?

The answer is **`0.33.0`**, not `0.38.0`.

## Why this corpus exists

Research 179 and 211 recorded the v1→v2 default flip as happening at `0.38.0`,
because `0.38.0` was the only point sampled. Research 270 sampled every
published point from `0.28.1` to `0.39.1` and found the flip three minor
versions earlier. Production had therefore been claiming
`0.33.0..=0.37.2` as qualified `kimi.headless.stream-json.v1` for releases
whose default `-p` path is v2. Swallowtail's v1 decoder rejects the v2
`system.version` preamble outright, so those points could not have worked.
Recorded host `0.34.0` sat inside that broken span.

## The evidence

`identity.json` freezes npm, tag, commit, tarball and platform-archive
identity for the last v1-default point (`0.32.0`) and the first v2-default
point (`0.33.0`). Every archive matches its release `manifest.json` entry.

`protocol.json` records the gate itself. At `0.32.0`,
`experimental-v2.ts` defines `isKimiV2Enabled()` as
`KIMI_CODE_EXPERIMENTAL_FLAG` truthy, so the default is v1 and the string
`KIMI_CODE_LEGACY_FLAG` does not appear in the shipped bundle at all. From
`0.33.0` the same file defines `isKimiV2Enabled()` as `!isLegacyEnabled()`,
keyed on `KIMI_CODE_LEGACY_FLAG`, so the default is v2. That file is
byte-identical from `0.33.0` through `0.39.1`. `run-prompt.ts` delegates to
`runV2Print` under the gate and is itself unchanged in substance across the
flip: only the gate's meaning moved.

Swallowtail never sets `KIMI_CODE_LEGACY_FLAG`, so the naked path is v2 from
`0.33.0` onward.

## Mapped v2 surface stability

The v2 stream-json emission surfaces — the JSON writer, the
`system.version` preamble writer, the `session.resume_hint` writer, and tool
output stringification — carry one digest each from `0.32.0` through
`0.39.1`, in both the npm bundle and the extracted single-executable archive.

One mapped surface does move: `dispatchNativeEvent` is retyped at `0.37.0`
(`DomainEvent` → `Event2<any>` plus casts). Both digests are recorded rather
than smoothed over. Every case label, writer call, argument, and stderr branch
is identical, so the emitted JSONL does not change. The `0.34.0`
`setClampedTimeout` change is in the print background-policy wait loop, which
the adapter never selects.

## Not in scope

This corpus does not touch `kimi-code.local-server`, Kimi Platform Chat, the
ACP route's own authority question, or any second family. No provider prompt,
authentication, install, host update, live probe, or execution of a downloaded
binary was involved. No credential, host path, account identity, session id, or
provider payload appears here.
