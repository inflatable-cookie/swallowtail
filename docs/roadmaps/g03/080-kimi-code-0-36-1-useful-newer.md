# 080 Kimi Code 0.36.1 Useful Newer

Status: completed
Owner: Tom
Created: 2026-08-18
Depends on: Research 127; Research 137; g03.079
Vision tags: maintained compatibility, exact interfaces, currentness
Contract refs: 001, 011, 029, 032, 036-039, 041, 044, 048-049, 052
Planning state: cards 248-249 completed

## Problem

Research 127 ranked AllowUnverified host drift after OpenCode HTTP. Kimi
ACP, headless, and local-server share `kimi-code.executable` through
`0.31.1` with AllowUnverified. This host is `0.34.0`. Official npm
`latest` is `0.36.1`. Raising the bound is useful-newer support, not a
silent `latest` bump and not a flatten onto Python `kimi-cli`.

## Generation Runway Goal

Freeze Kimi `0.34.0` / `0.36.1` identity against the `0.31.1` corpus, then
raise all three qualified ceilings only if that evidence names a
compatible extension.

## Goals

- [x] rank remaining AllowUnverified host-drift families and pick Kimi
- [x] freeze host `0.34.0` and official `0.36.1` identity and selected
      ACP / headless / local-server source
- [x] raise all three claims through exact `0.36.1`, adding local-server
      private milestones including heartbeat ping/pong
- [x] leave Ollama, Gemini, and other 127 families for later one-family
      work

## Non-Goals

- mixing Python `kimi-cli` into `kimi-code.executable`
- flattening ACP onto local-server
- mapping advertised ACP close/delete
- qualifying experimental v2 headless
- bulk-bumping other AllowUnverified families
- Gemini requalification
- provider prompts, install, update, local-server start, or publication

## Execution Plan

### Batch 80.1 — Identity Corpus

- [x] Execute card 248.
- [x] record host `0.34.0` and official `0.36.1` against exact `0.31.1`
- [x] name compatible-extension plus local-server private milestones for
      card 249 without changing production claims

### Batch 80.2 — Claim And Acceptance

- [x] Execute card 249 after card 248 names compatible-extension.
- [x] raise ACP and headless `0.29.0..=0.36.1` on existing behaviors
- [x] add local-server `0.32.0..=0.34.0` optional-meta-flags and
      `0.35.0..=0.36.1` heartbeat-ping, including ping/pong
- [x] keep baselines, AllowUnverified, and synthetic later `0.37.0`
- [x] refresh matrices, guides, contracts that name the ceiling, and
      focused Kimi proof

## Acceptance Criteria

- [x] host `0.34.0` and official `0.36.1` classify as Qualified
      Maintained on all three routes
- [x] `0.37.0` remains permitted UnverifiedNewer
- [x] advertised ACP close/delete stay unmapped
- [x] decoder specimens remain the existing ACP, headless, and
      local-server corpora
- [x] focused Kimi proof and package verify pass
- [x] matrices and guides name the new package ceiling

## Decision Gates

- Stop if exact artifact identity cannot be corroborated from official
  sources.
- Stop if selected ACP or default headless protocol differs from the
  recorded evidence.
- Stop if local-server ping/pong cannot be answered without a new public
  operation.
- Stop if acceptance depends on authentication, a provider prompt, or
  starting the local server.

## Next Planning Checkpoint

Return to the g03 compatibility-maintenance checkpoint after card 249.
Remaining Research 127 rows stay one-family-later and should qualify
useful-newer support. Next rank: Ollama. Gemini stays deferred.
