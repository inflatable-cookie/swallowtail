# 247 Grok Build ACP Web-Search Disable Evidence

Status: promoted; evidence stop
Owner: Tom
Created: 2026-08-28
Updated: 2026-08-28
Card: g04.088 / 250

## Question

Which exact maintained `grok-build.acp` version and lifecycle rows, if any,
can bind `--disable-web-search` as an immutable provider web-search restriction
with closed precedence, application, confirmation, replacement, and omission?

## Method And Boundary

Evidence was frozen on 2026-08-28 from current official public documentation,
exact published npm metadata and tarballs, exact decompressed darwin-arm64
binaries, isolated local help/parser cases, and unauthenticated no-prompt ACP
`initialize` only. No Grok install, host-binary replacement, login, account
inspection, credential capture, `authenticate`, `session/new`, provider prompt,
web search/fetch execution, or paid inference.

The selected operation remains `grok-build.acp`, driver
`swallowtail.grok-build.acp`, axis `grok-build.executable`, maintained exact
packages `1.0.4..=1.0.5`, model `grok-4.6`, ACP v1 stdio. Current argv is
exactly `grok --no-auto-update agent stdio`. Isolation stays `AmbientHost`.
Permission requests stay observe-and-stop and cancelled. The current search
matrix claim stays `No`.

Platform `bin/grok.br` blobs were brotli-decompressed in a disposable
worktree-local directory using the same `zlib.brotliDecompressSync` path as the
wrapper postinstall. Extracted binaries were executed only from that directory
with `HOME`/`GROK_HOME` pointed at an empty isolated tree. Host
`~/.grok/bin/grok` was not invoked and was not rewritten.

Current public `xai-org/grok-build` `SOURCE_REV`
`70ec060ec3d28e77b9c4593be43c2ab0128bcd21` is later than 1.0.4 gitHead
`d846eb93d94d603191984d97f5d9f48170e93c6a` and 1.0.5 gitHead
`5115b46bc909ae5c7f5fc064455197440e796b6b`. Later docs and strings are
corroboration only. Exact version source for those gitHeads is not in the
public filtered export; raw fetches of compiled crate paths at that
`SOURCE_REV` return 404.

The adapter, fixtures, and guide were inspected and not changed. No production
claim, public API, shared contract, or Contract 029 window movement follows.

Research 219 is the sibling stop pattern for `--no-subagents`: parser
acceptance without initialize delta or spawn-path coverage. This lane does not
repeat that inference as a positive search claim.

## Frozen Sources

| Source | Use | Retrieved | Digest / identity |
| --- | --- | --- | --- |
| [CLI reference](https://docs.x.ai/build/cli/reference) HTML | `--disable-web-search` among common session-disable flags | 2026-08-28 | SHA-256 `a9b082aeee2dfe954b172a305fc08d7c6563df906383f7f6789f07a8d2a40f21` (410943-byte SPA) |
| [CLI reference markdown](https://docs.x.ai/build/cli/reference.md) | same page as stable text; SHA matches Research 204/219 | 2026-08-28 | SHA-256 `d6c944c885ac72a4f4d6036c1796537b795f805bff512c8a31a5f5f10932ee6e` |
| [Headless & Scripting](https://docs.x.ai/build/cli/headless-scripting) HTML | ACP example is `grok agent stdio`; no `--disable-web-search` on the stdio example | 2026-08-28 | SHA-256 `647cc8efcf9cf03d7689e3641f664e68128d55d729748730eff1eb33aacd4f3c` |
| [Headless & Scripting markdown](https://docs.x.ai/build/cli/headless-scripting.md) | same ACP example; SHA matches Research 204/219 | 2026-08-28 | SHA-256 `a4f39daf25f81aba5dba79265d12d0e4ec444e6b28dcbe3c2335c05401052097` |
| [Settings reference markdown](https://docs.x.ai/build/settings/reference.md) | `GROK_WEB_SEARCH_MODEL`; `[models] web_search`; no disable-env for the flag | 2026-08-28 | SHA-256 `e994c349043c6957553e2e6cb672ec314d82a45b3feb030a70603e9033c080dd` |
| [SOURCE_REV](https://raw.githubusercontent.com/xai-org/grok-build/main/SOURCE_REV) | public tree tip `70ec060ec3d28e77b9c4593be43c2ab0128bcd21`; later than 1.0.4/1.0.5 | 2026-08-28 | SHA-256 `684ef6056e4429f7e554592be90139da30507bef81ff7d4202fabfd400f5e67e` |
| later `14-headless-mode.md` via `main` | additional-headless table: `--disable-web-search` "Disable web search and fetch tools" | 2026-08-28 | SHA-256 `333f8ccfb66e4a97019777788ed404c20cb305339063c63affe501147e8eac26` |
| later `05-configuration.md` via `main` | `[toolset.web_search]` domain policy; `GROK_WEB_SEARCH_MODEL`; backend vs client search | 2026-08-28 | SHA-256 `781c2b46bc68d153b8d9dc5843acce3634b06120aaf646bbd79b5ef40d21cea0` |
| npm `@xai-official/grok@1.0.4` tarball | wrapper identity | 2026-08-28 | SHA-256 `03ec0e4d398b919844547a558d7f2296cf4ab45c847a82790456c2fe575091c6` |
| npm `@xai-official/grok@1.0.5` tarball | wrapper identity | 2026-08-28 | SHA-256 `667bd0b99c0318e39d8dc4af501d1658df9ece3e7a7e2bd87b012c0bb25f6f31` |
| npm `@xai-official/grok-darwin-arm64@1.0.4` tarball | exact platform artifact | 2026-08-28 | SHA-256 `8747ab3f649a7a8006dbebb6ab1bd2995083948363591d1b4913f86582a363f7` |
| npm `@xai-official/grok-darwin-arm64@1.0.5` tarball | exact platform artifact; SHA matches `grok-1-0-5/compatibility.json` | 2026-08-28 | SHA-256 `94b9c4ec7574ef37daabc2fdc5824c5fd86cc84561f2726635fb233f66655192` |
| decompressed 1.0.4 `grok` | exact executable | 2026-08-28 | SHA-256 `39366f7756a090b735cc1df8c93a8c0c3c7871555cf6cbb28f9351ca82936485` |
| decompressed 1.0.5 `grok` | exact executable; size 134349648 | 2026-08-28 | SHA-256 `3dfa7f04fbb5427a8fbead286591543aaecb478b3a0ab222c4329eca1a3b2f86` |

Wrapper integrity matches Research 129/163/219:

- `@xai-official/grok@1.0.4` `sha512-Nu3SFXTqwvCQr/LQFwrQYgngJhUQwX2h9ZSgzW4HowidjbPBWtMVO0xI88d2z6/zlDSNaT5YP/uk+2DthKQMsg==`
- `@xai-official/grok@1.0.5` `sha512-kk5hez+Oz5CvWonDGkMNmL483CWRIGRF2ki8jQzpIXH56P0fhCgaX9lrr0IUoFCKh/rYAm5vfCPgQsdIIYLu8Q==`

Platform integrity matches frozen fixtures:

- `@xai-official/grok-darwin-arm64@1.0.4` `sha512-ddb7tn+7ygDCpqGAsw1ZQkirePoPa7bm91wxWVxv9ePqIyrOiaDBluE3NMSjt2JMwqbcUbMtmg0CMKVb4N9oHw==`
- `@xai-official/grok-darwin-arm64@1.0.5` `sha512-akEtE93V7nOHEMfj16kkl3Nxl/AjcNaRkZCnY5HoIayy3VR6qQ6VTsQdUka4ZgT1bGJCZff4UIcTVpFGDaMAQQ==`

Extracted wrapper `bin/grok` and `bin/postinstall.js` remain byte-identical
across 1.0.4 and 1.0.5 (`13a24055…` / `e80f047f…`). Versioned identity is
`package.json` plus the platform binary.

HTML digests identify retrieved SPA shells. Markdown exports are the digestable
documentation corpus. Moving docs and later GitHub files do not qualify
delivery.

Lane-local frozen summary:
`crates/swallowtail-adapter-grok/tests/fixtures/g04-088c-web-search-disable/`.

## Parser And Placement

Exact extracted help, isolated from host `~/.grok`:

| Case | 1.0.4 | 1.0.5 |
| --- | --- | --- |
| `--no-auto-update --version` | `grok 1.0.4 (d846eb93d94d)` | `grok 1.0.5 (5115b46bc909)` |
| root `--help` | `--disable-web-search` / "Disable web search and web fetch tools" | same |
| `--no-auto-update --disable-web-search agent stdio --help` | exit 0; stdio help | exit 0; stdio help |
| `agent --disable-web-search stdio --help` | unexpected argument | unexpected argument |
| `agent stdio --disable-web-search --help` | unexpected argument | unexpected argument |
| `--disable-web-search --disable-web-search agent stdio --help` | cannot be used multiple times | cannot be used multiple times |
| `--enable-web-search --help` | not required on 1.0.4 | unexpected argument; tip `--disable-web-search` |
| `--no-auto-update --disable-web-search --disallowed-tools web_search,web_fetch agent stdio --help` | not required for stop | exit 0; no parser conflict |
| `agent stdio --help` options | `--debug`, `--debug-file`, `--leader-socket`, `-h` | same |

`--disable-web-search` is a root/global clap flag (`disable_web_search` /
`DISABLE_WEB_SEARCH`). Canonical Swallowtail placement would be
`grok --no-auto-update --disable-web-search agent stdio`. The ACP subcommand
does not accept the flag. Repeats fail closed at parse. There is no
`--enable-web-search` clap option on 1.0.5.

`agent stdio` help exposes no protocol option for the same control. Official
ACP docs still spawn `["agent", "stdio"]` with `session/new` `{cwd,
mcpServers:[]}`.

Root help annotates `[env: GROK_SANDBOX=]` on the sandbox profile flag. The
`--disable-web-search` help block has no clap `[env: …]` annotation.
`GROK_SANDBOX` remains a separate host-network/filesystem control and is not
this family's deliverable.

`--disallowed-tools` is a separate headless denylist surface. Parser
coexistence with `--disable-web-search` is not application precedence and is
not selected here.

## Configuration, Env, Paths, And Later Docs

Exact binaries contain:

- `GROK_WEB_SEARCH` count 4 (including `export GROK_WEB_SEARCH_MODEL=…`)
- `web_search` / `web_fetch` tool paths under
  `crates/codegen/xai-grok-tools/src/implementations/…`
- clap field `disable_web_search`
- config-shaped token `disable_web_search` adjacent to `AgentTypeConfig`
- `DISABLE_EMBEDDED_SEARCH_TOOLS` near
  `util/config/resolve/toolset.rs`
- runtime strings `web_search disabled: resolved config has no API key` and
  `web_search disabled: configured model could not be resolved` (model/key
  resolution failures, not CLI-flag application proof)

Official settings markdown documents `GROK_WEB_SEARCH_MODEL` and
`[models] web_search`. It does not document a disable env paired to
`--disable-web-search`.

Later headless docs say the flag "Disable[s] web search and fetch tools" /
"Remove[s] web search tool from the agent toolset". Later configuration docs
separately describe:

- client `web_search` / `web_fetch` tools
- backend/server-side inline web search for models with hosted search
- `[toolset.web_search]` domain allow/block policy applying to both

Exact 1.0.4/1.0.5 source that maps the parsed root flag onto every client tool
registration and every backend search construction path is not available from
the public export. Ambient `GROK_WEB_SEARCH_MODEL`, `~/.grok/config.toml`,
plugins, agent profiles, and `--disallowed-tools` remain live under
`AmbientHost`. Flag-versus-env-versus-file-versus-backend precedence is
unfrozen from exact package source.

## ACP Initialize Observation

Unauthorized stages (`authenticate`, `session/new`, prompt, tools, search)
were not run. Authorized unauthenticated `initialize` on extracted 1.0.4 and
1.0.5, with and without `--disable-web-search`, returned structurally
identical results after scrubbing volatile `agentInstanceId` and normalizing
JSON key order.

Stable subset (both versions, both argv shapes):

- protocol version `1`
- `loadSession` / embedded context present
- isolated-home auth methods: `grok.com` only (`cached_token` absent because
  `GROK_HOME` was empty; host credentials were not read)
- `agentCapabilities._meta.x.ai/hooks.blockingEvents` includes `subagent_stop`
- `_meta.availableCommands` includes `deep-research`, `workflow`, and `goal`
- `_meta.modelState.currentModelId` is `grok-4.6`
- no `web_search` / `web_fetch` tool table in the initialize body
- no flag-dependent field that names an applied disabled search profile

Canonical initialize digests after scrub (SHA-256 of sorted JSON):

- 1.0.4 plain == 1.0.4 disabled:
  `c482c21415bf0dfe3f41898243a2a64fb8d06d9770ce999fa5b760eccf74b8db`
- 1.0.5 plain == 1.0.5 disabled:
  `2488f9480fa9dc77111c89eefcf0be384a91c817d3bc032773ac429d866a9b50`

Parser acceptance therefore reaches `agent stdio` startup. Initialize does not
expose whether `web_search` / `web_fetch` are registered, omitted, still
callable, or whether backend hosted search remains available.

## Registry And Search Paths

Exact binaries contain compiled web-search/fetch paths on both versions,
including:

- `crates/codegen/xai-grok-tools/src/implementations/grok_build/web_search/mod.rs`
- `crates/codegen/xai-grok-tools/src/implementations/grok_build/web_fetch/…`
- `crates/codegen/xai-grok-tools/src/implementations/web_search/…`
- pager/headless reducers and UI blocks for web search

Clap field names `disable_web_search` / `DISABLE_WEB_SEARCH` exist.
Constructor proof that the parsed flag clears every client tool registration
and every backend search seam is not in the binaries as a deterministic
specimen. Later public docs that say "remove … from the agent toolset" are not
exact 1.0.4/1.0.5 gitHead source.

`--disallowed-tools web_search,web_fetch` is a separate denylist. It was not
selected and does not close this family's application or confirmation gate.

## Claim Strength

| Stage | Exact finding |
| --- | --- |
| Requested restriction | not a Swallowtail input today |
| Argv dispatch | production remains `--no-auto-update agent stdio` |
| Parser acceptance | root `--disable-web-search` accepted before `agent stdio` on 1.0.4 and 1.0.5 |
| Configuration application | web-search model/env/config overlay exists; flag-vs-env-vs-file-vs-backend precedence unfrozen |
| Registry / tool absence | not observed; initialize has no tool table and no flag delta |
| Provider search behavior | not observed; would need a prompt or search/tool execution |
| Host networking / containment | not claimed; `AmbientHost` unchanged; flag ≠ `GROK_SANDBOX` |

## Lifecycle Disposition

| Lifecycle | Disposition |
| --- | --- |
| Interactive `session/new` | not run; initialize does not expose the restriction |
| Operation-private structured run | same owned `start_attachment` argv path; same unproved application |
| Later prompt on the owned child | no initialize evidence that the profile is applied at process start |
| Attachment recovery | current empty `SessionOptions`; no web-search field to restore |
| Fresh replacement | would re-emit current omitted argv |
| `--disallowed-tools` / plugins / agent profiles / config | parser- or config-present; ambient under `AmbientHost` |
| Load / resume | advertised; unqualified; not implied by initialize |
| `UnverifiedNewer` | no inheritance; keep current omitted argv |

## Production Seam Audit

`GrokPreparationInput` / `GrokSessionProfileInput` have no web-search member.
`GrokAcpDriver::start_attachment` hard-codes
`["--no-auto-update", "agent", "stdio"]`. Fixtures
`grok-1-0-4/compatibility.json` and `grok-1-0-5/compatibility.json` freeze that
command. Tests assert the same argv. `SessionOptions` must stay empty.
Permission observe-and-stop is unchanged. Isolation is `AmbientHost`. Driver
validation already requires `ExternalSearchPolicy::Disabled` for prepared runs;
that is the current matrix `No` claim, not a `--disable-web-search` binding.

Omission therefore retains exact current argv and the matrix's current `No`
search claim. An adapter-local disabled enum/builder is nameable only after
Research 247 admits a row. No row is admitted.

## Version / Profile / Lifecycle Table

| Version | Lifecycle | Parser-accepted `--disable-web-search` | Applied at ACP initialize | Search-path coverage | Ambient override frozen | Host-network claim | Deliver-now |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `1.0.4` | structured run / interactive / replacement | yes, root only | no observable delta | unfrozen | no | no | no |
| `1.0.5` | structured run / interactive / replacement | yes, root only | no observable delta | unfrozen | no | no | no |
| `0.2.114..=0.2.117` | any | not this lane's corpus | n/a | n/a | n/a | n/a | no |
| `UnverifiedNewer` | any | no inheritance | n/a | n/a | n/a | n/a | no |

No row is deliver-now. The empty set is because exact 1.0.4/1.0.5 evidence
stops at parser acceptance plus an initialize body that does not change with
the flag, while client-tool and backend-search application remain unfrozen.
It is not because the flag is absent from clap, and not because later docs that
say "remove … from the toolset" can be substituted for exact package source.

## Promotion

Research 247 promotes an empty deliver-now set.

Production binding stays blocked. A later lane may reopen this family only when
exact `1.0.4`/`1.0.5` (or a later qualified point) source or another secret-free
deterministic observation proves the parsed root flag is applied to every owned
ACP client web-search/fetch registration and every backend search construction
path for the child lifetime, without ambient override, and without a provider
prompt, account inspection, search execution, paid work, install/update, or a
host-network containment claim.

`--disallowed-tools`, sandbox/`GROK_SANDBOX`, reasoning, model selection, and
subagents remain out of scope.
