# 045 Gemini CLI Headless Currentness And Corpus

Status: promoted
Owner: Tom
Date: 2026-07-28

## Question

Can Gemini CLI expose one bounded structured run without relabeling ACP,
forcing a sandbox, or requiring live provider access in conformance?

## Method

The check compared official headless documentation and tagged source at
`v0.51.0` and `v0.52.0`. It traced argument parsing, non-interactive entry,
stream event types, terminal usage, exit codes, tool suppression, trust,
session persistence, and release currentness.

No credential, account, provider request, installed process, model invocation,
or paid operation was used.

## Result

Gemini CLI headless mode is a distinct production route:

- driver `swallowtail.gemini.headless`
- transport `gemini-stream-json-stdio`
- version axis `gemini-cli.headless-stream-json`
- guaranteed range `0.51.0..=0.52.0`
- one `StructuredRun` over `HarnessInteraction`

The selected source files are byte-identical across both tags:

| Source | Git blob |
| --- | --- |
| `packages/cli/src/config/config.ts` | `139ab5d0f1097ef57454991360ab4bda84370d5b` |
| `packages/cli/src/nonInteractiveCli.ts` | `00b48be9540b0a661825c36227b67a427d0f4dfc` |
| `packages/core/src/output/types.ts` | `1d4159a1f8e545ec2da2b8df413cb6778b3f79dc` |
| `packages/core/src/output/stream-json-formatter.ts` | `6475e6d482239dfedad965f078e34bc23d7af0b7` |

Tag commits:

- `v0.51.0`: `8d951de3855750d5f8219d65ae22524b606133b6`
- `v0.52.0`: `d14583b926769bd98f807cdc6b1ca50e91ae26ec`

Later stable releases stay executable only as visible `UnverifiedNewer`
observations. They do not extend the guaranteed range.

## Invocation

The qualified route uses:

- prompt over piped stdin
- `--output-format stream-json`
- explicit model
- `--approval-mode plan`
- `--extensions none`
- empty `--allowed-mcp-server-names`
- `--skip-trust` for an already-authorized working resource
- one driver-derived `--session-id`
- no `--sandbox`, `--yolo`, ACP, resume, or fallback flag

`plan` mode constrains provider-owned tools to the upstream read-only posture.
Extensions and MCP servers are disabled. Swallowtail exposes no consumer tool
exchange on this route.

Sandboxing is not implicit. An operator may configure it through the ambient
Gemini environment. Swallowtail does not claim provider- or host-enforced
containment when none was selected.

## Event And Terminal Boundary

The frozen `stream-json` corpus covers:

- `init`
- user and assistant `message`
- provider-owned `tool_use` and `tool_result`
- `error`
- `result`
- unknown events
- malformed and incomplete streams

Only assistant deltas, final output, progress, and bounded usage enter stable
runtime events. User content, tool parameters, tool results, provider error
text, and unknown payloads remain private.

Native exits retain separate safe codes:

| Exit | Meaning |
| --- | --- |
| 41 | authentication |
| 42 | input |
| 44 | sandbox |
| 52 | configuration |
| 53 | turn limit |
| 54 | fatal tool execution |
| 55 | workspace trust |
| 130 | external interruption |

Other non-zero exits remain a generic provider process failure. Cancellation
and host deadline remain distinct terminal outcomes and force-stop, wait, and
join the child.

## Retention And Access

Gemini CLI creates a local chat transcript for headless sessions and exposes
no qualified disable switch. The route therefore requires
`ProviderRetentionPolicy::DurableAllowed`. Run close does not imply transcript
deletion.

The route uses the provider-supported Gemini Developer API-key profile. It
does not generalize Gemini account OAuth, individual subscription access, or
another endpoint audience. Current upstream account-serving changes do not
alter the qualified API-key path.

## Promotion

- Gemini CLI now has one public typed facade with explicit `Acp` and
  `Headless` selection.
- ACP and headless keep separate driver, transport, version, operation, and
  lifecycle identities.
- The solution matrix may group both routes under one Gemini CLI row.
- Roadmap g02.023 and card 076 own implementation.

## Evidence

- [Gemini CLI headless mode](https://geminicli.com/docs/cli/headless/)
- [Gemini CLI v0.51.0](https://github.com/google-gemini/gemini-cli/releases/tag/v0.51.0)
- [Gemini CLI v0.52.0](https://github.com/google-gemini/gemini-cli/releases/tag/v0.52.0)
- [v0.52.0 CLI configuration source](https://github.com/google-gemini/gemini-cli/blob/v0.52.0/packages/cli/src/config/config.ts)
- [v0.52.0 non-interactive source](https://github.com/google-gemini/gemini-cli/blob/v0.52.0/packages/cli/src/nonInteractiveCli.ts)
- [v0.52.0 stream event types](https://github.com/google-gemini/gemini-cli/blob/v0.52.0/packages/core/src/output/types.ts)
- [Gemini CLI authentication currentness discussion](https://github.com/google-gemini/gemini-cli/discussions/28017)
