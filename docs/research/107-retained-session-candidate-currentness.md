# 107 Retained Session Candidate Currentness

Status: promoted
Owner: Tom
Date: 2026-08-05

## Question

Do current public Pi RPC and Alibaba Model Studio interfaces satisfy their
independent retained-session recovery gates?

## Method

Evidence was accessed 2026-08-05.

- checked current Pi package and release identity, tagged RPC documentation,
  public command handling, runtime switching, and CLI session selection
- checked current Alibaba Conversations retrieval, item listing, Responses
  continuation, response storage, retention, and deletion documentation
- applied Contracts 009, 017, 025, 029, 037, 038, and 050 before selecting
  implementation
- used no installed executable, account, credential, provider request,
  conversation mutation, or model inference

## Gate Result

| Dimension | Pi RPC `0.83.0` | Alibaba Conversations |
| --- | --- | --- |
| host | exact launch plan | exact endpoint and workspace plan |
| resource | blocked: stored cwd can replace the leased cwd | resource-free route |
| access | exact delegated local access | exact regional workspace credential audience |
| model | exact configured model | exact configured deployment and model |
| interface | exact maintained `0.83.0` | current documented Conversations and Responses surfaces |
| provider state | exact session path, but not attachment authority | exact opaque conversation id plus retrieval |
| replay | ordered `get_messages` available | ordered, paginated conversation items available |
| readiness | blocked: effective cwd is not publicly corroborated | exact retrieval plus complete bounded item listing |
| cleanup | process attachment cleanup only | explicit item deletion and separate conversation deletion |
| result | blocked | selected for contract promotion |

The results are independent. Alibaba support does not weaken Pi's resource
gate. Selection means contract work may begin; it is not a production support
claim.

## Pi RPC

The maintained release is `0.83.0`. Public RPC continuity remains unchanged:

- `switch_session` accepts only `sessionPath`
- `get_state` exposes session and model state but not effective cwd
- `get_messages` exposes ordered active conversation history
- the RPC handler calls `runtimeHost.switchSession(command.sessionPath)`
  without an attachment override

The internal runtime now accepts an optional `cwdOverride` when switching a
session. The public RPC path does not pass it. The tagged CLI session path also
does not expose an exact non-interactive override and correlated state still
cannot prove the resulting cwd.

Pi therefore remains blocked for the same reason as Research 053. A stored
session may select another existing directory. Process cwd, a copied session
path, or private source capability cannot replace caller-bound attachment and
public corroboration.

Sources:

- [Pi `0.83.0` release](https://github.com/earendil-works/pi/releases/tag/v0.83.0)
- [Pi `0.83.0` RPC](https://github.com/earendil-works/pi/blob/v0.83.0/packages/coding-agent/docs/rpc.md)
- [Pi `0.83.0` RPC command handling](https://github.com/earendil-works/pi/blob/v0.83.0/packages/coding-agent/src/modes/rpc/rpc-mode.ts)
- [Pi `0.83.0` session runtime](https://github.com/earendil-works/pi/blob/v0.83.0/packages/coding-agent/src/core/agent-session-runtime.ts)
- [Pi `0.83.0` CLI session selection](https://github.com/earendil-works/pi/blob/v0.83.0/packages/coding-agent/src/main.ts)

## Alibaba Conversations

The current public surfaces preserve the required operations separately.

- an exact conversation is retrievable by opaque id
- conversation items are listed in explicit order with bounded page size and
  continuation metadata
- a synchronous Responses request names the exact conversation; completed
  input and output are appended to it
- response-object storage is controlled separately by `store`; retained
  conversation items do not require stored response objects
- deleting a conversation does not delete its message items
- item deletion and conversation deletion are explicit independent operations

This opens a separate retained profile. The existing Alibaba profile remains
operation-owned and delete-on-close. Retained close must preserve the remote
conversation, while later destructive cleanup requires separately persisted,
exact authority and complete item cleanup before conversation deletion.

Provider retention is not an indefinite-availability promise. Missing,
deleted, inaccessible, incomplete, or over-bound replay must fail without
fallback or a ready session.

Sources:

- [Alibaba Model Studio Conversations](https://help.aliyun.com/en/model-studio/openai-compatible-conversations)
- [Alibaba Model Studio Responses](https://help.aliyun.com/en/model-studio/qwen-api-via-openai-responses)
- [Alibaba stored response retrieval](https://help.aliyun.com/en/model-studio/retrieve-a-response)
- [Alibaba regions and endpoints](https://help.aliyun.com/en/model-studio/regions/)
- [Alibaba workspace permissions](https://help.aliyun.com/en/model-studio/permission-management-overview)

## Contract Promotion Requirements

Card 098 must define, before implementation:

- a retained-conversation profile distinct from operation-owned
  delete-on-close
- one persisted exact attachment binding covering instance, host, endpoint,
  region, workspace, access, deployment, model, interface, and conversation
- retrieve-then-list readiness with explicit page, item, and byte bounds
- ordered replay completion before returning a live continuation handle
- preservation on ordinary retained-session close
- separate explicit cleanup authority, with item cleanup before conversation
  deletion and uncertain outcomes preserved
- strict missing, foreign, stale, malformed, incomplete, and oversized
  behavior without provider-state lookup or fallback

No generic provider router, consumer-owned retention policy, or conversation-id
authority follows from this selection.

## Decision

- Keep card 099 superseded behind Pi's unchanged public cwd gate.
- Advance Alibaba to card 098 for contract and corpus promotion.
- Keep card 100 gated on card 098 and card 101 responsible for final route
  truth.
