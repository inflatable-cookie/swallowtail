# Ollama 0.40.1 Attached Migration Qualification

**Task:** swallowtail#145, `ollama.attached` only  
**Observed:** 2026-10-08  
**Previous qualified ceiling:** `0.35.1`  
**Result:** compatible extension through official GitHub stable `0.40.1` on
adapter-private milestone `ollama.native-text-v1.manifest-list-runner`,
adapting the `0.40.0` inference-triggered provider-owned background
model-store migration. Prior qualified points stay on Deprecated
`ollama.native-text-v1`.

## Channel and artifact identity

The selected channel is official GitHub non-prerelease releases for
[`ollama/ollama`](https://github.com/ollama/ollama/releases). The official
[`releases/latest` endpoint](https://api.github.com/repos/ollama/ollama/releases/latest)
returned `v0.40.1` with `draft=false` and `prerelease=false`, published
`2026-10-07T23:22:59Z`, release id `406237079`. The previous Research 379 stop
observed `0.40.1` as prerelease-only; that historical evidence stays. This
observation is a new official stable hop, not a rewrite of the stop.

| Tag | GitHub release ID | Published at | Commit | Tree | Source archive SHA-256 |
| --- | ---: | --- | --- | --- | --- |
| `v0.40.0` | 396253453 | 2026-09-25T03:31:52Z | `0d0720e51fb2fd9aa58781c3d720c06d720c2e7b` | `d65785382b4d8b6dd98b25b09c59a4ec5b22d168` | `9fe3b69d3e539bd0168dfc1cbb7af9f15bac654d31744f96560f13f69672516f` |
| `v0.40.1` | 406237079 | 2026-10-07T23:22:59Z | `cf2a313a298066d572c36812e5ad30a21c0db13b` | `6ae219abaf2a32dc26c8dc14d71357b92532dec2` | `675cfca761a964d3b40d1331c006fd8a630bdb0b5a9b605ace3494e7d47c3e88` |

Complete recursive trees, blob identities, and the `0.40.0`→`0.40.1` hop
ledger are in
[dist-inventory.json](../../crates/swallowtail-adapter-ollama/tests/fixtures/ollama-0.40.1/dist-inventory.json).
Research 379 keeps the `0.34.4` through `0.40.0` trees.

No plain stable `v0.35.2`, `v0.36.*` through `v0.39.*`, or `v0.41.*` tag was
present. `v0.40.2` is a GitHub prerelease (`prerelease=true`, id 407063639);
`v0.40.2-rc0` remains a prerelease tag. Semantic prereleases stay incompatible.

The host client was `0.33.3`. No runtime was reachable. No local runtime,
model, credential, or host state was changed.

## Hop classification

`0.40.0` → `0.40.1`: 3 added, 1 removed, 17 changed, 1,413 identical files.
The hop ledger freezes those identical paths as `unchanged_path_list_sha256`
`b14ca9250dec8412ab4213f1dd6dc9bb9489a8e8ffa0a4357f722b13312558e8`.
Selected `api/types.go`, `server/model_list.go`, and
`compatmigrate/migrate.go` are byte-identical. `server/routes.go` adds
unselected `/api/balance` and `/api/usage`. `manifest/manifest.go` and
`manifest/paths.go` add store locks and Windows copy-in-place; catalogue wire
fields are unchanged. Cloud, MLX, TUI, docs, and Clef changes stay unselected.

## Adapter mapping

Tom ruling `29ebdfbc-ef0b-47f1-9a6b-47b5cd24245b` (2026-10-08, “Accept and
approve all”) accepts provider-owned background local compatibility migration
as a disclosed inference side effect. Contract 031 records that ruling.
This record is the provider-free qualification.

The adapter:

- binds the preflight model tag and manifest digest together
- pins `ggml` or `llamacpp` from the matching `/api/tags` row on show/chat
- observes extra same-tag sibling rows without substituting them
- skips unmapped non-gguf, empty-family, and unknown-runner siblings; empty family fails only when it is the selected tag and digest
- fails closed on `remote_model` / `remote_host`
- fails `swallowtail.ollama.selected_identity_drift` when the selected tag is
  present only under another digest
- issues no mutation during preparation; residency is not migration permission

No public `ProviderObservation` or residency variant was added. Runner pinning
is adapter-private on existing show/chat. Claim id
`ollama.native-runtime-window-2`, baseline `0.14.0`, exclusions `0.32.2` and
`0.32.10`, and `AllowUnverified` stay. Contract 029 requires a distinct
behavior revision for this adapter-private mapping: Maintained
`0.40.0..=0.40.1` is `ollama.native-text-v1.manifest-list-runner`. Prior
`0.14.0..=0.34.4` and `0.35.0..=0.35.1` remain `ollama.native-text-v1` and
are Deprecated. Unpublished `0.34.5` and `0.35.2` through `0.39.x` are
interior gaps. Synthetic `0.41.0` is the first visible `UnverifiedNewer`
point and carries the newest qualified revision.

## Verification boundary

No Ollama server was started. There was no provider prompt, live catalogue or
session, model download, inference, installation, credential access, host
mutation, or model-store write. Proof is source identity plus provider-free
HTTP fixtures.

Re-probe of `releases/latest` immediately before push still returned `v0.40.1`
id `406237079`. `v0.40.2` remains GitHub prerelease id `407063639`. No In-Run
Latest Movement.

Research 418 is unused on canonical GitHub main
`250263642a305a0dd0edce9ce968d2c05710d92b` (max numbered record 417). Open PRs
hold 415 and 416.
