# Bedrock Runtime SDK 1.153.0 and Catalogue 1.166.0 Identity

Task 223 removes `pin-utils` from the resolved workspace dependency graph by
qualifying the corrected official AWS SDK graph for both Bedrock axes at once:
`bedrock.runtime` extends from `1.148.0` through official stable
`aws-sdk-bedrockruntime 1.153.0`, and `bedrock.catalogue` extends from
`1.161.0` through official stable `aws-sdk-bedrock 1.166.0`, both observed on
2026-11-09. The two axes keep separate artifacts, selected surfaces, service
revisions, and claims; they share one coordinated dependency repair because
both SDK crates resolve the same Smithy runtime graph. Scope ruling:
Tom selected the bounded Bedrock security repair on the board
(decision51330692, 2026-11-05) and authorized dispatch on 2026-11-09.

## Scope ruling and security gate

[RUSTSEC-2026-0441](https://github.com/RustSec/advisory-db/blob/main/crates/pin-utils/RUSTSEC-2026-0441.md)
marks `pin-utils` unmaintained with no patched version. It entered the resolved
workspace graph only through `aws-smithy-http 0.64.1`,
`aws-smithy-runtime 1.16.0`, and `aws-smithy-types 1.8.1`; no other workspace
crate depends on it. PR 541's `effigy security:dependencies` run established
the gate failure. The upstream removal is
[`release-2026-10-20`](https://github.com/awslabs/aws-sdk-rust/releases/tag/release-2026-10-20),
which requires moving both SDK crates across the Smithy compatibility line
together: upgrading one SDK alone would leave the other holding the old line
and the advisory. The repair uses supported registry releases only — no vendor
fork, no `[patch]`, no resolver override, and `deny.toml` advisory ignores
stay empty. The gate passes with the ordinary configuration after the repair.

## Official channels and artifacts

The official channels are the crates.io stable release sets for
[`aws-sdk-bedrockruntime`](https://crates.io/api/v1/crates/aws-sdk-bedrockruntime/versions)
and [`aws-sdk-bedrock`](https://crates.io/api/v1/crates/aws-sdk-bedrock/versions),
selected by maximum stable non-yanked semver. At `2026-11-09T16:56:07Z`
(Runtime) and `2026-11-09T16:56:21Z` (catalogue), the versions API reported
`1.153.0` and `1.166.0` as the respective maxima, non-yanked, with
`1.154.0`, `1.153.1`, `1.167.0`, and `1.166.1` absent. A pre-push re-probe
at `2026-11-09T17:20:41Z` returned the same maxima with the same checksums
and no newer stable, so neither qualified point moved during the
qualification. Both channels ride the
same awslabs/aws-sdk-rust release train: the `1.149.0` and `1.162.0` artifacts
(2026-10-20) share source commit `b964fc77de60b0a492ff788c40bb66524fd26a44`,
and the train steps through the same commit ladder to `44d6380d22c14bd5c89c59ceafcda7bc4da5b6f7`
at `1.153.0`/`1.166.0` (2026-10-30). Each `.crate` download matched the
checksum returned by the versions API. No artifact was executed; no AWS call,
credential, prompt, host SDK installation, or host mutation was used.

| Version | Crate SHA-256 (runtime) | Crate SHA-256 (catalogue) | Packaged source commit | Published | Yanked |
| --- | --- | --- | --- | --- | --- |
| 1.148.0 / 1.161.0 (base) | `3f2542c0f038223ba2f9aefe3bd38c7395ccc6899d8fc58a99b8ddfeab3354f5` | `254169f7bd61c2189a0142067000ed89ab15a032b8b8cebf426fabeb425ad611` | `7101aefb7632e44cce586886a1df595151409b5f` | 2026-10-01 | no |
| 1.149.0 / 1.162.0 | `59f4e05517820d9e79c115695bd4ac9005a9ebbd3d6aff5f8d3d4731325b3b21` | `b0d44e3919d31d8d1b12798c304df84e93fa2490341e32e5fb1f33d58d5763d7` | `b964fc77de60b0a492ff788c40bb66524fd26a44` | 2026-10-20 | no |
| 1.150.0 / 1.163.0 | `c37e1cf8f963668fcb6d45313799b97616f65e431cf5e65e87194147e2b18bdd` | `ec6a0e49550d3bbcdcc51538a4e776a61f0cb1958dde09c7630cb396bfed7956` | `133c93f9af3783be5d41163784163cc80068ae1a` | 2026-10-21 | no |
| 1.151.0 / 1.164.0 | `f008e2a7a47e6c8f4a61c3783def307c6817532c1e05ce417da51657088fbe02` | `1dc81814f1abbb184e1da290cc982aaffefb5c1be145cb761668b5f5fc0255c4` | `3a3f39bd932690cab6d2321b677e97a24b687753` | 2026-10-22 | no |
| 1.152.0 / 1.165.0 | `dd3ff1fc9e5e8a2ac61bcc6ac82b13a57d431d6804d4461f9570a3b788b9549a` | `086d90d6919e0d36b1ed5d83b3773e15a98f8cfa6d30b264936b0f7f1215dec6` | `c42d314c972a95e7cfd9583eef5113a2e1d6dccb` | 2026-10-28 | no |
| 1.153.0 / 1.166.0 | `89b8aeb4b1b6d5f9d377264cff21b4e4d773e66ee3c5582b4099a8fbc702bdb3` | `6a2046512ecd16913d858634bfa07fe2769b794f852989940f2d5fcab7877968` | `44d6380d22c14bd5c89c59ceafcda7bc4da5b6f7` | 2026-10-30 | no |

All new artifacts declare `rust-version = "1.94.1"`, inside the existing
Bedrock boundary; the workspace requirement stays `1.95`.

## Runtime axis: selected ConverseStream surface unchanged

The companion
[`bedrock-runtime-1.153.0` fixture](../../crates/swallowtail-adapter-bedrock/tests/fixtures/bedrock-runtime-1.153.0/dist-inventory.json)
freezes the complete SHA-256 tree manifest for `1.148.0` through `1.153.0`
(544 packaged files per version, no additions or removals), both hashes for
each of the 33 classified changed-file records, and per-version hashes of the
21 selected source files. The generated ConverseStream input, message,
content, role, inference-configuration, EventStream, metadata, metrics, and
error types are byte-identical from `1.148.0` through `1.153.0`. The
`protocol.json` freeze keeps the exact selected boundary: one POST to
ConverseStream with model ID, a single user text message,
`inferenceConfig.maxTokens = 7`, optional members absent, one SDK attempt,
typed EventStream decoding, bounded text/usage projection, and fail-closed
unknown-event handling. The fake-transport regression drives the real
qualified `1.153.0` SDK and asserts the captured request body equals the
frozen body. The hop changes are bounded:

- `1.149.0` refactors two unselected InvokeModel response-header checks
  (`shape_invoke_model.rs`, `shape_invoke_model_with_response_stream.rs`)
  into a bounded `let skip = { .. }` form for `clippy::blocks_in_conditions`;
  the route selects only ConverseStream.
- `1.149.0` through `1.152.0` otherwise change package metadata, README,
  crate lock, and the `src/lib.rs` version doc comment.
- `1.153.0` switches the `#[cfg(test)]` endpoint rule-set test builders to
  `build_without_validators`; the generated resolver rules and the adapter's
  explicit endpoint configuration are unchanged.

The semantic claim `amazon-bedrock.runtime-sdk-window-1` keeps exact
`1.136.0`, the `1.137.0`/`1.138.0` gaps, and the yanked `1.144.0` exclusion,
and extends its maintained segment to `1.139.0..=1.153.0` under the same
`amazon-bedrock.runtime-sdk-1` behavior revision. Later stables are
`AllowUnverified`; `1.154.0` is the synthetic unpublished successor.

## Catalogue axis: selected ListFoundationModels surface unchanged

The companion
[`bedrock-control-plane-1.166.0` fixture](../../crates/swallowtail-adapter-bedrock/tests/fixtures/bedrock-control-plane-1.166.0/dist-inventory.json)
freezes the complete tree manifest for `1.161.0` through `1.166.0` (1,470
files at `1.161.0`, 1,475 from `1.162.0`), both hashes for each changed file,
per-version hashes of the 25 selected source files, and the five added-file
digests. The selected ListFoundationModels operation, response parser,
FoundationModelSummary/lifecycle/modality/inference/customization types,
error set, client, config, retry, and operation modules are byte-identical
across every hop. `1.162.0` adds GuardrailContextualGrounding tier types with
re-exports in `src/protocol_serde.rs`, `src/types.rs`,
`src/types/builders.rs`, and `src/serde_util.rs`; guardrails are not part of
the selected surface and the adapter maps no guardrail type, so the selected
behavior is unchanged and the protocol freeze's request (GET
`/foundation-models`, no filters, adapter-side single-entry bound, one
provider request, one SDK attempt) carries over verbatim. `1.166.0` applies
the same `#[cfg(test)]`-only endpoint test-builder change as Runtime.
`1.163.0`–`1.165.0` change package metadata only.

The semantic claim `amazon-bedrock.catalogue-sdk-window-1` keeps exact
`1.148.0`, `1.150.0..=1.155.0`, the `1.149.0` unqualified hole, and the
yanked `1.156.0` exclusion, and extends its maintained segment to
`1.157.0..=1.166.0` under the same `amazon-bedrock.catalogue-sdk-1` behavior
revision. Later stables are `AllowUnverified`; `1.167.0` is the synthetic
unpublished successor.

## Corrected Smithy dependency graph and pin-utils removal

The declared dependency minimums of both target crates cross the Smithy
compatibility line in the same release train: `aws-smithy-schema` moves from
the `0.2.x` line to `1.1.2`, `aws-smithy-json` from `0.63.x` to `0.65.1`,
`aws-smithy-http` to `0.64.3`, `aws-smithy-runtime` to `1.16.2`, and
`aws-smithy-types` to `1.9.x`. The dev-dependency
`aws-smithy-http-client = "=1.5.0"` conflicts with the corrected line
(`aws-smithy-runtime 1.16.2` requires `^1.5.2`), so it moves to
`=1.5.2` with its `test-util` fake-transport features intact. These are the
minimal necessary manifest changes; `aws-credential-types = "1.3.0"` still
resolves because the corrected graph declares `^1.3.1` inside the same
caret window.

The re-resolved workspace lock carries exactly one Smithy closure and drops
`pin-utils 0.1.0` entirely:

| Package | Resolved | Registry crate SHA-256 |
| --- | --- | --- |
| `aws-credential-types` | 1.3.1 | `576e8da35751dc85409ec1cd15999f692c277a1fbe9336780fe6f15e49dd637d` |
| `aws-runtime` | 1.10.2 | `9a0cc2144962cc5e358467b16e3d82981b1d424a7c3d34602be0036103814a1a` |
| `aws-sdk-bedrock` | 1.166.0 | `6a2046512ecd16913d858634bfa07fe2769b794f852989940f2d5fcab7877968` |
| `aws-sdk-bedrockruntime` | 1.153.0 | `89b8aeb4b1b6d5f9d377264cff21b4e4d773e66ee3c5582b4099a8fbc702bdb3` |
| `aws-sigv4` | 1.6.4 | frozen in Cargo.lock |
| `aws-smithy-async` | 1.3.1 | `de681a9df7ff79823b759ee17061076ec049cd61a8fa28ece4293d89c9f6964e` |
| `aws-smithy-eventstream` | 0.61.6 | frozen in Cargo.lock |
| `aws-smithy-http` | 0.64.3 | `19af609dd5edf8b056cd0bb001c706c8b3b1458055328b334ec13d7f46f6aaa8` |
| `aws-smithy-http-client` | 1.5.2 | `859a81364f8185349c584fb41cd8765a400cf982f447adada590cf008a9f3b85` |
| `aws-smithy-json` | 0.65.1 | `b45f9fc77ffe92c92c341165a01a565bd30c39ecf5aa8c0719b41719009604f9` |
| `aws-smithy-runtime` | 1.16.2 | `4cb80fb19c084da5fca9d797845735e184445e5b958599440d67529bf6ef1b64` |
| `aws-smithy-runtime-api` | 1.20.0 | `d06a8615310f03c73ea8eea607ee1a9cb17b7ebe055b9152feda1d86831c215c` |
| `aws-smithy-schema` | 1.2.0 | `f7ef30d39e131651359b60a1c560de9a9502a4c3754172ab2d6e8734fe21bcba` |
| `aws-smithy-types` | 1.9.3 | `6c3a040d8b963838a01d7d239b0118357388d0ab881857bb5fc2dd780bb87957` |
| `aws-types` | 1.7.0 | `6ba0d970d3947bb064d2ae1d823efa186c1dbde88f88afa8f6549946e5d8deee` |

No old dependency path retains `pin-utils`: the only three dependents were the
three Smithy crates above, both target SDK crates now declare the corrected
line (schema `^1.1.2`, json `^0.65.1`, incompatible with the old `0.2.x`/`0.63.x`
packages), and no other workspace crate depends on any `aws-smithy-*` package.
The new catalogue identity test fails the build if `"pin-utils"` reappears in
`Cargo.lock`, and `effigy security:dependencies` passes with `deny.toml`
advisory ignores unchanged (`ignore = []`).

## Claim decision

Both qualifications are compatible extensions on their existing claim ids,
behavior revisions, baselines, holes, and yanked exclusions. No public
operation, capability, access profile, authority, lifecycle, or consumer
contract is added or narrowed; the exact service revisions
`bedrock-runtime-converse-stream` and `bedrock-list-foundation-models` are
unchanged. The fixture identity and protocol JSON files freeze the official
metadata and selected wire/lifecycle bounds; the identity tests reconstruct
every complete tree manifest from the frozen base and deltas, check the
classifications cover each changed or added path exactly, and assert the
selected files stayed byte-identical.
