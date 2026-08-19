# 103 Kimi Code 0.37.2 Useful Newer

Status: completed
Owner: Tom
Created: 2026-08-19
Depends on: Research 159; Research 165; g03.102
Vision tags: maintained compatibility, exact interfaces, currentness
Contract refs: 001, 011, 029, 032, 036-039, 041, 044, 048-049, 052
Planning state: cards 318-319 completed

## Problem

Research 159 ranked remaining AllowUnverified families after Qwen. Kimi
ACP, headless, and local-server share `kimi-code.executable` through
`0.36.1` with AllowUnverified. This host is already qualified `0.34.0`.
Official npm `latest` is `0.37.2`. Raising the bound is useful-newer
support, not a silent `latest` bump and not a flatten onto Python
`kimi-cli`.

## Generation Runway Goal

Freeze Kimi host `0.34.0` / official `0.37.2` identity against the
`0.36.1` corpus, then raise all three qualified ceilings only if that
evidence names a compatible extension.

## Goals

- [x] rank remaining AllowUnverified registry-newer families and pick Kimi
      `0.37.2`
- [x] freeze host `0.34.0` and official `0.37.2` identity and selected
      ACP / headless / local-server source
- [x] raise all three claims through exact `0.37.2` on existing behaviors
- [x] leave Oh My Pi and later 159 families for later one-family work

## Non-Goals

- mixing Python `kimi-cli` into `kimi-code.executable`
- flattening ACP onto local-server
- mapping advertised ACP close/delete, `acp --login`, terminal-auth
  metadata, or watch-fs `runtime_id`
- qualifying experimental v2 headless
- Gemini requalification
- provider prompts, install, update, local-server start, or publication

## Execution Plan

### Batch 103.1 — Identity Corpus

- [x] Execute card 318.
- [x] record host `0.34.0` and official `0.37.2` against `0.36.1`
- [x] name compatible-extension on existing behaviors for card 319
      without changing production claims

### Batch 103.2 — Claim And Acceptance

- [x] Execute card 319 after card 318 names compatible-extension.
- [x] raise ACP and headless `0.29.0..=0.37.2` on existing behaviors
- [x] extend local-server `0.35.0..=0.37.2` heartbeat-ping
- [x] keep baselines, AllowUnverified, and synthetic later `0.37.3`
- [x] refresh matrices, Kimi guides, architecture, contracts that name
      the ceiling, and focused Kimi proof

## Acceptance Criteria

- [x] host `0.34.0` remains Qualified
- [x] official `0.37.2` classifies as Qualified Maintained on all three
      routes
- [x] published intermediates `0.37.0` and `0.37.1` classify as Qualified
- [x] `0.37.3` remains permitted UnverifiedNewer
- [x] advertised ACP close/delete stay unmapped
- [x] decoder and `0.36.1` specimens remain
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

## Closeout

g03.103 is complete. Next Upgrade Workflow family is Oh My Pi `17.3.8`.
Gemini stays deferred.
