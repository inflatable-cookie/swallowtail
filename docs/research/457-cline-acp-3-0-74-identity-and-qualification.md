# Research 457: Cline ACP 3.0.74 Identity and Qualification

Status: promoted.

Question: can `cline.acp` extend its `3.0.70` ceiling through the current
official npm stable while keeping its selected ACP contract unchanged?

## Identity

Re-probed the official npm `cline` package on 2026-10-26. The `latest` tag
is `3.0.74`, published `2026-10-26T04:04:22.234Z`. The pre-push recheck the
same day still reports that tag. The stable published hops
after the `3.0.70` ceiling are `3.0.71` (`2026-10-13T08:57:00.576Z`),
`3.0.72` (`2026-10-16T05:36:37.025Z`), `3.0.73`
(`2026-10-17T03:59:46.162Z`), and `3.0.74`. npm still has no `cline@3.0.59`
artifact. `3.0.75` is not published.

GitHub's latest stable release is editor product `v4.1.26`. That is not this
npm CLI axis. Annotated CLI tags `cli-v3.0.70` through `cli-v3.0.74` resolve
to the same commits as the npm SLSA provenance for those versions.

The fixture freezes the npm wrapper and Darwin ARM64 runtime package's exact
registry integrity, archive digest, file count, unpacked size, and complete
per-file tree for `3.0.70` through `3.0.74`. The `3.0.70` trees match the
Research 397 corpus. It also freezes SLSA publish attestations for the
wrapper and all six platform runtime packages. Registry SHA-512 values match
the attestation subjects, and every platform artifact at each version names
the same source commit as the wrapper.

`apps/cli/package.json` declares `@agentclientprotocol/sdk` `^0.16.1`, and
`bun.lock` resolves `0.16.1` at every hop in this window.

No downloaded artifact was executed or installed. No host Cline binary,
credential, account, provider model catalogue, prompt, or ACP session was
inspected or used.

## Selected route review

The selected route remains `cline --acp` over ACP v1 stdio. `AcpAgent` still
implements `initialize`, `newSession`, `loadSession`, `prompt`, `cancel`,
`setSessionMode`, `unstable_setSessionModel`, `setSessionConfigOption`, and
`authenticate`. Swallowtail still maps only `initialize`, `session/new`,
`session/set_config_option`, `session/prompt`, and `session/cancel`.

| Hop | Selected-route classification |
| --- | --- |
| `3.0.70 → 3.0.71` | ACP accepts optional reasoning and applies it only when the caller passes it. The CLI ACP early-return calls `resolveCliReasoning` without persisted provider settings, so only explicit `--thinking` turns reasoning on. Swallowtail spawns `cline --acp` with no extra argv, and the agent then keeps `thinking: false`. Private catalogue default ids change. The ACP SDK remains `0.16.1`. |
| `3.0.71 → 3.0.72` | No ACP server file changes. Provider request-start and stream retries split, and private request ids are attached to telemetry. ACP prompt errors still fail through the existing mapping. The error-classification edit is comment-only. |
| `3.0.72 → 3.0.73` | No ACP server file changes. The selected-slice behavior edit is an unselected OpenCode Go base URL. Manifests and the Bun lock update private dependencies. The ACP SDK remains `0.16.1`. |
| `3.0.73 → 3.0.74` | Config-option construction moves to `config-options.ts`. The mode option stays `id=mode`, `category=mode`, values `plan` and `act`. `setSessionConfigOption` still accepts only those two mode values and returns `configOptions`. The model option stays `id=model`, `category=model`. The provider option category changes from `model` to `_provider`; Swallowtail's model observation matches `id=model`, so that row was never the model option. The organization category changes from `account` to `_account` and stays unmapped. Private MCP client code adds apps capabilities and `resources/read` (blob `16849bb01963bc415a3de06572d308b1afd7767f` to `76f82111bb5f8356dabda10a07d2fd8970f7454c`). Swallowtail sends `mcpServers: []` and maps no client MCP. `tool-call-delta` is dropped by the runtime event adapter and is not an ACP session update. |

At `3.0.74` every platform package's registry file count drops from 142 to 2.
The selected Darwin ARM64 tree keeps `bin/cline` and `package.json` and
removes the hub webview plus `extensions/plugin-sandbox-bootstrap.js`. The
published wrapper README says compiled binaries embed the plugin bootstrap,
SDK modules, and dashboard assets. The wrapper `bin` stays `./bin/cline`, and
the six optional platform packages stay version-pinned. Those removed files
are not on the selected `cline --acp` path. The wrapper script itself is
byte-identical across this window; only `package.json`, and at `3.0.74`
`README.md`, change.

## Decision

Extend only the ACP package claim as Maintained `3.0.55..=3.0.74`, excluding
`3.0.59`, with `AllowUnverified` for newer stable versions. Retain the claim
id, `cline.acp.stdio-v1` behavior revision, and the `3.0.55` baseline.
`3.0.75` stays `UnverifiedNewer`. The independently qualified headless claim
remains Maintained `3.0.55..=3.0.70` except unpublished `3.0.59`, with
`QualifiedOnly`. Provider model catalogues, generated image/media output,
request identity headers, reasoning selection, client MCP, and `session/load`
remain outside Swallowtail's mapped surface. No public operation, authority,
or consumer-visible lifecycle change is introduced.

The feature-matrix `load_session` cell stays `evidence_pending` under Q-006.
This qualification does not implement session loading.

## Sources

- [npm package metadata](https://registry.npmjs.org/cline)
- [Cline 3.0.74 published package](https://www.npmjs.com/package/cline/v/3.0.74)
- [Cline source commit bound by package provenance](https://github.com/cline/cline/tree/071e1b25b48f9ef5dff7fc32085e6a16f02bf02e)
- [Contract 029](../knowledge/contracts/029-interface-version-qualification-and-compatibility.md)
- [Version-currentness procedure](../knowledge/operations/version-currentness-checkpoint.md)
- [Cline ACP integration guide](../guides/cline-acp-prepared-integration.md)
- [Research 397](397-cline-acp-3-0-70-identity-and-qualification.md)
- [Cline ACP 3.0.74 evidence](../../crates/swallowtail-adapter-cline/tests/fixtures/cline-acp-3.0.74/README.md)
