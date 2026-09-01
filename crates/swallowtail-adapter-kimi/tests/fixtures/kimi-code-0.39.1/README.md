# Kimi Code 0.39.1 currentness corpus

Secret-free identity corpus for official npm and GitHub
`@moonshot-ai/kimi-code@0.39.1`, frozen before Swallowtail moved any
`kimi-code.executable` claim. It covers the two installed-harness axes only:
`kimi-code.acp` and `kimi-code.headless`. `kimi-code.local-server` is a
separate currentness family and is untouched here even though it ships in the
same package.

`identity.json` holds npm, GitHub tag/commit, tarball, platform-archive, and
extracted-artifact identity for `0.39.1`, the publication adjacency through
`0.39.0`, and a revalidation of the frozen `0.38.0` corpus. Every `0.38.0`
digest recomputed in this run matches `kimi-code-0.38.0`. Host `kimi 0.34.0` is
byte-identical to the official `0.34.0` darwin-arm64 extracted artifact whose
archive matches the official release manifest. The host was not installed,
updated, replaced, or executed, and no host path appears here.

Downloaded official archives stayed in disposable scratch space and were never
executed. Identity comes from digests, git blobs, and static extraction only.

## Executing path

Swallowtail launches `kimi acp` and
`kimi --model <model> --prompt <content> --output-format stream-json`. It never
sets `KIMI_CODE_LEGACY_FLAG`. Both naked paths therefore run agent-core-v2:
ACP through `packages/acp-server`, headless through
`apps/kimi-code/src/cli/v2/run-v2-print.ts`. `packages/acp-adapter` and the
legacy v1 print body are reachable only behind that env flag and are not the
selected surface. Research 179 named `packages/acp-adapter` as the ACP
evidence surface; `protocol.json` records the corrected selected surface. The
correction changes no qualified point, because every mapped `acp-server` blob
is byte-identical from `0.37.2` through `0.39.1`.

## Selected protocol

Mapped ACP surfaces — `server.ts` initialize, capabilities and auth methods,
`events-map.ts`, `modes.ts`, `config-options.ts`, `approval.ts`, and
`model-catalog.ts` — are byte-identical from `0.37.2` through `0.39.1` in both
the `acp-server` and `acp-adapter` copies. Mapped headless v2 emission —
`prompt-render.ts`, the `system.version` preamble writer, the
`session.resume_hint` writer, tool-output stringification, and the event
dispatch table — is byte-identical from `0.38.0` through `0.39.1`.

Four `acp-server` files and one v2 runner file changed. `session.ts` is a
doc-comment edit. `start.ts` and `run-v2-print.ts` are agent-core-v2
dependency-injection plumbing with no emission-site change. `convert.ts`
relaxes stdio MCP server conversion and is inert because Swallowtail sends
`mcpServers: []`. `acpTerminalRunner.ts` replaces two hard errors with a local
process spawn; Swallowtail advertises `clientCapabilities.terminal: false`, so
this is reachable and material, but it is a provider-side fix to an
already-mapped tool-call lifecycle and changes no request, response,
notification, or decoded record shape.

## Cross-corpus oracle

`protocol.json` records brace-matched digests of each mapped surface taken
independently from three corpora — the npm `dist/main.mjs` bundle and both
extracted single-executable archives — at all three versions. A fabricated or
self-consistent fixture edit fails against the recomputable digest set. The
`AcpProcessService` digest is deliberately recorded as differing so the corpus
cannot be read as claiming a clean ACP no-op.

## Not in scope

The `kimi web` local-server deltas, including the removed
`--allow-remote-terminals` flag and the new Remote Control surface, are
recorded under `other_family_observations_not_acted_on` for the separate
local-server family. Python `kimi-cli` and Kimi Platform Chat stay separate
axes. The g05.009 provider-operation observation gate and card 034 are
untouched.

No fixture in this directory contains a credential, bearer token, host path,
account identity, device id, provider payload, model observation, or session
id.
