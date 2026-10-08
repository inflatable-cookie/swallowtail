# Research 383: Cline ACP 3.0.69 Identity and Qualification

Status: promoted.

Question: can `cline.acp` extend its `3.0.55` ceiling through the current
official npm stable while keeping its selected ACP contract unchanged?

## Identity

Re-probed the official npm `cline` package on 2026-10-08. The `latest` tag
remains `3.0.69`, published 2026-10-07. The stable published sequence after
the existing `3.0.55` ceiling is `3.0.56`, `3.0.57`, `3.0.58`, then
`3.0.60` through `3.0.69`. npm has no `cline@3.0.59` artifact, so that point
remains excluded.

The fixture freezes the npm wrapper and Darwin ARM64 runtime package's exact
registry integrity, archive digest, file count, unpacked size, and complete
per-file tree for all fourteen published points. It also freezes the SLSA
publish attestations for the wrapper and all six platform runtime packages at
each point. Registry SHA-512 values match the attestation subjects, and every
platform artifact at each version names the same source commit as the wrapper.
The selected Darwin ARM64 runtime is inventoried in full; other platform
identities are bounded by their registry digests and source attestations.

The published `apps/cli/package.json` declares `@agentclientprotocol/sdk`
`^0.16.1`, and `bun.lock` resolves `0.16.1` at every published hop. Both
manifests are included in the selected-source inventory and per-hop ledger.

The `cline.acp` source inventory includes every `apps/cli/src/acp` file plus
reviewed adjacent support for output, failure/completion, request identity,
session startup, provider model metadata, usage, lifecycle, and direct runtime
provider selection. It also includes the app and shared-package manifests and
the Bun lockfile. Source commits are bound by npm SLSA provenance. Git blob
identities and exact hop path sets
are in the route fixture's
[`source-tree-inventory.json`](../../crates/swallowtail-adapter-cline/tests/fixtures/cline-acp-3.0.69/source-tree-inventory.json);
the full wrapper and selected runtime package inventories are in its
[`dist-inventory.json`](../../crates/swallowtail-adapter-cline/tests/fixtures/cline-acp-3.0.69/dist-inventory.json).

No downloaded artifact was executed or installed. No host Cline binary,
credential, account, provider model catalogue, prompt, or ACP session was
inspected or used.

## Selected route review

The selected route remains `cline --acp` over ACP v1 stdio. Its operations,
permission choices, output mapping, failure mapping, and owned-child cleanup
remain unchanged. The adapter keeps `cline.acp.stdio-v1`, the existing claim
id, and the `3.0.55` baseline.

| Hop | Selected-route classification |
| --- | --- |
| `3.0.55 → 3.0.56` | Cline adds filtering to its private provider model catalogue and assistant image/media output. Swallowtail does not expose that catalogue; its text-only ACP decoder continues to reject non-text output. The already-advertised `session/load` remains unmapped. The ACP SDK dependency remains resolved to `0.16.1`. |
| `3.0.56 → 3.0.57` | Calling-client metadata is propagated into private runtime telemetry context. |
| `3.0.57 → 3.0.58` | Cline Pass help text changes; no selected ACP behavior or ACP SDK resolution changes. |
| `3.0.58 → 3.0.60` | Prompt helper source and a provider billing-display annotation change outside this Cline ACP route. The ACP SDK remains resolved to `0.16.1`. |
| `3.0.60 → 3.0.61` | Private provider runtime changes adjust rejected-tool result text, abort propagation during startup, image capability conversion, and model overrides; they add no ACP permission, operation, or lifecycle guarantee. |
| `3.0.61 → 3.0.62` | Provider retry, error typing, model settings, and request metadata change internally. Cline's retry remains provider-internal; no ACP permission or failure mapping changes. |
| `3.0.62 → 3.0.63` | Calling-client request headers and metadata are added; telemetry polling no longer blocks ACP startup. No credentials, permissions, or ACP operations are added. |
| `3.0.63 → 3.0.64` | Internal hook context and execution metadata change; no selected ACP mapping changes. |
| `3.0.64 → 3.0.65` | Upstream `session/load` adds display-only error handling and media-aware history replay. Cline advertised the method at the 3.0.55 baseline; Swallowtail still does not map it. Other changes concern private recovery and model metadata. |
| `3.0.65 → 3.0.66` | Provider content-filter errors gain a distinct internal finish reason, and reasoning usage flows through private runtime events. ACP prompt errors remain failures; Swallowtail exposes no Cline usage. |
| `3.0.66 → 3.0.67` | Provider-managed tool output may cache oversized results internally. No Swallowtail tool, callback, permission, or session operation is added. |
| `3.0.67 → 3.0.68` | Provider request headers omit empty session identifiers and allow the metadata to be absent. No session identifier is exposed by the adapter. |
| `3.0.68 → 3.0.69` | Unknown provider finish reasons no longer imply successful completion. Cline can make one private continuation when there was no tool activity; unresolved incomplete output uses the existing ACP prompt error path. The public ACP operations and authority remain unchanged. |

`protocol.json` classifies every added, removed, or changed file in the
complete reviewed source slice at every hop. The ledger separately lists
unchanged paths and preserves the `3.0.59` npm hole.

## Decision

Extend only the ACP package claim as Maintained `3.0.55..=3.0.69`, excluding
`3.0.59`, with `AllowUnverified` for newer stable versions. Retain the claim
id, `cline.acp.stdio-v1` behavior revision, baseline, and existing exclusions.
The headless claim remains exact `3.0.55`. Provider model catalogues, generated
image/media output, request identity headers, usage, and `session/load` remain
outside Swallowtail's mapped surface. No public operation, authority, or
consumer-visible lifecycle change is introduced.

The feature-matrix `load_session` cell is `evidence_pending` under Q-006:
Cline advertises and implements `session/load`, while Swallowtail does not map
it. Q-006 names the operator gate owner and the evidence-based decision tree
into `producer_gap` or `provider_limitation`. This qualification does not
implement session loading.

## Sources

- [npm package metadata](https://registry.npmjs.org/cline)
- [Cline 3.0.69 published package](https://www.npmjs.com/package/cline/v/3.0.69)
- [Cline source commit bound by package provenance](https://github.com/cline/cline/tree/ef9430ffb4ceae9a7ab0b95d27cad8133ea7c576)
- [Contract 029](../knowledge/contracts/029-interface-version-qualification-and-compatibility.md)
- [Contract 036](../knowledge/contracts/release.md)
- [Version-currentness procedure](../knowledge/operations/version-currentness-checkpoint.md)
- [Cline ACP integration guide](../guides/cline-acp-prepared-integration.md)
- [Cline ACP 3.0.55 baseline evidence](../../crates/swallowtail-adapter-cline/tests/fixtures/cline-acp-3.0.55/README.md)
