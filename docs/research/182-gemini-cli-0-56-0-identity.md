# 182 Gemini CLI 0.56.0 Identity

Status: promoted
Owner: Tom
Date: 2026-08-22
Card: g04.034 / 093

## Question

Is official npm `@google/gemini-cli` `0.56.0` a compatible extension of the
separate Gemini CLI ACP and headless claims, or does either selected axis
require a private milestone or stop?

## Method

Re-probed npm `latest` and the GitHub latest stable release, recorded the
installed host without changing it, and downloaded official npm, GitHub source,
and darwin-arm64 unsigned release artifacts into `/tmp`. Compared the
selected ACP launch/initialize/session/callback subset and the selected
headless invocation/event/terminal/retention subset across the existing
ceilings and every published stable point through `0.56.0`.

No provider prompt, authentication, live catalogue, live session, install,
update, or account inspection was used. The access boundary is the existing
enterprise-owned Gemini Developer API-key profile. Browser login and
individual-account service remain outside both routes.

## Identity

| Fact | Value |
| --- | --- |
| installed host | `gemini 0.53.0` |
| host executable SHA-256 | `4a8f99947eae4e1ff501269ba8b9ca2d1216db044fb75e01f4ee86fd1d8f175e` |
| host publisher | unsigned; codesign reports not signed |
| host install changed | no |
| npm package | `@google/gemini-cli@0.56.0` |
| npm published | `2026-08-19T19:29:01.177Z` |
| GitHub release | `v0.56.0`, published `2026-08-19T19:29:38Z` |
| GitHub commit | `b6e23a7dc29eb15fede4bbe646d91869e948b45a` |
| GitHub tree | `379c84605a494dd3adf650acb3cc2a6d82e82e53` |
| npm integrity | `sha512-q4oBfb/Oh/HNLMYBOJMp88/QQ8hLffnB0ykoVThi6A5isbGHJ/ylWLMosMGqukKY0Q1Jv/XRDpb46Q1BV+zQqw==` |
| npm tarball SHA-256 | `e25443a59b22f0000d6418ce42c5c0710bc04d8f41b5567417e30e038a80120b` |
| npm `package.json` SHA-256 | `d8daf35daa8fdaecc100a1d50e5df28be03ff0b24e6d82230ac7969d00b4ee23` |
| npm executable entry SHA-256 | `a196a6fa5b89124396776ab2cc647404c99e062c24574e576297f31488109d57` |
| GitHub source archive SHA-256 | `01da971fe62c4ee68e237c2fb501bafac4e71492c189ebc6eec2dac47aacd2ad` |
| darwin-arm64 unsigned asset SHA-256 | `be0c20ccf8b6be6ce01654736847168a9328e92db4db4c0d0b776de70703fb8f` |
| extracted darwin-arm64 executable SHA-256 | `fa84c229012862d3695775afafff6e07dcffaa6da22db4072a6dbe87e5265151` |

Published stable points after the existing ceilings are `0.52.0`, `0.53.0`,
`0.53.1`, `0.54.0`, `0.54.4`, `0.55.1`, and `0.56.0`. `0.56.1` is not published;
`0.57.0-preview.0` is a preview and is ignored.

## Selected protocol comparison

The official npm bundle's selected help output is byte-identical across the six
later published points. It contains the selected `--acp`, `--output-format
stream-json`, model, approval, extension, MCP, trust, session, and delete
flags.

### ACP

The selected ACP launch, initialize, session, callback, authentication
advertisement, and process lifecycle remain compatible with the frozen
`gemini-cli.acp.v0.51.0` behavior. The selected methods are `initialize`,
`session/new`, `session/prompt`, and `session/cancel`; the selected callback
is `session/update`, with the existing filesystem callback boundary. The
comparison explicitly includes the `0.52.0` tag
(`d14583b926769bd98f807cdc6b1ca50e91ae26ec`): both the read-only Plan Mode
profile (`writeTextFile: false`) and bounded-write Auto Edit profile
(`writeTextFile: true`) retain their mode ids and filesystem callbacks through
`0.56.0`. The selected `acpSessionManager.ts` and
`acpFileSystemService.ts` source digests are identical across every compared
stable release.

The `0.53.1` source delta adds provider-invalid-stream categories to the
graceful `end_turn` path. Wire methods, callbacks, auth advertisement, session
lifecycle, and stop reason do not change. Provider-private metadata, model
selection, browser login, consumer accounts, Vertex, gateway, and auth
negotiation remain unmapped.

Decision: compatible extension. Keep behavior revision
`gemini-cli.acp.v0.51.0`, baseline `0.51.0`, and qualify published points
through `0.56.0` in card 094.

### Headless

The selected headless invocation, `stream-json` event names and fields,
terminal result shape, exit boundary, and retention sources remain compatible
through `0.56.0`. The historical decoder corpus remains the authoritative
specimen set; this identity run does not invent transcript management
support.

The `0.53.1` source delta adds typed `InvalidStreamError` details and more
specific provider error guidance. Event names, fields, terminal result shape,
invocation flags, and retention behavior remain unchanged. Model fallback
session-id rotation, thought filtering, merged function responses, account
authentication, and unused provider feature surfaces remain unmapped.

Decision: compatible extension. Keep behavior revision
`gemini-cli.headless.stream-json.v1`, baseline `0.51.0`, and qualify published
points through `0.56.0` in card 094.

## Boundary decision

Both axes retain `AllowUnverified`. The first later stable after the claimed
ceiling is `0.56.1`, so it remains the synthetic `UnverifiedNewer` test point.
No new public operation, auth flow, route family, or behavior revision is
required. Gemini Live, Gemini Models, browser login, and individual-account
access remain unchanged and separate.

## Sources

- [npm `@google/gemini-cli@0.56.0`](https://registry.npmjs.org/@google%2Fgemini-cli/0.56.0)
- [GitHub `v0.56.0` release](https://github.com/google-gemini/gemini-cli/releases/tag/v0.56.0)
- [GitHub `v0.56.0` source tree](https://github.com/google-gemini/gemini-cli/tree/v0.56.0)
- [Gemini CLI headless documentation](https://geminicli.com/docs/cli/headless/)
- frozen ACP corpus at `crates/swallowtail-adapter-gemini/tests/fixtures/gemini-cli-acp-v0.51.0/`
- frozen headless corpus at `crates/swallowtail-adapter-gemini/tests/fixtures/gemini-headless-0.51.0-0.52.0/`
- promoted identity corpus at `crates/swallowtail-adapter-gemini/tests/fixtures/gemini-cli-0.56.0/`
