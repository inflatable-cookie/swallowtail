# 219 Grok Build ACP Subagents-Disabled Evidence

Status: promoted; evidence stop
Owner: Tom
Created: 2026-08-26
Updated: 2026-08-26
Card: g04.072 / 198

## Question

Which exact Grok Build `1.0.4..=1.0.5` rows can Swallowtail bind as an
adapter-local `--no-subagents` launch profile on `grok-build.acp` so that every
ACP session owned by one child is immutably restricted, without raw flags,
explicit enabling, ambient override, child observation/control, permission, or
an isolation claim?

## Method And Boundary

Evidence was frozen on 2026-08-26 from current official public documentation,
exact published npm metadata and tarballs, exact decompressed darwin-arm64
binaries, isolated local help/parser cases, and unauthenticated no-prompt ACP
`initialize` only. No Grok install, host-binary replacement, login, account
inspection, credential capture, `authenticate`, `session/new`, provider prompt,
tool/subagent execution, or paid inference.

The selected operation remains `grok-build.acp`, driver
`swallowtail.grok-build.acp`, axis `grok-build.executable`, maintained exact
packages `1.0.4..=1.0.5`, model `grok-4.6`, ACP v1 stdio. Current argv is
exactly `grok --no-auto-update agent stdio`. Isolation stays `AmbientHost`.
Permission requests stay observe-and-stop and cancelled.

Platform `bin/grok.br` blobs were brotli-decompressed in a disposable
worktree-local directory using the same `zlib.brotliDecompressSync` path as the
wrapper postinstall. Extracted binaries were executed only from that directory
with `HOME`/`GROK_HOME` pointed at an empty isolated tree. Host
`~/.grok/bin/grok` was not invoked and was not rewritten.

Current public `xai-org/grok-build` (`SOURCE_REV`
`28439e8a8712c363321cf6ff0c2d70cd058d2a7d`) is later than 1.0.4 gitHead
`d846eb93d94d603191984d97f5d9f48170e93c6a` and 1.0.5 gitHead
`5115b46bc909ae5c7f5fc064455197440e796b6b`. Later files are corroboration only
where their strings match those binaries. Exact version source for those
gitHeads is not in the public filtered export.

The adapter, fixtures, and guide were inspected and not changed. No production
claim, public API, shared contract, or Contract 029 window movement follows.

## Frozen Sources

| Source | Use | Retrieved | Digest / identity |
| --- | --- | --- | --- |
| [CLI reference](https://docs.x.ai/build/cli/reference) HTML | `--no-subagents` listed among common session-disable flags | 2026-08-26 | SHA-256 `88af991df23e1cbf90290be06b44c09ecdee20abe4279b74ad7e6bc889c152b7` (410779-byte SPA) |
| [CLI reference markdown](https://docs.x.ai/build/cli/reference.md) | same page as stable text; SHA matches Research 204 | 2026-08-26 | SHA-256 `d6c944c885ac72a4f4d6036c1796537b795f805bff512c8a31a5f5f10932ee6e` |
| [Headless & Scripting](https://docs.x.ai/build/cli/headless-scripting) HTML | ACP example is `grok agent stdio`; no `--no-subagents` on the stdio example | 2026-08-26 | SHA-256 `3e4fc5f7a3ff7671499bb095e532fbf762a783a371409145fd514d64dd6c2a65` |
| [Headless & Scripting markdown](https://docs.x.ai/build/cli/headless-scripting.md) | same ACP example; SHA matches Research 204 | 2026-08-26 | SHA-256 `a4f39daf25f81aba5dba79265d12d0e4ec444e6b28dcbe3c2335c05401052097` |
| [Settings reference markdown](https://docs.x.ai/build/settings/reference.md) | `GROK_SUBAGENTS`; `[subagents] enabled` | 2026-08-26 | SHA-256 `e994c349043c6957553e2e6cb672ec314d82a45b3feb030a70603e9033c080dd` |
| changelog HTML `https://x.ai/build/changelog` | 403 from this environment | 2026-08-26 | SHA-256 `c6896d9e0764bef548782f549546adb38e5488565415ab139f128c88d173f88d` |
| [SOURCE_REV](https://raw.githubusercontent.com/xai-org/grok-build/main/SOURCE_REV) | public tree `28439e8a8712c363321cf6ff0c2d70cd058d2a7d`; later than 1.0.4/1.0.5 | 2026-08-26 | SHA-256 `7f7c34484dae30e6d66670930e5286bea043d70a780e6de04354790a27556551` |
| [16-subagents.md](https://raw.githubusercontent.com/xai-org/grok-build/main/crates/codegen/xai-grok-pager/docs/user-guide/16-subagents.md) | later disable docs name env/config, not `--no-subagents` | 2026-08-26 | SHA-256 `db62f14435140a5434c68e3fca740bd5972fa7d6ff99da4d95c441f3c90c4f70` |
| [14-headless-mode.md](https://raw.githubusercontent.com/xai-org/grok-build/main/crates/codegen/xai-grok-pager/docs/user-guide/14-headless-mode.md) | later additional-headless table includes `--no-subagents` | 2026-08-26 | SHA-256 `10912fab1fe5ea842cb143b412d642ef00d2f240484a061ebb65edbebb80edee` |
| [05-configuration.md](https://raw.githubusercontent.com/xai-org/grok-build/main/crates/codegen/xai-grok-pager/docs/user-guide/05-configuration.md) | later `GROK_SUBAGENTS` overlay | 2026-08-26 | SHA-256 `b956f0fefab7fa82a9284743cfc30f32311a8c357c01f112c232e23357b39459` |
| later `config/mod.rs` | `--subagents` enable-highest resolve; not exact package source | 2026-08-26 | SHA-256 `6406d1bd6b2523532448b7e1f993018ebbac63485f72177700e5e78f268a0b9e` |
| npm `@xai-official/grok@1.0.4` tarball | wrapper identity | 2026-08-26 | SHA-256 `03ec0e4d398b919844547a558d7f2296cf4ab45c847a82790456c2fe575091c6` |
| npm `@xai-official/grok@1.0.5` tarball | wrapper identity | 2026-08-26 | SHA-256 `667bd0b99c0318e39d8dc4af501d1658df9ece3e7a7e2bd87b012c0bb25f6f31` |
| npm `@xai-official/grok-darwin-arm64@1.0.4` tarball | exact platform artifact | 2026-08-26 | SHA-256 `8747ab3f649a7a8006dbebb6ab1bd2995083948363591d1b4913f86582a363f7` |
| npm `@xai-official/grok-darwin-arm64@1.0.5` tarball | exact platform artifact; SHA matches `grok-1-0-5/compatibility.json` | 2026-08-26 | SHA-256 `94b9c4ec7574ef37daabc2fdc5824c5fd86cc84561f2726635fb233f66655192` |
| decompressed 1.0.4 `grok` | exact executable | 2026-08-26 | SHA-256 `39366f7756a090b735cc1df8c93a8c0c3c7871555cf6cbb28f9351ca82936485` |
| decompressed 1.0.5 `grok` | exact executable; size 134349648 | 2026-08-26 | SHA-256 `3dfa7f04fbb5427a8fbead286591543aaecb478b3a0ab222c4329eca1a3b2f86` |

Wrapper integrity matches Research 129/163:

- `@xai-official/grok@1.0.4` `sha512-Nu3SFXTqwvCQr/LQFwrQYgngJhUQwX2h9ZSgzW4HowidjbPBWtMVO0xI88d2z6/zlDSNaT5YP/uk+2DthKQMsg==`
- `@xai-official/grok@1.0.5` `sha512-kk5hez+Oz5CvWonDGkMNmL483CWRIGRF2ki8jQzpIXH56P0fhCgaX9lrr0IUoFCKh/rYAm5vfCPgQsdIIYLu8Q==`

Platform integrity matches frozen fixtures:

- `@xai-official/grok-darwin-arm64@1.0.4` `sha512-ddb7tn+7ygDCpqGAsw1ZQkirePoPa7bm91wxWVxv9ePqIyrOiaDBluE3NMSjt2JMwqbcUbMtmg0CMKVb4N9oHw==`
- `@xai-official/grok-darwin-arm64@1.0.5` `sha512-akEtE93V7nOHEMfj16kkl3Nxl/AjcNaRkZCnY5HoIayy3VR6qQ6VTsQdUka4ZgT1bGJCZff4UIcTVpFGDaMAQQ==`

Extracted wrapper `bin/grok` and `bin/postinstall.js` are byte-identical across
1.0.4 and 1.0.5 (`13a24055…` / `e80f047f…`). Versioned identity is
`package.json` plus the platform binary.

HTML digests identify retrieved SPA shells. Markdown exports are the digestable
documentation corpus. Moving docs and later GitHub files do not qualify
delivery.

## Parser And Placement

Exact extracted help, isolated from host `~/.grok`:

| Case | 1.0.4 | 1.0.5 |
| --- | --- | --- |
| `--no-auto-update --version` | `grok 1.0.4 (d846eb93d94d)` | `grok 1.0.5 (5115b46bc909)` |
| root `--help` | `--no-subagents` / "Disable subagent spawning" | same |
| `--no-auto-update --no-subagents agent stdio --help` | exit 0; stdio help | exit 0; stdio help |
| `agent --no-subagents stdio --help` | unexpected argument | unexpected argument |
| `agent stdio --no-subagents --help` | unexpected argument | unexpected argument |
| `--no-subagents --no-subagents agent stdio --help` | cannot be used multiple times | cannot be used multiple times |
| `--subagents --help` | not re-run; 1.0.5 rejects with tip `--no-subagents` | unexpected argument; tip `--no-subagents` |
| `--no-subagents --agents '[]' --help` | not required for stop | exit 0; no parser conflict |
| `agent stdio --help` options | `--debug`, `--debug-file`, `--leader-socket`, `-h` | same |

`--no-subagents` is a root/global clap flag. Canonical Swallowtail placement
would be `grok --no-auto-update --no-subagents agent stdio`. The ACP
subcommand does not accept the flag. Repeats fail closed at parse. The old
enable flag `--subagents` is not a 1.0.4/1.0.5 clap option.

`agent stdio` help exposes no protocol option for the same control. Official
ACP docs still spawn `["agent", "stdio"]` with `session/new` `{cwd,
mcpServers:[]}`.

Root help also advertises `--agents <JSON>` ("Inline subagent definitions as
JSON") on the same parser. That combination parses. Parser coexistence is not
application precedence.

1.0.4 root help still lists `--no-memory` and `--experimental-memory`. 1.0.5
omits those two flags. That delta is recorded; it is not this family's
deliverable.

## Configuration, Env, And Later Source

Exact binaries contain `GROK_SUBAGENTS` (count 7) and user-guide text
`export GROK_SUBAGENTS=0`. Official settings markdown documents
`GROK_SUBAGENTS` and `[subagents] enabled`. `--no-subagents` help has no clap
`[env: GROK_SUBAGENTS=]` annotation; `GROK_SANDBOX` does.

Later public `SubagentsConfig::resolve` is not exact 1.0.4/1.0.5 source. It
says:

1. CLI flag `--subagents` (absolute highest — always enables)
2. `GROK_SUBAGENTS` env: `1`/`true` enables, `0`/`false` force-disables
3. config `[subagents]`
4. default enabled

and implements `cli_flag` as `Some(true)` or `None`. Exact 1.0.5 contains zero
`absolute highest` strings. Exact 1.0.4's one `absolute highest` string is the
memory flag (`--no-memory`), not subagents. Exact `--subagents` is a parse
error. Later resolve() therefore cannot be used as the 1.0.4/1.0.5 mapping
from `--no-subagents` to `enabled=false`.

Exact packages still have a config/env overlay: `config/mod.rs` debug events
and `invalid GROK_SUBAGENTS_MAX_DEPTH (expected integer); ignoring` are in both
binaries. Ambient `GROK_SUBAGENTS`, `~/.grok/config.toml`, plugins, and
`--agents` JSON remain live host surfaces under `AmbientHost`. Their
precedence versus `--no-subagents` is unfrozen from exact source.

## ACP Initialize Observation

Unauthorized stages (`authenticate`, `session/new`, prompt, tools) were not
run. Authorized unauthenticated `initialize` on extracted 1.0.4 and 1.0.5,
with and without `--no-subagents`, returned structurally identical results
after stripping volatile `agentInstanceId`. Stable subset:

- protocol version `1`
- `loadSession` / embedded context present
- isolated-home auth methods: `grok.com` only (`cached_token` absent because
  `GROK_HOME` was empty; host credentials were not read)
- `agentCapabilities._meta.x.ai/hooks.blockingEvents` includes `subagent_stop`
- `availableCommands` includes `deep-research` ("bounded parallel agents"),
  `workflow`, and `goal`
- no `spawn_subagent` name in the initialize body
- no flag-dependent field that names an applied disabled profile

Parser acceptance therefore reaches `agent stdio` startup. Initialize does not
expose whether `spawn_subagent` is registered, omitted, or still spawnable.
`subagent_stop` remains advertised with the flag present.

## Spawn Paths And Registry

Exact binaries contain `spawn_subagent` (count 17) and these compiled paths
on both versions:

- `crates/codegen/xai-grok-shell/src/agent/subagent/mod.rs`
- `crates/codegen/xai-grok-shell/src/agent/subagent/handle_request.rs`
- `crates/codegen/xai-grok-shell/src/agent/mvp_agent/subagent_coordinator.rs`
- `crates/codegen/xai-grok-shell/src/session/acp_session_impl/spawn.rs`
- `crates/codegen/xai-grok-pager/src/acp/spawn.rs`
- `crates/codegen/xai-grok-pager/src/app/subagent.rs`
- `crates/codegen/xai-grok-subagent-resolution/src/context.rs`

Clap field names `no_subagents` / `NO_SUBAGENTS` exist. Constructor proof that
the parsed flag clears every registration and spawn seam is not in the
binaries as a deterministic specimen. Later `acp_session_impl/spawn.rs`
threads a `subagents_enabled: bool` into agent rebuild; that file is not the
1.0.4/1.0.5 gitHead.

`--disallowed-tools Agent` is a separate headless denylist documented in later
user-guide text also embedded in the binaries. It is not `--no-subagents` and
was not selected.

## Claim Strength

| Stage | Exact finding |
| --- | --- |
| Requested restriction | not a Swallowtail input today |
| Argv dispatch | production remains `--no-auto-update agent stdio` |
| Parser acceptance | root `--no-subagents` accepted before `agent stdio` on 1.0.4 and 1.0.5 |
| Configuration application | env/config overlay exists; flag-vs-env-vs-file precedence unfrozen |
| Registry / tool absence | not observed; initialize has no tool table |
| Attempted spawn | not observed; would need a prompt or subagent tool |
| Provider behavior | not observed |
| OS process containment | not claimed; `AmbientHost` unchanged |

## Lifecycle Disposition

| Lifecycle | Disposition |
| --- | --- |
| Interactive `session/new` | not run; initialize does not expose the restriction |
| Operation-private structured run | same owned `start_attachment` argv path; same unproved application |
| Later prompt on the owned child | no initialize evidence that the profile is even applied at process start |
| Attachment recovery | current empty `SessionOptions`; no subagent field to restore |
| Fresh replacement | would re-emit current omitted argv |
| `--agents` JSON / plugins / agent profiles | parser-present; ambient under `AmbientHost` |
| Load / resume | advertised; unqualified; not implied by initialize |
| `UnverifiedNewer` | no inheritance; keep current omitted argv |

## Production Seam Audit

`GrokPreparationInput` / `GrokSessionProfileInput` have no subagent member.
`GrokAcpDriver::start_attachment` hard-codes
`["--no-auto-update", "agent", "stdio"]`. Fixtures
`grok-1-0-4/compatibility.json` and `grok-1-0-5/compatibility.json` freeze that
command. Tests assert the same argv. `SessionOptions` must stay empty.
Permission observe-and-stop is unchanged. Isolation is `AmbientHost`.
`direct_subagent_control` remains a private compatibility fixture token, not a
public Contract 045 claim.

Omission therefore retains exact current argv and behavior. An adapter-local
disabled enum/builder is nameable only after Research 219 admits a row. No
row is admitted.

## Version / Profile / Lifecycle Table

| Version | Lifecycle | Parser-accepted `--no-subagents` | Applied at ACP initialize | Spawn-path coverage | Ambient override frozen | Deliver-now |
| --- | --- | --- | --- | --- | --- | --- |
| `1.0.4` | structured run / interactive / replacement | yes, root only | no observable delta | unfrozen | no | no |
| `1.0.5` | structured run / interactive / replacement | yes, root only | no observable delta | unfrozen | no | no |
| `0.2.114..=0.2.117` | any | not this lane's corpus | n/a | n/a | n/a | no |
| `UnverifiedNewer` | any | no inheritance | n/a | n/a | n/a | no |

No row is deliver-now. The empty set is because exact 1.0.4/1.0.5 evidence
stops at parser acceptance plus an initialize body that does not change with
the flag. It is not because the flag is absent from clap, and not because
later GitHub `resolve(--subagents)` can be substituted for exact packages.

## Promotion

Research 219 promotes an empty deliver-now set.

Cards 199-200 stay blocked. A later lane may reopen this family only when exact
`1.0.4`/`1.0.5` (or a later qualified point) source or another secret-free
deterministic observation proves the parsed root flag is applied to every ACP
registry and spawn path for the owned child lifetime, without env/config/
`--agents`/plugin/session-metadata override, and without a provider prompt,
account inspection, tool/subagent execution, or paid work.

`--disallowed-tools Agent`, agent-definition selection, and Contract 045
observation/control remain out of scope.
