# 070 DeepSeek Harness Web `/api` Foundation

Status: completed
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
- [x] card 225: complete route truth, operator guidance, and deterministic
      plus live acceptance evidence

## Goals

- [x] add route `deepseek-harness.local-server` on
      `swallowtail-adapter-deepseek-harness`
- [x] qualify only exact `@deepseek-ai/dsh@0.1.0-rc.6` on axis
      `deepseek-harness.web`
- [x] spawn host-approved `dsh web` on loopback, not a browser and not the
      JSON-RPC binary
- [x] bind host-approved Cordis patch, cwd, provider, and model
- [x] preserve catalogue, control-free history candidate, mux events, native
      cancel, fork, and archive
- [x] keep JSON-RPC, ACP, headless CLI, credentials/settings, and
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
- no Contract 054 live claim; corpus-qualified history remains operator-gated
- no claim of DeepSeek-official SSE behavior
- no version bump, tag, GitHub Release, or registry mutation

## Acceptance Criteria

- [x] the route rejects version or method-allowlist drift before provider work
- [x] deterministic fixtures cover list, history, prompt, cancel, fork,
      archive, denied methods, bounds, and malformed input
- [x] `session.history` fixtures prove no Agent resume
- [x] the prepared facade binds exact CLI, config, loopback endpoint,
      provider, model, resource, and host services
- [x] focused and extracted-package validation pass without network
      credentials
- [x] one separately gated live smoke passes through the prepared facade
- [x] route matrix, feature matrix, guide map, example, architecture, package
      contract, and release tooling remain mutually honest with JSON-RPC

Card 225 live smoke passed through the prepared facade on host-local Ollama.
Exact npm `0.1.0-rc.6` is live-proven for one structured Web run. That does
not qualify `deepseek-official`. Contract 054 stays corpus-qualified only.

## Planning Checkpoint

After card 225, reassess Contract 054 promotion, ACP, JSON-RPC session-id
continuity, and DeepSeek-official live as separate gates.
