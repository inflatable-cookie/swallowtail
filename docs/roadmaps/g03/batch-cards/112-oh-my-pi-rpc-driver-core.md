# 112 Oh My Pi RPC Driver Core

Status: completed
Owner: Tom
Created: 2026-08-05
Milestone: `../040-oh-my-pi-rpc-foundation.md`
Depends on: card 111

## Goal

Implement exact discovery, v2 transport, catalogue, structured-run, and
interactive-session behavior for OMP 17.2.9.

## Acceptance

- [x] ready and pre-turn lifecycle frames are admitted safely
- [x] v2 chunks reassemble within exact bounds and fail closed on drift
- [x] catalogue and operation paths negotiate v2
- [x] terminal and cleanup behavior match exact OMP events

## Completion

`OhMyPiRpcDriver` implements catalogue, structured-run, and interactive-session
roles over negotiated RPC v2. Exact corpus tests cover ready/command ordering,
chunk completion and drift, `agent_end`, deadlines, cancellation, and joined
cleanup.
