# 179 OpenCode And Alibaba Provider History

Status: done
Owner: Tom
Created: 2026-08-08
Milestone: `../058-multi-route-provider-session-history.md`
Depends on: g03.057 cards 176-178

## Goal

Implement Contract 054 history paging on every route that can page without a
live control handle: OpenCode HTTP and Alibaba retained conversations.

## Scope

1. OpenCode: descriptor role, `ProviderSessionHistoryDriver`, prepared
   `prepare_session_history`, fixtures for newest-first paging and overflow.
2. Alibaba retained: resource-free history driver, shared ascending replay
   walk with load, prepared facade, fixtures including empty history.
3. Runtime validation: allow resource-free DirectModelInference history
   posture matching the binding.

## Out Of Scope

- Claude Agent ACP / Kimi ACP load-as-history wrapping
- native provider pagination qualification
- live provider probes
- release tagging

## Acceptance

- [x] OpenCode and Alibaba focused package validation passes
- [x] pages grant no control / handle / delete side effects
- [x] overflow fails closed

## Stop Conditions

- stop if a candidate route's only history wire attaches a live control handle

## Auto-Continuation

Card 180.

## Closeout

Log: `docs/logs/2026-08-08-multi-route-provider-history.md`
