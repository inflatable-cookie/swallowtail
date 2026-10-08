# 382 Pi RPC 1.1.0 Identity and Qualification

Observed 2026-10-08. Scope is `pi.rpc` on `pi.package` only. The official npm
`latest` channel for `@earendil-works/pi-coding-agent` and the GitHub latest
stable release for [`earendil-works/pi`](https://github.com/earendil-works/pi)
both identify `1.1.0`. GitHub release `v1.1.0` was published at
`2026-10-07T22:26:31Z`, is neither draft nor prerelease, and points to commit
`abe508e1b89912adde45528136c3221eb69acdd7`. The complete exact npm and tag
artifact identities, all twelve package trees, source correlation, and the
selected-source hop ledger are frozen in the
[`pi-rpc-1.1.0` fixture](../../crates/swallowtail-adapter-pi/tests/fixtures/pi-rpc-1.1.0/).

The previous qualified ceiling is `0.86.1`. The stable published hops after
that point are `0.87.0`, `0.87.1`, `0.99.0`, `0.99.1`, `0.99.2`, `1.0.0`,
`1.0.1`, `1.0.2`, `1.0.3`, `1.0.4`, and `1.1.0`. Each npm package has its
integrity, shasum, tarball SHA-256, package file count, complete recursive tree
hash, and corresponding GitHub release/tag identity in `identity.json` and
`dist-inventory.json`. The `1.1.0` npm metadata has no `gitHead`; its embedded
source maps contain `rpc-mode.ts` and `agent-session.ts` matching their exact
`v1.1.0` tagged sources. Earlier package `gitHead` values match their tags.
The package requires Node `>=22.19.0`; the read-only host identity was Pi
`0.87.1` and Node `v22.23.2`. No package or host runtime was installed,
updated, or executed.

| Published hop | Added | Removed | Changed | Identical | Selected changed files reviewed |
| --- | ---: | ---: | ---: | ---: | ---: |
| `0.86.1` → `0.87.0` | 4 | 4 | 109 | 987 | 4 |
| `0.87.0` → `0.87.1` | 14 | 6 | 60 | 1,034 | 2 |
| `0.87.1` → `0.99.0` | 139 | 21 | 570 | 517 | 101 |
| `0.99.0` → `0.99.1` | 4 | 3 | 17 | 1,206 | 0 |
| `0.99.1` → `0.99.2` | 29 | 28 | 109 | 1,090 | 47 |
| `0.99.2` → `1.0.0` | 30 | 15 | 106 | 1,107 | 27 |
| `1.0.0` → `1.0.1` | 30 | 30 | 115 | 1,098 | 31 |
| `1.0.1` → `1.0.2` | 14 | 14 | 24 | 1,205 | 0 |
| `1.0.2` → `1.0.3` | 35 | 30 | 42 | 1,171 | 8 |
| `1.0.3` → `1.0.4` | 14 | 14 | 85 | 1,149 | 30 |
| `1.0.4` → `1.1.0` | 40 | 34 | 142 | 1,072 | 24 |

`protocol.json` records each changed source and source map feeding selected
RPC, lifecycle, configuration, permission, usage, or tool behavior, including
the exact classification and runtime chunk identity. `dist-inventory.json`
hashes the full path sets and each package file, so an empty selected-file
classification at `0.99.0 → 0.99.1` and `1.0.1 → 1.0.2` is independently
checked against the changed path set. The large `0.99.0` and `1.0.4` internal
extension/MCP changes are bounded by the selected invocation: it passes
`--no-extensions` and no extension paths. At `1.0.4`, although plain
`--tools` permits configured MCP names and upstream adds `--no-mcp`, the
built-in MCP extension is omitted by this invocation. At `1.1.0`, the exact
plain tool allowlist remains accepted by stricter modifier parsing.

The selected RPC command table remains 33 upstream commands, of which the
adapter selects `prompt`, `steer`, `follow_up`, `abort`, `get_state`,
`get_available_models`, `set_auto_compaction`, `set_auto_retry`,
`set_steering_mode`, and `set_follow_up_mode`. It continues to exclude `bash`,
`switch_session`, `fork`, `clone`, `extensions`, `clear_queue`, and `compact`.
The strict-LF JSONL parser, selected RPC operations, failure and permission
surface, retry suppression, and usage projection remain unchanged.

Two private mappings extend the existing public contract:

- Starting at `0.99.0`, prompt success dispositions are `handled`, `queued`,
  or `started`; steering and follow-up success dispositions are `handled` or
  `queued`. A `handled` prompt starts no run, so the adapter fails it without
  waiting for settlement. A `handled` steering or follow-up input is consumed
  without queueing; the adapter rejects it and releases its pending slot.
- At `1.1.0`, `agent_settled.aborted` is a required boolean. `true` maps to the
  existing terminal cancellation, while `false` follows existing completion
  and usage checks. Caller cancellation and deadline take precedence. Older
  qualified points keep this field optional.

The qualified shape is a compatible extension under the approved major-line
transition, with new private behavior milestones
`pi.rpc.strict-lf-v0.99.0-command-disposition` and
`pi.rpc.strict-lf-v1.1.0-aborted-settled`. Baseline `0.80.10`, claim
`pi.rpc.package-window-2`, every prior behavior revision, and all exclusions
remain. Earlier exact segments stay Deprecated; only `1.1.0` is Maintained.
The complete segment and hole ledger is encoded by `pi_rpc_claim()` and
asserted by the new exact identity regression.

Unpublished or unqualified versions remain incompatible: `0.83.1`, `0.84.5`,
`0.85.2`, `0.86.2`, `0.87.2` through `0.98.x`, `0.99.3` and later `0.99.x`,
and `1.0.5` and later `1.0.x`. Stable releases after `1.1.0` remain
`UnverifiedNewer`. The separate Pi SDK sidecar and Oh My Pi routes are outside
this evidence. There is no Pi SDK or OMP claim, no new public operation or
lifecycle change, and no provider prompt, live catalogue or session,
credential use, artifact execution, installation, or host mutation.
