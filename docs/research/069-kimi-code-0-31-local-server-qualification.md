# 069 Kimi Code 0.31 Local-Server Qualification

Status: promoted
Owner: Tom
Date: 2026-07-30

## Question

May Swallowtail extend the Kimi Code local REST/WebSocket server guarantee
through `0.31.0` without widening its public activity semantics?

## Method

Evidence was checked on 2026-07-30.

- compared exact signed `0.29.2`, `0.30.0`, and `0.31.0` tags
- compared selected REST, WebSocket, status-projection, catalogue, and session
  source
- inspected the exact upstream tests for the changed status projection
- started installed native Kimi Code `0.31.0` as a bounded foreground
  loopback server
- checked unauthenticated health, bearer rejection, authenticated metadata,
  and authenticated catalogue access
- stopped and reaped the foreground process

No credential, bearer token, account identity, model name, raw provider
payload, session identifier, or private diagnostic is repository evidence.

## Exact Releases

| Version | Commit | Tree | Result |
| --- | --- | --- | --- |
| `0.29.2` | `8a45f10eddbb35c317047e82e567cdb59a220b4f` | `4b583494b6dda5277333b9a4ec4523587f93819a` | prior upper guarantee |
| `0.30.0` | `16c7189bd54a42fae65b1bbafd0843420523f797` | `109a9e75b6f9ba9e3b9243734d5ad09c20e4b373` | selected local-server source byte-identical |
| `0.31.0` | `bc28e9d802fbec29395a7aed85e880679a050145` | `44634aa54e11f6d67e7807edf77bdfe19b3b99aa` | compatible status-projection milestone |

## Source Delta

`0.30.0` is byte-identical to `0.29.2` across the selected local-server
surface.

At `0.31.0`:

- REST metadata, catalogue, session, prompt, archive, and restore routes are
  unchanged
- WebSocket v2 control, event schemas, cursor behavior, and close behavior are
  unchanged
- every `agent.status.updated` event now folds the full legacy model, context,
  and usage snapshot, rather than doing so only for the main agent
- the derived secondary-model identifier is projected as its display name or
  wire name when catalogue resolution succeeds

The change fixes subagent status cards whose initial model slice arrived
before `subagent.spawned`. It does not add or remove an event type.

Swallowtail already treats `agent.status.updated` as non-rendered progress.
Subagent activity comes from the dedicated spawned, started, suspended,
completed, and failed events. The richer status payload is therefore accepted
and bounded without adding model, context, usage, or raw status content to the
portable activity stream.

## Live Evidence

The installed native executable reported `0.31.0`. One foreground loopback
server:

- returned healthy status
- rejected an unauthenticated metadata request with `401`
- returned authenticated metadata reporting `0.31.0`
- returned a bounded authenticated model catalogue with four entries
- stopped within the bounded probe and left no child running

No prompt or provider inference was needed. This proves startup, bearer
enforcement, metadata, and catalogue compatibility; the deterministic corpus
owns the changed event shape.

## Selection

Extend `kimi.local-server.executable` maintained support through `0.31.0`.

Use four behavior segments:

- exact `0.28.1`: REST/WebSocket v2 baseline
- exact `0.29.0`: profile-tool support
- `0.29.1..=0.30.0`: filtered catalogue and global events
- exact `0.31.0`: the same route plus full subagent status projection and
  derived-model display aliases

Keep later stable releases visible as unverified newer. Advance the
compatibility claim identifier because the upper guarantee changes.

## Contract Fit

Contracts 029, 032, 037, 038, 042, and 044 already require exact version
bindings, explicit behavior milestones, visible unverified-newer execution,
bounded provider events, retention truth, and joined owned-child cleanup.

No contract or architecture-shape change is required. The realized
architecture and route guidance need only report the new upper guarantee.

## Sources

- [Kimi Code 0.31.0 release](https://github.com/MoonshotAI/kimi-code/releases/tag/%40moonshot-ai%2Fkimi-code%400.31.0)
- [Kimi Code changelog](https://moonshotai.github.io/kimi-code/en/release-notes/changelog.html)
- [Kimi command and local-server reference](https://moonshotai.github.io/kimi-code/en/reference/kimi-command.html)

