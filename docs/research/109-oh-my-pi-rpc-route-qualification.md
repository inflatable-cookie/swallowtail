# 109 Oh My Pi RPC Route Qualification

Status: promoted
Owner: Tom
Updated: 2026-08-05

## Question

Can Oh My Pi reuse `pi.rpc`, or does its current public surface require a
separate Swallowtail route?

## Evidence

`@oh-my-pi/pi-coding-agent@17.2.9` is a separate published artifact with its
own `omp` executable, release commit, package version axis, local auth/config
store, and RPC evolution. Its RPC mode emits an initial v1 ready frame,
advertises protocol versions 1 and 2, negotiates v2 explicitly, and chunks
logical frames above 1 MiB up to a 64 MiB reassembled ceiling. It emits
`available_commands_update` before the first prompt and terminates an actual
settled turn through `agent_end.isTerminal`.

The route also exposes explicit model and thinking-level commands, configured
model discovery, typed extension UI, todo and subagent surfaces, host-tool
injection, and session mutation. Those capabilities do not grant Swallowtail
authority merely because the wire supports them.

Exact release evidence:

- package: `@oh-my-pi/pi-coding-agent@17.2.9`
- npm shasum: `22bcb3163192861726d1a920f26f899f5610795f`
- npm integrity: `sha512-VTFxYdQxjr5TY/UyERlstPF2fjIsW4QUEhPATvrf1yZjLGIQqb2Q5nJGQahazDdDZUhLu1a5h7Ktm+sxK01Log==`
- release commit: `f7f8e040ee04710414fbd775431091fa301b9786`
- installed launcher: `#!/usr/bin/env bun`
- installed version output: `omp/17.2.9`

The local identity-only probe on 2026-08-05 observed `omp/17.2.9` through the
Bun launcher. The probe did not inspect auth state or send provider work.

The later operator-gated authenticated probe used OMP's stored local auth and
the exact `openai-codex` / `gpt-5.6-luna` / `low` selection. The prepared
catalogue exposed that route; the prepared structured run confirmed the exact
provider, model, and reasoning state, returned `OMP_LIVE_OK`, supplied usage,
and joined cleanly.

That probe also exposed two qualified lifetime shapes absent from the initial
fixture. OMP emits `thinking_level_changed` after the startup control command
and may emit `model_changed` independently of an active prompt. RPC UI emits
empty `setWidget` frames to clear startup display state. These are lifecycle
or clear observations, not malformed turn activity. The exact source types,
live ordering, and deterministic fixtures now agree.

Sources:

- https://github.com/can1357/oh-my-pi/tree/f7f8e040ee04710414fbd775431091fa301b9786
- https://github.com/can1357/oh-my-pi/blob/f7f8e040ee04710414fbd775431091fa301b9786/docs/rpc.md
- https://github.com/can1357/oh-my-pi/blob/f7f8e040ee04710414fbd775431091fa301b9786/packages/coding-agent/DEVELOPMENT.md
- https://www.npmjs.com/package/@oh-my-pi/pi-coding-agent/v/17.2.9

## Decision

Add a distinct `oh-my-pi.rpc` route and `swallowtail-adapter-oh-my-pi` crate.
Do not alias it to `pi.rpc` or extend `pi.package`.

The first qualified subset includes:

- approved installed `omp` executable discovery
- local OMP authentication with no Swallowtail credential lease
- configured model catalogue
- one-prompt structured runs and reusable interactive sessions
- exact provider/model selection and optional reasoning selection
- read-only working-resource tools plus typed question exchange
- bounded PNG input, streamed activity, usage, cancellation, and fresh-session
  replacement
- RPC v2 negotiation and bounded chunk reassembly

The first subset excludes write-capable tools, permission exchange, host-tool
injection, session switching/import, and subagent observation or control.
These need separate authority and corpus work.

## Promotion

Contracts 029, 036-037, 041, 044, and 050 own the durable boundary. Roadmap
g03.040 owns implementation and package acceptance.
