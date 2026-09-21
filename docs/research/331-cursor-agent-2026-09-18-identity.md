# Research 331: Cursor Agent 2026.09.18 Identity

Status: complete; identity evidence only. Production claim changes land with
this family's claim commit after this record.

Observed 2026-09-21 on the `cursor-agent.release-date` axis:

- Host `cursor-agent` is not on `PATH`. Missing install is not a gap. The
  official ACP-registry/stable channel is qualified without flattening onto
  another family.
- ACP registry Cursor entry: `2026.09.18`, archives
  `https://downloads.cursor.com/lab/2026.09.18-9a7762b/{darwin,linux}/…/agent-cli-package.tar.gz`.
  Registry git history and official downloads agree. No newer stable exists
  at observation.
- Published hops after the prior `2026.09.10-fd3934a` ceiling are exactly
  `2026.09.15-d2fe57e` and `2026.09.18-9a7762b` (registry commits
  `57a30180` and `ee2660b2`). No publication index exists, so unobserved
  calendar dates stay unclaimed gaps.
- Independently unqualified older published points `2026.08.25-3e8eec8` and
  `2026.09.08-6caf4ff` stay calendar gaps. They are not raised now.
- npm `cursor-agent@1.0.3` remains a different axis.

## Official hop identities

| Version | darwin-arm64 SHA-256 | linux-x64 SHA-256 | Runtime index | Package files |
| --- | --- | --- | --- | --- |
| `2026.09.10-fd3934a` (ceiling) | `aec0b01a…e0285423` | `27997c83…8966e96e` | `230df535…e23468` | 443 |
| `2026.09.15-d2fe57e` | `f51579a1…7a8d3fbc` | `4b7b026d…96a15c97` | `f76fa45b…4d222a98` | 444 |
| `2026.09.18-9a7762b` | `4e67b9ac…fd02095d` | `b1308f5a…8132bad9` | `19af1927…57c8a14b` | 445 |

The recomputed `2026.09.10` darwin-arm64 archive, runtime index, ACP chunk
`7214.index.js`, and headless chunk `1186.index.js` match Research 312
exactly. `package.json` and the launcher are byte-identical across all three
compared trees. AppleDouble `._*` junk from macOS xattrs is excluded from
counts.

The per-hop complete-tree inventory is frozen in the
[2026.09.18 dist inventory](../../crates/swallowtail-adapter-cursor/tests/fixtures/cursor-agent-2026.09.18/dist-inventory.json).
Per-hop tree deltas are `+62/-61/~55` and `+61/-60/~77`. 305 files are
byte-identical through all three hops. Chunk filenames embed content hashes,
so renames dominate added/removed sets.

## Selected-surface classification

Selected CLI definitions (`models`, `acp`, `--print`,
`--output-format text|json|stream-json`, `--model`, `--trust`,
`--mode plan|ask`) persist. Command and option registrations are identical
across hops. The `models` command remains
`.command("models").description("List available models for this account")`.

Selected ACP initialize subset is unchanged on Swallowtail's client:
protocol v1, `cursor_login`, load/list advertised, image prompt, HTTP/SSE
MCP, no `agentInfo`, authenticated session-create gate. Swallowtail
initialize sends only `fs.readTextFile` / `fs.writeTextFile`.

`2026.09.15` adds a client-negotiated `sessionCapabilities.subagents` object
when `clientCapabilities` enable it. The agent default is false. That extra
is unmapped. It does not add a selected public operation. Load-history
replay text remains; load stays advertised without a proven replay.
Continuation recovery stays blocked.

Stream-json event keys (`type`, `subtype`, `call_id`, `tool_call`,
`model_call_id`, `session_id`, `timestamp_ms`) keep the same counts.
`2026.09.15` → `2026.09.18` ACP chunks are the same size and differ by 663
bytes, mostly embedded version literals.

Worker/persist/plugin/cloud flags and SEA/native rebuilds feed no selected
surface. Downloaded archives were hashed and never executed.

## Contract 029 decision

The family is a `compatible-extension` of
`cursor-agent.catalogue.calendar-release-v1`,
`cursor-agent.acp-v1.interactive-v1`, and
`cursor-agent.stream-json.structured-v1`. Keep the baseline
`2026.07.01-41b2de7` and all three claim ids. Qualify both published hops
and raise the ceiling to `2026.09.18-9a7762b`. Dates between the nine exact
points stay incompatible. Feature-specific exact sets (Ask, high-effort,
model parameters) stay on their probed points except that Ask continues to
follow exactly qualified releases. No provider prompt, live session, login,
installation, host update, or credential was used.
