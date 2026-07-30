# 068 Kimi Code 0.31 Currentness And Live Evidence

Status: promoted
Owner: Tom
Date: 2026-07-30

## Question

May Swallowtail extend its Kimi Code ACP and headless guarantees from `0.29.2`
through `0.31.0`, and does that remove any practical need for a separate
Python `kimi-cli` route?

## Method

Evidence was checked on 2026-07-30.

- compared exact signed `0.29.2`, `0.30.0`, and `0.31.0` tags
- compared only Swallowtail-selected ACP, headless, catalogue, session, and
  local WebSocket source
- probed the installed native macOS arm64 `kimi` executable
- ran one bounded authenticated headless stream-JSON prompt
- ran one bounded ACP initialize, session creation, prompt, and terminal turn
- authorized no callback, tool, workspace write, destructive action, or
  local-server launch

The two live prompts created normal Kimi retained session state. No session
identifier, credential, raw provider payload, account identity, or private
diagnostic is repository evidence.

## Exact Releases

| Version | Commit | Tree | Result |
| --- | --- | --- | --- |
| `0.29.2` | `8a45f10eddbb35c317047e82e567cdb59a220b4f` | `4b583494b6dda5277333b9a4ec4523587f93819a` | prior upper guarantee |
| `0.30.0` | `16c7189bd54a42fae65b1bbafd0843420523f797` | `109a9e75b6f9ba9e3b9243734d5ad09c20e4b373` | selected source byte-identical |
| `0.31.0` | `bc28e9d802fbec29395a7aed85e880679a050145` | `44634aa54e11f6d67e7807edf77bdfe19b3b99aa` | selected deltas classified below |

## Source Delta

`0.30.0` is byte-identical to `0.29.2` across:

- CLI option parsing, prompt runner, and stream-JSON renderer
- the complete ACP adapter tree
- local WebSocket control and event schemas
- local session event broadcasting
- model catalogue and session routes

At `0.31.0`:

- the stream-JSON renderer remains byte-identical
- headless option changes only admit custom-agent selection
- headless session creation forwards that optional selection
- ACP protocol source remains byte-identical; its package moves from `0.3.5`
  to `0.3.6` through dependency-only changes
- local WebSocket control, event schemas, catalogue, and session routes remain
  byte-identical
- local session broadcasting changes subagent status projection

The ACP and default headless behavior revisions do not change. The
local-server broadcaster does. Its guarantee must remain capped at `0.29.2`
until that event delta has a deterministic corpus.

## Live Evidence

The installed executable is native Kimi Code `0.31.0` at
`~/.kimi-code/bin/kimi`. It is not the Python `kimi-cli` distribution.

The installed-version probe passed after the exact executable directory was
supplied to the probe environment. The original PATH-only attempt failed
before launch because this Codex process does not inherit
`~/.kimi-code/bin`. Official Kimi IDE guidance documents the same macOS GUI
PATH condition and recommends an absolute command path.

The authenticated headless probe:

- selected stream-JSON output
- returned the exact requested assistant token
- emitted the already-qualified `session.resume_hint` terminal record
- exited successfully

The authenticated ACP probe:

- reported agent version `0.31.0`
- advertised one authentication method and three configuration options
- created one session under an empty temporary workspace
- streamed the exact requested assistant token
- terminated the prompt with `end_turn`
- kept stderr bounded

This is compatibility evidence for Kimi Code. It is not a general account,
quota, billing, model-catalogue, tool, callback, or local-server guarantee.

## Selection

Extend:

- `kimi.acp.executable` maintained support through `0.31.0`
- `kimi.headless.executable` maintained support through `0.31.0`

Retain:

- ACP legacy reasoning exact `0.28.1`
- ACP declared-effort behavior `0.29.0..=0.31.0`
- headless stream-JSON behavior `0.29.0..=0.31.0`
- local-server maintained support only through `0.29.2`
- later stable releases as visible unverified-newer attempts

The compatibility claim identifiers must advance because their upper
guarantees change.

## Python Route Decision

Decline the separate Python `kimi-cli` route for now. The maintained native
Kimi Code distribution already supplies:

- authenticated ACP interaction
- non-interactive stream-JSON structured runs
- explicit model selection
- retained sessions and managed recovery
- a separate local REST/WebSocket route

Adding another distribution would duplicate the practical headless need while
introducing a second executable identity, version axis, state root, protocol,
reasoning model, package surface, and consumer choice. Reopen only on a
concrete capability that Kimi Code cannot provide.

## Contract Fit

Contracts 011, 023, 033, 036, 037, 042, and 044 already require:

- exact interface bindings
- explicit compatibility claims and behavior revisions
- visible unverified-newer execution
- ambient harness authority
- durable retention and managed recovery truth
- prepared evidence and activity profiles
- no silent capability widening

No contract or architecture delta is required.

## Sources

- [Kimi Code 0.30.0 release](https://github.com/MoonshotAI/kimi-code/releases/tag/%40moonshot-ai%2Fkimi-code%400.30.0)
- [Kimi Code 0.31.0 release](https://github.com/MoonshotAI/kimi-code/releases/tag/%40moonshot-ai%2Fkimi-code%400.31.0)
- [Kimi Code changelog](https://moonshotai.github.io/kimi-code/en/release-notes/changelog.html)
- [Kimi command and stream-JSON reference](https://moonshotai.github.io/kimi-code/en/reference/kimi-command.html)
- [Kimi ACP reference](https://moonshotai.github.io/kimi-code/en/reference/kimi-acp)
- [Kimi IDE and executable-path guidance](https://moonshotai.github.io/kimi-code/en/guides/ides.html)

