# 042 Complete Integration Guide System

Status: completed
Owner: Tom
Created: 2026-08-05
Depends on: g03.041
Vision tags: consumer integration, operator usability, documentation
Contract refs: 004-006, 009-010, 020, 029, 032-052
Planning state: cards 118-123 complete

## Problem

Swallowtail has 18 useful route guides, 31 adapter examples, several deep
feature guides, and exact route and feature matrices. It does not yet guarantee
task-oriented instructions for every route and feature. Seven routes lack a
guide, six lack an example, and most feature columns rely on distributed route
notes.

## Goal

Give agents and operators one traceable, compiling, route-exact documentation
system for all 33 production routes and all 34 feature columns.

## Execution Plan

- [x] card 118: audit, documentation contract, and guide map
- [x] card 119: close seven missing route guides and six missing examples
- [x] card 120: deepen installed and attached harness route guides
- [x] card 121: deepen hosted, local-runtime, and realtime route guides
- [x] card 122: add cross-cutting consumer and operator feature runbooks
- [x] card 123: automate coverage, compile examples, reconcile indexes, and close

## Boundaries

- no guide may widen runtime or provider capability truth
- no generic router, default provider, fallback, retry, credential, or
  persistence policy
- no provider-native payload parsing in consumer instructions
- no credential values or authenticated work in deterministic acceptance
- composite facade guides keep branch-specific capabilities explicit
- examples use public prepared paths before low-level escape hatches

## Acceptance

- [x] all route rows in the guide map are complete
- [x] all feature families cover every matrix column and named portable surface
- [x] every applicable route has a compiling normal-path example
- [x] operator prerequisites and optional live probes remain separate from
  deterministic integration
- [x] a deterministic coverage check detects missing routes, features, guides, and
  examples
- [x] docs, examples, routes, and affected-package validation pass

## Lane Runway

Cards 118-123 are complete. All routes and features have traceable guidance,
and `effigy qa:guides` keeps route, feature, owner, index, and example coverage
aligned. g03 returns to its evidence gate.
