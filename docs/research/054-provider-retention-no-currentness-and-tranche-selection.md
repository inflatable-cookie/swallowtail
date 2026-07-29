# Provider Retention `No` Currentness And Tranche Selection

Status: promoted
Owner: Tom
Date: 2026-07-28

## Question

Which of the 75 provider archive, restore, delete, and operation-owned cleanup
`No` cells are false negatives, non-applicable operation shapes, ready work,
separate-route work, or honest upstream absences?

## Method

Evidence was accessed on 2026-07-28.

- parsed all 22 canonical solution rows and froze the 75 starting cells
- compared each cell with its exact route, operation shape, prepared facade,
  retention policy, and qualified interface window
- checked Contracts 014-017, 021, 025, 029, 037-039
- reused the exact Codex, ACP, Claude Agent, OpenCode, Kimi, Gemini, and
  structured-run corpora promoted by Research 036-046
- checked Gemini CLI `0.51.0` and `0.52.0` tagged source and documentation
- checked the current official OpenAI background guide and Responses OpenAPI
- inspected realized OpenCode structured cleanup and Claude Agent deletion

No executable, provider account, credential, external provider request, paid
operation, container, or model server was used.

## Classification

The table classifies every starting `No` exactly once. A dash was already
`Yes` or `Not applicable` and is outside the 75-cell inventory.

- `NA` — the selected operation has no user-managed provider session or owns
  no qualifying remote resource
- `U` — the exact selected surface does not expose the action
- `F` — realized matrix false negative
- `E` — ready under existing contracts and exact corpora
- `S` — separate selected transport and exact corpus required
- `C` — shared contract and exact corpus required

| Solution route | Archive | Restore | Delete | Owned cleanup |
| --- | --- | --- | --- | --- |
| `qwen.headless` | NA | NA | NA | NA |
| `alibaba.conversations` | NA | NA | NA | — |
| `bedrock.catalogue; bedrock.runtime` | NA | NA | NA | NA |
| `claude-agent.acp` | U | U | — | E |
| `claude-code.headless` | NA | NA | NA | NA |
| `anthropic.managed-agent` | NA | NA | NA | — |
| `anthropic.messages` | NA | NA | NA | NA |
| `pi.rpc` | NA | NA | NA | NA |
| `deepseek.continuation` | NA | NA | NA | NA |
| `gemini-cli.acp + gemini-cli.headless` | U | U | S | S |
| `gemini.live` | NA | NA | NA | NA |
| `llama-cpp.attached` | NA | NA | NA | NA |
| `llama-cpp.owned` | — | — | — | — |
| `kimi-code.acp + kimi-code.headless` | U | U | U | U |
| `kimi-code.local-server` | — | — | U | U |
| `kimi-platform.chat` | NA | NA | NA | NA |
| `ollama.attached` | NA | NA | NA | NA |
| `codex.app-server; codex.exec` | — | — | — | NA |
| `openai.realtime` | NA | NA | NA | NA |
| `openai.background` | NA | NA | NA | C |
| `opencode.http` | U | U | — | F |
| `xai.responses-websocket` | NA | NA | NA | NA |

Exact totals:

| Classification | Cells |
| --- | ---: |
| operation-shape non-applicability | 58 |
| selected-surface absence | 12 |
| separate selected transport and corpus | 2 |
| realized false negative | 1 |
| ready under existing contracts | 1 |
| shared contract and corpus | 1 |
| **Total** | **75** |

## False Negative

`opencode.http` already implements operation-owned cleanup.

Its structured role creates one private session, prompts once, closes the
turn, sends the qualified session-delete request, joins the task, and records
`OwnedRemoteResourceKind::Session = Confirmed` before releasing access. The
prepared run advertises the exact capability and deterministic local plus
remote-authoritative tests prove the request ordering.

The matrix cleanup cell is stale. It may become `Yes` without runtime work.

## Non-Applicability

Provider-session management applies only to a durable provider session bound
for later user-directed management. Direct one-attempt inference, realtime
connections, resource-free headless runs, consumer-owned continuation, and
attached model runtimes do not gain archive, restore, or delete semantics.

Operation-owned cleanup applies only when the selected operation creates a
qualifying resource and owns its removal. Preserving an external runtime,
closing a connection, or releasing a process is not remote-resource deletion.

This produces 58 `Not applicable` corrections. They are matrix integrity
changes, not new capabilities.

## Gemini CLI Delete Route

Gemini CLI `0.51.0` tagged documentation omits `--delete-session`, while
`0.52.0` documents it. Tagged source corrects that documentation-only
impression: both releases contain the same implementation and source digests
for the delete controller, storage deletion, main dispatch, and option
declaration.

The whole existing `0.51.0..=0.52.0` headless window exposes:

- `--list-sessions`
- `--delete-session <index-or-session-id>`
- deletion of a selected project-scoped chat session
- `/quit --delete` for the active interactive session

The selected Swallowtail solution already separates ACP and headless
transports. ACP still does not advertise ACP `session/delete`; its cell cannot
borrow the CLI action.

The viable route is a third explicit installed-executable management role
across the existing qualified window. The first binding should be issued only
for a Swallowtail-created headless transcript under the exact executable,
execution host, working resource, and project scope. No arbitrary id,
session-list selection, ACP binding relabelling, or filesystem deletion is
permitted.

That role can support:

- explicit bound provider-session deletion
- opt-in deletion of an operation-private headless transcript

It cannot support archive or restore. Exact tagged source, output, missing
target, repeated delete, cancellation, deadline, and joined-process evidence
must be frozen before implementation. Deletion strength must follow the
provider's exact local-data wording.

## Claude Agent Operation Cleanup

Claude Agent ACP already has exact `0.53.0..=0.61.0` provider-data deletion
evidence and production bound deletion. Its structured role creates an
operation-private session, closes it natively, and currently retains the
transcript under `DurableAllowed`.

An explicit ephemeral structured profile can reuse the qualified delete
mapping after native close. It must:

- be opt-in and plan-bound
- advertise `OwnedRemoteResourceDeletion(Session)`
- reject durable-retention agreement
- delete only the operation-private session
- preserve the existing durable profile unchanged
- report exact confirmed or unconfirmed cleanup truth

This needs no new provider method, credential, endpoint, version range, or
shared runtime type.

## OpenAI Background Response Cleanup

OpenAI background mode creates a retrievable response resource even with
`store=false`; official guidance states that response data is temporarily
stored to support asynchronous execution and polling.

The current official Responses OpenAPI exposes
`DELETE /v1/responses/{response_id}`. A successful response carries the same
response id and `deleted=true`. The Swallowtail route already binds the
response id, endpoint, credential lease, cancellation, retrieval, and terminal
state, but it releases access without sending delete.

This is operation-owned cleanup, not provider-session management. The shared
gap is one exact `Response` owned-resource kind plus rules for:

- no cleanup claim before a valid response id exists
- one bounded delete attempt with no retry
- confirmed deletion only from the exact successful response
- 404, 5xx, disconnect, cancellation, or malformed acknowledgement as
  unconfirmed, never inferred success
- joined deletion before credential release

Deletion must not imply ZDR, secure erasure, conversation deletion, provider
session deletion, or cancellation of still-running work.

## Honest Absence

Archive and restore remain unsupported for Claude Agent, Gemini CLI, Kimi
Code installed routes, and OpenCode. Provider checkpoint, resume, history
filtering, or consumer-local archive cannot substitute.

Kimi Code ACP and headless expose no selected provider-session management.
Kimi local server exposes archive and restore but no deletion. The retained
local-server structured session and Kimi headless state therefore cannot claim
operation-owned deletion. This preserves the operator's accepted Kimi
no-delete posture.

## Selected Tranche

Card 100 should freeze one three-shape tranche:

1. Gemini CLI `0.51.0..=0.52.0` bound transcript deletion and opt-in headless
   cleanup
2. Claude Agent opt-in operation-private session cleanup
3. OpenAI background response cleanup

It should also correct the realized OpenCode false negative and 58
non-applicable cells at matrix closeout.

This tranche converts five genuine capability cells if the corpora pass:

- Gemini provider-session delete
- Gemini owned cleanup
- Claude Agent owned cleanup
- OpenAI background owned cleanup
- OpenCode owned cleanup

The remaining twelve starting `No` cells are exact selected-surface absences.
No provider, credential, endpoint, billing authority, topology, or consumer
policy choice is required.

## Sources

- [Gemini CLI `0.51.0` configuration](https://github.com/google-gemini/gemini-cli/blob/v0.51.0/docs/reference/configuration.md)
- [Gemini CLI `0.52.0` configuration](https://github.com/google-gemini/gemini-cli/blob/v0.52.0/docs/reference/configuration.md)
- [Gemini CLI `0.52.0` commands](https://github.com/google-gemini/gemini-cli/blob/v0.52.0/docs/reference/commands.md)
- [OpenAI background mode](https://developers.openai.com/api/docs/guides/background)
- [OpenAI Responses API reference](https://developers.openai.com/api/reference/resources/responses)
- [Claude Agent ACP `0.61.0` source](https://github.com/agentclientprotocol/claude-agent-acp/blob/v0.61.0/src/acp-agent.ts)
- [OpenCode `1.18.4` schema](https://github.com/anomalyco/opencode/blob/v1.18.4/packages/sdk/openapi.json)
- [Kimi Code `0.29.2` session routes](https://github.com/MoonshotAI/kimi-code/blob/%40moonshot-ai%2Fkimi-code%400.29.2/packages/kap-server/src/routes/sessions.ts)
