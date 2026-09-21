# Research 334: Qwen Code 0.24.2 Identity

Status: promoted
Question: remaining AllowUnverified family after official `0.23.3`; is
`0.23.4` through official npm/GitHub `0.24.2` a compatible-extension of
`qwen-code.headless.v0.21.15-reasoning-control`, a private milestone, or a
stop?

Observed 2026-09-21 on the `qwen-code.package` axis:

- Host `qwen` was not on `PATH`. Missing install is not a gap. The host was
  not installed, updated, or otherwise mutated.
- `npm view @qwen-code/qwen-code version`: `0.24.2`.
- Official npm stable hops after the prior `0.23.3` ceiling are exactly
  `0.23.4`, `0.24.0`, `0.24.1`, and `0.24.2`. Preview and nightly channels
  were excluded. `0.20.2`, `0.21.16`, `0.22.4`, `0.23.5`, and `0.24.3` were
  not published stable points at observation.
- The family release channel is the direct QwenLM/qwen-code tags and
  releases `v0.23.4` through `v0.24.2`. npm `gitHead` matches each tag
  commit. The repository-wide generic `latest` release endpoint was not
  used.

## Official hop identities

| Version | npm published | GitHub tag commit | npm shasum | extracted package files |
| --- | --- | --- | ---: | ---: |
| `0.23.3` | `2026-09-10T15:20:05.693Z` | `b695664b8df06d06c625db3e30b97045d82092c7` | `3336056e130b9199bf365e0c2a34416523240e18` | 1050 |
| `0.23.4` | `2026-09-14T15:41:29.586Z` | `fdcd7c2761e4ae18d578cf1413ae8d43f58b3e2c` | `bc2e1d58a35d8924c96e85d388c13827e8d398a2` | 1067 |
| `0.24.0` | `2026-09-16T13:37:16.481Z` | `56b003be0785412ed06673948f781dc70a686b5a` | `86565f6f14953e2ef461a304b8db55251adfdff5` | 1128 |
| `0.24.1` | `2026-09-19T08:28:42.511Z` | `e443e2d2384e3ba61d783bd96a265082298f4fe8` | `ad2b5f2a10b6240dc67d598082ddcc2de92277c9` | 1267 |
| `0.24.2` | `2026-09-20T14:28:21.292Z` | `1026c4a50f4a32f77da98bdacfba2e5faa8cc70a` | `2146374639fc36a807c3617d2fddbef0979ebb3a` | 1267 |

The official `0.24.2` npm integrity is
`sha512-Dx9A8SVIPcT7P9R3hyoBy7aWn1askUssB60OO0E0MCKAcbrPqrGStHB5lgHoLFREfAopRzjNiYUIUr5MNEQZlA==`.
Its tarball SHA-256 is
`ea8b977eac79977603f52c9a3f4d279f3f70e230910a087372a51d70cd2759fa`.
The extracted `cli.js` is 14,716 bytes with SHA-256
`01ec0c6cb09f1f04caf0da15d45cd70e56191778365d93e198d502213d774f7f`.
`cli-entry.js` remains
`68cb29eb7ccc936d78ece5564ef55cae41a55b630e6657dc417c1f2e561cf4c9`
from `0.21.15` through `0.24.2`.

The complete package path/hash inventory and per-hop added, removed, changed,
and identical sets are frozen in the
[0.24.2 dist inventory](../../crates/swallowtail-adapter-qwen/tests/fixtures/qwen-code-0.24.2/dist-inventory.json).
The per-hop category counts are `377/360/37/653`, `449/388/31/648`,
`507/368/27/733`, and `489/489/29/749` respectively. Artifacts stayed in
`/tmp`. No prompt, login, install, host update, or downloaded-artifact
execution.

## Selected-surface classification

Selected flags, approval values, stream formats, read tools, and catalogue
controls remain present through `0.24.2`. Mapped top-level stream types stay
`system`, `stream_event`, `assistant`, and `result`. Nested `goal_state`
stays unmapped. `0.24.1` removes the unmapped nested `active_goal` event.

`plan-mode-shell-policy.ts` and `exitPlanMode.ts` are byte-identical to
`0.23.3`. `permissionFlow.ts` changes L3 match params and abort forwarding;
the `0.23.2` Plan execution-confirmation counterexample still keeps the
exact Plan set bounded at `0.22.3`. Exact `0.21.15` reasoning and
caller-decreasing budgets are not widened.

Changed files feeding board/sandbox commands, goal budgets, web-search and
omni settings, DashScope metadata, session-recovery tool-call ids,
agent-execution, trusted-folder messaging, hidden ToolCall concurrency,
Web Shell/ACP/chunk rebuilds, and transcript export are named and bounded
in `protocol.json`. None changes Qwen headless execution authority or
justifies flattening into Model Studio or Qwen ACP.

## Contract 029 decision

The family is a `compatible-extension` of
`qwen-code.headless.v0.21.15-reasoning-control`. Keep the deprecated
`0.19.11..=0.20.1` and `0.21.0..=0.21.14` windows, exact `0.21.15`, and
extend maintained `0.22.0..=0.24.2` with the unpublished interior `0.22.4`
and `0.23.5` stables kept incompatible. Qualify every published hop. Keep
the remaining unpublished gaps and `AllowUnverified`; after qualification,
`0.24.3` is the first unverified newer stable candidate. No new public mapped
operation. No major-line reset.
