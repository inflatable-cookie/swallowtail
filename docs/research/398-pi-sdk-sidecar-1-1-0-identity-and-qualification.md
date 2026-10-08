# Pi SDK Sidecar 1.1.0 Identity and Qualification

## Decision

Qualify only `pi.sdk-sidecar` from its previous `0.84.2` ceiling through
the official npm latest and GitHub latest stable `1.1.0`. Both documented
channels agreed on `1.1.0` when re-probed on 2026-10-08. Keep
`pi.sdk-sidecar.package-window-1`, `pi.sdk-sidecar-v1`, the baseline,
`QualifiedOnly` posture, every existing point, and every exclusion.

The package claim admits these 18 exact points only:
`0.84.2`, `0.84.3`, `0.84.4`, `0.85.0`, `0.85.1`, `0.86.0`,
`0.86.1`, `0.87.0`, `0.87.1`, `0.99.0`, `0.99.1`, `0.99.2`,
`1.0.0`, `1.0.1`, `1.0.2`, `1.0.3`, `1.0.4`, and `1.1.0`.
The other three axes stay separate and unchanged: exact Node `22.23.2`,
exact opaque `swallowtail-pi-sdk-jsonl-v1`, and the single exact source tag
`swallowtail-pi-sdk-sidecar@0.5.1`. The sidecar asset content SHA-256 is
`b4a87c838d8884813d90eb4dac7306f3c571aef7892ee112a4f448ebfd463415`; it is
frozen as artifact identity without changing the existing source-tag axis.

Unpublished points are not inferred: `0.84.5`, `0.85.2`, `0.86.2`,
`0.87.2`, `0.88.x` through `0.98.x`, `0.99.3`, `1.0.5`, and
`1.1.1` remain excluded. Prereleases and all other unlisted package points
remain rejected. Nothing here advances Pi RPC, Oh My Pi, Node, or another
route axis.

## Frozen official artifacts

Root npm tarballs were downloaded from the official registry and verified
against npm SHA-512 integrity metadata. Every regular package file was
inventoried by package-relative path and SHA-256. The table gives npm tarball
SHA-256, complete-tree SHA-256, and regular-file count. Exact integrity
strings, timestamps, package metadata, source identities, and all file
hashes are in the [identity fixture](../../crates/swallowtail-adapter-pi/tests/fixtures/pi-sdk-sidecar-1.1.0/identity.json)
and [complete inventory](../../crates/swallowtail-adapter-pi/tests/fixtures/pi-sdk-sidecar-1.1.0/dist-inventory.json).

| npm version | tarball SHA-256 | complete tree SHA-256 | files |
| --- | --- | --- | ---: |
| 0.84.2 | `95b899cd7b1a0c1f0174c7bf33ab427435e3553a7d1f4756661aa9c7f1a68ffa` | `612303b0e49bb2b3c86c0c1a6e99dcc8263957ccefc7d2e6fc936516ae9d459f` | 972 |
| 0.84.3 | `d07dc417f78a14dac376a878b6556b51961f118f79771ee375333dc51356bc75` | `af463ad048bcc55280bc4ade1bf4c76d7dc07ec1de8507063a2c6ce4f5c4c68a` | 1044 |
| 0.84.4 | `5bce766d19c3ceba18f3fbaad91c449c9f9d73981f9e3400ecef932006f06968` | `bc44cbb135890605431fa7b34e44542d883b845d474dda0d3d589dd9cac06e1c` | 1044 |
| 0.85.0 | `a0895f70a9efd9dde2a69b9cee04cb3b7c5aab68f5d47aad92b63f27a4ca13c8` | `210be5fdd953056a6f45a52c58b83ad8bcb2865bd72e90cd2c9d71596a328678` | 1249 |
| 0.85.1 | `1f498729649bdce647d1160993b4d92bf3c614cc819213bee2f91dd34f2a7af4` | `ecae935670bfed3deafae10d796c20badc56a30438d5e3cb208b0a4ce8ca77e8` | 1056 |
| 0.86.0 | `3f0f502f497c44888ddbd68a11897651b739eaabdf4b235e3a7ed9735353b0cd` | `bff7fedd633cc4fde55a3e872303d3610e0483cb8af1e93c290abfa38eb54581` | 1094 |
| 0.86.1 | `8dff93e6fa03e0d498e72a78d2c7bb5f094f5e06ee268e6abd000ba2984a0b6a` | `38920eb7b8cf3232eae59063a4f81b1c8fd6d7a6a2600fdab6206c79e2779ce5` | 1100 |
| 0.87.0 | `9a6733c0e6a31d592b53dc60df43dd0c26fa793ccf2384ed123fc17b0448866a` | `3312cce2863588249ebbea6c4608d05a814fbb5ca2a39de86f7a655c5d8c7969` | 1100 |
| 0.87.1 | `1423ee3c61e7c96464e1cbf3c8dc24d3056cb3410995c3671a98c3ecc527540f` | `d6f039d6a5ba209287537c9f2ae5d71cc00acda15060c98ad496361b6fc17217` | 1108 |
| 0.99.0 | `19a8dbd8d697a599ea972a8ba307c109a814bb9122604b2553c62006b9617724` | `7ddfa796b9c5b70dcbe08d60119ede96d19ba11a9e1e7860bead859f1c00c052` | 1226 |
| 0.99.1 | `6686592adaea19092c85c94f5d40323dbf3db141e90eb3ede9e9e87302abdd1d` | `913bf8cdb5db68249233691f66b15cffb147c3db39487abd6bea3838c157596c` | 1227 |
| 0.99.2 | `5bb197bed8e46b5352a7a940ddc868c358725b214f27f3ad4d33e77ee9832558` | `4e2034d74e6d2f9ea9a49bee3b73e6b761c6dc1ce5922907eb05d79c01ef5586` | 1228 |
| 1.0.0 | `638ed3abbe54ef70cbf8673ae4bc531e791613756aac04644cfcdcc4af0fafaf` | `14313b8e739381719bc844df12dc648dcf531924252e10551cd969e019488134` | 1243 |
| 1.0.1 | `99c2e1958ac6d4c6a36e7f1c3690ae38778bb397c63e9d611d2c09521be735c5` | `e0a3267a0269ba4a009ff7faeb42bf00e251ed45661c768e78b6ceb424dcef03` | 1243 |
| 1.0.2 | `eda5ae7875343bd902ffe55718fb65b2406d7b03abecc5cb89d8e4bb09ceeda2` | `a31b4fbf5ee8aae33dd4ee46a0637e850f1f5ddf8114d02a667ed5e38453f662` | 1243 |
| 1.0.3 | `106eadb1f823f72f012c08f23bd36e435f9e62f6c81e98a5d8f70c8a9543dd05` | `4a47c39bc26dc237e97779c44ad2f77698c255d4b4438b7fc52a8455c857180c` | 1248 |
| 1.0.4 | `04910bdae661a6529e9d6869b04006f6aad01186398b04a16c6d9793667b96c5` | `3d433e7858f53a48b6a693cb0791e38abe54c5e942b8c1ca1976553581403ab8` | 1248 |
| 1.1.0 | `09cd8a0a43dbb1d81a67346b09400b439ce71d818caba4e963ea958846a1aed4` | `62fb9de5c48a64d72f8f8fb2ad7a1f7aae15ca389dc52348b7d973cc9bdf82c8` | 1254 |

The latest package was published at `2026-10-07T22:16:26.802Z`, reports
npm SHA-1 `0ebd064bf1342f812f9676dde85b2f0b3e02b7e6`, and has npm integrity
`sha512-SeEi/4hdcHNgA9UWlefZl7ZZpm3dzi2OoxNjDHsBJ9o298LNOtbL4DGKgitlEj6uCTccvtw6f2hlCkTPVJ2RXg==`.
Its official tag is `v1.1.0` at
`abe508e1b89912adde45528136c3221eb69acdd7`; npm `gitHead` is null. The
v1.1.0 source archive SHA-256 is
`63b17b48b855e36e64c5013523acd48131ffcfa90ae48fe2f3e6fa9fe3d0da32`,
and all 118 source-map `sourcesContent` entries in the npm artifact match
that tag tree. At the other end, .84.2 npm `gitHead` equals the matching tag
commit. For .84.3 npm `gitHead` and the release-tag commit differ by two
workflow/changelog-only commits; all 220 packaged source-map sources match
the v0.84.3 tag archive, SHA-256
`00e138713eba79ca06890e507f87b0b7007fa3be38eac06ed0b3b63b16dd979a`.

## Published-hop ledger

For each adjacent published pair the inventory records every package-relative
path as added, removed, changed, or identical. The counts below pair with
exact path sets and hashes in the [hop inventory](../../crates/swallowtail-adapter-pi/tests/fixtures/pi-sdk-sidecar-1.1.0/dist-inventory.json).
The last column counts compiled files and source maps feeding selected
behavior; their source path, classification, and exact content hash are in
the [source ledger](../../crates/swallowtail-adapter-pi/tests/fixtures/pi-sdk-sidecar-1.1.0/protocol.json).

| Hop | Added | Removed | Changed | Identical | Selected files |
| --- | ---: | ---: | ---: | ---: | ---: |
| 0.84.2 → 0.84.3 | 72 | 0 | 209 | 763 | 30 |
| 0.84.3 → 0.84.4 | 7 | 7 | 94 | 943 | 11 |
| 0.84.4 → 0.85.0 | 246 | 41 | 207 | 796 | 27 |
| 0.85.0 → 0.85.1 | 20 | 213 | 48 | 988 | 0 |
| 0.85.1 → 0.86.0 | 61 | 23 | 233 | 800 | 32 |
| 0.86.0 → 0.86.1 | 10 | 4 | 41 | 1049 | 2 |
| 0.86.1 → 0.87.0 | 4 | 4 | 109 | 987 | 23 |
| 0.87.0 → 0.87.1 | 14 | 6 | 60 | 1034 | 2 |
| 0.87.1 → 0.99.0 | 139 | 21 | 570 | 517 | 37 |
| 0.99.0 → 0.99.1 | 4 | 3 | 17 | 1206 | 2 |
| 0.99.1 → 0.99.2 | 29 | 28 | 109 | 1090 | 10 |
| 0.99.2 → 1.0.0 | 30 | 15 | 106 | 1107 | 7 |
| 1.0.0 → 1.0.1 | 30 | 30 | 115 | 1098 | 10 |
| 1.0.1 → 1.0.2 | 14 | 14 | 24 | 1205 | 4 |
| 1.0.2 → 1.0.3 | 35 | 30 | 42 | 1171 | 4 |
| 1.0.3 → 1.0.4 | 14 | 14 | 85 | 1149 | 10 |
| 1.0.4 → 1.1.0 | 40 | 34 | 142 | 1072 | 8 |

### Selected surface classifications

- All points load the same package entry `dist/index.js`; the sidecar checks
  only established SDK exports required to construct a runtime, sessions,
  settings, and model lookup. New top-level exports do not create a sidecar
  operation.
- The selected facade remains catalogue, new/load/resume session, prompt,
  steer, follow-up, abort, state, and close over the existing private wire.
  The literal read-only tool set remains `read`, `grep`, `find`, and
  `ls`; the SDK 1.1 tool-modifier API is not passed. The bounded attachment
  path remains text/PNG, and usage remains the four integer counters
  `input`, `output`, `cacheRead`, and `cacheWrite`.
- SDK 0.86.0 introduced CacheWarmer, whose default streaming mode may replay a
  provider request after `agent_settled` and append usage. The sidecar now
  sets `cacheWarming: "off"` in its private in-memory settings alongside the
  existing retry and compaction disables. The existing execution boundary
  stays intact; no public operation is added.
- At 0.99.0, pi-ai changes the model registry key to
  `chat:claude-opus-4-5`; the selected row's `id`, provider, API, base URL,
  input types, reasoning capability, costs, context window, and token ceiling
  remain frozen. SDK model lookup still selects by provider plus model `id`,
  so this registry-key prefix does not change the prepared route. The precise
  `anthropic/claude-opus-4-5` Research 228 row and five admitted modes remain.
- SDK 1.0.4 adds structured tool content for codemode callers; the sidecar
  continues to project existing text and image blocks. SDK 1.1.0 adds an
  `aborted` property to `agent_settled`; the private wire maps the established
  event and does not project that property. Additive APIs and exports do not
  change the selected wire or lifecycle.

The source and dependency ledgers classify every changed file selected for
wire, lifecycle, failure, permission, usage, configuration, model, tool, and
attachment behavior. Unmapped package changes remain bounded by the complete
tree and per-hop inventory. Exact corresponding `pi-ai` and
`pi-agent-core` artifacts and selected model/runtime source hashes are in
[dependency-surface.json](../../crates/swallowtail-adapter-pi/tests/fixtures/pi-sdk-sidecar-1.1.0/dependency-surface.json).
The root package uses caret ranges and ships `npm-shrinkwrap.json` through
1.0.0, then omits it from 1.0.1 through 1.1.0. The host-resolved transitive
dependency tree is not asserted. No live provider proof transfers from these
static artifacts.

## Implementation boundary and proof

The package claim retains earlier supported points and the exact behavior
revision. The sidecar accepts only the 18 exact package strings above, reports
`1.1.0` by default, and rejects unlisted points. The existing
`0.84.2`/Node/`0.5.1` source-tag/wire tuple remains valid. The source-tag
axis and current tag remain one exact point while the asset's content hash is
frozen separately in the complete sidecar-source identity.

Static proof inspects downloaded public npm and GitHub artifacts only. No
official package was executed; no prompt, live catalogue, provider session,
credential, installation, or host update occurred. Route capabilities, wire
commands, Node version, session lifecycle, selected model row, exclusions,
and exact live gates remain unchanged. Research 228, Research 181, Pi RPC,
and sibling route qualifications are not promoted as evidence for these
package points.

The applicable prepared route and exact point list are documented in
[the prepared integration guide](../guides/pi-sdk-sidecar-prepared-integration.md),
[the provider route matrix](../guides/provider-route-matrix.md), and the
feature matrix.

## Identity fixture

The four JSON files have SHA-256 values checked by
`pi_sdk_sidecar_artifact_identity`. Its test pins the official artifact
points and tarball digests, complete tree counts and digests, every hop count,
the exact selected-source key set, and each hop's selected file-path set.
Existing sidecar selection, decoder/producer, session lifecycle, and
historical Pi RPC identity fixtures remain in the narrow Effigy selector.
