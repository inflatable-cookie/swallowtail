# 011 Host-Approved Interpreted Executable Launch

Status: completed
Owner: Tom
Created: 2026-07-31
Depends on: g03.010
Vision tags: installed harnesses, execution hosts, compatibility maintenance
Contract refs: 010-011, 029, 032, 037
Planning state: cards 027-029 completed

## Problem

The installed Pi `0.83.0` command is an npm script whose
`#!/usr/bin/env node` launcher works under ambient `PATH` but fails safely
through `swallowtail-host-local`, which clears child environment. Native-only
executable approval cannot represent this common installed-harness shape.

## Goal

Realize Contract 010's host-private launch recipe, prove it independently of a
provider, then accept exact installed Pi discovery through the same opaque
Contract 032 target without ambient environment inheritance or adapter-side
fallback.

## Execution Plan

### Batch 11.1 — Provider-Neutral Launch Recipe

- [x] Execute card 027.
- [x] represent one exact program, immutable prefix arguments, and bounded
  bootstrap environment behind an opaque executable reference
- [x] keep native executable approval as the zero-prefix case
- [x] prove ordering, bounds, environment precedence, and redacted formatting
  with deterministic local-host fixtures

### Batch 11.2 — Installed Pi Host Proof

- [x] Execute card 028.
- [x] bind the exact installed Pi script to one host-selected Node interpreter
- [x] run Pi's fixed version command through `LocalProcessHost`
- [x] retain exact `0.83.0` harness classification and joined cleanup without a
  provider prompt

### Batch 11.3 — Package And Authority Acceptance

- [x] Execute card 029.
- [x] verify the affected host/runtime and Pi package surfaces
- [x] reconcile architecture, front doors, roadmap state, and closeout evidence
- [x] return to the g03 compatibility-maintenance checkpoint

## Boundaries

- no ambient `PATH` or environment inheritance
- no shell launch, adapter-side executable search, or executable fallback
- no Node, npm, Pi, Python, Ruby, or JVM rule in a provider adapter
- no credential, provider configuration, model, prompt, or working-resource
  authority in a launcher recipe
- no provider prompt, authentication mutation, installation, or update
- no consumer, candidate, registry-publication, Claude, or Gemini work

## Acceptance Criteria

- [x] one opaque executable reference can select native or interpreted launch
- [x] prefix arguments are immutable, ordered before driver arguments, and
  counted against host argument limits
- [x] bootstrap environment is bounded, redacted, and applied before explicit
  request environment
- [x] stable records and default formatting expose no launcher material
- [x] deterministic native and interpreted fixtures pass under `env_clear()`
- [x] installed exact Pi `0.83.0` classifies through the local process host
- [x] focused and extracted-package validation pass
- [x] architecture, front doors, roadmap state, and one closeout log are current

## Decision Gates

- Stop if the recipe must leak path or environment material into portable
  runtime records.
- Stop if deterministic proof requires a shell, inherited environment, or a
  provider-specific rule.
- Stop if installed Pi proof requires authentication, a provider prompt, or
  workspace access.

## Next Planning Checkpoint

Run the g03 compatibility-maintenance checkpoint before compiling g03.012.
Standalone Claude ACP and further Gemini range qualification remain paused.
