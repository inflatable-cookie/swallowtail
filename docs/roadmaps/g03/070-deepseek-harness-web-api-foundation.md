# 070 DeepSeek Harness Web `/api` Foundation

Status: active
Owner: Tom
Created: 2026-08-17
Depends on: Research 125; Spec 009; g03.069
Vision tags: installed harness, DeepSeek Harness, local server, catalogue,
  history, cancellation
Contract refs: 005-006, 009-010, 017, 023, 029, 032-033, 036-039, 044-045,
  051-052, 054

## Problem

DeepSeek Harness Web `/api` exposes catalogue, paged history, native cancel,
fork, and archive that JSON-RPC stdio does not. Swallowtail has no route for
that host. Research 125 qualifies exact `@deepseek-ai/dsh@0.1.0-rc.6` for one
dedicated local-server driver, distinct from `deepseek-harness.jsonrpc` and
Open Platform `deepseek.continuation`.

## Generation Runway

Advance g03's high-value installed-harness goal without chasing ACP, the
browser UI, or the configuration plane. Web `/api` qualifies because it is a
documented HTTP+WebSocket automation wire with native session lifecycle
methods. JSON-RPC stays the one-shot stdio run.

## Execution Plan

- [x] card 222: freeze exact `dsh` web artifact, loopback trust fence, method
      allowlist, unary and mux corpus, and private-state boundaries
- [x] card 223: implement discovery, compatibility, allowlisted decode,
      history, prompt, native cancel, fork, archive, deadline, and cleanup
- [x] card 224: expose host-approved preparation and exact catalogue,
      history, structured-run, fork, and archive facades
- [ ] card 225: complete route truth, operator guidance, and deterministic
      plus live acceptance evidence

## Goals

- [ ] add route `deepseek-harness.local-server` on
      `swallowtail-adapter-deepseek-harness`
- [ ] qualify only exact `@deepseek-ai/dsh@0.1.0-rc.6` on axis
      `deepseek-harness.web`
- [ ] spawn host-approved `dsh web` on loopback, not a browser and not the
      JSON-RPC binary
- [ ] bind host-approved Cordis patch, cwd, provider, and model
- [ ] preserve catalogue, control-free history candidate, mux events, native
      cancel, fork, and archive
- [ ] keep JSON-RPC, ACP, headless CLI, credentials/settings, and
      `deepseek.continuation` outside this route

## Boundaries

- no screen scraping, TUI, or Web UI automation
- no settings, credentials, llm configuration, directory picker, or ZIP
      export
- no unverified-newer execution on the RC pin
- no default `danger-full-access` composition
- no non-loopback bind and no invented bearer token
- no ingestion of tool bodies, reasoning text, prompts, or raw export bytes
      into stable diagnostics
- no Contract 054 support claim until history proof
- no claim of DeepSeek-official SSE behavior
- no version bump, tag, GitHub Release, or registry mutation

## Acceptance Criteria

- [ ] the route rejects version or method-allowlist drift before provider work
- [ ] deterministic fixtures cover list, history, prompt, cancel, fork,
      archive, denied methods, bounds, and malformed input
- [ ] `session.history` fixtures prove no Agent resume
- [ ] the prepared facade binds exact CLI, config, loopback endpoint,
      provider, model, resource, and host services
- [ ] focused and extracted-package validation pass without network
      credentials
- [ ] one separately gated live smoke passes through the prepared facade
- [ ] route matrix, feature matrix, guide map, example, architecture, package
      contract, and release tooling remain mutually honest with JSON-RPC

## Planning Checkpoint

After card 225, reassess Contract 054 promotion, ACP, JSON-RPC session-id
continuity, and DeepSeek-official live as separate gates.
