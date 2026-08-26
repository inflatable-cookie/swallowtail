# 218 Copilot CLI ACP Built-In Tool Allowlist Evidence

Status: promoted; evidence stop
Owner: Tom
Created: 2026-08-26
Updated: 2026-08-26
Card: g04.071 / 195

## Question

Which exact Copilot CLI `1.0.80` built-in tool-allowlist rows can Swallowtail
bind at ACP child startup as a closed adapter-local `--available-tools`
selection without raw tool strings, ambient registry inference, permission
widening, or a false isolation claim?

## Method And Boundary

Evidence was collected on 2026-08-26 with no Copilot install, login, account
inspection, native-binary execution, ACP initialize, provider prompt, tool
invocation, or paid inference. Official GitHub documentation was retrieved as
a lead. Exact `@github/copilot@1.0.80` wrapper and
`@github/copilot-darwin-arm64@1.0.80` artifacts were downloaded to a disposable
worktree-local directory and digested as the binding corpus.

The platform tarball was extracted only far enough to read `package.json`,
`app.js`, bundled `copilot-sdk` types/JS, `changelog.json`, and to inventory
printable strings in `cli-native.node` and `runtime.node`. The `copilot`
executable was not extracted for execution and was not run.

The route remains `copilot-cli.acp`, driver `swallowtail.copilot-cli.acp`,
axis `copilot-cli.package` `1.0.80`, behavior `copilot-cli.acp.stdio-v1`.
Current argv is exactly `copilot --acp --stdio`. Isolation stays `AmbientHost`.
Permission requests stay observe-and-stop and cancelled.

Current official ACP-server documentation is a lead. Exact `1.0.80` `app.js`
is the package finding for parser and ACP startup. Native filter membership,
unknown-name diagnostics, and default `toolFilterPrecedence` live behind FFI
and were not executed.

The adapter, fixtures, and guide were inspected and not changed. No production
claim, public API, shared contract, or Contract 029 window movement follows.

## Frozen Sources

| Source | Use | Retrieved | Digest / identity |
| --- | --- | --- | --- |
| [ACP server](https://docs.github.com/en/copilot/reference/copilot-cli-reference/acp-server) `.md` | server-start `--available-tools`, session inheritance, example `bash,view` | 2026-08-26 | SHA-256 `71b0c24e0b0a0200950c74077b23300e529cd0082d814c2c08724eee5bb92845` (13919 bytes) |
| same URL HTML shell | corroboration only | 2026-08-26 | SHA-256 `f3f74f95f0ea822c469edbd35808d097c004fd88d720b906b6429ab2612ae09d` (697406-byte SPA) |
| [Allowing tools](https://docs.github.com/en/copilot/how-tos/copilot-cli/use-copilot-cli/allowing-tools) `.md` | availability vs permission layers; docs say allowlist ignores denylist | 2026-08-26 | SHA-256 `bbc5081d6e58ce5f26081d65b8b495399e6365f09c2eb5e0cde44ef5d3b80f36` (11023 bytes) |
| same URL HTML shell | corroboration only | 2026-08-26 | SHA-256 `332069a0ffd032f473647e11998b62ceafb538a83bafd709a9f5288993e2f267` (615330-byte SPA) |
| [CLI command reference](https://docs.github.com/en/copilot/reference/copilot-cli-reference/cli-command-reference) `.md` | documented `--available-tools` values and `--allow-tool` kinds | 2026-08-26 | SHA-256 `2f85b3402691207cec9f6c19b9f71df2e599f69a18e8ffbff4d612781c008ea4` (347937 bytes) |
| same URL HTML shell | corroboration only | 2026-08-26 | SHA-256 `0ce87c8d0ba091c15e4cb0d2daf96443120a801f83962420eefd6c6fe63e30eb` (1631763-byte SPA) |
| npm `@github/copilot@1.0.80` wrapper tarball | identity reconfirm | 2026-08-26 | SHA-256 `799457937f8f87de6fdc95599380de5f5a0f761ab2fdfbba7f8d1c82d2988892` |
| npm `@github/copilot-darwin-arm64@1.0.80` tarball | exact package source | 2026-08-26 | SHA-256 `98640ca0de6576807f369c533c839b5742b038f105a970bdd7cb0d7efc8a7a71` |
| extracted wrapper `package.json` | version `1.0.80`; `buildMetadata.gitCommit` `a3a2697` | 2026-08-26 | SHA-256 `ea1e51458998a9ee87379137fa7e10dc38467596a0d8792acdcf76eea8401c88` |
| extracted `package/app.js` | commander, tokenizer, ACP `session/new`, MCP load, permission configure | 2026-08-26 | SHA-256 `fa438a4959c3f8b44c123dd95cc0d8b0760faa055a458f81ddd0dcf8a2ae8f58` |
| extracted `package/package.json` | platform `1.0.80` | 2026-08-26 | SHA-256 `9faf9b290dc9a99b66aaad8d6b06fa2154c153e032cc5d259cf4e3a730f0c476` |
| extracted `copilot-sdk/index.js` | `ToolSet` prefixes; `BuiltInTools.Isolated`; empty-mode excluded-wins | 2026-08-26 | SHA-256 `c5457b7460a3b3de517b24a5f7f745cdfa106d1234ea3b6819bb4274fe60c7a5` |
| extracted `copilot-sdk/types.d.ts` | bare names match any source; excluded always wins in SDK session config | 2026-08-26 | SHA-256 `9233c40bf06500bd13cba1e3ea9469e4cdb2d7eb03f46aa47291369f28c0f4d3` |
| extracted `copilot-sdk/toolSet.d.ts` | built-in vs MCP vs custom classified at registration, not name parse | 2026-08-26 | SHA-256 `41e53ae25809c90fd1a78f493cb912c6a9194c6e0a02e3e32545971c41a1cb56` |
| extracted `changelog.json` | ACP inherits `--available-tools` at server start | 2026-08-26 | SHA-256 `79192e19dca082ad3d26937189f2779c4a2746be0f0786d7cf2967bdb5faa693` |
| `cli-native.node` string inventory | no tool-id table | 2026-08-26 | SHA-256 `6b6afa7d79a16e1d649a3c8448a60310d68739b09d89b3566b6eb77d753966ed` |
| `runtime.node` string inventory | concatenated native names; `unknown tool: `; `builtin:` | 2026-08-26 | SHA-256 `58e504c1203a3b562c36125588047e7611105d816aee15057fe92440836e0f8b` |

Wrapper integrity matches Research 149/188:
`sha512-6tf93ZF56KOiTTAjK/UhLZkl1W543IzaTQly288kockJZFswpRTnQEI00Yvacpb39DTvTYu3/ha9SeKpo/pgZQ==`.
Platform optional `@github/copilot-darwin-arm64@1.0.80` integrity remains
`sha512-fzn4PnSx3+O/a3ip72KVsjnzORsEygK+0i21bFAnFBYS+0Wi1Pk+o/CmNsJ7aRbf1enSJrcH8UDVkyc9pMGEBg==`.
`app.js` SHA-256 matches Research 188.

HTML digests identify retrieved SPA shells. They are not the converted text.
Markdown exports are the digestable documentation corpus. Moving docs do not
qualify delivery.

## Exact Package Specimens

Commander registers a variadic optional option, not a required arity-1 string:

```js
.option("--available-tools [tools...]","Only these tools will be available to the model")
.option("--excluded-tools [tools...]","These tools will not be available to the model")
```

There is no collect `argParser`. `--plugin-dir` concatenates repeats; these
flags do not.

ACP startup tokenizes through `T5` then `xW`:

```js
T5=t=>t.flatMap(e=>{let n=[],r="",o=0;for(let a of e)if(a==="("?o++:a===")"&&o--,a===","&&o===0){let l=r.trim();l&&n.push(l),r=""}else r+=a;let s=r.trim();return s&&n.push(s),n})
xW=t=>{if(t===void 0||t===!0)return;let e=T5((Array.isArray(t)?t:[t]).filter(n=>typeof n=="string"&&n.length>0));return e.length>0?e:void 0}
```

ACP applies that result at `session/new` / `session/load`, not as a
`session/new` field:

```js
resolveInitialToolFilters(){return{availableTools:xW(this.options.options?.availableTools),excludedTools:xW(this.options.options?.excludedTools)}}
// session/new:
{availableTools:u,excludedTools:g}=this.resolveInitialToolFilters()
await Hd(this.sessionManager,{...,clientKind:"acp",...,availableTools:u,excludedTools:g,...})
```

`session/new` still reads client `cwd` and `mcpServers` only. That matches
Research 149 and the official lead. `session/set_config_option` handles
`mode`, `model`, `reasoning_effort`, `allow_all`, and `agent`. It has no
available-tools case; unknown ids fail with `Unknown config option`.

Unknown filter names are not a spawn failure. JS `validateToolFilters` calls
native `sessionPlanToolFilterDiagnosticsForSessionJson`, then emits
`session.info` and records `markWarned`. Native strings include `unknown tool: `
and `Unknown tool name in the `.

Filtering itself is native:

```js
isToolEnabled(e){return an(h.sessionFilterEnabledToolIndexesJson(JSON.stringify({tools:[e],availableTools:...,excludedTools:...,precedence:...}))).includes(0)}
```

CLI ACP does not set `toolFilterPrecedence`. Bundled SDK empty-mode defaults
it to `"excluded"` and types.d.ts says excluded always wins. Official allowing-
tools docs say `--available-tools` ignores `--excluded-tools`. That
contradiction is unresolved without executing native.

ACP still loads host MCP config at server start (`Base MCP config prepared for
ACP mode`), auto-sets `github-mcp-server` unless the user already named it, and
merges client `mcpServers` on top. Swallowtail sends `mcpServers: []`; `hlt`
treats an empty array as omitted, so host MCP config remains. Plugins come from
`additionalPlugins` / installed plugins. `disableBuiltinMcps` is a separate
unselected flag.

Permission configure on the same `session/new` path is a different layer:

```js
await m.permissions.configure({approveAllToolPermissionRequests:o,approveAllReadPermissionRequests:!0,rules:c,...})
this.wirePermissionHandling(m,o)
```

`--allow-tool` / `--yolo` / `--allow-all` feed that layer. They are out of
scope. Swallowtail still observes `session/request_permission` and returns
`cancelled`.

SDK `ToolSet.addBuiltIn` prefixes `builtin:<name>`. CLI `xW` does not. Bare
CLI tokens are the form official docs and commander produce. SDK types say a
bare name is an exact match across any source.

`BuiltInTools.Isolated` is an SDK empty-mode curated list
(`ask_user`, `task_complete`, `exit_plan_mode`, `task`, `read_agent`,
`write_agent`, `list_agents`, `send_inbox`, `context_board`, `skill`). It is
not the CLI `--available-tools` table and is not a Swallowtail isolation
claim.

## Syntax And Lifetime

| Item | Exact 1.0.80 finding | Disposition |
| --- | --- | --- |
| Canonical flag | `--available-tools [tools...]` | documented `--available-tools=TOOL ...` is a lead, not the commander arity |
| Value syntax | commander variadic optional; `T5` comma-splits tokens at paren depth 0 and trims | `--available-tools=bash,view` and `--available-tools bash view` both tokenize in JS |
| Empty / bare flag | `undefined` or `true` → `xW` returns omitted | `--available-tools` alone is not a closed empty set; it is no filter |
| Empty after split | `""`, `","`, whitespace-only → omitted | fail-open as current argv semantics |
| Case / aliases | no JS lowercasing or alias map in `T5`/`xW` | `rg` vs `grep` is documentation prose, not a tokenizer rule |
| Duplicates | `T5` does not unique | passed through to native |
| Unknown names | JS warns via `session.info`; does not fail spawn | Swallowtail cannot reject before spawn without a frozen membership table |
| Repeats | no collect parser | JS does not concatenate; last-wins is commander default and was not executed |
| `--excluded-tools` | same tokenizer; evidence only | not selected |
| Precedence | CLI ACP does not set `toolFilterPrecedence`; docs vs SDK disagree | unfrozen |
| `session/new` | cwd and MCP servers only | no per-session argv substitute |
| First prompt | inherits server-start filters stored on the session | registry may still grow from MCP/plugins after start |
| Later prompt | same process | same ambient registry risk |
| Fresh replacement | new child would re-pass the same argv | host `~/.copilot` MCP/plugins still load |
| Permission flags | `--allow-tool` / `--deny-tool` / `--yolo` | separate layer; not this family |
| `--yolo` / `--allow-all` | ACP `allow_all` config option also exists | not Swallowtail authority |

## Identifier And Registry Disposition

Official CLI reference lists `--available-tools` values including `bash` /
`powershell`, `list_bash` / `list_powershell`, `view`, `edit`, `create`,
`apply_patch`, `glob`, `grep` (or `rg`), `skill`, `web_fetch`, `ask_user`,
`task`, and agent tools. That table is documentation prose.

Exact `app.js` has no quoted `list_bash` / `list_powershell`. Native
`runtime.node` concatenates `read_bashstop_bashlist_bash`. JS display switches
also name `web_search`, `local_shell`, `task_complete`, and
`github-mcp-server-web_search`. Allowing-tools docs exclude `web_search` as if
it were a built-in.

No closed built-in identifier set can be frozen from JS plus unread native
without executing the binary. Source-qualified `builtin:<name>` is an SDK
filter language, not the CLI argv form Swallowtail would emit.

ACP registry composition is not the documented built-in table:

- host MCP config loads in ACP mode even when the client sends `mcpServers: []`
- built-in `github-mcp-server` is set up unless the user already named it
- installed plugins can contribute tools
- skills, custom agents, and extensions are separate registry sources
- `AmbientHost` plus `~/.copilot` cannot be bound by current preparation

A useful closed subset has to stay stable with extensions, MCP, user
configuration, model, and account absent. Exact `1.0.80` does not prove that
for any documented name.

## Permission, Invocation, And Isolation

| Claim | Strength |
| --- | --- |
| `--available-tools` is a model-visible tool filter | observed in JS help text and native filter call |
| filter membership is permission | false; `--allow-tool` / `permissions.configure` are separate |
| Swallowtail observe-and-stop remains | observed in adapter `reject_permission` → `cancelled` |
| allowlist grants one-shot or persistent approval | false; not selected |
| `approveAllReadPermissionRequests: true` on ACP session create | observed; still not Swallowtail permission authority |
| `view` / `glob` / `grep` are filesystem-free | false even if labelled read |
| `bash` / `web_fetch` / `task` are process/network/subagent | observed in docs and JS display names |
| `BuiltInTools.Isolated` is a Swallowtail isolation profile | false; SDK empty-mode list, includes `task`/`skill` |
| `AmbientHost` changes if a filter is passed | false |

Requested restriction, startup argv, parser acceptance, registry filtering,
permission request, invocation, effect, and terminal outcome stay distinct.
This record does not claim Copilot honors a flag at runtime.

## Production Seam Audit

| Seam | Current exact `1.0.80` route | Allowlist fit |
| --- | --- | --- |
| Prepared input | package, host-approved executable, host-account access; no tool field | none |
| Plan / evidence | immutable ACP interactive session; `AmbientHost`; `Ambient` config | none |
| Child argv | `["--acp", "--stdio"]`; `--available-tools` listed unmapped | omission is current |
| `session/new` | `{cwd, mcpServers: []}` | no filter field; host MCP still loads |
| Permission | observe-and-stop; cancel; no `allow_always` | preserve |
| Replacement | fresh context-losing session with same plan/request | would re-pass argv; would not bind host MCP/plugins |
| Fixtures | `available-tools-unmapped` | retain |
| Guide | no server-start tool flags | retain |
| Isolation | `AmbientHost` | cannot become `ProviderEnforced` from a filter |

The only candidate public shape remains a closed adapter-local profile or
typed frozen identifier set. Exact evidence admits neither. Raw strings and
shared provider-tool vocabulary stay forbidden.

## Claim Strength

| Claim | Strength at exact evidence boundary |
| --- | --- |
| `--available-tools` exists on exact `1.0.80` commander | observed in `app.js` |
| tokenizer is `T5` + `xW`; bare/empty collapses to omit | observed in `app.js` |
| ACP `session/new` inherits server-start filters | observed in `app.js`; changelog agrees |
| unknown names fail closed before spawn | false in JS; native warns |
| documented identifier table is the exact built-in set | unproved; `list_bash` absent from `app.js` |
| bare name matches only built-ins | false in SDK types; CLI emits bare names |
| `mcpServers: []` disables host MCP and github MCP | false in ACP MCP load path |
| available-tools wins over excluded-tools | docs lead; SDK/native default unfrozen |
| passing `--available-tools=view` proves only `view` is effective | unproved; live prompt forbidden |
| filter is filesystem/network/process containment | false |
| filter is permission | false |

## Deliver-Now Table

| Row | Exact evidence | Disposition |
| --- | --- | --- |
| documented `bash,view` / `bash,edit,view,grep,glob` | docs lead; membership, aliases, and ambient registry unfrozen | withheld |
| any single documented built-in name | no closed JS table; bare names match any source | withheld |
| SDK `builtin:<name>` / `BuiltInTools.Isolated` | SDK session config, not CLI argv | not applicable |
| `--excluded-tools` denylist | out of scope; precedence unfrozen | rejected |
| raw consumer tool strings | forbidden | rejected |
| omitted `--available-tools` | current Swallowtail argv | retain current absent path |
| permission / yolo / allow-all | separate dangerous layer | not applicable |
| isolation from tool filtering | Contract 023 provider-native behavior only | not applicable |

Deliver-now rows: **none**.

No new private behavior, guide capability claim, matrix row, or production
binding follows. Cards 196 and 197 remain blocked.

## Decision

Card 195 is complete as an evidence stop. Exact `1.0.80` can parse
`--available-tools` into a string list and store that list on the ACP session
at child start. It cannot give Swallowtail a closed built-in identifier set
whose membership and effect are independent of host MCP, plugins, extensions,
unknown-name warning, or unread native precedence.

Portable dispatch would need raw names, ambient registry inference, or
executing the native binary. All three are forbidden. Mapping any documented
subset would over-claim filter effectiveness and isolation.

The existing one-child/one-session topology could keep a spawn argv fixed
across first prompt, later prompts, and fresh replacement. That lifetime fit
does not rescue the identifier, registry, unknown-name, or precedence gaps.

No facade revision, behavior-segment split, or contract change is proposed.
The current `copilot --acp --stdio` boundary stands.

Cards 196 and 197 are blocked and were not executed. Keep g04 open. A later
lane may reopen this family only with an exact built-in table and fail-closed
unknown handling that do not depend on ambient host MCP/plugins, or with an
upstream interface that accepts source-qualified built-ins without executing
native.

`--excluded-tools`, TCP, permissions, login, BYOK, model selection, effort,
and session load/resume remain out of scope.
