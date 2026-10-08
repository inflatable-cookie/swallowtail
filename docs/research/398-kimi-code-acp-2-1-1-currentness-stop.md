# Kimi Code ACP 2.1.1 Currentness Stop

Observed 2026-10-08 for `kimi-code.acp` only. This checkpoint changes no
production claim.

## Finding

Official npm `latest` and the latest GitHub release are both
`@moonshot-ai/kimi-code@2.1.1`. Thirteen stable releases follow the existing
`0.38.0` ceiling, including the same-package major reset at `2.0.0`. The exact
published sequence, npm integrity and tarball digests, GitHub tag objects,
commits, and source trees are frozen in
[`identity.json`](../../crates/swallowtail-adapter-kimi/tests/fixtures/kimi-code-2.1.1-acp/identity.json).
The installed CLI remains `0.34.0`; no artifact was installed or executed.

The complete npm package trees from `0.43.0` through `2.1.1` are in
[`dist-inventory.json`](../../crates/swallowtail-adapter-kimi/tests/fixtures/kimi-code-2.1.1-acp/dist-inventory.json).
Every file has a SHA-256 digest. Adjacent added, removed, and changed file
sets are recorded with their digests. The selected ACP source-region ledger
and all changed bundle source regions are in
[`protocol.json`](../../crates/swallowtail-adapter-kimi/tests/fixtures/kimi-code-2.1.1-acp/protocol.json).
Older hops and the existing authority boundary remain covered by
[Research 270](./270-kimi-code-0-39-1-identity.md) and
[Research 325](./325-kimi-code-0-43-0-installed-identity.md).

| Published hop | npm files added / removed / changed / unchanged | ACP server change |
| --- | ---: | --- |
| `0.43.0` → `0.43.1` | 75 / 73 / 4 / 466 | None across 25 source regions |
| `0.43.1` → `2.0.0` | 69 / 73 / 3 / 469 | `slash.ts` skips skills with explicit scopes when listing available commands |
| `2.0.0` → `2.0.1` | 0 / 0 / 3 / 538 | Generated initializer name only |
| `2.0.1` → `2.0.2` | 69 / 69 / 3 / 469 | Generated initializer names only |
| `2.0.2` → `2.1.0` | 69 / 69 / 3 / 469 | None across 25 source regions |
| `2.1.0` → `2.1.1` | 0 / 0 / 2 / 539 | None across 25 source regions |

The package file sets include the full `dist/main.mjs`, worker, web asset, and
metadata changes. Source regions outside `packages/acp-server` are also
listed with exact hashes; they are Kimi implementation and dependency changes
without a Swallowtail ACP mapping. They do not transfer a public capability.
At the major reset, the available-command discovery filter is not observed by
this adapter: `AvailableCommands` produces no activity.

## Process authority

The tagged `acpTerminalRunner.ts` Git blob is
`9016d48b643f35b263449d98dee25597a9a24d30` at `0.43.1` and `2.1.1`; its
bundled source region is byte-identical across all seven inventoried npm
points. Its fallback condition is
`!this.connection.terminalEnabled || !isBashToolInvocation(args, options)`.
When terminal is disabled, the condition takes the provider-local
`this.local.spawn(...)` path.

Swallowtail advertises `terminal: false` and `auth.terminal: false`. Its
terminal request dispatcher has no host callback route. Existing fake ACP
fixtures confirm those exact capabilities and the unsupported request
boundary. Contract 015 assigns terminal callbacks to the execution host and
leaves terminal support optional. Neither the adapter nor runtime mediates the
nested process. This is the same authority stop first recorded in Research
270; the current artifact confirms it persists through the major reset and
`2.1.1`.

## Disposition

Keep claim `kimi.acp.executable-window-5` at exact `0.28.1` and maintained
`0.29.0..=0.38.0` under `QualifiedOnly`. Preserve exact exclusions `0.39.0`
and `0.39.1`; do not add exclusions for points already rejected by posture.
Every published point above `0.38.0` through `2.1.1` remains incompatible.
There is no published stable 1.x line. Unpublished points remain gaps.

The currentness lane needs a separate ruling from the Contract 015 owner before
this route can advance. The ruling must either authorize a precisely stated
process-authority guarantee for the local-spawn branch, or direct an adaptation
that disables or mediates that branch and can be proved with fake controls.
This evidence task authorizes neither a contract exception nor a new runtime,
public operation, lifecycle, or capability. No provider prompt, authenticated
catalogue/session, credential, install, or host update was used.

## Sources

- [Official npm package](https://www.npmjs.com/package/%40moonshot-ai/kimi-code)
- [Official 2.1.1 GitHub release](https://github.com/MoonshotAI/kimi-code/releases/tag/%40moonshot-ai%2Fkimi-code%402.1.1)
- [Kimi ACP reference](https://github.com/MoonshotAI/kimi-code/blob/main/docs/en/reference/kimi-acp.md)
- [Contract 015: ACP v1 Negotiation and Client Callbacks](../knowledge/contracts/015-acp-v1-negotiation-and-client-callbacks.md)
- [Contract 029: Interface Version Qualification and Compatibility](../knowledge/contracts/029-interface-version-qualification-and-compatibility.md)
