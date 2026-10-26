# 447 Oh My Pi 18.12.6 Identity and RPC Qualification

**Scope:** `oh-my-pi.rpc` only. This record qualifies official
`@oh-my-pi/pi-coding-agent` stable after the Research 382 ceiling `18.8.3`.
`pi.rpc` and `pi.sdk-sidecar` stay on their own axes.

Status: promoted. Question: compatible extension of the maintained `18.x`
segment. Outcome: compatible extension.

## Official identity

On 2026-10-26, npm `latest` and GitHub's latest non-prerelease agreed on
`18.12.6`. npm published it at `2026-10-21T21:21:39.454Z`. GitHub release
`v18.12.6` was published at `2026-10-21T21:02:53Z`, commit
[`45c35e40c46166dc726ccc52f5d27f40f2449f99`](https://github.com/can1357/oh-my-pi/commit/45c35e40c46166dc726ccc52f5d27f40f2449f99),
`prerelease: false`, `target_commitish: main`.

`gitHead` is null on every hop in this window, so the published npm tarball
is the artifact authority. GitHub tags match the npm versions one for one
and are corroboration. The `18.12.6` tarball SHA-256 is
`8088b050a405cc4073c8f4b32d338ebb0ecc2f0d18be05852f20960b73e819e1`
(15,233,742 bytes). npm integrity is
`sha512-Wo+FcJ7A3RY/OFuafj4hcdIAXc/W7NMxbanwQh3Cmy8s57+ihLOLgAv+HROxHBkssEMg0MaScwkQ9xwqLPiC/g==`
and SHA-1 is `7762bf3888fa4a841545ea2e13f6e22e74f24792`. `dist/cli.js` is
SHA-256
`4475566bfd0a89b045d4b41c7efe905360aac9662d84746a3e8ad5e735542078` at
17,895,119 bytes. The package has 3,342 files, unpacked size 49,180,942
bytes, `bin.omp` → `dist/cli.js`, and Bun `>=1.3.14`. npm signatures use
key id `SHA256:DhQ8wR5APBvFHLF/+Tc+AYvPOdTpcIDqOhxsBHRwC7U`. The SLSA
provenance attestation is
`https://registry.npmjs.org/-/npm/v1/attestations/@oh-my-pi%2fpi-coding-agent@18.12.6`.

The tree digest is SHA-256 over sorted UTF-8 path, NUL, lowercase file
SHA-256, and LF. The `18.12.6` tree digest is
`13057a74c47625b83d803e0b401789685a11a77c646233ff4cec571568a9de32`.
The included `18.8.3` tree digest
`05d484a933e70894931f66fec4a9ef6bcf493584872f83e230063f011d85c551`
matches Research 382. `omp` and `bun` were absent from `PATH`. No artifact
was installed or executed.

The identity freeze is
[`identity.json`](../../crates/swallowtail-adapter-oh-my-pi/tests/fixtures/oh-my-pi-18.12.6/identity.json).

## Every published hop

Twenty-six stables from `18.8.3` through `18.12.6` are 25 hops. npm and
GitHub tags agree. No new GitHub-only or npm-only gap appears in this
window. The older GitHub-only tags `18.0.2`, `18.1.7`, `18.4.7`, and
`18.6.2` stay excluded. `17.4.3` and `17.4.4` stay incompatible.

The inventory holds 26 complete path-to-SHA-256 trees. `18.8.3` has 3,179
files; `18.12.6` has 3,342. The hop ledger recomputes every added, removed,
and changed path and classifies 194 selected-source records across 59
paths. Those 194 version/path pairs match the npm tree digests. The unique
`case "..."` set in `src/modes/rpc/rpc-mode.ts` stays the same 67 names on
every hop. Selected command spec blocks and the `read` / `grep` / `glob` /
`todo` / `ask` input schemas are byte-identical across the window.

At `18.11.1` the bundled `dist/cli.js` shrinks from about 30 MB to about
17 MB and the npm unpacked size drops from about 59 MB to about 47 MB while
the file count rises from 3,261 to 3,264. The selected command set, tool
schemas, frame caps, and startup fields do not change on that hop.

Fixtures:
[`complete-tree-inventory.json`](../../crates/swallowtail-adapter-oh-my-pi/tests/fixtures/oh-my-pi-18.12.6/complete-tree-inventory.json),
[`hop-ledger.json`](../../crates/swallowtail-adapter-oh-my-pi/tests/fixtures/oh-my-pi-18.12.6/hop-ledger.json),
[`protocol.json`](../../crates/swallowtail-adapter-oh-my-pi/tests/fixtures/oh-my-pi-18.12.6/protocol.json).

| Published hop | Selected-surface class | Finding |
| --- | --- | --- |
| `18.8.3→18.8.4` | compatible model internal | Model resolver internals change. The 67-case dispatcher, 14 selected commands, tool schemas, frame caps, and startup fields stay. |
| `18.8.4→18.8.5` | compatible model, session, and read internal | Model registry, session, and JSON-read helpers change. Selected command cases, input keys, and wire markers stay. |
| `18.8.5→18.8.6` | compatible model and session internal | Model resolver and session internals change. Selected commands, schemas, frames, and startup fields stay. |
| `18.8.6→18.8.7` | compatible session and read internal | Session, message, and read-helper internals change. URI types stay on the existing read tier. |
| `18.8.7→18.8.8` | compatible model and session internal | Model registry, resolver, and session internals change. Selected commands, schemas, and frames stay. |
| `18.8.8→18.8.9` | compatible launch-flag internal | Flag-table internals change. Selected `--mode rpc`, `--no-session`, `--provider`, `--model`, `--tools`, suppression flags, `--no-prewalk`, and `--approval-mode always-ask` stay accepted. |
| `18.8.9→18.9.0` | additive queue state | `get_state` `queuedMessages` gains optional `liveSteered`, default 0. Startup still reads `queuedMessageCount`. Mapped dialog methods stay. |
| `18.9.0→18.9.1` | compatible model internal | Models-config schema bundle changes. Selected provider/model binding and command set stay. |
| `18.9.1→18.9.2` | compatible read-resource internal | Model registry, `omp://` routing, and session internals change. Scheme set and read tier stay. |
| `18.9.2→18.10.0` | compatible session internal | Session internals change. Commands, schemas, frames, terminal, failure, and usage mapping stay. |
| `18.10.0→18.10.1` | compatible package hop | Package metadata and the bundled executable change. No selected source file changes. |
| `18.10.1→18.10.2` | compatible read internal | Read-tool implementation changes. The read input schema stays required `path`. |
| `18.10.2→18.10.3` | compatible model and session internal | Model patch, registry, session controls, and flag-table internals change. Selected argv values and startup fields stay. |
| `18.10.3→18.10.4` | compatible session internal | Session internals change. Commands, schemas, frames, and startup fields stay. |
| `18.10.4→18.11.0` | compatible tool and wire internal | Ask, grep, read, and generated wire files change outside the selected command cases. Tool input keys stay. |
| `18.11.0→18.11.1` | compatible session and read internal | Session, model, and read formatting change. Selected command cases stay. |
| `18.11.1→18.11.2` | compatible tool internal | Grep and generated event files change. Event type names, assistant message events, and usage fields stay. |
| `18.11.2→18.11.3` | compatible read-resource and session internal | Internal URL handlers and session internals change. `cfg://` still redacts credentials. |
| `18.11.3→18.12.0` | compatible read and frame internal | Read helpers, model resolution, and frame-encoder internals change. Physical cap stays 1 MiB and reassembled cap stays 64 MiB. Ready still advertises protocol versions 1 and 2. |
| `18.12.0→18.12.1` | compatible package hop | Package metadata and the bundled executable change. No selected source file changes. |
| `18.12.1→18.12.2` | compatible session internal | URL filesystem and session internals change. Selected commands, schemes, and tool schemas stay. |
| `18.12.2→18.12.3` | compatible tool internal | Glob, grep, path, and read internals change, with model and `proc://` routing. Tool input keys and the read tier stay. |
| `18.12.3→18.12.4` | additive `mr` read resource | Adds `mr://`, an immutable remote GitLab merge-request diff reader. Resolve runs ambient `glab` as the process principal, forwards `GITLAB_TOKEN`/`GITLAB_ACCESS_TOKEN`/`OAUTH_TOKEN` only after a host-trust check (without those env tokens, egress has no host-trust check), reads `glab` `config.yml`, and performs GitLab network egress at the default read tier; tool-effect is exec of `glab`. No write path. Unselected `set_todos` records a custom entry. |
| `18.12.4→18.12.5` | compatible read internal | Read-tool implementation changes. The read input schema stays required `path`. |
| `18.12.5→18.12.6` | compatible abort cleanup | `abort` still calls `session.abort` and returns success. It also aborts unselected builtin commands. MCP manager is wired into RPC; prompt-text `/mcp reauth` (browser OAuth plus credential persistence) and `reconnectMcpServer` are reachable. `--no-extensions` disables extension discovery only and does not disable MCP. Swallowtail maps none of `/mcp`, reconnect, or OAuth; reconnect stays TUI-only. RPC steer and deferred extension loading stay inert under the selected launch. |

`--approval-mode` gains accepted value `always-ask-cached` and still accepts
`always-ask`, `write`, and `yolo`. The selected launch value stays
`always-ask`. `ssh` stays `readTier: exec`. `cfg://` still redacts
credentials. Methods `ask`, `cancel`, and `open_url` already existed at
`18.8.3` and stay unmapped.

`18.12.7` was not observed on npm or GitHub. It is the synthetic later
stable.

## Selected boundary and claim

The selected 14 commands stay `negotiate_protocol`, `set_model`,
`set_thinking_level`, `set_auto_retry`, `set_auto_compaction`,
`set_steering_mode`, `set_follow_up_mode`, `set_interrupt_mode`,
`get_state`, `get_available_models`, `prompt`, `steer`, `follow_up`, and
`abort`. The selected tools stay `read`, `grep`, `glob`, `todo`, and `ask`.
Physical frames stay capped at 1 MiB and reassembled frames at 64 MiB.
Ready still advertises protocol version 1 with supported `[1, 2]`; the
route still negotiates 2. Response fields stay `id`, `command`, `success`,
and `data`. Terminal `agent_end` stays `isTerminal: true`. Provider failure
stays assistant `message_end` with `stopReason: error`. Usage stays
`input`, `output`, `cacheRead`, and `cacheWrite`. Startup still requires
`model.provider`, `model.id`, `thinkingLevel`, `isStreaming: false`,
`isCompacting: false`, one-at-a-time steering and follow-up, `interruptMode:
wait`, `autoCompactionEnabled: false`, and `queuedMessageCount: 0`.

`mr://` is `{ backing: "remote", selectors: "lines", immutable: true }` with
the default read tier, resolve-only, and no write. Resolve runs ambient
`glab` as the process principal, reads `glab` `config.yml`, and performs
GitLab network egress; tool-effect is exec of `glab`.
`assertGitlabHostTrustedForEnvToken` constrains
`GITLAB_TOKEN` / `GITLAB_ACCESS_TOKEN` / `OAUTH_TOKEN` forwarding: it
returns immediately when none of those env tokens is set, so a
model/prompt-chosen `mr://` host is contacted with no trust check. When an
env token is set, the host must be `gitlab.com`, `GITLAB_HOST` /
`GITLAB_URI` / `GL_HOST`, a `glab` `config.yml` host, or the checkout
GitLab remote. That is the same class as the earlier `gh`-based `pr://` /
`issue://` resources. Swallowtail adds no URI-specific operation.

`--no-extensions` disables extension discovery only. MCP stays enabled on
the selected `--tools read,grep,glob,todo,ask` launch because `enableMCP`
is `!restrictToolNames && (options.enableMCP ?? true)`, the root RPC
launch never sets `restrictToolNames` or `enableMCP`, and `--no-tools`
only sets `toolNames=[]`. `18.12.6` passes `mcpManager` into
`runRpcMode` and wires `/mcp reauth` plus `reconnectMcpServer` into
prompt-text builtin dispatch. Reauth opens a browser OAuth flow and
persists credentials through `runtime.session.modelRegistry.authStorage`.
The `reconnect` verb stays TUI-only. `18.8.3` rejected reauth as TUI-only.
Swallowtail maps none of `/mcp`, reconnect, or OAuth.

Extend `oh-my-pi.rpc.package-window-2` as a compatible extension: maintained
`18.0.0..=18.12.6` on `oh-my-pi.rpc-v2-v18.0.0`. Keep deprecated
`17.2.9..=17.4.2` on `oh-my-pi.rpc-v2-v17.2.9`, the four unpublished `18.x`
gaps, incompatible `17.4.3`/`17.4.4`, `AllowUnverified`, and the frozen
`oh-my-pi-rpc-17.2.9` decoder corpus. `18.12.7` stays `UnverifiedNewer`.

No artifact was executed. There was no provider prompt, login, live
catalogue or session, credential use, installation, or host update.
