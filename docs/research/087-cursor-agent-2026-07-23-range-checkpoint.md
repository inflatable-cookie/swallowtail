# 087 Cursor Agent 2026.07.23 Range Checkpoint

Status: promoted
Owner: Tom
Date: 2026-07-31

## Question

Can Swallowtail guarantee current Cursor Agent `2026.07.23-e383d2b` alongside
installed `2026.07.01-41b2de7` without inferring a continuous calendar range or
adding portable authority?

## Method

The g03 checkpoint reconciled installed versions, current package releases,
official Antigravity tags, the maintained ACP registry, Cursor documentation,
and the exact current Cursor macOS arm64 artifact. It then compared selected
source chunks and ran one prompt-free ACP initialize exchange.

No login, provider prompt, authenticated model catalogue, session creation,
workspace mutation, provider-state mutation, installation, or update ran.

## Checkpoint Result

Codex, Grok Build, OpenCode, Pi RPC, Qwen Code, and Antigravity match their
current upstream identities. Installed Kimi Code remains `0.31.0`; current
`0.31.1` is already qualified from its exact artifact. Claude Agent and Gemini
remain paused by operator direction.

Cursor is the only material unqualified drift. The ACP registry publishes
`2026.07.23-e383d2b`, while Swallowtail guarantees only installed
`2026.07.01-41b2de7`.

## Exact Artifact Evidence

The official registry archive SHA-256 is
`f2eb25851f2079dcdf0558a816e06c402d187abfca93255d35167020439ebbf2`.
The downloaded archive matched it and reported `2026.07.23-e383d2b`.

One initialize-only ACP exchange retained:

- protocol version 1
- `cursor_login`
- load and list advertisement
- image input advertisement
- HTTP and SSE MCP advertisement
- no `agentInfo`
- zero stderr bytes

Advertisement still grants no new Swallowtail operation.

## Selected Deltas

The ACP chunk changed. Its selected v1 message surface remains stable, while
the session implementation adds an internal disabled-web-search guard. The
headless chunk also changed, but its output-format and prompt-builder modules
are byte-identical. Exact help retains models, ACP, print, stream JSON, explicit
model, plan and ask, resume and continue, trust, workspace roots, and optional
sandbox controls.

New auto-review, private-worker, and empty-chat commands are not selected.
They do not change catalogue, ACP, or headless authority.

## Compatibility Decision

Use two exact calendar milestones with the same three private route behavior
revisions:

- `2026-07-01` only with build `41b2de7`
- `2026-07-23` only with build `e383d2b`

Do not infer dates between them. Dates above `2026-07-23` remain permitted and
visibly unverified. The parser must enforce the qualified build suffix for
every qualified date, not only whichever date is currently latest.

## Contract Result

No contract change is required. Contracts 011, 029, and 032 already separate
exact executable observation from compatibility claims. Contracts 015, 020,
037, 039, and 044 bound the existing ACP, catalogue, prepared, structured, and
activity surfaces.

## Risks

- opaque same-day builds remain independent evidence points
- calendar gaps remain unsupported without exact evidence
- advertised ACP lifecycle and MCP surfaces remain broader than production
  authority
- authenticated catalogue and provider prompts were not rerun on the current
  artifact

## Primary Sources

- [ACP registry](https://cdn.agentclientprotocol.com/registry/v1/latest/registry.json)
- [Cursor CLI installation](https://docs.cursor.com/en/cli/installation)
- [Cursor headless CLI](https://cursor.com/docs/cli/headless)
- [Cursor output formats](https://cursor.com/docs/cli/reference/output-format)
- [Cursor changelog](https://cursor.com/en-US/changelog)
