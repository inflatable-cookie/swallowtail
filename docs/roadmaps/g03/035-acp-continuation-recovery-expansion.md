# 035 ACP Continuation Recovery Expansion

Status: completed
Owner: Tom
Created: 2026-08-05
Depends on: g03.034
Vision tags: restart continuity, ACP sessions, prepared recovery
Contract refs: 013, 015, 017, 029, 037, 050
Planning state: card 090 completed; cards 091-092 superseded after no route passed

## Problem

Cursor Agent ACP and Grok Build ACP advertise `loadSession`, but Swallowtail
only qualifies continuation recovery for Claude Agent ACP and Kimi ACP.
Protocol advertisement alone cannot establish replay ordering, exact resource
binding, readiness, or safe cleanup.

## Generation Runway Goal

Extend continuation recovery to further ACP routes only where exact
route-specific evidence satisfies the existing load/replay contract.

## Goals

- [x] qualify Cursor and Grok load/replay independently
- [x] keep both mappings unsupported after their exact replay gates failed
- [x] preserve route, version, resource, access, model, and attachment binding
- [x] freeze deterministic negative evidence and public route truth

## Execution Plan

- [x] card 090: freeze exact Cursor and Grok load/replay evidence and select
      each route independently
- [x] card 091: superseded because neither route passed card 090
- [x] card 092: superseded because no production mapping changed

## Boundaries

- no capability inheritance from stable ACP, Claude Agent, or Kimi
- no authenticated prompt, live provider mutation, ambient session lookup, or
  raw-id attachment
- no terminal-state inference from retained replay
- no recovery mapping without a bounded replay-completion boundary
- no change to ordinary open, close, callback, permission, or model behavior

## Acceptance Criteria

- [x] both exact Cursor source bundles expose silent replay-failure suppression
- [x] all four Grok artifacts retain insufficient client-visible replay proof
- [x] every unqualified negative load case remains explicit in frozen corpora
- [x] no driver, facade, binding, capability, or public API changed
- [x] focused validation passes without authenticated work

## Lane Runway

Complete negatively. Cursor and Grok remain unsupported for continuation
recovery. Cards 091-092 are superseded. Continue to g03.036 card 093; no
failure-triggered fallback is implied.
