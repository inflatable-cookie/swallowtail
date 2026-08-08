# 058 Multi-Route Provider Session History

Status: completed
Owner: Tom
Created: 2026-08-08
Depends on: g03.057
Vision tags: consumer stability, session continuity, compatibility maintenance
Contract refs: 017, 025, 038, 044, 046, 048, 054
Planning state: cards 179-180 completed

## Problem

g03.057 landed portable history pages and a Codex-only proof. Consumers need
the same browse role on every route that can page honestly without granting a
live control handle.

## Goals

- [x] wire OpenCode HTTP history over the existing ascending message replay
- [x] wire Alibaba retained history over the shared ascending items walk with
      resource-free posture
- [x] keep Claude Agent ACP and Kimi ACP unsupported until a control-free
      history wire exists
- [x] update guides, route matrix, contract acceptance notes, and API baselines

## Execution Plan

- [x] Execute card 179 (OpenCode + Alibaba history drivers and fixtures).
- [x] Execute card 180 (docs, baselines, and unsupported-route inventory).

## Boundaries

- no wrapping `session/load` as history on ACP routes
- no weakening of load complete-before-ready
- no live provider work
- no tag or release in this milestone

## Acceptance Criteria

- [x] OpenCode and Alibaba retained fixtures prove newest-first paging without
      control side effects
- [x] runtime history validation accepts resource-free DirectModelInference
      posture matching retained bindings
- [x] focused runtime, OpenCode, and Alibaba package validation passes
- [x] public API baselines include the new prepared history surfaces
- [x] guides list advertising routes and explicit unsupported ACP stop reasons

## Next Planning Checkpoint

Return to the g03 evidence gate, or open a qualification card only when a
control-free ACP history wire is evidenced.
