# 455 Pi RPC 1.4.0 Identity and Qualification

Observed 2026-10-26. Scope is `pi.rpc` on `pi.package` only. The official npm
`latest` channel for `@earendil-works/pi-coding-agent` and the GitHub latest
stable release for [`earendil-works/pi`](https://github.com/earendil-works/pi)
both identify `1.4.0`. GitHub release `v1.4.0` was published at
`2026-10-21T22:49:48Z`, is neither draft nor prerelease, and points to commit
`4a080252f06caf63a525d7873c5d94d3c23887e0`. The previous qualified ceiling is
`1.1.0`. The published stable hops are `1.2.0`, `1.3.0`, and `1.4.0`.

Each npm package has a null `gitHead`. Embedded sourcemap `sourcesContent`
for `rpc-mode.ts`, `agent-session.ts`, `settings-manager.ts`,
`resource-loader.ts`, and `args.ts` matches the same path at GitHub tags
`v1.1.0`, `v1.2.0`, `v1.3.0`, and `v1.4.0`. npm `1.1.0` tarball SHA-256
`09cd8a0a43dbb1d81a67346b09400b439ce71d818caba4e963ea958846a1aed4` and tree
digest `62fb9de5c48a64d72f8f8fb2ad7a1f7aae15ca389dc52348b7d973cc9bdf82c8`
reproduce Research 383. The package still requires Node `>=22.19.0`. Pi was
not on `PATH`. No package or host runtime was installed, updated, or executed.

| Published hop | Added | Removed | Changed | Identical | Selected changed files reviewed |
| --- | ---: | ---: | ---: | ---: | ---: |
| `1.1.0` → `1.2.0` | 62 | 41 | 153 | 1,060 | 41 |
| `1.2.0` → `1.3.0` | 19 | 18 | 88 | 1,169 | 7 |
| `1.3.0` → `1.4.0` | 59 | 56 | 95 | 1,125 | 8 |

`dist/modes/rpc` is byte-identical across all four points: 16 files, including
the strict-LF JSONL parser, the 33-command table, prompt dispositions
(`handled`, `queued`, `started`), steering and follow-up dispositions
(`handled`, `queued`), and `agent_settled.aborted` as a required boolean.
`dist/cli/args.js` is byte-identical, so the selected flags and the plain
`--tools read,grep,find,ls` allowlist parse the same way.

Selected files that did change stay off the mapped wire:

- `1.2.0` moves settings literals into `SETTINGS_DEFAULTS`. Mapped defaults
  stay `steeringMode` and `followUpMode` `one-at-a-time`, compaction enabled,
  and retry enabled. The selected invocation still sets those through RPC.
- `1.2.0` changes ancestor context-file shadowing. `--no-context-files` skips
  that load.
- `1.2.0` splits prompt startup so extension-triggered runs share
  `before_agent_start`, and an abort requested during startup calls
  `agent.abort()` at `agent_start`. The selected invocation passes
  `--no-extensions` and sends no extension paths.
- `1.3.0` restores a resumed session's tool loadout and adds
  `sessionShareDestination`. The route is fresh-only and passes `--no-session`.
  `/share` is not a selected command.
- `1.4.0` finds overflow-recovery tool results in the projected context and
  walks every system message when restoring a resumed loadout. Compaction
  event fields stay the same. `--no-session` keeps that restoration off the
  selected path.
- Codemode and MCP files changed on these hops. `--no-extensions` does not
  load them.

The qualified shape is a compatible extension of
`pi.rpc.strict-lf-v1.1.0-aborted-settled`. Baseline `0.80.10`, claim
`pi.rpc.package-window-2`, and every prior behavior revision stay. Exact
`1.1.0`, `1.2.0`, and `1.3.0` are Deprecated. `1.4.0` is Maintained.
`AllowUnverified` stays. The first unpublished later stable is `1.4.1`.

Unpublished or unqualified holes stay incompatible, including `0.83.1`,
`0.84.5`, `0.85.2`, `0.86.2`, `0.87.2` through `0.98.x`, `0.99.3` and later
`0.99.x`, `1.0.5` and later `1.0.x`, `1.1.1` and later `1.1.x`, `1.2.1` and
later `1.2.x`, and `1.3.1` and later `1.3.x`. The Pi SDK sidecar and Oh My Pi
routes are outside this evidence. There is no new public operation, no
lifecycle contract change, and no provider prompt, live catalogue or session,
credential use, artifact execution, installation, or host mutation.

Evidence is frozen in the
[`pi-rpc-1.4.0` fixture](../../crates/swallowtail-adapter-pi/tests/fixtures/pi-rpc-1.4.0/).
