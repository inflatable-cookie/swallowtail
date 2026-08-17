# 069 DeepSeek Harness JSON-RPC Foundation

Status: completed
Owner: Tom
Created: 2026-08-17
Depends on: Research 124; Spec 008
Vision tags: installed harness, DeepSeek Harness, structured execution, JSON-RPC
Contract refs: 005-006, 009-010, 023, 029, 032-033, 036-037, 039-041, 044-045, 051-052

## Problem

DeepSeek Harness exposes an unattended JSON-RPC stdio runtime. Swallowtail has
no route for it. Research 124 qualifies exact runtime-bin `0.1.0rc6` for one
dedicated installed structured-run driver, distinct from Open Platform
`deepseek.continuation`.

## Generation Runway

Advance g03's high-value installed-harness goal without chasing ACP, the Web
UI, or every plugin composition. JSON-RPC qualifies because it has a pinned
payload, a documented automation wire, and live turn/tool/usage evidence.

## Execution Plan

- [x] card 218: freeze exact runtime-bin artifact, JSON-RPC, session-log,
      failure, and private-state boundaries
- [x] card 219: implement discovery, compatibility, decode, idle fold,
      activity, process-kill cancellation, deadline, and cleanup
- [x] card 220: expose host-approved preparation and an exact structured-run
      facade with explicit provider, model, config, and cwd
- [x] card 221: complete package, guide, example, matrices, live acceptance,
      release-baseline handling, and closeout evidence

## Goals

- [x] add one separately selectable `swallowtail-adapter-deepseek-harness`
      package
- [x] qualify only exact runtime-bin `0.1.0rc6` on axis
      `deepseek-harness.runtime-bin`
- [x] spawn the host-approved `dsh-jsonrpc-agent` payload, not a Python SDK
      wrapper
- [x] bind host-approved Cordis config, cwd, provider, and model
- [x] preserve JSON-RPC stream ownership, thinking progress, text, tool
      lifecycle, usage, and terminal evidence
- [x] keep ACP, Web `/api`, headless CLI, and `deepseek.continuation` outside
      the first route

## Boundaries

- no screen scraping, TUI, Web UI automation, login, or credential extraction
- no ACP, headless CLI, or Web `/api` driver in this milestone
- no unverified-newer execution on the RC pin
- no default `danger-full-access` composition
- no ingestion of tool bodies, reasoning text, prompts, or raw JSONL into
      stable diagnostics
- no claim of interactive continuation, catalogue, native cancel, subagent
      control, or DeepSeek-official SSE behavior
- no version bump, tag, GitHub Release, or registry mutation

## Acceptance Criteria

- [x] the route rejects version or payload drift before provider work
- [x] deterministic fixtures cover text success, tool success, tool error,
      missing credential, unknown events, bounds, and malformed input
- [x] the prepared facade binds exact executable, config, provider, model,
      resource, and host services
- [x] focused and extracted-package validation pass without network
      credentials
- [x] one separately gated live probe passes through the prepared facade
- [x] route matrix, feature matrix, guide map, example, architecture, package
      contract, and release tooling remain mutually honest

Card 221's package and deterministic acceptance work merged on `52263993`
(PR 1), including payload-digest admission at `96db297e`. The gated
installed and prepared-facade live probes later passed on the host-approved
`0.1.0rc6` payload through local Ollama. That does not qualify
`deepseek-official`.

## Planning Checkpoint

g03.070 now owns Web `/api` as `deepseek-harness.local-server`. Session-id
continuity, ACP, and DeepSeek-official live remain later gates.
