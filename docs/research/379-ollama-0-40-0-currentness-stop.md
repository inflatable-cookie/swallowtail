# Ollama 0.40.0 Currentness Stop After 0.35.1

**Task:** swallowtail#106, `ollama.attached` only  
**Observed:** 2026-10-08  
**Previous qualified ceiling:** `0.34.4`  
**Result:** compatible extension through exact stable `0.35.1`; current official `0.40.0` remains `UnverifiedNewer` pending an operator ruling.

## Channel and artifact identity

The selected channel is official GitHub non-prerelease releases for
[`ollama/ollama`](https://github.com/ollama/ollama/releases). The official
[`releases/latest` endpoint](https://api.github.com/repos/ollama/ollama/releases/latest)
returned release `v0.40.0` with `draft=false` and `prerelease=false`. Published
stable tags after the previous ceiling are `v0.35.0`, `v0.35.1`, and
`v0.40.0`. The exact current tag, commit, tree, and source archive digest are
frozen below and in the complete [distribution inventory](../../crates/swallowtail-adapter-ollama/tests/fixtures/ollama-0.40.0/dist-inventory.json).

| Tag | GitHub release ID | Published at | Commit | Tree | Source archive SHA-256 |
| --- | ---: | --- | --- | --- | --- |
| `v0.34.4` | 394248783 | 2026-09-23T02:24:43Z | `b2da9e468af2479058ae18c6d908ed29de410684` | `525c37066242cc8feb9f7d909927270b168c6a7c` | `c265dfff78cd2fe909f9f9fa0bb21b1e34dfcd354e1bfa13ed8eac2cfce90940` |
| `v0.35.0` | 398626503 | 2026-09-28T21:23:22Z | `cc4069396f3ad2c370c53eed2e4a42ac13adab84` | `d663e3c722adf4ddde99352c02d3ffee583cab65` | `2d24c59fe4aec09fa68994816cd24e233dff4d57cbc31ee9b46973fb0b3f55e7` |
| `v0.35.1` | 399431834 | 2026-09-29T20:14:22Z | `b0c1ca4f7549d7acdfa52a7dcffc934bc63a43ce` | `e9516fd9467dfe72505fb2abde9b26e0c5652a01` | `757839465b3eee23d143bddcb655180f148299b72399ce2585d22cb2a342d07e` |
| `v0.40.0` | 396253453 | 2026-09-25T03:31:52Z | `0d0720e51fb2fd9aa58781c3d720c06d720c2e7b` | `d65785382b4d8b6dd98b25b09c59a4ec5b22d168` | `9fe3b69d3e539bd0168dfc1cbb7af9f15bac654d31744f96560f13f69672516f` |

The `v0.40.0` release record also returned `created_at=2026-10-06T17:46:39Z`,
after its `published_at`, and the tag commit has the same October 6 timestamp.
This metadata discrepancy is preserved as observed. The exact current tag
identity, not the release timestamp, determines the inspected source tree.

The tag query found no plain stable `v0.34.5`, `v0.35.2`, `v0.36.*` through
`v0.39.*`, or `v0.40.1` tag. The `v0.40.1-rc0` ref is prerelease-only and has
no GitHub release object. Existing plain prerelease exclusions `0.32.2` and
`0.32.10` remain unchanged. The host client was `0.33.3` with executable
SHA-256 `75b6b83ab9c06712c7097807c91d2bb86a3ef3dd143d59359a5fbb3314970c0a`;
no runtime was reachable. No local runtime, model, or host state was changed.

## Complete hop inventory

`dist-inventory.json` keeps every recursive Git tree entry, its mode, type,
object ID, and file size. Each changed file has a per-hop classification and
before/after Git blob identity; unchanged file counts are derivable from the
complete point trees. The selected source paths have exact Git blob identities
for every frozen point. Counts below include file entries; directory tree
objects remain in each full snapshot.

| Version hop | Added files | Removed files | Changed files | Identical files |
| --- | ---: | ---: | ---: | ---: |
| `0.34.4` → `0.35.0` | 25 | 8 | 41 | 1,261 |
| `0.35.0` → `0.35.1` | 8 | 0 | 38 | 1,289 |
| `0.35.1` → `0.40.0` | 96 | 0 | 110 | 1,225 |

The `0.35.0` selected-path changes are limited to a `TypicalP` comment and
request warning behavior; this adapter sends no `typical_p` field. Its new
SystemOne route is separate from the five selected paths. At `0.35.1`, Ollama
adds public decision-model capabilities; the existing decoder accepts only
`completion` and `thinking`, so a decision-only selected model remains
fail-closed. The ordinary text facade gains no operation or capability.

At `0.40.0`, API structs add optional `runner`, `all_manifests`, and manifest
summary fields. The adapter sends neither `runner` nor `all_manifests`, and
unknown additive fields remain ignored. Manifest-list models are outside the
mapped catalogue shape: `/api/tags` now returns one row per child runner.
Source identities for all changed files and per-hop classifications remain in
the inventory; new MLX, decision, renderer, and model-authoring changes are
provider-internal and are not mapped as Swallowtail operations.

## Currentness stop and exact adaptation needed

`0.40.0` changes selected chat scheduling. `server/routes.go` calls
`compatmigrate.StartLocalCompatibilityMigration` for a selected model. That
starts a best-effort background conversion for supported legacy GGUF families.
The migration writes converted model blobs and manifest-list entries, plus a
legacy anchor, into Ollama's local model store. After migration, the selected
catalogue can expose the manifest-list child rows. This is a model-store
mutation and catalogue identity change beyond inference-caused residency; no
local model observation or live call was used to infer it.

The compatibility claim therefore keeps baseline `0.14.0`, ID
`ollama.native-runtime-window-2`, behavior `ollama.native-text-v1`, exclusions
`0.32.2` and `0.32.10`, and `AllowUnverified`. It adds only the maintained
segment `0.35.0..=0.35.1`. Unpublished `0.34.5` remains an interior
incompatibility. `0.35.2` and absent later stable tags are not maintained and
remain visibly `UnverifiedNewer` under the existing posture. `0.40.0` is also
reported as `UnverifiedNewer`; `AllowUnverified` remains unchanged, so the
assessment does not block an attempt by itself.

Tom's ruling is needed on whether `ollama.attached` inference may trigger
Ollama's background local model-store migration. If allowed, define how the
adapter reports changed model identity, multiple runner rows, digest binding,
and migration failures, then qualify those paths with deterministic fake
fixtures while keeping provider lifecycle ownership downstream. If disallowed,
authorize a consumer-visible gate or opt-in before chat. No new capability,
public operation, or authority behavior was invented here.

## Verification boundary

The local Ollama client was queried with `--version` only. No server was
started; there was no provider prompt, live catalogue/session, model download,
inference, installation, credential access, or host mutation. The source
comparison used public GitHub release metadata, recursive tag trees, and
downloaded source archives without executing them.

## Task validation record

An initial targeted run exited 100 after 18 tests passed and two assertions
failed. The failures showed that `parse_version` rejects interior `0.34.5`
before returning a binding and `AllowUnverified` classifies `.35.2` and later
points as `UnverifiedNewer`; those expectations were corrected. On the
authorized continuation, the final rebased code passed the targeted selector:
54 passed, 16 skipped. `effigy check:current-ollama-attached` also passed.

The first push attempt stopped at a canonical Research 373 number collision.
Repair 142 was merged before the first continuation. At that refresh, canonical
main assigned 373 to Claude Agent ACP, 374 to Claude Code headless, 375 to
Qwen Code, and 376 to OpenCode HTTP; the refreshed
`effigy qa:docs:research:numbers` and `effigy qa:docs` selectors passed then.
Canonical main subsequently allocated Research 377 to OpenCode ACP and 378 to
Claude Code response-only. This Ollama record was renumbered to the next free
id, Research 379, after that movement was found.

Final `effigy qa:routes` passed route, lifecycle, feature, activity, and
historical boundary checks. `effigy skill run northstar/retired-concepts`
passed all 3 retired concepts.
