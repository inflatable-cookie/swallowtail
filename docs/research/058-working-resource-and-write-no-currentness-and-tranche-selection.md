# Working Resource And Write `No` Currentness And Tranche Selection

Status: promoted
Owner: Tom
Date: 2026-07-28

## Question

Which of the 31 working-resource and bounded-workspace-text-write `No` cells
are non-applicable operation shapes, exact selected-surface absences, or
contract-gated implementation candidates?

## Method

Evidence was accessed on 2026-07-28.

- parsed all 22 canonical solution rows and froze the 31 starting cells
- compared every cell with Contracts 010, 013, 015, 017, 023, 029, 033, 037,
  039, and 041
- inspected realized prepared requirements, access policies, capabilities,
  commands, callback dispatch, and fixtures
- inspected exact Gemini CLI `0.51.0` source at commit
  `8d951de3855750d5f8219d65ae22524b606133b6`
- checked current maintained Qwen Code, Claude Code, Pi, Gemini CLI,
  OpenCode, and Kimi Code documentation

No executable, account, credential, provider request, paid operation,
container, sandbox, or model server was used.

## Terms

- a working resource is one opaque consumer reference resolved by the
  execution host; a provider file, attachment, conversation, model artifact,
  or remote sandbox is not a substitute
- working-directory selection establishes location, not containment
- bounded workspace text write means one host-mediated UTF-8 create or replace
  callback under the leased root
- bounded callback I/O does not contain an `AmbientHost` harness or its child
  processes
- provider tools, permission policy, provider sandboxing, and host
  containment remain independent

## Classification

The table classifies every starting `No` exactly once. A dash was already
`Yes` or `Not applicable` and is outside the inventory.

- `NA` — the feature does not apply to the selected operation shape
- `U` — the selected surface does not expose the feature
- `C` — the selected route needs an exact contract/corpus gate

| Solution route | Working resource | Bounded text write |
| --- | --- | --- |
| `qwen.headless` | — | U |
| `alibaba.conversations` | NA | NA |
| `bedrock.catalogue; bedrock.runtime` | NA | NA |
| `claude-agent.acp` | — | U |
| `claude-code.headless` | — | U |
| `anthropic.managed-agent` | NA | NA |
| `anthropic.messages` | NA | NA |
| `pi.rpc` | — | U |
| `deepseek.continuation` | NA | NA |
| `gemini-cli.acp + gemini-cli.headless` | — | C |
| `gemini.live` | NA | NA |
| `llama-cpp.attached` | NA | NA |
| `kimi-code.local-server` | — | U |
| `kimi-platform.chat` | NA | NA |
| `ollama.attached` | NA | NA |
| `openai.realtime` | NA | NA |
| `openai.background` | NA | NA |
| `opencode.http` | — | U |
| `xai.responses-websocket` | NA | NA |

Exact totals:

| Classification | Cells |
| --- | ---: |
| operation-shape non-applicability | 24 |
| selected-surface absence | 6 |
| contract and exact corpus required | 1 |
| **Total** | **31** |

There are no realized matrix false negatives. Matrix values do not change
during this audit.

## Operation-Shape Findings

Direct hosted inference, realtime media, attached model inference, catalogue
observation, and provider-managed remote execution do not operate on a
consumer workspace in the selected routes. Adding a local working resource to
those operations would grant an unrelated authority and change their
operation shape.

The Anthropic managed-agent route owns a provider environment and durable
provider session. Its remote files and repositories are provider resources,
not a Swallowtail host working-resource lease.

The attached llama.cpp and Ollama routes consume prompts and model runtime
state. Their server working directories and model artifacts are not consumer
workspace inputs.

## Selected-Surface Absence

Qwen and Claude Code are intentionally qualified in read-only or Plan modes.
Their writable modes and optional native sandboxes are separate profiles and
cannot widen the selected route.

Claude Agent's ACP initialization advertises read but not write callbacks.
Its write-capable structured projection exposes provider-native `Edit` and
`Write` tools under `AmbientHost`; it does not expose bounded host text-write
callbacks.

Pi RPC operates relative to its selected cwd and has no built-in permission or
sandbox boundary. Its maintained sandbox extension is an optional separately
configured interface, not part of the selected RPC profile.

Kimi local server and OpenCode HTTP can perform ambient harness writes, but
neither selected surface routes those writes through `WorkingResourceIo`.
Their `ReadWrite` working-resource leases establish explicit authority and
location without converting ambient provider tools into bounded callbacks.

## Gemini Candidate

Gemini CLI `0.51.0` implements ACP filesystem reads and writes independently.
For a path inside the session root, `AcpFileSystemService.writeTextFile`
forwards the whole UTF-8 replacement to the ACP client. Gemini's `write_file`
and edit tools reject paths outside the workspace before calling that service.

Swallowtail's current Gemini ACP profile is intentionally read-only:

- client capability `writeTextFile` is `false`
- the resource lease and capability use `ResourceAccess::Read`
- the invocation selects `--approval-mode plan`
- the returned session must report mode `plan`
- only `fs/read_text_file` is dispatched

This is an honest missing profile, not an upstream absence. The smallest
conversion is a separate explicit Gemini ACP write profile:

- exact qualified version `0.51.0`
- `ResourceAccess::ReadWrite`
- client capabilities `readTextFile: true` and `writeTextFile: true`
- `--approval-mode auto_edit`
- returned ACP mode `autoEdit`
- bounded `fs/write_text_file` dispatch through the existing host service
- empty session MCP list and rejection of permission callbacks
- unchanged `AmbientHost` and ambient-configuration posture

The profile claims one bounded host callback. It does not claim process
containment, shell containment, a provider sandbox, approval exchange, or
general filesystem safety. Consumers still see and accept the ambient harness
posture.

## Selected Tranche

Cards 108-110 should close the Gemini ACP bounded-write profile.

It is the only candidate that exercises an already standardized host callback
instead of requiring a new provider route, optional sandbox, container,
provider-native tool interpretation, or local-host containment mechanism.
Provider, credential, endpoint, billing, topology, and support authority stay
unchanged.

Expected final matrix movement:

- 24 non-applicable starting cells: `No` to `Not applicable`
- Gemini CLI bounded workspace text write: `No` to `Yes`
- six selected-surface absences remain `No`

## Sources

- [Gemini CLI `0.51.0` ACP filesystem service](https://github.com/google-gemini/gemini-cli/blob/8d951de3855750d5f8219d65ae22524b606133b6/packages/cli/src/acp/acpFileSystemService.ts)
- [Gemini CLI `0.51.0` write-file tool](https://github.com/google-gemini/gemini-cli/blob/8d951de3855750d5f8219d65ae22524b606133b6/packages/core/src/tools/write-file.ts)
- [Gemini CLI `0.51.0` ACP modes](https://github.com/google-gemini/gemini-cli/blob/8d951de3855750d5f8219d65ae22524b606133b6/packages/cli/src/acp/acpUtils.ts)
- [Gemini CLI ACP mode](https://github.com/google-gemini/gemini-cli/blob/main/docs/cli/acp-mode.md)
- [Qwen Code configuration](https://github.com/QwenLM/qwen-code/blob/main/docs/users/configuration/settings.md)
- [Claude Code CLI reference](https://docs.anthropic.com/en/docs/claude-code/cli-usage)
- [Pi sandbox extension](https://github.com/badlogic/pi-mono/blob/main/packages/coding-agent/examples/extensions/sandbox/index.ts)
- [OpenCode permissions](https://opencode.ai/v2/docs/permissions)
- [Kimi Code releases](https://github.com/MoonshotAI/kimi-code/releases)
