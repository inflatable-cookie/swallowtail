# Research 311: Codex 0.154.0 Identity

Status: complete; identity evidence only. Production claim changes land in
the g05.061 claim batch after this record.

Observed 2026-09-14 on the `codex.cli` axis:

- Host `codex --version`: `codex-cli 0.153.3`; executable SHA-256
  `0e1f892695844ad0798dab8895955846450a9e7663476ebf24615814dd377216`,
  220584480 bytes, Developer ID `OpenAI OpCo, LLC (2DC432GLL2)`. The digest
  equals the official `0.153.3` darwin-arm64 package value frozen this run.
  The host was not installed, updated, or otherwise mutated.
- `npm view @openai/codex version`: `0.154.0`, published
  `2026-09-09T22:40:10.746Z`. GitHub release `rust-v0.154.0` published
  `2026-09-09T22:35:38Z`. npm and GitHub agree; npm is latest and no newer
  stable exists. The `alpha` dist-tag is `0.155.0-alpha.3.10` and is ignored.
- Official npm stable hops after the prior `0.152.1` ceiling are exactly
  `0.153.0`, `0.153.1`, `0.153.2`, `0.153.3`, `0.153.4`, and `0.154.0`.
  `0.152.2` was never published and stays a gap. `0.154.1` was not published
  at observation.

## Official hop identities

| Version | npm published | GitHub tag commit | npm shasum | darwin-arm64 CLI SHA-256 |
| --- | --- | --- | --- | --- |
| `0.152.1` | `2026-09-01T22:36:50.784Z` | `5adb68a4` | `9e51ebd177c5523b299636a2d5f788922fe6eb03` | `8194ea31…497a7bf` |
| `0.153.0` | `2026-09-03T01:42:05.774Z` | `41e22fee` | `0eaa6aa8ef127c2d3123965cc56d87d0b00b4a87` | `a29d9e86…7eaa22` |
| `0.153.1` | `2026-09-03T21:09:57.109Z` | `98564127` | `c38c10819015b65ee54f2dece083b4ed4b23823c` | `62709f1e…090c7a` |
| `0.153.2` | `2026-09-03T23:57:21.074Z` | `657a993c` | `c15bedb54903b9ecbd5d38a67aef72f20eb944c9` | `195ace41…400424` |
| `0.153.3` | `2026-09-04T19:07:16.079Z` | `b1a547b1` | `1e7737ad9611d387194f71d280a2b14bf2570ba2` | `0e1f8926…377216` |
| `0.153.4` | `2026-09-04T23:31:18.513Z` | `3d2ee51c` | `7f0283a793e438733df5dccbe5d79ffd778ab0e8` | `b973d440…1261e3` |
| `0.154.0` | `2026-09-09T22:40:10.746Z` | `6b9826e3` | `eb1ec011cca427c606ef6cd4814014e59a1c3d70` | `4f859826…95afcc` |

The official `0.154.0` npm integrity is
`sha512-FV/x1OHXYv/ifjf3mXj9ThTTAWcUZN6cGIRQRhRxkKNOPuImu1WW0c8ev1vUkE9XGH90dEnYG1tBjIkxRikg0w==`.
Its tarball SHA-256 is
`870663d4e65042dd358305e96a22af58708a28317cd4e74a85aa867c69f5859b`.
The extracted darwin-arm64 CLI is 222,655,232 bytes with SHA-256
`4f85982624b3898c8991cb80c0981b2aa71070e3537046c9a95950318a95afcc`;
the linux-x64 CLI is 262,858,016 bytes with SHA-256
`3188814c35471432d4123203e0eb38e5bddc60226e3d7ddf0e59e649ea140022`.
The extracted binaries contain the `0.154.0` version literal (17 and 16
occurrences) and no `0.152.1` literal. The recomputed `0.152.1` digests match
Research 275 exactly.

The per-hop wrapper/platform/complete-source-tree inventory is frozen in the
[0.154.0 dist inventory](../../crates/swallowtail-adapter-codex/tests/fixtures/codex-cli-0.154.0/dist-inventory.json).
Per-hop source deltas are `+151/-12/~576`, `+1/-0/~10`, `+0/-0/~2`,
`+5/-0/~8`, `+0/-0/~3`, and `+430/-37/~955`. Wrapper trees keep 3 files,
darwin-arm64 7, linux-x64 8; wrapper `README.md` is byte-identical across all
hops, `package.json` files are version-and-pin only, and `bin/codex.js`
changes once for vite-plus installer detection and is then stable.

## Selected-surface classification

The exec JSONL wire sources (`exec_events.rs`,
`event_processor_with_jsonl_output.rs`) are byte-identical at all seven tags,
and every selected exec flag definition is unchanged; `0.154.0` only makes
the new opt-in `--worktree` flag global, and it is explicitly rejected with
selected `--ephemeral`, `--ignore-user-config`, and resume/review. CLI
`long`-flag comparison shows no selected removal or respelling: `0.153.0`
touches exit-message formatting only, and `0.154.0` adds AWS/exec-server
remote options. The app-server stdio flags are untouched (`0.154.0` adds a
recursion limit and a musl allocator).

`ModelListParams`, `TurnStartParams`, `ThreadStartParams`,
`ThreadReadParams`, archive/delete, turn/interrupt, and initialize schemas
are byte-identical across the window. `ThreadResumeParams` gains only
`ConfigurationReasoning`/`ReasoningEffort` definitions and the
`ConfigurationUpdateResponseItem` variant with `required: [threadId]` and
every selected property (including `excludeTurns`) unchanged.
`ThreadListParams` gains only optional `originators`; item notifications gain
only optional `questions`. `ClientRequest`/`ServerNotification` gain only
definitions; all 15 selected method titles are present. The adapter has no
`deny_unknown_fields`, so additive fields are tolerated.

Archive/delete RPC semantics are untouched (realtime-removal and unload-delay
configurability only); selected dispatch arms are untouched (new
plugin-reconcile and user-verification arms only); the redaction function is
unchanged; the deprecated `codex mcp-server` entry-point removal, daemon
lifecycle commands, model catalog data, and vendored tool refreshes are
unmapped. Changelog mapped-keyword bullets name only these bounded surfaces.

## Contract 029 decision

The family is a `compatible-extension` of `codex.exec.jsonl-v1` and
`codex.app-server.v2.workspace-roots` with lifecycle
`codex.app-server.lifecycle.v1.strict-descendant-hard-delete`. Keep baseline
`0.80.0` and both claim ids. Qualify all six published hops and raise the
ceiling to `0.154.0`. Pin newly interior unpublished `0.152.2` incompatible
alongside `0.149.2`, `0.150.2`, and `0.151.1`. After qualification, `0.154.1`
is the first unverified newer stable candidate. Feature-specific exact pins
(model verbosity, fast mode, personality, plan-mode effort) stay on the
`0.147.0..=0.149.1` probed points. No provider prompt, live session, login,
installation, host update, or credential was used.
