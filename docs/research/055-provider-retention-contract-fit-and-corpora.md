# Provider Retention Contract Fit And Corpora

Status: promoted
Owner: Tom
Date: 2026-07-28

## Question

What exact contracts and offline corpora are required for Gemini CLI stored-
transcript deletion, optional Claude Agent operation cleanup, and OpenAI
background-response cleanup?

## Method

Evidence was accessed on 2026-07-28.

- checked Contracts 009, 021, 029, 037-039 before fixing the corpus
- inspected Gemini CLI tags `v0.51.0` and `v0.52.0` at exact commits
- hashed the four Gemini source files that own option parsing, dispatch,
  session selection, and storage deletion
- reused the exact Claude Agent close/delete range and failure corpus
- fetched the current official OpenAI background guide and Responses OpenAPI
  deletion operation
- froze bounded synthetic success, rejection, mismatch, cancellation,
  deadline, disconnect, reconciliation, diagnostic, and cleanup cases

No executable, account, credential, provider request, paid operation,
container, model server, or consumer repository was used.

## Contract Promotion

Three narrow rules were missing.

Contract 038 now qualifies a Gemini stored-transcript management route that is
separate from Gemini ACP. A management binding can originate only from a
successful Swallowtail headless run and binds the exact executable, host,
working resource, project scope, and driver-selected session id.

Contract 039 now permits separate durable and opt-in temporary-retention
structured profiles for one exact harness. The temporary profile must own and
report deletion of only its operation-private session or transcript. Durable
profiles remain unchanged.

Contract 021 now defines `Response` as an operation-owned remote resource and
permits one terminal OpenAI response-delete attempt before credential release.
Deletion never substitutes for native cancellation.

No generic session, prompt, deletion, reconciliation, or routing API was
added.

## Gemini CLI Exact Range

The qualified Gemini headless range remains `0.51.0..=0.52.0`.

| Release | Tag commit |
| --- | --- |
| `0.51.0` | `8d951de3855750d5f8219d65ae22524b606133b6` |
| `0.52.0` | `d14583b926769bd98f807cdc6b1ca50e91ae26ec` |

The selected files are byte-identical at both tags:

| Source | SHA-256 |
| --- | --- |
| `packages/cli/src/utils/sessions.ts` | `fa8a3ab8ce762cd4b714362d843bdd73e73f0894f0480778fcdf2254a507af50` |
| `packages/core/src/utils/sessionOperations.ts` | `21ac4a3153dd94770a7d0cd87480cfece9c6c1777e520b3eef12b702be03f24b` |
| `packages/cli/src/gemini.tsx` | `1ac297b1af4cca39f358fc5c90c18059e275c4c393cfba152f73215e03ead828` |
| `packages/cli/src/config/config.ts` | `5100bcd48f798d04b9463bd72680af7202f331de566321b1c29f5f8710c2c44c` |

This corrects the documentation-only impression recorded during the first
audit pass. `0.51.0` source already contains the same `--delete-session`
implementation as `0.52.0`.

### Effect Truth

The command:

1. lists project-scoped sessions
2. resolves a UUID or list index
3. rejects the current active session
4. calls the storage deletion helper
5. prints a deletion line
6. exits through the general success path

The stable route never accepts list indexes. Swallowtail supplies only the
opaque session id already fixed in its binding.

Exit status and deletion text are insufficient. Rejection paths also exit
successfully, the success line includes the first user message, and the
storage helper catches some unlink failures. The strongest portable result is
therefore `HistoryRemoved` after one bounded `--list-sessions`
reconciliation proves the exact bound id absent.

The decoder may inspect bounded output internally. It must discard provider
text and expose no session id, path, prompt, transcript, stdout, stderr, or
environment through stable diagnostics.

### Gemini Corpus

`retention.json` freezes:

- exact arguments and source digests
- applied deletion plus absent reconciliation
- already-absent bound history
- active-target rejection
- misleading success text with target still present
- failed reconciliation
- provider-defined descendant posture
- diagnostic exclusions

Cancellation or deadline before delete-process dispatch produces no effect.
After dispatch, missing or incomplete reconciliation is unconfirmed. Both
delete and reconciliation children join before working-resource release.

## Claude Agent Cleanup Corpus

The provider method and version evidence are already qualified across
`0.53.0..=0.61.0`, excluding unpublished `0.58.0`.

The new corpus changes only structured-profile composition:

| Profile | Retention | Native close | Delete |
| --- | --- | --- | --- |
| existing durable | `DurableAllowed` | yes | no |
| new ephemeral | `TemporaryAllowed` | yes | operation-private session |

`owned-cleanup.json` freezes completion, cancellation after session creation,
delete-response loss, cancellation before session creation, durable-profile
preservation, exact ordering, and joined release.

The ephemeral profile reports `OwnedRemoteResourceKind::Session`. Confirmed
provider acknowledgement yields confirmed deletion. Any lost or contradictory
truth after dispatch yields unconfirmed deletion and degraded cleanup without
changing the inference terminal status.

## OpenAI Response Cleanup Corpus

Current official evidence establishes:

- background responses are temporarily retained for asynchronous execution
  and polling even with `store=false`
- `DELETE /v1/responses/{response_id}` is an official Responses operation
- successful deletion returns the response id and `deleted=true`
- the OpenAPI operation also declares 404

`response-delete.json` freezes:

- the exact terminal DELETE request
- exact-id plus `deleted=true` success
- mismatched id
- `deleted=false`
- 404
- no dispatch while provider state remains active or unconfirmed
- stream, terminal-state, delete, join, and credential-release ordering

The driver sends no more than one delete. Confirmed response deletion is not
ZDR, secure erasure, conversation deletion, provider-session deletion, or
provider cancellation.

## Implementation Boundary

Card 101 is contract-ready for four runtime cells:

1. Gemini provider-session history removal
2. Gemini opt-in headless transcript cleanup
3. Claude Agent opt-in session cleanup
4. OpenAI background response cleanup

OpenCode needs no runtime work; card 102 corrects its stale matrix cell.

Implementation must add `OwnedRemoteResourceKind::Response`, but no new common
operation role. Gemini reuses provider-session management. Claude and OpenAI
reuse structured-run terminal outcomes.

## Sources

- [Gemini CLI `0.51.0` source](https://github.com/google-gemini/gemini-cli/tree/v0.51.0)
- [Gemini CLI `0.52.0` source](https://github.com/google-gemini/gemini-cli/tree/v0.52.0)
- [Gemini CLI `0.52.0` configuration](https://github.com/google-gemini/gemini-cli/blob/v0.52.0/docs/reference/configuration.md)
- [Claude Agent ACP `0.61.0` source](https://github.com/agentclientprotocol/claude-agent-acp/blob/v0.61.0/src/acp-agent.ts)
- [OpenAI background mode](https://developers.openai.com/api/docs/guides/background)
- [OpenAI Responses API reference](https://developers.openai.com/api/reference/resources/responses)

