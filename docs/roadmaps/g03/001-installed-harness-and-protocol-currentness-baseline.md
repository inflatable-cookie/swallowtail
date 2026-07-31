# 001 Installed Harness And Protocol Currentness Baseline

Status: completed
Owner: Tom
Created: 2026-07-31
Depends on: completed g02
Vision tags: maintained compatibility, exact interfaces, consumer stability
Contract refs: 005, 011, 029, 036, 037, 044
Planning state: cards 001-003 completed

## Problem

Installed harnesses and shared protocols release independently and often.
Swallowtail has exact qualified ranges and visible unverified-newer posture,
but maintenance evidence is distributed across adapter corpora, matrices,
research, and release records.

A consumer release may encounter six months of installed harness versions.
Swallowtail needs a repeatable way to locate range gaps and behavior
milestones without requiring a library release for every compatible upstream
release or hard-denying versions above the last qualified point.

## Generation Runway Goal

Establish the g03 compatibility-maintenance baseline. Select later
implementation only from current authoritative evidence and material
consumer value.

## Goals

- [x] inventory every maintained installed-harness route and shared protocol
- [x] map exact baselines, milestone segments, exclusions, and upper posture
- [x] identify authoritative currentness sources and deterministic corpus gaps
- [x] revalidate the highest-risk surfaces against current evidence
- [x] rank material range extensions or compatibility repairs
- [x] compile the first implementation tranche only when evidence is ready

## Non-Goals

- automatic support claims from semver or latest release numbers
- hard rejection solely because a version is above the qualified upper point
- implementation before current evidence and corpus shape agree
- provider installation, update, login, logout, model calls, or billing effects
- direct-API or attached-runtime currentness unless the inventory exposes a
  shared protocol dependency or immediate consumer risk
- consumer repository edits, publication, or candidate replacement

## Execution Plan

### Batch 1.1 — Maintenance Inventory And Source Map

- [x] Execute card 001.
- [x] derive the exact route and protocol inventory from canonical repository
  surfaces
- [x] record qualification segments, evidence dates, probes, corpora, and
  consumer exposure
- [x] rank revalidation priority without selecting implementation

### Batch 1.2 — Current Evidence And Gap Classification

- [x] Execute card 002 after card 001 fixes the bounded source set.
- [x] revalidate selected surfaces against official provider or maintained-
  project evidence
- [x] classify compatible extension, behavior milestone, breaking drift,
  evidence-only refresh, or no action
- [x] confirm no shared contract delta is required before implementation
  planning

### Batch 1.3 — First Maintenance Tranche Selection

- [x] Execute card 003 after current evidence is complete.
- [x] select only materially useful, contract-ready work
- [x] compile implementation and conformance roadmaps with exact ranges
- [x] leave unrelated or externally gated surfaces in the inventory

## Acceptance Criteria

- [x] every inventoried route identifies integration family, driver,
  transport, interface version source, and support posture separately
- [x] baseline, intermediate milestones, latest qualified, exclusions, and
  unverified-newer behavior are explicit where applicable
- [x] evidence dates and authoritative source ownership are visible
- [x] no provider/model/access/endpoint fallback is introduced
- [x] deterministic fixture work precedes live authenticated effects
- [x] the first implementation tranche maximizes compatibility value rather
  than provider count
- [x] unresolved authority or product-policy choices return to the operator

## Decision Gates

- Stop if an upstream surface lacks maintained authoritative evidence.
- Promote research or a provisional spec if a compatibility decision changes
  shared contracts.
- Do not infer breaking change from a version bump alone.
- Do not infer compatibility from successful discovery alone.
- Do not run the full workspace suite during inventory or roadmap compilation.

## Next Planning Checkpoint

Roadmap g03.002 owns the selected Claude and Gemini tranche. Reassess Qwen,
Pi, and consumer-proven defects after that tranche, not after each upstream
release.
