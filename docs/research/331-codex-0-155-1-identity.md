# Research 331: Codex 0.155.1 Identity

Status: promoted. Identity evidence for the Codex family qualification from
qualified `0.154.0` through official npm/GitHub stable `0.155.1`.

Observed 2026-09-21 on the `codex.cli` axis:

- Host `codex` is not on `PATH` in this Linux observation environment.
  Missing host install is not a gap. Nothing was installed, updated, or
  otherwise mutated.
- `npm view @openai/codex version`: `0.155.1`, published
  `2026-09-18T20:09:23.628Z`. GitHub release `rust-v0.155.1` published
  `2026-09-18T20:03:04Z`. npm and GitHub agree; npm `latest` is `0.155.1`
  and no newer stable exists. The `alpha` dist-tag is `0.156.0-alpha.14`
  and is ignored.
- Official npm stable hops after the prior `0.154.0` ceiling are exactly
  `0.155.0` then `0.155.1`. `0.154.1` was never published and is a new
  interior gap. `0.155.2` was not published at observation.

## Official hop identities

| Version | npm published | GitHub tag commit | npm shasum | darwin-arm64 CLI SHA-256 |
| --- | --- | --- | --- | --- |
| `0.154.0` | `2026-09-09T22:40:10.746Z` | `6b9826e3` | `eb1ec011cca427c606ef6cd4814014e59a1c3d70` | `4f859826…95afcc` |
| `0.155.0` | `2026-09-17T23:19:02.781Z` | `f0a1b8f0` | `d0feb7b169b84ee02dd9712c14eee1abe8bf0caf` | (platform tree + voice-host) |
| `0.155.1` | `2026-09-18T20:09:23.628Z` | `be2951ea` | `7190987abfcabc8fcba7774518539d694a490fc8` | `8eaf1ad1…c5fc0a9e` |

Tag peels: `rust-v0.154.0` object `36eab010…` commit `6b9826e3…` (matches
Research 311); `rust-v0.155.0` object `799f378e…` commit `f0a1b8f0…`;
`rust-v0.155.1` object `4e21628f…` commit `be2951ea…`. GitHub
`rust-v0.155.0` published `2026-09-17T23:14:43Z`.

The official `0.155.1` npm integrity is
`sha512-02fAAGyBtlA1zPjEo3kTj/bOSYbPz5DvjLwRZJdV7weFFEDzNFOMjQGmZ/+5CuirYV0hE+AZTrnjzwXYU4AdAQ==`.
Its tarball SHA-256 is
`fded5b71797aaaf9b1c3229c0e2747b53b39887ef25f36ec7196f6d511db1a66`.
The extracted darwin-arm64 CLI is 228,803,200 bytes with SHA-256
`8eaf1ad12fe6bf89b1710330f58900014322c7c5af677e43be116d8ac5fc0a9e`;
the linux-x64 CLI is 269,273,536 bytes with SHA-256
`0753dfe1d8b87a52436deb13eb1c549661ef4c84fee2c5aa688385eebeccb761`.
The extracted binaries contain the `0.155.1` version literal (14 and 15
occurrences) and no `0.154.0` or `0.155.0` literal. Downloaded binaries
were hashed and never executed. Recomputed `0.154.0` wrapper and platform
CLI hashes match Research 311.

The per-hop wrapper/platform/complete-source-tree inventory is frozen in the
[0.155.1 dist inventory](../../crates/swallowtail-adapter-codex/tests/fixtures/codex-cli-0.155.1/dist-inventory.json).
Per-hop source deltas are `+399/-49/~890` then `+0/-0/~3`. Wrapper trees
keep 3 files. Platform trees grow at `0.155.0` (darwin 7→44, linux 8→46)
because the shipped voice-host/GStreamer tree is unmapped. Wrapper
`README.md` and `bin/codex.js` are byte-identical across the window;
`package.json` is version-and-pin only. `0.155.0→0.155.1` source is
`Cargo.toml` plus two TUI files.

## Selected-surface classification

The entire `codex-rs/exec/` tree is byte-identical across
`rust-v0.154.0`, `rust-v0.155.0`, and `rust-v0.155.1`. Every selected exec
flag definition is unchanged.

`ModelListParams`, `TurnStartParams`, `ThreadResumeParams`,
`ThreadListParams`, archive/delete/read/start, turn/interrupt, and
initialize schemas equal the frozen `0.154.0` corpus. `ClientRequest`
gains only thread/attachment add|list|remove. `ServerNotification` gains
only `Thread/attachment/updated`. `ClientNotification` and `ServerRequest`
are byte-identical. Selected required fields including `thread/resume`
`excludeTurns` stay `[threadId]` with identical properties. All 15
selected method titles are present. The adapter has no
`deny_unknown_fields`, so additive fields are tolerated.

Delete still enumerates descendants, calls `thread_store.delete_threads`
descendants-before-root, and emits `thread/deleted`.
`state_db.delete_threads_strict` is removed as internal cleanup after the
selected store delete. Turn admission is drain-only. `--managed-daemon` is
hidden and unmapped.

Voice-host/GStreamer, code-mode-host, attachments, daemon, TUI,
user-verification, and turn-admission stay unmapped. Do not flatten onto
voice or code-mode-host.

## Contract 029 decision

The family is a `compatible-extension` of `codex.exec.jsonl-v1` and
`codex.app-server.v2.workspace-roots` with lifecycle
`codex.app-server.lifecycle.v1.strict-descendant-hard-delete`. Keep
baseline `0.80.0` and both claim ids. Raise exec and app-server together.
Qualify both published hops and raise the ceiling to `0.155.1`. Pin newly
interior unpublished `0.154.1` incompatible alongside `0.149.2`,
`0.150.2`, `0.151.1`, and `0.152.2`. After qualification, `0.155.2` is the
first unverified newer stable candidate. Feature-specific exact pins
(model verbosity, fast mode, personality, plan-mode effort) stay on the
`0.147.0..=0.149.1` probed points. No provider prompt, live session,
login, installation, host update, or credential was used.
