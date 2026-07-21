# 006 Kimi Code ACP Currentness And Persistent Session Evidence

Status: promoted
Owner: Tom
Updated: 2026-07-20

## Currentness Correction

Research 010 confirms this record already targets the maintained TypeScript
Kimi Code successor. It corrects two annotated tag ids that were mislabeled as
commits. The ACP behavior and selected subset remain current.

## Question

Which current Kimi Code ACP behaviors are authoritative enough to support a
second-agent portability proof, and which shared boundaries must exist first?

## Method

Official Moonshot AI release records, tagged source, maintained Kimi Code
documentation, the exact dependency lock, the current stable ACP v1
documentation, and the current stable schema release were checked on
2026-07-20.

No Kimi binary was launched. No login state, provider configuration, model
catalogue, credential store, or account was inspected. No authentication or
inference request was made.

## Version And Authority Pin

| Identity | Exact evidence | Authority |
| --- | --- | --- |
| Kimi Code agent | `0.28.1`, tag object `0032545b65f95c139ecba5a48ba1b911844e1ffe`, source commit `efacf0452d46f5dbd67499eabc053869495d5213`, published 2026-07-20 | Moonshot AI provider release |
| Kimi ACP adapter package | `@moonshot-ai/acp-adapter` `0.3.4` inside the `0.28.1` tag | Moonshot AI maintained implementation |
| ACP TypeScript SDK used by Kimi | exact lock `@agentclientprotocol/sdk` `0.23.0` | upstream ACP SDK artifact |
| ACP wire protocol | integer `1` negotiated during `initialize` | stable upstream protocol |
| ACP schema artifact | `schema-v1.19.1`, tag object `0331ddaea7814afa147194cd5e93495a2a0f8c82`, source commit `d0549a115750a0a25c1c5631dd72e0d248859aa4`, published 2026-07-20 | current upstream stable schema release |
| Transport | UTF-8 newline-delimited JSON-RPC 2.0 over `kimi acp` stdio | stable ACP transport plus Moonshot entry point |

Schema `v1.19.1` fixes schema-artifact generation and does not create a new ACP
wire version. Agent, adapter package, SDK, schema, and wire versions remain
independent pins.

Research 005's `0.28.0` Kimi snapshot is superseded for implementation
planning by this record.

## Current Capability Evidence

Kimi `0.28.1` advertises and implements the ACP v1 baseline plus stable load,
resume, list, mode/configuration mutation, image and embedded-resource input,
permission requests, and text filesystem reads and writes. It does not
implement stable `session/close`, `logout`, or terminal reverse RPC.

The portability proof should not claim the full matrix. The useful bounded
subset is:

- initialize and exact capability observation
- new session only to mint a Swallowtail-owned persistent binding
- load with ordered replay completed before the load response
- resume without historical replay
- one active text prompt, ordered text updates, and native turn cancellation
- bounded text replacement through `fs/write_text_file`
- connection EOF, owned-process stop, joined harness cleanup, and resource
  release

Session list, arbitrary provider-session ids, configuration or model mutation,
MCP forwarding, image or resource input, provider permission approval,
terminal calls, native close, logout, and unstable methods stay excluded.

## Load, Resume, And Replay

Current ACP v1 defines three different operations:

- `session/new` creates a provider session and returns a new opaque id
- `session/load` restores an existing id, replays the entire conversation as
  `session/update`, then responds
- `session/resume` restores an existing id without replaying history, then
  responds

Kimi implements the split directly. Both load and resume call the same
provider-owned on-disk rehydration path. Load then awaits its replay loop;
resume skips that loop. Kimi does not echo the session id in either response.

The implementation also records that the `cwd` supplied to load or resume is
not enforced because the stored session already owns a work directory. That
prevents safe attachment to an arbitrary Kimi session id. The first
Swallowtail route may load or resume only a provider reference carried by a
prior Swallowtail binding that also fixes configured instance, execution host,
model route, working resource, resource access, and session access policy.
Session discovery cannot manufacture that authority.

Replay is provider history transport, not a new model turn and not proof of
consumer persistence. It needs its own ordered load-phase observation. A load
response before replay completion, replay attached to a different provider
session, replay overflow, or any replay during resume must fail the load or
resume without returning a usable session handle.

## Filesystem And Approval Boundary

ACP text filesystem RPC is client-mediated. The client advertises read and
write support independently and remains responsible for path authorization.
`fs/write_text_file` replaces or creates one UTF-8 text file and returns only
after the client completes the write.

Kimi routes its text and byte write helpers through that callback when the
client advertises it. It still delegates directory operations, metadata,
globbing, process execution, and other host behavior to its local execution
layer. The callback is therefore not a process sandbox.

Three authorities remain separate:

1. a `ReadWrite` working-resource lease lets the execution host service one
   bounded write callback under one root
2. a provider permission response decides whether Kimi may run the Write,
   Edit, Bash, or another tool
3. process filesystem containment prevents the harness or its child work from
   escaping the declared resource by another path

The first proof may implement the first authority. It must not invent the
second. It also needs the third before a production Kimi process can satisfy
Swallowtail's bounded interactive access policy. Merely setting `cwd`,
advertising ACP filesystem methods, or denying a permission prompt is not
host-enforced containment.

The exact write fixture is bounded UTF-8 replacement or creation. Append,
binary fidelity, rename, delete, directory creation, symlink following, and
multi-root writes remain excluded.

## Authentication And Access

Kimi's maintained IDE guidance says the ACP child reuses existing Kimi Code
authentication state. The `login` auth method is terminal delegated: a capable
client may run the same agent binary with additional login arguments, then
call `authenticate` to recheck state. The ACP server does not return a token.

Kimi `0.28.1` also permits a configured non-OAuth model credential to satisfy
its ACP auth gate. That is a separate access route, not a transparent
substitute for Kimi account login.

The first proof binds pre-existing harness-delegated authentication in one
host-approved isolated provider-state environment. It does not advertise
terminal-auth capability, execute advertised auth commands, call logout,
inspect files, or extract a secret. Missing state becomes sign-in-required.
An operator-authorized sign-in action is later host work.

OAuth-backed Kimi Code access and each configured non-OAuth provider route
need separate configured instances, credential mechanisms, endpoint audiences,
entitlement evidence, model routes, and support authority. The driver cannot
infer those identities by scanning configuration.

## Cancellation, Disconnect, And Cleanup

Kimi implements active-turn cancellation with `session/cancel`. It lacks
stable native session close. Its stdio runner closes the harness once on EOF,
SIGINT, or SIGTERM and removes signal listeners.

Swallowtail therefore owns the process connection. Turn cancellation waits for
the prompt terminal result or escalates under the existing lifecycle contract.
Session close ends input, stops the owned process when necessary, joins the
reader and harness work, then releases resource and delegated-auth leases.
Disconnect invalidates the runtime attachment but does not delete the
provider-owned persistent session.

## Recommendation

Proceed with deterministic Kimi `0.28.1` ACP fixtures for the exact subset
above. Promote the provider-neutral persistent-session, replay, write-callback,
delegated-auth, and containment rules before those fixtures.

Do not start the production Kimi driver until a separate host batch proves
process filesystem containment for both local and remote-authoritative
execution hosts. Do not add provider permission approval to obtain a write
demo.

## Risks

- Kimi's ACP and SDK pins may move independently of the CLI release.
- Kimi does not enforce the load/resume request `cwd`; arbitrary stored
  sessions are outside the first authority boundary.
- Kimi replay depends on provider-owned history and does not prove consumer
  transcript completeness.
- ACP text callbacks do not contain the Kimi process or its child commands.
- terminal auth and the legacy `_meta` fallback are executable actions, not
  credential evidence.
- `0.28.1` accepts materially different OAuth and configured-provider access
  routes through the same harness auth gate.

## Primary Sources

- [Kimi Code `0.28.1` release](https://github.com/MoonshotAI/kimi-code/releases/tag/%40moonshot-ai%2Fkimi-code%400.28.1)
- [Kimi Code tagged ACP implementation](https://github.com/MoonshotAI/kimi-code/tree/%40moonshot-ai%2Fkimi-code%400.28.1/packages/acp-adapter)
- [Kimi ACP capability matrix](https://moonshotai.github.io/kimi-code/en/reference/kimi-acp)
- [Kimi IDE and existing-login guidance](https://moonshotai.github.io/kimi-code/en/guides/ides)
- [ACP v1 session setup](https://agentclientprotocol.com/protocol/v1/session-setup)
- [ACP v1 filesystem](https://agentclientprotocol.com/protocol/v1/file-system)
- [ACP v1 authentication](https://agentclientprotocol.com/protocol/v1/authentication)
- [ACP v1 transports](https://agentclientprotocol.com/protocol/v1/transports)
- [ACP schema `v1.19.1`](https://github.com/agentclientprotocol/agent-client-protocol/releases/tag/schema-v1.19.1)

## Promotion

- durable lifecycle and authority rules: Contract 017
- delivery sequence: g01 roadmap 018 and cards 055-059
- next execution batch: card 056
