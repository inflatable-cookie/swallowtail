# 035 ACP Continuation Recovery Expansion

Status: ready
Owner: Tom
Created: 2026-08-05
Depends on: g03.034
Vision tags: restart continuity, ACP sessions, prepared recovery
Contract refs: 013, 015, 017, 029, 037, 050
Planning state: card 090 ready; cards 091-092 planned

## Problem

Cursor Agent ACP and Grok Build ACP advertise `loadSession`, but Swallowtail
only qualifies continuation recovery for Claude Agent ACP and Kimi ACP.
Protocol advertisement alone cannot establish replay ordering, exact resource
binding, readiness, or safe cleanup.

## Generation Runway Goal

Extend continuation recovery to further ACP routes only where exact
route-specific evidence satisfies the existing load/replay contract.

## Goals

- [ ] qualify Cursor and Grok load/replay independently
- [ ] implement recovery only for candidates that pass every exact gate
- [ ] preserve route, version, resource, access, model, and attachment binding
- [ ] close deterministic, public, and extracted-package acceptance

## Execution Plan

- [ ] card 090: freeze exact Cursor and Grok load/replay evidence and select
      each route independently
- [ ] card 091: add load drivers and prepared recovery mappings only for
      selected routes
- [ ] card 092: close cross-host conformance, public truth, and package proof

## Boundaries

- no capability inheritance from stable ACP, Claude Agent, or Kimi
- no authenticated prompt, live provider mutation, ambient session lookup, or
  raw-id attachment
- no terminal-state inference from retained replay
- no recovery mapping without a bounded replay-completion boundary
- no change to ordinary open, close, callback, permission, or model behavior

## Acceptance Criteria

- [ ] each promoted route proves exact load request, ordered bounded replay,
      readiness, foreign-update rejection, and joined cleanup
- [ ] a saved binding cannot attach another host, resource, version, model,
      access profile, or configured instance
- [ ] recovery returns one live loaded session and no interrupted-turn state
      claim
- [ ] a failed candidate remains visibly unsupported without blocking an
      independently qualified candidate
- [ ] focused and affected-package validation pass without authenticated work

## Lane Runway

Card 090 is ready. Cards 091-092 remain conditional on its independent route
decisions. Roadmap g03.036 follows with explicit reconciliation-then-attachment
composition; no failure-triggered fallback is implied.
