# 011 Specialized Runtime Facades

Status: completed
Owner: Tom
Created: 2026-07-25
Depends on: g02.008
Vision tags: realtime media, embedded SDK, self-hosted runtime
Contract refs: 018-020, 026-027, 029, 031, 037
Planning state: cards 031-033 complete

## Problem

Realtime connections, embedded SDK clients, and owned local serving cannot be
modeled as ordinary HTTP request preparation.

## Goals

- [x] Add xAI, OpenAI, and Gemini realtime facades.
- [x] Add Bedrock Runtime and catalogue facades.
- [x] Add llama.cpp attached and owned-serving facades.
- [x] Preserve connection, SDK executor, artifact, and serving ownership.

## Execution Plan

- [x] Card 031: all three realtime connection routes.
- [x] Card 032: both Bedrock SDK routes.
- [x] Card 033: both llama.cpp runtime routes.

## Acceptance Criteria

- [x] media formats, rollover, cancellation, and duplex cleanup remain explicit
- [x] region, SDK identity, client configuration, and delegated credentials
      remain exact
- [x] attached and owned serving remain different constructors and handles
- [x] artifact leases, readiness, residency, and teardown do not flatten
- [x] low-level specialized roles remain independently callable

## Decision Gate

All 22 production routes now have prepared normal paths. g02.012 owns the
cross-route acceptance and candidate return.
