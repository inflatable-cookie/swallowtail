# 071 ZCode App-Server Foundation

Status: completed
Owner: Tom
Created: 2026-08-17
Depends on: Research 126; Spec 010
Vision tags: installed harness, ZCode, structured execution, app-server
Contract refs: 005-006, 009-010, 023, 029, 032-033, 036-037, 039-041,
  044-045, 051-052

## Problem

ZCode exposes an unattended app-server stdio protocol. Swallowtail has no
route for it. Research 126 qualifies exact runtime `0.16.3` for one
dedicated installed structured-run driver, distinct from OpenCode and from
hosted GLM HTTP.

## Generation Runway

Advance g03's high-value installed-harness goal without chasing ACP, the
desktop ADE, or every plugin. App-server qualifies because it has a pinned
payload, a documented automation wire, and a handshake-proven create path.

## Execution Plan

- [x] card 226: freeze exact runtime artifact, app-server framing,
      handshake, session-log, failure, and private-state boundaries
- [x] card 227: implement discovery, compatibility, decode, idle fold,
      activity, process-kill cancellation, deadline, and cleanup
- [x] card 228: expose host-approved preparation and an exact structured-run
      facade with explicit config, cwd, mode, provider, and model
- [x] card 229: complete package, guide, example, matrices, live acceptance,
      release-baseline handling, and closeout evidence

## Goals

- [x] add one separately selectable `swallowtail-adapter-zcode` package
- [x] qualify only exact runtime `0.16.3` on axis `zcode.runtime`
- [x] spawn host-approved `node` + `zcode.cjs app-server`, not a TUI or ACP
      wrap
- [x] bind host-approved config, cwd, mode, provider, and model
- [x] preserve stream ownership, thinking progress, text, tool lifecycle,
      usage, and terminal evidence
- [x] keep `--print`, ACP, history, native stop, OpenCode, and Z.AI official
      outside the first route

## Boundaries

- no screen scraping, TUI, desktop UI automation, login, or credential
      extraction
- no `--print`, community ACP, or OpenCode driver in this milestone
- no unverified-newer execution on the exact pin
- no default `yolo` mode
- no ingestion of tool bodies, reasoning text, prompts, session ids, or
      raw JSONL into stable diagnostics
- no claim of interactive continuation, catalogue, native cancel, subagent
      control, or Z.AI official behavior
- no version bump, tag, GitHub Release, or registry mutation

## Acceptance Criteria

- [x] the route rejects version or payload drift before provider work
- [x] create fails closed if runtime-preferences is not answered
- [x] deterministic fixtures cover handshake, text success, tool success,
      tool error, missing credential, unknown events, bounds, and malformed
      input
- [x] the prepared facade binds exact Node, payload, config, provider,
      model, mode, resource, and host services
- [x] focused and extracted-package validation pass without network
      credentials
- [x] one separately gated live probe passes through the prepared facade
- [x] route matrix, feature matrix, guide map, example, architecture,
      package contract, and release tooling remain mutually honest

Card 229 live smoke passed through the prepared facade on host-local Ollama
through custom provider id `zai`. Exact runtime `0.16.3` is live-proven for
one structured app-server run. That does not qualify Z.AI official.

## Planning Checkpoint

After card 229, reassess native `session/stop`, `--print`, history, ACP, and
Z.AI official as separate later gates. Keep OpenCode and hosted GLM HTTP
outside the first ZCode route.
