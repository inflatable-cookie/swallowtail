# 002 Portable Contract Kernel

Status: completed
Owner: Tom
Updated: 2026-07-19

## Goal

Turn Contract 003 into a small provider-neutral Rust vocabulary and
deterministic fixtures without introducing runtime or provider behavior.

## Cards

- `batch-cards/003-contract-kernel-vocabulary.md` — completed
- `batch-cards/004-contract-kernel-conformance-fixtures.md` — completed
- `batch-cards/005-contract-kernel-validation.md` — completed

## Stop Condition

Do not add async traits, process supervision, transport, credential loading, or
Codex wire types in this milestone.

## Outcome

The pure core vocabulary and reusable testkit fixtures satisfy Contract 003.
The crate graph has no external dependencies and no runtime behavior.
