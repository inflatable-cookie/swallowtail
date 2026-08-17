# 010 ZCode App-Server Route

Status: draft
Owner: Tom
Updated: 2026-08-17

## Purpose

Record the settled app-server subset for ZCode so g03 can add one
installed-harness route without flattening it onto OpenCode, hosted GLM
HTTP, or a community ACP bridge.

Research 126 already answers whether the harness qualifies. This spec holds
the first production subset, identities, and exclusions until cards promote
them into architecture, package topology, and public route truth.

## Scope

In:

- new package `swallowtail-adapter-zcode`
- one route `zcode.app-server` / driver `swallowtail.zcode.app-server`
- exact runtime `0.16.3` plus `zcode.cjs` digest
  `3e3433d90fa502e5d02498dfde6c2090df898331359bcfe5f3dbc9a1d00b685f`
- owned-process line-delimited JSON stdio (JSON-RPC-shaped; no `jsonrpc`
  field)
- one structured run: create (including runtime-preferences reply),
  subscribe, send, idle fold, kill
- projection of turn lifecycle, assistant text, content-free reasoning
  progress, harness-owned tool call/result lifecycle, usage, and terminal
  completed / error
- host-approved Node, `zcode.cjs`, config, cwd, mode, provider, and model
  as immutable prepared evidence

Out:

- OpenCode HTTP or any OpenCode package change
- hosted Z.AI / BigModel HTTP as this route
- `--prompt` / `--print`, TUI, desktop GUI
- community `zcode-acp` wrap
- session resume, list, fork, rewind, goal, compact, steer
- native `session/stop` cancel
- Contract 054 history
- permission / elicitation
- subagent topology
- model catalogue
- default `yolo` mode
- version bump, tag, or registry publication
- unverified-newer until a second exact payload is qualified

## Decisions Needed

Settled 2026-08-17 from Research 126 and operator direction to follow the
DeepSeek Harness installed-harness lane:

1. identity — new family/package/route; not an extension of
   `swallowtail-adapter-opencode`
2. first surface — app-server stdio; `--print` and ACP later and distinct
3. pin — exact `zcode.runtime` `0.16.3` plus the payload digest above;
   launcher `3.7.7-13` and desktop About `3.7.7` are not the version axis
4. spawn — interpreted `node` + `zcode.cjs` `app-server`; not the TUI
   launcher as the protocol process
5. first role — structured run only
6. cancel — force-stop the owned process; do not advertise `session/stop`
7. live gate — first authenticated/live proof may use a host-local
   OpenAI-compatible endpoint through the custom-provider path with
   provider id `zai`; that does not qualify Z.AI official
8. mode — host supplies `plan` or `build`; Swallowtail does not default
   `yolo`
9. preferences — answering `session/requestRuntimePreferences` is part of
   create, not optional

No remaining product-policy forks block the first tranche.

## Acceptance Criteria

- [x] Research 126 remains the evidence owner for the pin and handshake
- [x] milestone g03.071 sequences corpus, driver, facade, and acceptance
- [x] public route truth keeps OpenCode and hosted GLM HTTP unchanged
- [x] first live selector is separately gated and credential-honest
- [x] redacted fixtures contain no prompts, reasoning text, tool bodies,
      session ids, or secrets

## Promotion Targets

- `docs/architecture/system-architecture.md` and
  `docs/architecture/release-and-package-topology.md` on package acceptance
- Contract 036 package list on package acceptance
- route, feature, and activity matrices plus a canonical guide
- `docs/roadmaps/g03/071-zcode-app-server-foundation.md`
- Research 126 promotion into logs on close
