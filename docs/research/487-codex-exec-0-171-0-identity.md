# Codex Exec 0.171.0 Identity and Currentness Qualification

Status: promoted.
Date: 2026-11-23.
Scope: `codex.exec` only on `codex.cli` (Contract 029 one-family currentness). Research 487.

## Question

Can `codex.exec` extend its maintained `AllowUnverified` ceiling from `0.164.0`
through official stable `0.171.0` (`0.165.0`–`0.171.0`, ten published stable
hops) on the existing `codex.exec.jsonl-v1` behavior revision, closing the
Research 463/467 Linux `bwrap` binary provenance gate while keeping
`codex.app-server` and lifecycle claims independent at `0.164.0`?

## Decision

Compatible extension for **`codex.exec` only**. Raise `CODEX_LATEST_QUALIFIED_VERSION`
from `0.164.0` to `0.171.0` (`0.156.0..=0.171.0` on `codex.exec.jsonl-v1`,
`Maintained`, `AllowUnverified`). Preserve claim id `codex.exec.cli-window-2`,
baseline `0.80.0`, older segments, and all retained exclusions (`0.82.0`,
`0.83.0`, `0.108.0`, `0.109.0`, `0.149.2`, `0.150.2`, `0.151.1`, `0.152.2`,
`0.154.1`, `0.155.2`). `0.171.1` is the synthetic `UnverifiedNewer` point.
`codex.app-server` and `codex.app-server` lifecycle claims stay untouched at
`0.164.0` (owned by the sibling task).

## Method

Re-probed npm `@openai/codex` `latest` and GitHub `openai/codex` non-prerelease
releases on 2026-11-23. Both agree on `0.171.0` / `rust-v0.171.0`. Ignored
`alpha` (`0.173.0-alpha.4`), `beta`, and `native` dist-tags. Walked all ten
published stables after `0.164.0`: `0.165.0`, `0.166.0`, `0.167.0`, `0.167.1`,
`0.167.2`, `0.168.0`, `0.169.0`, `0.169.1`, `0.170.0`, and `0.171.0`.

Hashed and inventoried wrapper tarballs and native `optionalDependencies`
tarballs (`darwin-arm64` and `linux-x64`) for all eleven points (`0.164.0`–`0.171.0`).
Verified local host `codex` (`codex-cli 0.171.0`, Darwin ARM64, signed by
`Developer ID Application: OpenAI OpCo, LLC (2DC432GLL2)`): its SHA-256
`fc8d77cf7585aded2b9bc487dad29c4ee2f066123e4e76dd3b17653a8322ce42` matches the
official npm `@openai/codex@0.171.0-darwin-arm64` CLI binary byte-for-byte.
Did not execute any downloaded vendor binary; no prompt, login, live session,
or host mutation occurred.

Computed recursive public-tag source inventories (`SHA-256` over sorted
`path\tmode\tblob`) for all eleven tags (reproducing Research 463's exact
`0.165.0` digest `c39481bcf4c96bb514d1fe54047c78e451a66067d0e833874287f6b07f8a3eb0`
across 9507 files), and classified every changed file feeding selected
`codex.exec` argv, config, JSONL output, and Linux/macOS read-only sandbox paths.

## Identity

| Version | npm published | GitHub tag | Commit | Source files | darwin-arm64 tgz | linux-x64 tgz | Linux `bwrap` | Rekor logIndex |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `0.164.0` | 2026-10-23T04:52:29.240Z | `rust-v0.164.0` | `6aaac694803004a63c6f393d153c96d22521a6b9` | 9438 | `9b5a35be1975ceb3…` | `62659fefd9a8efa2…` | `3a81c19ad10a7043…` | 3409176533 |
| `0.165.0` | 2026-10-27T20:14:25.378Z | `rust-v0.165.0` | `360319d8dee62d87e2d59fc0c21805d79519cfed` | 9507 | `a991b0abcb6b6d2a…` | `f997cd6ef5987126…` | `755c8d6a9b893905…` | 3522973325 |
| `0.166.0` | 2026-10-29T19:56:19.746Z | `rust-v0.166.0` | `6ff1e1ca3b30fc8b914720681fcf5fe62e95f856` | 9732 | `181d2eda8addbeca…` | `cbc00a26d3d0e2ce…` | `77360cb751ccedc5…` | 3573128412 |
| `0.167.0` | 2026-11-06T01:46:01.347Z | `rust-v0.167.0` | `086ccc833bc9cf01024ad2dd6af7dbd421b6bf9a` | 9816 | `6989512a93d5fc74…` | `2dc9aa871abb513c…` | `77360cb751ccedc5…` | 3704500433 |
| `0.167.1` | 2026-11-10T21:39:51.990Z | `rust-v0.167.1` | `0a94398fa60c4fac0e7659e4213a0a36e613dd46` | 9818 | `402f0e6678eef284…` | `944cbabe58a2c599…` | `77360cb751ccedc5…` | 3791032838 |
| `0.167.2` | 2026-11-11T22:50:04.191Z | `rust-v0.167.2` | `ce21072c90d32c40af7a55b60b2fb35c84a43a6a` | 9818 | `7dd03256d9c38110…` | `637bf17e7a3082b6…` | `7df960565a0dece9…` | 3818183184 |
| `0.168.0` | 2026-11-13T04:00:31.594Z | `rust-v0.168.0` | `9480ec100c3edee9fc69172e8e80a4730444094d` | 10040 | `085d6ea222b80c89…` | `de415213cf1ca175…` | `77360cb751ccedc5…` | 3840060480 |
| `0.169.0` | 2026-11-13T08:43:21.818Z | `rust-v0.169.0` | `d32063f96c2dff1dc96af839b3952abb6acfeda4` | 10235 | `d67d443253294438…` | `89801b4b97faba56…` | `15fd5f2673d2e1c2…` | 3846725555 |
| `0.169.1` | 2026-11-17T22:26:21.610Z | `rust-v0.169.1` | `c376cf7396eff83ce4c6ac61b9553df142e7a0be` | 10238 | `1a11a5744b18a211…` | `4bda4d718be83d16…` | `15fd5f2673d2e1c2…` | 3953943746 |
| `0.170.0` | 2026-11-18T03:45:45.798Z | `rust-v0.170.0` | `29eb79e9866b3262a777fdf8195c1b9e63684c91` | 10267 | `ab7ba154261db88e…` | `67c4bccce8e6c3a2…` | `15fd5f2673d2e1c2…` | 3959192985 |
| `0.171.0` | 2026-11-20T00:51:29.762Z | `rust-v0.171.0` | `ccf9be8cc458bd6b8336fe84d68d8893f2691a5e` | 10460 | `185e1c789e5fd528…` | `e13d67afb0533cc6…` | `91a245ea678a7ae6…` | 4009668327 |

Full SHA-256 digests, integrity strings, tag objects, file counts, and per-hop
inventories live in `crates/swallowtail-adapter-codex/tests/fixtures/codex-exec-0.171.0/`.

## Linux `bwrap` Provenance Resolution (Closing Research 463/467 Gate)

Research 463 stopped `0.165.0` because shipped `codex-resources/bwrap` bytes
differed from `0.164.0` without an established build cause or provenance link.
That gate is now closed across all eleven releases (`0.164.0`–`0.171.0`):

1. **Source byte-identity:** `codex-rs/bwrap/src/main.rs`, `Cargo.toml`,
   `build.rs`, `config.h`, `codex-rs/linux-sandbox/src/bundled_bwrap.rs`, and
   the entire vendored C tree `codex-rs/vendor/bubblewrap/` (`bubblewrap.c`,
   `bind-mount.c`, `network.c`, `utils.c`) are **byte-identical** across all
   eleven tags.
2. **Cryptographic Sigstore provenance:** Every GitHub Release publishes
   `bwrap-x86_64-unknown-linux-musl.sigstore`. For all eleven versions,
   `openssl dgst -sha256 -verify` against the Fulcio X.509 certificate public
   key passes on the exact npm-shipped `codex-resources/bwrap` binary, the
   Rekor transparency log hash matches the binary SHA-256, and the Fulcio
   certificate binds the build to
   `https://github.com/openai/codex/.github/workflows/rust-release.yml@refs/tags/rust-v<VER>`
   at that tag's exact commit SHA.
3. **Self-checked digest inside `codex`:** `.github/workflows/rust-release.yml`
   builds and strips `bwrap` before `codex` and exports `CODEX_BWRAP_SHA256`
   into the `codex` build. All eleven shipped Linux `x86_64` `codex` binaries
   embed the exact 64-hex-char SHA-256 of their paired `codex-resources/bwrap`.
4. **Build cause of byte deltas:** `codex-bwrap` is a Rust wrapper binary
   (`version.workspace = true`) linking the vendored C `bwrap_main`. ELF
   `.comment` inspection shows:
   - `0.164.0` and `0.165.0` both build with `rustc 1.99.1`; the workspace
     version bump (`0.164.0` → `0.165.0`) shifts crate SVH/codegen layout with
     identical strings and source.
   - `0.166.0`–`0.170.0` roll `codex-rs/rust-toolchain.toml` to `rustc 1.95.0`
     (`0.166.0`, `0.167.0`, `0.167.1`, and `0.168.0` produce identical bytes
     `77360cb751ccedc5…`; `0.169.0`, `0.169.1`, and `0.170.0` produce identical
     bytes `15fd5f2673d2e1c2…`).
   - `0.171.0` returns `codex-rs/rust-toolchain.toml` to `rustc 1.99.1`.

## Selected `codex.exec` Protocol Classification

Selected invocation: `codex exec --json --ephemeral --color never --ignore-user-config --ignore-rules --skip-git-repo-check --sandbox read-only --model <M> --config approval_policy="never" --config shell_environment_policy.inherit="none" --config hide_agent_reasoning=false --config show_raw_agent_reasoning=false --config web_search="disabled"|"live" [--config model_reasoning_effort=...] [--config model_verbosity=...] [--image ...] [--output-schema ...] -`.

Byte-identical across all ten hops (`0.164.0`–`0.171.0`):
- `codex-rs/exec/src/exec_events.rs`
- `codex-rs/protocol/src/config_types.rs`
- `codex-rs/linux-sandbox/src/bundled_bwrap.rs`
- `codex-rs/bwrap/{src/main.rs,Cargo.toml,build.rs,config.h}` and `codex-rs/vendor/bubblewrap/{bubblewrap.c,bind-mount.c,network.c,utils.c}`

Classified changed files feeding `codex.exec`:
- `codex-rs/exec/src/cli.rs`: adds only unselected `--remote`, `--remote-auth-token-env`, and `--trust` flags.
- `codex-rs/exec/src/event_processor_with_jsonl_output.rs` and `event_processor.rs`: internal `ThreadItem` → `ThreadItemPayload` rename, `print_thread_started` (unselected `--remote`), and `BoundOutput` for unselected `--output-last-message`. All mapped JSONL event types, item payloads, and `usage.{input_tokens,output_tokens}` fields stay unchanged.
- `codex-rs/exec/src/lib.rs`: unselected `--remote` and `--trust` branches; untrusted project-local config warning writes to stderr only; `LoaderOverrides.default_cwd` replaces `ThreadStartParams.cwd` without weakening read-only sandbox; `ThreadHistoryMode::Paginated` backfill is skipped because `should_backfill_turn_completed_items` returns `false` when `thread_ephemeral` (`--ephemeral`) is true.
- `codex-rs/config/src/{loader/local.rs,loader/mod.rs,config_toml.rs}`: zero diff lines touching `ignore_user_config`, `ignore_rules`, `approval_policy`, `shell_environment_policy`, `hide_agent_reasoning`, `show_raw_agent_reasoning`, `web_search`, `model_reasoning_effort`, or `model_verbosity`.
- `codex-rs/linux-sandbox/src/{bwrap.rs,linux_run_main.rs,launcher.rs}` and `codex-rs/sandboxing/src/{bwrap.rs,seatbelt.rs}`: replaces external `rg` deny-glob scan with in-process `ignore` walk, adopts `resolve_for_enforcement`, adds unselected `--transparent-tcp` / `BwrapNetworkMode::PreparedProxy`, adds `pidfd_open`+`poll` wait on system bwrap probe, and denies `TIOCSTI` on `ProcessWithInheritedTerminal`. Selected `--sandbox read-only` keeps zero workspace writable roots and retains `PROTECTED_METADATA_PATH_NAMES` (`.git`, `.agents`, `.codex`, `.aws`).
- npm wrapper `bin/codex.js`: uses `process.execve` on non-Windows when available and improves child fallback signal forwarding.

## Claim Shape

- `codex.exec.cli-window-2`: maintained `0.156.0..=0.171.0` on `codex.exec.jsonl-v1`, `AllowUnverified`; `0.171.1` is `UnverifiedNewer`.
- `codex.app-server.cli-window-2` and `codex.app-server` lifecycle: unchanged at `0.164.0`.

## Sources

- [Official npm package](https://www.npmjs.com/package/@openai/codex)
- [Codex `0.171.0` source tag](https://github.com/openai/codex/tree/rust-v0.171.0)
- [Research 467](./467-codex-attached-child-lifecycle-and-paginated-resume.md)
- [Research 463](./463-codex-exec-app-server-0-165-0-identity.md)
- [Research 452](./452-codex-exec-app-server-0-164-0-identity.md)
