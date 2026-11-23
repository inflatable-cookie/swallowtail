# Codex Exec 0.171.0 Identity and Currentness Qualification

Status: promoted.
Date: 2026-11-23.
Scope: `codex.exec` only on `codex.cli` (Contract 029 one-family currentness). Research 487.

## Question

Can `codex.exec` extend its maintained `AllowUnverified` ceiling from `0.164.0`
through official stable `0.171.0` (`0.165.0`–`0.171.0`, ten published stable
hops) on the existing `codex.exec.jsonl-v1` behavior revision, closing the
Research 463/467 Linux `bwrap` binary provenance gate and classifying the
`0.166.0` managed-configuration (`config_server`) path while keeping
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
across 9507 files), froze per-version SHA-256 digests and per-hop changed-file
sets for all 23 selected feeding files across `exec`, `cloud-config`, `config`,
`login`, `http-client`, embedded `app-server` (`in_process_bootstrap.rs`,
`application_network.rs`, `config_manager.rs`, `turn_processor.rs`),
`linux-sandbox`, and `sandboxing`, and classified every delta.

## Identity

| Version | npm published | GitHub tag | Commit | Source files | Linux `bwrap` | Sigstore bundle SHA-256 | Rekor logIndex |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `0.164.0` | 2026-10-23T04:52:29.240Z | `rust-v0.164.0` | `6aaac694803004a63c6f393d153c96d22521a6b9` | 9438 | `3a81c19ad10a7043…` | `b570206b8366…` | 3409176533 |
| `0.165.0` | 2026-10-27T20:14:25.378Z | `rust-v0.165.0` | `360319d8dee62d87e2d59fc0c21805d79519cfed` | 9507 | `755c8d6a9b893905…` | `fc367e3276ae…` | 3522973325 |
| `0.166.0` | 2026-10-29T19:56:19.746Z | `rust-v0.166.0` | `6ff1e1ca3b30fc8b914720681fcf5fe62e95f856` | 9732 | `77360cb751ccedc5…` | `502664c0d0aa…` | 3573128412 |
| `0.167.0` | 2026-11-06T01:46:01.347Z | `rust-v0.167.0` | `086ccc833bc9cf01024ad2dd6af7dbd421b6bf9a` | 9816 | `77360cb751ccedc5…` | `9b1fa9713fe2…` | 3704500433 |
| `0.167.1` | 2026-11-10T21:39:51.990Z | `rust-v0.167.1` | `0a94398fa60c4fac0e7659e4213a0a36e613dd46` | 9818 | `77360cb751ccedc5…` | `9f3a185ed36c…` | 3791032838 |
| `0.167.2` | 2026-11-11T22:50:04.191Z | `rust-v0.167.2` | `ce21072c90d32c40af7a55b60b2fb35c84a43a6a` | 9818 | `7df960565a0dece9…` | `9f78eba08978…` | 3818183184 |
| `0.168.0` | 2026-11-13T04:00:31.594Z | `rust-v0.168.0` | `9480ec100c3edee9fc69172e8e80a4730444094d` | 10040 | `77360cb751ccedc5…` | `bf929c4c320e…` | 3840060480 |
| `0.169.0` | 2026-11-13T08:43:21.818Z | `rust-v0.169.0` | `d32063f96c2dff1dc96af839b3952abb6acfeda4` | 10235 | `15fd5f2673d2e1c2…` | `fd228f908d99…` | 3846725555 |
| `0.169.1` | 2026-11-17T22:26:21.610Z | `rust-v0.169.1` | `c376cf7396eff83ce4c6ac61b9553df142e7a0be` | 10238 | `15fd5f2673d2e1c2…` | `2b491cb40be3…` | 3953943746 |
| `0.170.0` | 2026-11-18T03:45:45.798Z | `rust-v0.170.0` | `29eb79e9866b3262a777fdf8195c1b9e63684c91` | 10267 | `15fd5f2673d2e1c2…` | `c910242fcaad…` | 3959192985 |
| `0.171.0` | 2026-11-20T00:51:29.762Z | `rust-v0.171.0` | `ccf9be8cc458bd6b8336fe84d68d8893f2691a5e` | 10460 | `91a245ea678a7ae6…` | `77787c674164…` | 4009668327 |

Full SHA-256 digests, integrity strings, leaf-cert DER SHA-256s, tag objects,
and complete per-hop inventories live in
`crates/swallowtail-adapter-codex/tests/fixtures/codex-exec-0.171.0/`.

## Linux `bwrap` Trusted Provenance Resolution (Closing Research 463/467 Gate)

Research 463 stopped `0.165.0` because shipped `codex-resources/bwrap` bytes
differed from `0.164.0` without an established provenance link. That gate is
now closed across all eleven releases (`0.164.0`–`0.171.0`) using Sigstore's
Public Good TUF trusted root (`gh attestation trusted-root`, trusted root
SHA-256 `844a1c6de3986c9f02070266b25e0d1a2fa99ceccc89f6b9ad90aae47b62a16e`):

1. **Source byte-identity:** `codex-rs/bwrap/{src/main.rs,Cargo.toml,build.rs,config.h}`,
   `codex-rs/linux-sandbox/src/bundled_bwrap.rs`, and the entire vendored C tree
   `codex-rs/vendor/bubblewrap/{bubblewrap.c,bind-mount.c,network.c,utils.c}`
   are **byte-identical** across all eleven tags.
2. **Trusted Fulcio CA chain verification:** For every release, the Fulcio leaf
   X.509 certificate in `bwrap-x86_64-unknown-linux-musl.sigstore` verifies
   against the trusted Fulcio root and intermediate certificates from the TUF
   trusted root at Rekor's `integratedTime` (`openssl verify -CAfile ... -untrusted ... -attime <integratedTime>`).
3. **Trusted Rekor SET verification:** For every release, the `rekorBundle.Payload.logID`
   matches the trusted Rekor log key (`c0d23d6ad406973f9559f3ba2d1ca01f84147d8ffc5b8445c224f98b9591801d`),
   and ECDSA-P256-SHA256 verification of `SignedEntryTimestamp` over canonicalized
   `rekorBundle.Payload` passes against that trusted key. The decoded
   `hashedrekord` entry binds the exact npm `bwrap` SHA-256, `base64Signature`,
   and leaf certificate PEM.
4. **Blob signature and OIDC identity extensions:** For every release, the
   verified leaf certificate's public key verifies `base64Signature` over the
   exact npm-shipped `codex-resources/bwrap` bytes, and its Fulcio X.509
   extensions bind:
   - SAN URI: `https://github.com/openai/codex/.github/workflows/rust-release.yml@refs/tags/rust-v<VER>`
   - OIDC Issuer (`1.3.6.1.4.1.57264.1.8`): `https://token.actions.githubusercontent.com`
   - Source Repository Digest (`1.3.6.1.4.1.57264.1.13`) and Build Config Digest (`1.3.6.1.4.1.57264.1.19`): exact tag commit SHA.
5. **Self-checked digest inside `codex`:** `.github/workflows/rust-release.yml`
   strips `bwrap` before hashing and exports `CODEX_BWRAP_SHA256` into the
   `codex` build. All eleven shipped Linux `x86_64` `codex` binaries embed the
   exact 64-hex-char SHA-256 of their paired `codex-resources/bwrap`.
6. **Scope of binary-delta explanation:** ELF `.comment` records `rustc 1.99.1`
   (`0.164.0`–`0.165.0`), `rustc 1.95.0` (`0.166.0`–`0.170.0`, where
   `0.166.0`/`0.167.0`/`0.167.1`/`0.168.0` share `77360cb751ccedc5…` and
   `0.169.0`/`0.169.1`/`0.170.0` share `15fd5f2673d2e1c2…`), and `rustc 1.99.1`
   (`0.171.0`). The 16-byte `.text` delta between `0.164.0` and `0.165.0` under
   the same `rustc 1.99.1` compiler is not independently reproduced from a local
   Linux musl build and remains unresolved as a compiler-level cause; trusted
   Sigstore provenance plus byte-identical `bwrap` and `vendor/bubblewrap`
   sources establish that both artifacts were built by the exact release
   workflow from identical source.

## Managed Configuration Path (`0.166.0` `config_server`) Classification

In `0.165.0`, `codex-rs/cloud-config/src/{bundle_loader.rs,config_server.rs}`
and `codex-rs/config/src/config_server.rs` introduced administrator-enrolled
custom configuration servers (`[config_server]` in `ConfigRequirementsToml`).
In `0.166.0`, `codex-rs/exec/src/lib.rs` replaced
`cloud_config_bundle_loader_for_storage` with
`managed_config_bundle_loader_for_storage(bootstrap_auth_config, requirements_toml(), false)`:

- **Policy authority:** `config_server` is loaded only from host administrator/MDM
  `requirements.toml` (`codex-rs/config/src/config_requirements.rs`), never from
  user or project configuration. `--ignore-user-config` leaves host-managed
  requirements authoritative, matching Research 410's existing acceptance of
  system/MDM `application.network` and managed requirements without
  `ignore_managed_requirements`. When `[config_server]` is absent, the loader
  executes the same ChatGPT managed-config service path as `0.164.0`.
- **Bootstrap destinations:** `ConfigServer::validated_base_url()` enforces
  HTTPS-only URLs without userinfo, fragments, or `base_url` query strings.
  `config_server_http_client_factory` calls
  `AuthRouteConfig::http_client_factory_for_bootstrap(endpoints)`
  (`codex-rs/login/src/outbound_proxy.rs`), which uses the administrator's
  `local_bootstrap_factory` and intersects its `NetworkPolicy` via
  `restrict_to_endpoints` (`codex-rs/http-client/src/network_policy.rs`) to only
  `config_bundle_url`, `oauth.token_url`, and optional
  `oauth.device_authorization_url`.
- **Non-interactive OAuth and bounded failure:** `ConfigServerSource` creates
  `OAuthSessionManager::new(..., Arc::new(DefaultKeyringStore)).map(OAuthSessionManager::without_interactive_login)`,
  so `codex exec` never launches an interactive browser or device-code sign-in.
  Missing OAuth grants return fatal `CloudConfigBundleLoadErrorCode::Auth`;
  at `0.168.0`, `with_bearer_token` becomes fallible into fatal `auth_failure()`;
  at `0.171.0`, `codex-rs/cloud-config/src/validation.rs` redacts raw TOML and
  provider parse errors in `InvalidBundle`. Any fatal/timeout bundle-load error
  fails `codex exec` closed before turn start, mapping to
  `swallowtail.codex.exec.process_failed` with private stderr omitted and joined
  cleanup.
- **Identity-matched startup cache (`codex-rs/cloud-config/src/service.rs`):**
  `load_startup_bundle` checks `source.auth()` first and prefers a valid,
  identity-matched cached bundle keyed by `source.identity(&auth)` (for
  `ConfigServerSource`: `(authorization_id_or_"anonymous", sha256(config_bundle_url, credential_id))`)
  that passes `validate_bundle(&signed_payload.bundle, &codex_home)`, falling
  back to remote `fetch_remote_bundle_and_update_cache_with_retries` on miss or
  invalid cache contents.
- **Embedded-server retention and `application_network` refresh (`0.166.0`–`0.171.0`):**
  In `0.166.0`, `codex-rs/app-server/src/in_process_bootstrap.rs` replaces both
  `replace_cloud_config_bundle_loader` calls in `configure()` with
  `config_manager.update_managed_config_loader(auth_manager, config)`
  (`codex-rs/app-server/src/config_manager.rs`), which returns early (revoking
  MCP EMA) when an administrator-enrolled `[config_server]` loader is already
  active so ChatGPT/API-key auth updates do not overwrite it.
  `codex-rs/app-server/src/application_network.rs` switches
  `refresh_application_network_policy` to `current_managed_config_loader()`,
  consuming the active ChatGPT or `[config_server]` snapshot.
- **First-turn provider revalidation (`0.166.0`–`0.167.2` vs `0.168.0`–`0.171.0`):**
  In `0.166.0`, `0.167.0`, `0.167.1`, and `0.167.2`,
  `codex-rs/app-server/src/request_processors/turn_processor.rs` (`turn_start_inner`)
  switches from `check_thread_model_provider` to
  `check_thread_model_provider_for_input` (`config_manager.rs`), which reloads
  effective configuration via `load_latest_config` and rejects the turn with
  `InvalidInput(ModelProviderMismatch)` if `configured.model_provider_id != current.model_provider_id`
  — even without `[config_server]`. In `0.168.0`–`0.171.0`, upstream removes
  `check_thread_model_provider_for_input` and restores managed-requirements-only
  `check_thread_model_provider`. On selected one-shot `codex.exec`
  (`--ephemeral --ignore-user-config --ignore-rules`), `thread/start` and the
  sole `turn/start` run back-to-back in the same process without user config;
  any concurrent provider-selection change that triggers this refusal fails
  before turn start via `config_load_error` and exits non-zero, which
  `CodexExecDriver` maps to bounded `swallowtail.codex.exec.process_failed`
  with private stderr omitted and joined cleanup.
- **Per-hop feeding-file ledger:** Exact SHA-256 hashes across all eleven
  versions and per-hop changed sets for all 23 feeding files (including the
  embedded-server and `service.rs` files) are frozen in `source-inventory.json`.

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
- `codex-rs/exec/src/lib.rs`: wires `managed_config_bundle_loader_for_storage` (`0.166.0`, classified above), unselected `--remote` and `--trust` branches, stderr-only untrusted project-local config warning, `LoaderOverrides.default_cwd` replacing `ThreadStartParams.cwd`, and `ThreadHistoryMode::Paginated` backfill skipped when `--ephemeral` is true.
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
- [Research 410](./410-codex-exec-0-161-0-identity-and-policy-qualification.md)
