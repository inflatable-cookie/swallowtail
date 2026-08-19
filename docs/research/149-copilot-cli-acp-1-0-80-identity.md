# 149 Copilot CLI ACP 1.0.80 Identity

Status: promoted
Owner: Tom
Date: 2026-08-18
Card: g03 batch 270

## Question

Is official GitHub Copilot CLI `1.0.80` ACP a distinct stdio wire that can
freeze initialize plus one bounded `session/prompt` without flattening onto
TCP `--port`, interactive-only slash commands, `--yolo` / allow-all, or
GitHub account policy?

## Method

Reconciled Research 144/145 with official ACP server docs, npm
`@github/copilot@1.0.80` metadata and wrapper tarball, GitHub tag
`v1.0.80`, the ACP registry entry `github-copilot-cli` `1.0.80`, and the
tagged public changelog.

Downloaded the 4-file npm wrapper tarball. Did not install Copilot. Did
not download a platform binary. Did not log in, send `initialize`, or send
a prompt. Host PATH has no `copilot`.

Ignored npm dist-tag `prerelease` `1.0.81-1` and GitHub prerelease tags
`v1.0.81-0` / `v1.0.81-1`. Observed versions are not qualified claims.

## Identity

| Surface | Value |
| --- | --- |
| Route | `copilot-cli.acp` |
| Axis (provisional) | `copilot-cli.package` |
| Package (provisional) | `swallowtail-adapter-copilot-cli` |
| Maturity | public preview; must remain visible |
| npm | `@github/copilot@1.0.80`, published 2026-08-14T02:30:35.923Z, `latest` on 2026-08-18 |
| Wrapper tarball SHA-256 | `799457937f8f87de6fdc95599380de5f5a0f761ab2fdfbba7f8d1c82d2988892` |
| Wrapper integrity | `sha512-6tf93ZF56KOiTTAjK/UhLZkl1W543IzaTQly288kockJZFswpRTnQEI00Yvacpb39DTvTYu3/ha9SeKpo/pgZQ==` |
| Wrapper files | 4; unpacked 12967 bytes |
| Wrapper `buildMetadata.gitCommit` | `a3a2697` |
| GitHub tag | `v1.0.80` lightweight commit `ef627e1baad937d3c8da45f8a5541c6fc3c97b6a` |
| GitHub release | published 2026-08-14T02:28:39Z, not prerelease |
| Public repo tree | README, changelog, LICENSE, install.sh only |
| ACP registry | `github-copilot-cli` `1.0.80`, npx args `["--acp"]` |
| Host | absent |

`@github/copilot` on npm is a Node wrapper (`npm-loader.js`) that resolves
an optional platform package (`@github/copilot-darwin-arm64@1.0.80` and
siblings) and spawns that native `copilot` binary with the original argv.
Swallowtail binds the host-approved `copilot` executable plus
`--acp --stdio`. It does not wrap Node, search `PATH`, or honor
`COPILOT_CLI_PATH` as discovery.

Platform package metadata (not extracted):

| Package | Integrity |
| --- | --- |
| `@github/copilot-darwin-arm64@1.0.80` | `sha512-fzn4PnSx3+O/a3ip72KVsjnzORsEygK+0i21bFAnFBYS+0Wi1Pk+o/CmNsJ7aRbf1enSJrcH8UDVkyc9pMGEBg==` |

The public `github/copilot-cli` tag is not the ACP implementation source.
The tagged changelog still opens at `1.0.79`. Identity follows npm
`latest` `1.0.80` plus the matching GitHub release tag.

## Selected wire

Entrypoint: `copilot --acp --stdio`. NDJSON JSON-RPC over the child
stdin/stdout. Official docs: stdio is the default when `--acp` is passed
alone; `--stdio` disambiguates. `--stdio` and `--port` are mutually
exclusive. ACP registry discovery args are `["--acp"]` only.

Selected Swallowtail argv is still `["--acp", "--stdio"]`. Passing only
`--acp` would match registry discovery and the default, but `--stdio`
keeps TCP off the selected command.

First useful op:

1. `initialize`
2. `session/new` with `{cwd, mcpServers: []}`
3. one bounded `session/prompt` of text blocks
4. `session/cancel` if the turn is still live
5. join/kill the child

Official docs example spawns `["--acp", "--stdio"]`, calls `initialize`
with `protocolVersion: PROTOCOL_VERSION` and empty `clientCapabilities`,
`newSession({cwd: process.cwd(), mcpServers: []})`, one text `prompt`,
and cleans up with stdin end plus `SIGTERM`. Permission requests in that
example return `cancelled`.

Initialize **result** fields (`agentCapabilities`, `authMethods`,
`agentInfo`) are not recovered from public TypeScript. The platform
binary was not extracted. The first driver must fail closed on unexpected
initialize results rather than inventing them.

`session/new` does not carry tool-filter or reasoning effort. Those bind
only as **server-start** flags (`--available-tools`, `--excluded-tools`,
`--effort` / `--reasoning-effort`). Swallowtail does not pass them on the
first argv.

Slash commands can be sent as ordinary text prompts and advertised via
`available_commands_update`. Interactive-only commands (`/diff`,
`/resume`, `/theme`, `/settings`, `/login`, `/help`, `/tasks`, `/undo`)
are not ACP operations. First driver does not send slash commands as
Swallowtail operations.

## Authority

Auth is host-owned GitHub Copilot login, `GH_TOKEN` / `GITHUB_TOKEN`, or
BYOK `COPILOT_PROVIDER_*`. Official docs: ACP sessions with configured
BYOK may run without GitHub login, matching `-p` / interactive. Swallowtail
does not log in, run `copilot login`, or bind those variables as a
credential lease.

Working resource is `session/new` `cwd`. Isolation is one owned stdio
child. Cleanup is cancel in-flight prompt, then join or kill. TCP mode
keeps a listener after the client disconnects; that is not this route.

`--yolo`, `--allow-all`, and the ACP `allow_all` session config option are
not Swallowtail authority. Contract 015: do not auto-select
`allow_always`.

## Unmapped on this corpus

TCP `--port`, `--available-tools`, `--excluded-tools`, `--effort`,
`--reasoning-effort`, `--yolo`, `--allow-all`, `copilot login`,
interactive-only slash commands, GitHub Copilot IDE/API product coverage,
`session/load`, `closeSession` as a Swallowtail management binding, model
or agent config-option writes, usage as portable usage-evidence, client
MCP servers, and prerelease `1.0.81-*`.

Changelog advertises load, close, plan, usage_update, custom agents, and
allow-all. Advertisement is not first-driver coverage.

## Decision

Admit `copilot-cli.acp` as a first-party ACP stdio route with **public
preview** visible. Freeze identity and named fixtures under
`crates/swallowtail-adapter-copilot-cli/tests/fixtures/copilot-cli-acp-1.0.80/`.
Card 271 may create the package and decoder. No production claim in this
card.

## Non-goals

- installing Copilot or extracting a platform archive
- live initialize, prompt, login, or BYOK
- TCP `--port`, version-range claims, prerelease tags
