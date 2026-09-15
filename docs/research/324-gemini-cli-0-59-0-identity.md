# 324 Gemini CLI 0.59.0 Identity

Status: promoted
Owner: Tom
Date: 2026-09-15
Card: g05.076 (Research 308 useful-newer campaign)
Authority: Contracts 017, 023, and 029; Research 182, 230, 235, 239, 244,
255, 308, and 323; the Gemini CLI selection, prepared ACP/headless guides,
commands, drivers, decoders, and frozen fixtures; and the official npm and
GitHub channels.

## Question

Do official npm `@google/gemini-cli` `0.57.0`, `0.58.0`, and `0.59.0`
independently extend the Gemini CLI ACP and headless claims qualified through
`0.56.0`, or does either selected route stop at its first exact incompatible
hop?

## Remaining AllowUnverified rank

Gemini is the sixteenth named family in Tom's 2026-09-14 Research 308
direction, which explicitly lifted the former Gemini deferral. The serial
order still has Kimi installed routes, Kimi local server, and Oh My Pi after
it. `1.1.8`-style separately incompatible points stay independently bounded.

## Method

Re-probed npm `latest`, the GitHub latest stable release, and the installed
host `gemini --version` on 2026-09-15. npm `latest` is
`@google/gemini-cli@0.59.0`, published `2026-09-08T21:19:17.301Z`, integrity
`sha512-RHcjpQEMwVkrWz75mEFsuMuM0JRVJgzLQzkGhEY6SE0KjwOWFA1wYKTUqpzDUHprcE2mgcTActyd4ihVdC2dpg==`.
GitHub latest is `v0.59.0`, published `2026-09-08T21:13:43Z`, at commit
`fb0d535af931b27c51e87e5e6ade72905b1e8390`, tree
`a080c8f15d68e93f65a29c6b177090206038dc97`, with darwin-arm64 unsigned asset
`sha256:0c8938c68df7e46fd1de63583e4c0a1d7a452e1ecb1d33065f9ac12b8814876a`.
The published stable points after the previous ceilings are exactly `0.57.0`,
`0.58.0`, and `0.59.0`; `0.59.1` is the first unpublished later stable and
`0.60.0-preview.0` is an ignored preview.

Downloaded official npm tarballs for `0.56.0`, `0.57.0`, `0.58.0`, and
`0.59.0` into `/tmp`, reproduced every registry `integrity` and `shasum`, and
downloaded the corresponding GitHub tagged source archives and darwin-arm64
unsigned assets. Every asset SHA-256 matches the GitHub-declared digest, and
every `0.56.0` value reproduces the historical `gemini-cli-0.56.0` corpus
byte-for-byte. No downloaded artifact was executed, installed, or
authenticated; no prompt, catalogue call, login, credential, host update, or
live session occurred.

Built one deterministic file inventory per extracted tagged source tree: a
sorted `path -> SHA-256` map over the whole archive. Each hop's changed-path
set is the symmetric difference of those maps, so the ledger is reproducible
and mutation-sensitive without repeated whole-tree scans. The window changes
89 paths `0.56.0..0.57.0`, 36 paths `0.57.0..0.58.0`, and 24 paths
`0.58.0..0.59.0`.

## Identity

Host `gemini 0.53.0` is unchanged: executable SHA-256
`4a8f99947eae4e1ff501269ba8b9ca2d1216db044fb75e01f4ee86fd1d8f175e`, unsigned,
linked from `/opt/homebrew/bin/gemini`, observed only through `--version` and
its digest.

| Version | npm published | npm tarball SHA-256 | GitHub tag commit / tree | source archive SHA-256 | darwin-arm64 asset / extracted executable |
| --- | --- | --- | --- | --- | --- |
| `0.56.0` | 2026-08-19T19:29:01.177Z | `e25443a59b22f0000d6418ce42c5c0710bc04d8f41b5567417e30e038a80120b` | `b6e23a7dc29eb15fede4bbe646d91869e948b45a` / `379c84605a494dd3adf650acb3cc2a6d82e82e53` | `01da971fe62c4ee68e237c2fb501bafac4e71492c189ebc6eec2dac47aacd2ad` | `be0c20ccf8b6be6ce01654736847168a9328e92db4db4c0d0b776de70703fb8f` / `fa84c229012862d3695775afafff6e07dcffaa6da22db4072a6dbe87e5265151` |
| `0.57.0` | 2026-08-25T18:36:40.273Z | `60f3d49767f414a90c6425c4a33295720af38766cd60dec6ca6f2321e8b304eb` | `6b0ae9a6c37aa117cc8b070d8b41c5bb4fa6d253` / `104819a15a187922b298a75e6a6791f1b2084b5f` | `36cd1287a3ed98c4ac9654f2fa5b860668642339dd9856dadd517e83d29217f3` | `584d470ebd8e520f1f9e74c4d270234625877a4faceeb81f8d6031a8f7f44a24` / `9f16db53342bdf25b4fcd7b0e55bdf930d7ab0f5fcf223ef5a5b9d15c55dc877` |
| `0.58.0` | 2026-09-01T20:50:39.295Z | `8ffeb9e7edddffb054764d00749f39e8cc9804ca9b38b9093f906dd2157322ae` | `ac9431c9e2290d68af31a77614ff2fddb2391ca3` / `5dbf74073ce1f63693abdf02d88136d7f018eeb6` | `ab6da6495692daadd17f9768612b68b728601c6fb7eca76eb60d307dbbf59e72` | `2f3708ab95187215db759440eb010ea20162a4268f35b0230e7ce3ccdc45fb86` / `58afc51cf60bb4926184d4fd43cc6ea02b494cd0f1f9904ce90f99a8d3345c41` |
| `0.59.0` | 2026-09-08T21:19:17.301Z | `59dc2cdb098b3000d36e34a185fc873932df4fd9d00900e817f2b19cd349d98b` | `fb0d535af931b27c51e87e5e6ade72905b1e8390` / `a080c8f15d68e93f65a29c6b177090206038dc97` | `6e698510dcae4341f94efe93704447b2fe456062c138f64f95da1c30996d0f33` | `0c8938c68df7e46fd1de63583e4c0a1d7a452e1ecb1d33065f9ac12b8814876a` / `f78acf4241ae6b1c9b04c9a2cb201c6a876e9e79266a9d1078b88cf833edf36c` |

The npm `package.json` and `bin` entry digests are frozen for every point in
`identity.json`. The npm bundle was not executed: the selected option surface
is proved from the tagged source instead.

## Published selected-path classification per claim

Release notes discovered candidate changes; the tagged source and exact
selected-file digests establish the selected result.

| Hop | ACP | Headless |
| --- | --- | --- |
| `0.56.0→0.57.0` | unchanged | `geminiChat.ts` moves the provider retry nudge from `systemInstruction` to an appended contents turn, preserves empty text turns that carry tools or media, and rolls back the whole multi-turn request on cancellation or abort; `config.ts` passes `getSafeGitEnv()` to its two worktree `git` subprocesses; context-aware silent retries and a capacity-availability TTL land in the provider request loop |
| `0.57.0→0.58.0` | unchanged | `geminiChat.ts` simplifies the empty-text key check in `stripToolCallIdPrefixes` |
| `0.58.0→0.59.0` | unchanged | none on the selected sources; workspace trust becomes fail-closed through `checkPathTrust`, restricted mode filters `mcpServers`, and MCP OAuth metadata discovery gains an SSRF repair |

### ACP

The selected ACP launch, initialize, session, callback, authentication
advertisement, permission-stop, process ownership, and cleanup shapes remain
compatible with the frozen `gemini-cli.acp.v0.51.0` behavior. Every file under
`packages/cli/src/acp/**` is byte-identical across `0.56.0..=0.59.0` — not
only the six selected sources but the errors, utils, and command registry
too — and no changed path anywhere in the three hops lies under that
directory. The ACP SDK pin stays `@agentclientprotocol/sdk@0.16.1`, the wire
version stays `1`, and the selected read-only Plan and bounded-write Auto Edit
profiles keep their mode ids, filesystem capabilities, and callbacks.

Decision: compatible extension. Keep `gemini-cli.acp.v0.51.0`, baseline
`0.51.0`, and qualify `0.57.0`, `0.58.0`, and `0.59.0`.

### Headless

The selected headless invocation, explicit model and approval inputs,
`stream-json` event names and fields, terminal result shape, native exit
codes, process-level cancellation, and retention sources remain compatible
through `0.59.0`. Of the selected mapped sources only `geminiChat.ts` changes
(`0.56.0..0.57.0` and `0.57.0..0.58.0`) and `config.ts` changes once
(`0.56.0..0.57.0`); `nonInteractiveCli.ts`, `turn.ts`, `output/types.ts`,
`output/stream-json-formatter.ts`, `utils/exitCodes.ts`, `utils/sessions.ts`,
`sessionCleanup.ts`, `sessionOperations.ts`, and `gemini.tsx` are
byte-identical. The `config.ts` delta is confined to the worktree git helper,
and every selected CLI option literal (`acp`, `approval-mode`,
`output-format`, `model`, `extensions`, `allowed-mcp-server-names`,
`skip-trust`, `session-id`, `delete-session`, `stream-json`) is present with
the same multiplicity at every point. The `geminiChat.ts` deltas are internal
provider-request retry, empty-part, and abort-rollback mechanics; they do not
add or rename a stream event, change a terminal record, change an exit code,
or change the process-level cancellation path.

Decision: compatible extension. Keep `gemini-cli.headless.stream-json.v1`,
baseline `0.51.0`, and qualify `0.57.0`, `0.58.0`, and `0.59.0`.

## Unmapped surfaces

Workspace trust, policy safety-checker declaration, sandbox, and MCP stay
separate. `packages/core/src/config/config.ts` resolves folder trust through
`checkPathTrust` from `0.59.0`, and `packages/core/src/utils/trust.ts` makes
env and `GEMINI_RESTRICTED_MODE` sources explicit, but the selected route
neither sets `GEMINI_RESTRICTED_MODE` nor
`GEMINI_CLI_TRUST_WORKSPACE=false`, and the selected headless command already
passes `--skip-trust`, which resolves trusted before any file or IDE source is
read. The `0.57.0` known-safe-git `ASK_USER` guard and the `0.58.0`
`write.toml` top-level safety-checker declaration are provider tool policy
behind that same trust gate, distinct from the ACP host filesystem callbacks
and the headless plan approval mode. The macOS Seatbelt socket-isolation
changes, the ambient MCP `mcpServers` filtering, and the MCP OAuth SSRF repair
stay on their explicitly separate surfaces. Gemini Live, the Gemini Models
HTTP catalogue, browser/individual-account auth, Vertex, and gateway remain
untouched.

## Decision

Both axes are compatible extensions. ACP advances to maintained
`0.51.0..=0.59.0` on `gemini-cli.acp.v0.51.0` with baseline `0.51.0` and
`AllowUnverified` unchanged; headless advances to maintained
`0.51.0..=0.59.0` on `gemini-cli.headless.stream-json.v1` with the same
baseline and posture. No new public operation, flag, driver, facade, or
behavior revision is required. Unpublished `0.59.1` stays the visible
`UnverifiedNewer` point. The ACP activity corpus and the headless stream-JSON
decoder corpus remain authoritative.

## Sources

- [npm `@google/gemini-cli@0.59.0`](https://registry.npmjs.org/@google%2Fgemini-cli/0.59.0)
- [GitHub `v0.59.0` release](https://github.com/google-gemini/gemini-cli/releases/tag/v0.59.0)
- [GitHub `v0.59.0` source tree](https://github.com/google-gemini/gemini-cli/tree/v0.59.0)
- frozen corpus at `crates/swallowtail-adapter-gemini/tests/fixtures/gemini-cli-0.59.0/`
- historical corpus at `crates/swallowtail-adapter-gemini/tests/fixtures/gemini-cli-0.56.0/`
- frozen ACP corpus at `crates/swallowtail-adapter-gemini/tests/fixtures/gemini-cli-acp-v0.51.0/`
- frozen headless corpus at `crates/swallowtail-adapter-gemini/tests/fixtures/gemini-headless-0.51.0-0.52.0/`
