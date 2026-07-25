# 010 Hosted Direct And Provider-State Facades

Status: completed
Owner: Tom
Created: 2026-07-25
Depends on: g02.008
Vision tags: direct inference, retained state, managed agents
Contract refs: 014, 016, 020-022, 024-025, 029-030, 037
Planning state: cards 027-030 complete

## Problem

Hosted routes need simple endpoint and credential composition without treating
one-shot streaming, consumer-continued tools, provider conversations,
background runs, and managed agents as one lifecycle.

## Goals

- [x] Add Kimi Platform and DeepSeek compatible-chat facades.
- [x] Add Alibaba Model Studio conversation facades.
- [x] Add OpenAI background Responses facades.
- [x] Add Anthropic Managed Agent facades.

## Execution Plan

- [x] Card 027: Kimi Platform and DeepSeek direct inference.
- [x] Card 028: Alibaba provider-owned conversation.
- [x] Card 029: OpenAI background run and bounded reattachment.
- [x] Card 030: Anthropic Managed Agent resource lifecycle.

## Acceptance Criteria

- [x] endpoint audience and credential provenance remain exact
- [x] model catalogue observations do not imply entitlement or routing
- [x] continuation, retention, reattachment, recovery, and deletion stay
      separately typed
- [x] every provider attempt remains explicitly authorized
- [x] no authentication, retry, or provider fallback is added

## Decision Gate

Every non-realtime hosted production route has prepared construction and typed
bound operations before specialized connection and runtime work begins.

Decision gate passed. Card 031 starts g02.011 with separate xAI, OpenAI, and
Gemini realtime connection facades.
