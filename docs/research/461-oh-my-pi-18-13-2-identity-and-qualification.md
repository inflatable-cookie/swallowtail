# 461 Oh My Pi 18.13.2 Identity and RPC Qualification

**Scope:** `oh-my-pi.rpc` only. This record qualifies official
`@oh-my-pi/pi-coding-agent` stable after the Research 447 ceiling `18.12.6`.
`pi.rpc` and `pi.sdk-sidecar` stay on their own axes.

Status: promoted. Question: compatible extension of the maintained `18.x`
segment. Outcome: compatible extension.

## Official identity

On 2026-10-28, npm `latest` and GitHub's latest non-prerelease agreed on
`18.13.2`. npm published it at `2026-10-27T18:17:00.689Z`. GitHub release
`v18.13.2` was published at `2026-10-27T18:07:53Z`, commit
[`e996e1c7ac476f60e02b8ed9764cf30fa8c00a8e`](https://github.com/can1357/oh-my-pi/commit/e996e1c7ac476f60e02b8ed9764cf30fa8c00a8e),
`prerelease: false`, `target_commitish: main`. The only npm dist-tag is
`latest`.

`gitHead` is null on every hop in this window, so the published npm tarball
is the artifact authority. GitHub tags match the npm versions one for one
and are corroboration. The `18.13.2` tarball SHA-256 is
`0a2e716dafea4d7e49964ae344db662f92491bb1bd4c9fbde63d2df4d83a16b5`
(15,513,557 bytes). npm integrity is
`sha512-RNaGEOpLtG5GgPYP9ssOxtWKAVIVEF2frSjd2tLbmK43uXGoohIm+pXIVKPsxK+sonJ8blXM+bPYyvG+eMauCA==`
and SHA-1 is `8555c89e7630696ceb5b6c1358345860936055a9`. `dist/cli.js` is
SHA-256
`0b7355fb5489a0bdec1289353093ac51e015f438e5c89e8a036bcf955752cfeb` at
18,137,288 bytes. The package has 3,366 files, unpacked size 50,022,292
bytes, `bin.omp` → `dist/cli.js`, `main` `./src/index.ts`, `type` `module`,
and Bun `>=1.3.14`. The eleven `@oh-my-pi/*` runtime dependencies are
version-locked to `18.13.2`. npm signatures use key id
`SHA256:DhQ8wR5APBvFHLF/+Tc+AYvPOdTpcIDqOhxsBHRwC7U`. The SLSA provenance
attestation is
`https://registry.npmjs.org/-/npm/v1/attestations/@oh-my-pi%2fpi-coding-agent@18.13.2`.

The tree digest is SHA-256 over sorted UTF-8 path, NUL, lowercase file
SHA-256, and LF. The `18.13.2` tree digest is
`ffd81f9fcd5b79fca84ae993cfc7953dd889d2be9172d9a78e762fece8c6cb75`.
The included `18.12.6` tree digest
`13057a74c47625b83d803e0b401789685a11a77c646233ff4cec571568a9de32`
matches Research 447, as do its tarball and `dist/cli.js` digests. `omp`
and `bun` were absent from `PATH`. No artifact was installed or executed.

The identity freeze is
[`identity.json`](../../crates/swallowtail-adapter-oh-my-pi/tests/fixtures/oh-my-pi-18.13.2/identity.json).

| Version | npm published | GitHub commit | Files | Tree digest |
| --- | --- | --- | --- | --- |
| `18.12.6` | `2026-10-21T21:21:39.454Z` | `45c35e40c46166dc726ccc52f5d27f40f2449f99` | 3,342 | `13057a74c47625b83d803e0b401789685a11a77c646233ff4cec571568a9de32` |
| `18.13.0` | `2026-10-26T20:40:11.413Z` | `011ab732fa2d8657203d17f6af80d57809eb24f4` | 3,352 | `54e8b4c064d2ae5185e3bf415d154ceb07c264aa80327d80972f4b09ccb7b3c5` |
| `18.13.1` | `2026-10-27T01:04:47.127Z` | `0838d2ab4407b74e870de07e1a6780a13c7d542c` | 3,352 | `38b009f9638bd50a05ad036a4e67f3cf0a787b2c6d3964ebaf7ad678d0e8e01e` |
| `18.13.2` | `2026-10-27T18:17:00.689Z` | `e996e1c7ac476f60e02b8ed9764cf30fa8c00a8e` | 3,366 | `ffd81f9fcd5b79fca84ae993cfc7953dd889d2be9172d9a78e762fece8c6cb75` |

## Every published hop

Four stables from `18.12.6` through `18.13.2` are 3 hops. npm and GitHub
tags agree. No new GitHub-only or npm-only gap appears in this window. No
`18.12.7`–`18.12.x` tag and no extra `18.13` tag was published. The older
GitHub-only tags `18.0.2`, `18.1.7`, `18.4.7`, and `18.6.2` stay excluded.
`17.4.3` and `17.4.4` stay incompatible.

The inventory holds 4 complete path-to-SHA-256 trees. The hop ledger
recomputes every added, removed, and changed path and classifies 73
selected-source records across 49 paths. Those 73 version/path pairs match
the npm tree digests. `src/modes/rpc/rpc-frame.ts`,
`src/modes/rpc/wire/events.ts`, and `src/modes/rpc/wire/content.ts` are
byte-identical on every hop (`rpc-frame.ts` SHA-256
`9592c30c7bf12660584f67860f7abd64b752aeea15d1ce58430cfa3e5795bef3`).

The `case "..."` set in `src/modes/rpc/rpc-mode.ts` stays the Research 447
67 names through `18.13.1` and becomes 72 at `18.13.2`. The five additions
are `abort_branch_summary`, `dictation_start`, `dictation_stop`,
`navigate_tree`, and `set_label`. The selected `read` / `grep` / `glob` /
`todo` / `ask` input schemas stay byte-identical, including `QuestionItem`
and the todo schema.

Fixtures:
[`complete-tree-inventory.json`](../../crates/swallowtail-adapter-oh-my-pi/tests/fixtures/oh-my-pi-18.13.2/complete-tree-inventory.json),
[`hop-ledger.json`](../../crates/swallowtail-adapter-oh-my-pi/tests/fixtures/oh-my-pi-18.13.2/hop-ledger.json),
[`protocol.json`](../../crates/swallowtail-adapter-oh-my-pi/tests/fixtures/oh-my-pi-18.13.2/protocol.json).

| Published hop | Selected-surface class | Finding |
| --- | --- | --- |
| `18.12.6→18.13.0` | compatible optional cost cap | Optional `--max-cost`, `--warn-cost`, `--wait-async`, and `--no-context-files` are unselected. `PromptError.code` gains const `cost_cap`. `session.maxCost` defaults unset. The selected launch does not pass `--max-cost`. No command case is added. Selected commands, tool schemas, frame caps, schemes, and startup fields stay. |
| `18.13.0→18.13.1` | compatible ask result text | `ask` result text gains `noteOption`. The ask input schema, question fields, command cases, and wire definitions stay. |
| `18.13.1→18.13.2` | compatible unselected RPC additions | Adds unselected `abort_branch_summary`, `dictation_start`, `dictation_stop`, `navigate_tree`, and `set_label`, plus optional `input.secret`. The selected 14 commands, tool schemas, frame caps, schemes, and `get_state` startup fields stay. Dictation frames emit only after `dictation_start`. |

`get_state` calls `isSessionSettled` where `18.12.6` called
`isRpcSessionSettled`. The predicate is the same. Startup still reads
`model.provider`, `model.id`, `thinkingLevel`, `isStreaming`,
`isCompacting`, `steeringMode`, `followUpMode`, `interruptMode`,
`autoCompactionEnabled`, and `queuedMessageCount`.

`18.13.2` reads stdin during extension init, holds non-dialog frames, and
dispatches them in order after init. Ready is still emitted before that
hold, with the same protocol advertisement. The selected launch passes
`--no-extensions`.

`--approval-mode` still accepts `always-ask`. `always-ask-cached` stays an
unselected extra. The scheme set stays the same 20 names. `mr://` stays
the ambient `glab` read resource. `cfg://` still redacts credentials.
`ssh` stays exec. `--no-extensions` still does not disable MCP.
Prompt-text `/mcp reauth` stays reachable-unmapped.

`18.13.3` was not observed on npm or GitHub. It is the synthetic later
stable. `18.12.7` was the Research 447 synthetic point and was never an
npm version or GitHub tag. A continuous `18.0.0..=18.13.2` range includes
that number. It is not a new exclusion. Prereleases such as
`18.12.7-rc.1` stay rejected.

## Selected boundary and claim

The selected 14 commands stay `negotiate_protocol`, `set_model`,
`set_thinking_level`, `set_auto_retry`, `set_auto_compaction`,
`set_steering_mode`, `set_follow_up_mode`, `set_interrupt_mode`,
`get_state`, `get_available_models`, `prompt`, `steer`, `follow_up`, and
`abort`. The selected tools stay `read`, `grep`, `glob`, `todo`, and
`ask`. Physical frames stay capped at 1 MiB and reassembled frames at 64
MiB. Ready still advertises protocol version 1 with supported `[1, 2]`;
the route still negotiates 2. Response fields stay `id`, `command`,
`success`, and `data`. Usage stays `input`, `output`, `cacheRead`, and
`cacheWrite`. Dialog methods stay `select`, `confirm`, `input`, and
`editor`.

The five new RPC commands are host-invoked and stay unselected. Closing
this pin does not require a new Swallowtail public operation. Swallowtail
does not send `dictation_start`, so dictation frames stay unemitted.
Optional `input.secret` is forwarded only when a dialog caller sets it.
Login still rejects secret prompts. No in-tree builtin passes `secret:
true` except the extension dialog type, and the selected launch disables
extensions. Swallowtail's UI decoder ignores unknown fields and keeps
input as non-secret text. Optional `PromptError.code` `cost_cap` does not
fire while `session.maxCost` is unset and the selected argv omits
`--max-cost`.

Extend `oh-my-pi.rpc.package-window-2` as a compatible extension:
maintained `18.0.0..=18.13.2` on `oh-my-pi.rpc-v2-v18.0.0`. Keep
deprecated `17.2.9..=17.4.2` on `oh-my-pi.rpc-v2-v17.2.9`, the four
unpublished `18.x` gaps, incompatible `17.4.3`/`17.4.4`,
`AllowUnverified`, and the frozen `oh-my-pi-rpc-17.2.9` decoder corpus.
`18.13.3` is `UnverifiedNewer`. Pi axes are untouched.

No artifact was executed. There was no provider prompt, login, live
catalogue or session, credential use, installation, or host update.
