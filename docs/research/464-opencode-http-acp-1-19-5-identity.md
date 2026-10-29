# 464 OpenCode HTTP and ACP 1.19.5 Identity

Status: promoted
Date: 2026-10-28

Official npm `opencode-ai` `latest` and GitHub stable `v1.19.5` agree.
`opencode.http` and `opencode.acp` remain qualified through `1.19.4`.
`1.19.5` is unverified on both separate axes. ACP registry
`opencode` `2.2.0` is the OpenCode 2 line and is not this axis.

## Question

Does published stable `1.19.5` keep the selected HTTP route surface and the
selected ACP behavior, or does the hop change identity?

## Rank

| Surface | Host | Official | Boundary at observation | Result |
| --- | --- | --- | --- | --- |
| `opencode.http` / `opencode.server` | absent | npm and GitHub `1.19.5` | `1.19.4`, AllowUnverified | stop |
| `opencode.acp` / `opencode.executable` | absent | same package and tag | `1.19.4`, AllowUnverified | stop |

`opencode` was not on `PATH`. No binary was executed. Missing host is not a
gap. HTTP MCP live honouring stays exact `1.18.18` and does not transfer.

## Method

Re-probed npm `opencode-ai` dist-tag `latest` and GitHub
`anomalyco/opencode` releases on 2026-10-28. Both named stable `1.19.5`.
npm published `2026-10-28T06:17:11.100Z`. The GitHub release published
`2026-10-28T06:17:49Z` and is not a prerelease. Snapshot, dev, next, beta,
and other non-latest dist-tags were ignored. No prerelease sits between
`1.19.4` and `1.19.5`. The only published stable after the ceiling is
`1.19.5`. No stable `1.18.36` or `1.19.6` exists.

The ACP registry lists `opencode` and `opencode-v2`, both at `2.2.0`.
`opencode` names GitHub `v2.2.0` archives. `opencode-v2` names
`opencode.ai` copies with the same digests. GitHub `v2.2.0` is a separate
non-prerelease published `2026-10-27T09:04:44Z`, tag commit
`1097e2668b1e08ad63bdf0b1c4860b9bc01e420c`. Its notes describe the OpenCode 2
desktop and plugin line. npm `opencode-ai` has no `2.x` version.
`releases/latest` is `v1.19.5` because that publish is newer, not because
`v2.2.0` is missing. `2.2.0` is a distinct major line. This record does not
qualify it.

Downloaded npm `opencode-ai` and `opencode-linux-x64` tarballs and the
standard GitHub tag archives `archive/refs/tags/vX.tar.gz` for `1.19.4` and
`1.19.5`. npm has no `gitHead`. The `1.19.5` release `target_commitish` is
the branch name `dev`, not a commit. No attestation was observed. The tag
archive is evidence about the tag tree only: no available proof correlates
the native payload to that tag. Only Linux x64 native contents were inventoried;
the wrapper selects twelve native packages, leaving the other targets unproved:
`opencode-linux-x64`, `opencode-darwin-x64`, `opencode-linux-arm64`,
`opencode-windows-x64`, `opencode-darwin-arm64`, `opencode-windows-arm64`,
`opencode-linux-x64-musl`, `opencode-linux-arm64-musl`,
`opencode-linux-x64-baseline`, `opencode-darwin-x64-baseline`,
`opencode-windows-x64-baseline`, and `opencode-linux-x64-baseline-musl`.
Linux x64 release-asset digests are the GitHub release API digests; those
binaries were not downloaded. The re-downloaded `1.19.4` archive SHA-256
`2e553ed005b2df4a7efb9edd26db8fc3caafb1b4dfc3c5c3869192334584d8bb` and
canonical tree `ae0d6884639fda9deb79082e5b27483c9b6a6f45fb8868dadd89714722cbf287`
match Research 453. npm tarball digests for `1.19.4` match that record too.

Canonical tree hash: sort relative paths bytewise; each line is
`path<TAB>sha256:<file>` or `path<TAB>symlink:<target>` plus LF; SHA-256 the
UTF-8. Path-list hashes are the newline-joined paths with a trailing
newline. The empty list hashes to SHA-256 of the empty string.

No prompt, login, live server, catalogue, session, or provider contact.

Fixtures:

- `crates/swallowtail-adapter-opencode/tests/fixtures/opencode-1.19.5/`
- `crates/swallowtail-adapter-opencode/tests/fixtures/opencode-acp-1.19.5/`

## Identities

| Version | npm `opencode-ai` SHA-256 | Tag commit | Source archive SHA-256 | Canonical tree | Files |
| --- | --- | --- | --- | --- | ---: |
| `1.19.4` | `2ceca45c7e0bbd50ddd3f4d6d651ed43645647d74279d656e7c7747a4b6a9137` | `148842ef7cb1bc34192b3fff57d0f9721ab9c801` | `2e553ed005b2df4a7efb9edd26db8fc3caafb1b4dfc3c5c3869192334584d8bb` | `ae0d6884639fda9deb79082e5b27483c9b6a6f45fb8868dadd89714722cbf287` | 6576 |
| `1.19.5` | `1f97e49381f8a42a629482eff8be98aa96d62e2e94410a8ec98e738f9c787680` | `810c0eba1a7348036f8bdf61e6632d0c36c6876c` | `7d2ac43b5083a3a4687fe23b79ab153b8d93a6fb186fa07209ed28d4731e5c7a` | `98d156015c7d31198a689d93fbe18bdc9a11c833228473e8334ee36c9973a56b` | 6567 |

| Hop | Added | Removed | Changed | Identical |
| --- | ---: | ---: | ---: | ---: |
| `1.19.4` → `1.19.5` | 4 | 13 | 83 | 6540 |

npm `opencode-ai` keeps four files. `LICENSE`, `bin/opencode.exe`, and
`postinstall.mjs` are byte-identical; only `package.json` changes. The
`opencode-linux-x64` hop changes both `bin/opencode` and `package.json`.
Bundled Bun stays `1.4.4`. `@agentclientprotocol/sdk` stays `0.21.0`.

## Selected protocol

HTTP selected routes stay `global.health`, `provider.list`,
`session.create`, `session.prompt_async`, `event.subscribe`, `session.abort`,
`session.delete`, `session.list`, `session.status`, `session.get`, and
`session.messages`. Declaration and handler files for those routes are
byte-identical. Selected session files `compaction.ts`, `message-v2.ts`,
`session.ts`, and `status.ts` are byte-identical. `packages/sdk/openapi.json`
is byte-identical. The path set stays 162 paths. `info.version` stays
`1.0.0`.

ACP sources under `packages/opencode/src/acp/`, `cli/cmd/acp.ts`, and the
MCP config and HTTP files are byte-identical. The only selected ACP file
that changes is `packages/opencode/package.json`, and that change is the
version plus the GitLab provider pin.

`packages/opencode/src/util/rpc.ts` now answers a thrown TUI worker request
instead of leaving the caller waiting. Its only importers are
`cli/cmd/tui.ts` and `cli/tui/worker.ts`, both byte-identical. That is the
release note. It is not the HTTP server and not `opencode acp`.

PTY connect authorization now matches the Effect route path plus the ticket
query. Credential checks for every other route are unchanged. Swallowtail
does not call PTY. The selected OpenAPI document does not change.

Eight removed files under `packages/opencode/src/` have no importer or path
reference in the `1.19.4` tree. Three of them are prompt texts. They were
not loaded. Other source edits drop unused imports or unused exports,
including the image resizer, session instruction, LLM request, and overflow
helpers. Function bodies on those files stay.

`gitlab-ai-provider` moves from `6.19.0` to `6.23.1` in `bun.lock` and in
the `packages/opencode` and `packages/core` manifests. The unchanged in-repo
wrapper does not establish dependency behavior. Static comparison of npm
packages and their embedded source maps gives these bounded observations:

- `createGitLab` chooses `instanceUrl` from options, then
  `GITLAB_INSTANCE_URL`, then `https://gitlab.com`; key lookup prioritizes an
  explicit key, OpenCode auth, then `GITLAB_TOKEN`. Direct access posts to the
  instance endpoint and uses the returned token and headers for the gateway.
  These paths are present in both packages; the audience and complete effects
  through the selected environment remain unproved.
- The 6.23.1 provider adds workflow defaults merged with defined per-call
  overrides; namespace resolution uses an explicit value or, for self-managed
  instances, detected project namespace, including reconnect. Model mappings add
  Haiku 5.5 chat and workflow and Gemini 4 Argon workflow entries. Whether those
  changes reach selected catalogue or prompt envelopes remains unproved.
- Direct-access caching gains an optional private POSIX shared directory from
  `directAccessCacheDir` or `GITLAB_DIRECT_ACCESS_CACHE_DIR`; empty disables it.
  Without a directory the in-memory cache remains. Scope includes endpoint,
  gateway URL, headers and feature flags. A lock can wait approximately 60
  seconds and refresh times out after 30 seconds. The direct-access 401 path
  refreshes credentials once; Anthropic retries token errors once, and stream
  retry is limited to before emitted content. EnvironmentRef boundaries and
  interactions with these paths remain unproved.
- Anthropic adds `skipCache` (default false), scoped provider options and finish
  mappings for `pause_turn`, `model_context_window_exceeded` and `refusal`;
  OpenAI Responses defaults omitted tool `strict` to false. These are potential
  prompt and session effects, not evidence of compatibility.

The package tarball SHA-256 values are `0ddd9d12e4d1b832c2ae7da10fab41d5d6a5c1454e5e1323c2fad10ca6cdbdbd`
for 6.19.0 and `37f7145ae868eff1104172277bcd380167ec6605cc9073908a6f90c27195e038`
for 6.23.1. Their `dist/index.mjs.map` SHA-256 values are
`774bba8b53a3efe3aeb3c5388e143498d370dd59f01e25ae96e1ec1b577b6c89` and
`1285dfdc89efa5b6bb518027bcba9fa0174a9ce014f4db8c5925c36ceba2c1f6`.
These static observations do not establish the bundled dependency or its
complete transitive behavior.

The lock adds `proper-lockfile` `^4.1.2` to the GitLab dependency. It re-hoists
`signal-exit` 3.0.7 from `proper-lockfile/signal-exit` to the root and places
4.1.0 under `execa`, `foreground-child` and `write-file-atomic`. The fixture
records each coordinate transition.

The remaining paths include CI workflows, `nix/hashes.json`, workspace
version bumps, release scripts, and tests. Byte equality in this section is
limited to the compared tag trees, not the shipped executable.

## Decision

Stop. Both existing qualified ceilings and behavior revisions remain at
`1.19.4`; `1.19.5` is `UnverifiedNewer`. Preserve the historical segments,
holes, hard-delete contract and HTTP MCP live exact `1.18.18` pin. OpenCode 2
remains separate. Owning adaptation lead
`5b445586-3c98-47c5-aa88-7950aace016b`
(`opencode.http-acp.shipped-native-and-dependency-proof`) must close native
provenance and selected-target inventories and qualify the dependency effects,
or propose a separately reviewed contract-compatible evidence design.

## Sources

- npm registry `opencode-ai` and `opencode-linux-x64`, dist-tag `latest`
- npm `gitlab-ai-provider` tarballs `6.19.0` and `6.23.1`, embedded source maps
- GitHub `anomalyco/opencode` stable releases and tag archives
- ACP registry `https://cdn.agentclientprotocol.com/registry/v1/latest/registry.json`, discovery only
- Frozen fixtures named above
- Prior ceiling evidence: Research 453
