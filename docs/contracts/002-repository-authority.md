# 002 Repository Authority

Status: active
Owner: Tom
Updated: 2026-07-19

## Purpose

Keep Swallowtail standalone while it extracts mechanisms proven in consumer
applications.

## Contract

- This repository is canonical for Swallowtail vision, architecture,
  contracts, API plans, release history, and roadmaps.
- Consumer documents may describe adoption but cannot redefine a Swallowtail
  contract.
- Consumer code is not copied wholesale. Each mechanism is separated from
  product policy before promotion.
- Swallowtail production crates cannot depend on Nucleus, Soundcheck, Monkey,
  their filesystem layouts, or their state stores.
- Consumer-specific prompts, tools, task models, schemas, scheduling,
  credentials, and persistence remain downstream.
- Cross-repository decisions are recorded here and referenced by consumers.
- Source evidence must identify provenance without making a foreign repository
  a permanent build-time authority.

## Acceptance

- the crate graph can build without consumer repositories present
- public types use Swallowtail vocabulary
- consumer conformance tests can prove adoption without reverse dependencies
- repository docs contain the sole active Swallowtail roadmap pointer
