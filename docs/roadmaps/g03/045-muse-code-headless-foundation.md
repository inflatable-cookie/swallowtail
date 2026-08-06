# 045 Muse Code Headless Foundation

Status: completed
Owner: Tom
Created: 2026-08-06
Depends on: g03.044
Vision tags: installed harness, Meta, Muse Spark, structured execution
Contract refs: 005-006, 009-010, 023, 029, 032-033, 036-037, 039-041, 044-045, 051-052

## Problem

Meta Muse Code is a first-party terminal harness for Muse Spark with an
explicit headless JSONL event protocol. Swallowtail has no route for it. The
installed launcher also auto-updates, so an ordinary executable-name binding
would weaken exact version qualification between preparation and dispatch.

## Generation Runway

Advance g03's high-value installed-harness goal without turning the generation
into a catalogue of every new CLI. Muse qualifies because it is the
provider-owned Spark harness, exposes deterministic and authenticated machine
evidence, and exercises existing activity, task, access, reasoning, and
lifecycle contracts.

## Execution Plan

- [x] card 135: freeze the exact signed artifact, command, JSONL, failure, and
      auto-update boundary
- [x] card 136: implement discovery, compatibility, bounded event decoding,
      activity projection, cancellation, deadline, and cleanup
- [x] card 137: expose local-account preparation and an exact read-only
      structured-run facade with model and reasoning control
- [x] card 138: complete package, guide, example, matrices, live acceptance,
      release-baseline handling, and closeout evidence

## Goals

- [x] add one separately selectable `swallowtail-adapter-muse` package
- [x] qualify only exact Muse Code release `0.1.0-R708.1`
- [x] bind the versioned payload without launcher-driven update drift
- [x] execute `meta` / `muse-spark-1.2` through local Meta account state
- [x] preserve JSONL stream ownership, correlation, terminal, and task
      lifecycle evidence
- [x] expose every supported reasoning effort without inventing a default
- [x] keep retained sessions and Meta Model API outside the first route

## Boundaries

- no screen scraping, TUI automation, launcher installation, login, logout, or
  credential extraction
- no generic Meta router and no reuse of OpenAI adapter identity
- no unverified-newer execution on the opaque release-build axis
- no workspace writes, shell, web tools, prompt replay, provider fallback, or
  hidden retry authority
- no claim of interactive continuation, recovery, task-list snapshots,
  approvals, questions, subagent topology/control, or usage without exact
  event evidence
- no movement or alteration of `v0.1.0` or `v0.1.1`; Muse remains unreleased
  until a later operator-authorized source candidate

## Acceptance Criteria

- [x] the route rejects the launcher or version drift before provider work
- [x] deterministic echo fixtures cover success, malformed input, unknown
      namespaces, bounds, correlation, terminal, cancellation, and cleanup
- [x] the prepared facade binds exact provider, model, effort, resource,
      access, retention, isolation, and host services
- [x] focused and extracted-package validation pass without credentials
- [x] one separately gated authenticated low-effort probe passes
- [x] the route matrix, feature matrix, guide map, example, architecture,
      package contract, and release tooling remain mutually honest

## Planning Checkpoint

After card 138, reassess the documented session export and `exec --session-id`
surfaces. Promote retained continuation or recovery only if headless replay,
ownership, cleanup, and private-record handling are exact. Assess the direct
Meta Model API separately.
