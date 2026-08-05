# 040 Oh My Pi RPC Foundation

Status: completed
Owner: Tom
Created: 2026-08-05
Depends on: g03.039
Vision tags: installed harness, local auth, RPC v2
Contract refs: 029, 036-037, 041, 044, 050
Planning state: cards 111-114 complete

## Problem

Oh My Pi descends from Pi but now owns a distinct package, executable, local
state, protocol evolution, and capability surface. Reusing `pi.rpc` would
erase exact artifact identity and accept incompatible wire behavior.

## Goal

Add one separately qualified OMP route with a consumer-ready prepared facade
without inheriting unproven write, session, host-tool, or subagent authority.

## Execution Plan

- [x] card 111: promote exact artifact, protocol, and contract evidence
- [x] card 112: implement discovery, RPC v2, catalogue, run, and session core
- [x] card 113: implement local-auth prepared facade, reasoning, input, and activity
- [x] card 114: close route matrices, package proof, and public guidance

## Boundaries

- no alias to `pi.rpc` or `pi.package`
- no API-key requirement or credential lease for local OMP auth
- no automatic install, login, model fallback, or provider fallback
- no write-capable tools or persistent permission grants
- no session switching/import or provider-state retention
- no host-tool injection or subagent authority in this tranche
- deterministic acceptance does not require authenticated provider work
- any authenticated smoke remains separately operator-gated

## Acceptance

- exact `17.2.9` artifact and RPC v2 corpus
- separate driver, package axis, transport, and prepared integration
- strict 1 MiB physical and 64 MiB logical frame bounds
- exact model and optional reasoning confirmation
- typed questions, activity, usage, PNG input, and fresh replacement remain portable
- operator-gated `openai-codex` / `gpt-5.6-luna` / `low` prepared smoke passes
- focused and affected-package validation pass

## Lane Runway

Cards 111-114 are complete. g03 returns to its evidence gate.
