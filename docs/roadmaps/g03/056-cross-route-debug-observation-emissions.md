# 056 Cross-Route Debug Observation Emissions

Status: completed
Owner: Tom
Created: 2026-08-08
Depends on: g03.055
Vision tags: consumer stability, safe diagnostics, compatibility maintenance
Contract refs: 003, 004, 009, 010, 044, 051, 053
Planning state: cards 172-175 completed

## Problem

Contract 053's host debug sink is live, but only Codex app-server emits.
Hosts that opt in still see silence on the other 33 production routes when
discovery, prep, wire, or process failures occur.

## Goals

- [x] emit failure-path debug observations from shared installed discovery and
      plan-family readiness edges
- [x] plumb `HostServices` into ACP and harness-RPC connections and emit on
      malformed inbound
- [x] emit from headless process pumps and Codex exec failure paths
- [x] emit from hosted/attached HTTP/SSE/WS and remote ACP failure paths
- [x] update the debug-observation guide emitter inventory for realized routes

## Execution Plan

- [x] Execute card 172 (shared discovery/prep emissions).
- [x] Execute card 173 (ACP + Pi/OhMyPi plumbing and wire/parse).
- [x] Execute card 174 (headless + Codex exec).
- [x] Execute card 175 (HTTP/SSE/WS + remote ACP + guide closeout).

## Boundaries

- failure-path emissions only; no routine happy-path wire spam
- no safe diagnostic code/message/classification changes
- protocol crates stay free of `HostServices`
- no consumer-repo commits, tag, or release in this milestone
- no live provider work unless a card explicitly requires it

## Acceptance Criteria

- [x] shared discovery/prep emits correlated observations when an observer is
      registered
- [x] every transport family has at least one deterministic failure path that
      records a correlated `DebugObservation`
- [x] focused validation passes for touched packages
- [x] guide Current Emitters table matches realized emitters

## Next Planning Checkpoint

Return to the g03 evidence gate after cards 172-175.
