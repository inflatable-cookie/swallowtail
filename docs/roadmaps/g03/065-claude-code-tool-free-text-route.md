# 065 Claude Code Tool-Free Text Route

Status: completed
Owner: Tom
Created: 2026-08-11
Depends on: Research 122; Contract 039 amendment
Vision tags: installed harness, response only, consumer hardening
Contract refs: 005-006, 009-010, 023, 029, 032-033, 037, 039, 044, 051-052

## Problem

Figmatic needs one local-subscription Claude response without model-visible
tools or filesystem authority. The schema-enforced candidate failed in
g03.064. Exact `2.1.227` evidence now qualifies a narrower plain-text boundary
whose output stays untrusted consumer input.

## Generation Runway

Advances g03's consumer-proven defect and integration-friction goal. The lane
adds one explicit installed-harness route without changing the current
read-only Claude Code profile or importing Figmatic policy.

## Execution Plan

- [x] card 202: qualify exact tool-free text behavior, promote Contract 039,
      and select a distinct route identity
- [x] card 203: freeze the `2.1.227` corpus and implement discovery,
      compatibility, command, parser, lifecycle, cancellation, and cleanup
- [x] card 204: add the provider-specific prepared API, deterministic facade
      proof, safe live probe, and minimal consumer example
- [x] card 205: update route/feature guidance and architecture, run focused,
      affected-package, docs, and live acceptance, then close the lane

## Goals

- [x] one caller prompt produces one bounded assistant text result
- [x] exact empty tool and MCP surfaces fail closed on drift
- [x] local OAuth/Max access remains provider-supported and API-key-free
- [x] no working resource, callback, schema, session, continuation, retry,
      fallback, or write capability enters the route
- [x] existing `claude-code.headless` behavior and exact `2.1.220` claim remain
      unchanged

## Boundaries

- plain text only; JSON-looking bytes carry no validation claim
- distinct `claude-code.response-only` route at exact `2.1.227`
- provider-suppressed configuration and honest ambient-host isolation
- no Figmatic edit or product abstraction
- no version bump, tag, GitHub Release, or registry mutation

## Acceptance Criteria

- [x] deterministic fixtures reject tools, MCP, user/tool results, second
      assistant messages, multi-turn result, missing text, malformed input,
      bounds, and version drift
- [x] prepared evidence carries exact route, model, access, configuration,
      isolation, retention, host-service, and absent-capability truth
- [x] cancellation, deadline, process failure, event failure, and joined cleanup
      remain distinct and redacted
- [x] safe authenticated live probe passes against exact `2.1.227`
- [x] focused, affected-package, guide, route, and public API gates pass; broad
      docs remains blocked only by the recorded Effigy roadmap-index defect
- [x] Figmatic receives an exact commit and compiling API example

## Planning Checkpoint

Cards 202-205 are complete. Card 206 holds the downstream Figmatic adoption
step. Release work remains operator-gated.
