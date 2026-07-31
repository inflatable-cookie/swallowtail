# 005 Cursor Installed Dual-Route Foundation

Status: active
Owner: Tom
Created: 2026-07-31
Depends on: g03.001
Vision tags: harness breadth, shared protocols, structured execution, prepared integration
Contract refs: 005-006, 011, 015, 020, 023, 029, 032-033, 037, 039, 041, 043-045
Planning state: cards 010-011 completed; card 012 ready after active card 019;
cards 013-014 planned

## Problem

Cursor is a high-priority installed harness with no Swallowtail adapter. Its
first-party CLI exposes an authenticated model catalogue, an ACP server for
interactive sessions, and a provider-specific stream-JSON headless route.
Treating those as one transport would flatten materially different lifecycle,
capability, and activity behavior.

The qualification host also proves an executable-name collision: `agent`
identifies as Grok, while `cursor-agent` identifies as Cursor. Discovery must
be explicit and identity-safe without hardcoding rejection of host-approved
paths.

## Generation Runway Goal

Add a materially different installed harness through the existing portable
contracts while proving that one provider facade can keep catalogue, ACP, and
headless roles explicit and simple for consumers.

## Goals

- [x] freeze exact installed and registry artifacts before support claims
- [x] add one Cursor integration family and focused adapter crate
- [ ] expose separate catalogue, ACP, and headless driver identities
- [ ] provide one `prepare_cursor` facade with explicit route selection
- [x] preserve delegated local authentication without credential extraction
- [ ] project only exact observed capabilities, activity, usage, and lifecycle
- [ ] keep optional provider sandboxing separate from ambient-host baseline
- [ ] accept through focused conformance and extracted-package evidence

## Non-Goals

- Cursor Agent SDK or cloud-agent API integration
- provider account creation, billing, or credential extraction
- generic `agent` automatic fallback
- inferring continuous compatibility from calendar-labelled builds
- ACP load, resume, deletion, consumer MCP, or model-selection claims without
  exact corpus evidence
- synthesized thinking or reasoning activity
- implicit sandboxing or dangerous permission bypass
- consumer repository edits or publication

## Execution Plan

### Batch 5.1 — Exact Artifacts And Behavioral Corpus

- [x] Execute card 010.
- [x] freeze the installed and ACP-registry executable identities
- [x] capture sanitized version, catalogue, ACP, and headless source records
- [x] classify exact behavior groups and unsupported surfaces

### Batch 5.2 — Discovery, Catalogue, And Package Foundation

- [x] Execute card 011 after card 010 passes.
- [x] add the adapter package, executable discovery, version classification,
  access descriptors, and auth-aware catalogue driver
- [x] prove command collision handling and safe diagnostics

### Batch 5.3 — Separate Execution Drivers

- [ ] Execute card 012 for ACP interactive sessions.
- [ ] Execute card 013 for provider-specific headless structured runs.
- [ ] keep lifecycle, streaming, authority, and activity claims route-local

### Batch 5.4 — Prepared Facade And Acceptance

- [ ] Execute card 014.
- [ ] expose explicit prepared operations through one Cursor facade
- [ ] reconcile public route and feature truth
- [ ] validate focused crates and extracted packages without a live prompt

## Acceptance Criteria

- [ ] `cursor-agent` discovery cannot silently select Grok's `agent` command
- [ ] exact executable points have explicit qualified or unverified posture
- [ ] model catalogue does not claim model invocability
- [ ] ACP and headless sessions retain distinct transports and operation shapes
- [ ] headless output projects available tool, assistant, usage, and result
  evidence without invented thinking
- [ ] local delegated authentication remains provider-owned
- [ ] ambient host and optional provider sandbox profiles remain distinct
- [ ] `prepare_cursor` is simple without hiding route selection
- [ ] deterministic, cross-host, focused, and package evidence passes
- [ ] no provider prompt, account mutation, consumer edit, or publication runs

## Decision Gates

- Stop if exact Cursor artifacts cannot be preserved without account data.
- Stop if ACP negotiation contradicts the maintained registry or needs a new
  portable capability.
- Stop if headless structured output cannot be correlated without raw-payload
  leakage.
- Do not infer one build hash's behavior from another same-day build.

## Next Planning Checkpoint

After card 014, begin Antigravity exact-artifact qualification. Reassess Qwen
separately if the operator's account setup is ready; do not couple it to Cursor
acceptance.
