# Contract Kernel Vocabulary

Status: completed
Owner: Tom
Updated: 2026-07-19

## Goal

Define and implement the smallest pure Rust record vocabulary required by
Contract 003.

## Scope

- create a minimal Cargo workspace
- create focused `swallowtail-core` and `swallowtail-testkit` crates
- define adapter identity and capability records
- define model catalog identity and metadata records
- define opaque provider session and run references
- define common event envelopes and namespaced extension records
- define structured safe diagnostic and error records

## Out Of Scope

- execution traits or async runtime choice
- real provider calls, processes, transports, or credentials
- Codex-specific wire records
- consumer dependencies or migration
- product prompts, tools, tasks, schemas, or persistence

## Acceptance Criteria

- public types contain no Nucleus or Soundcheck concepts
- crate dependency direction matches the planned architecture
- capability checks fail explicitly
- references expose no parseable provider semantics
- safe diagnostics separate public and internal detail
- the design leaves unsettled runtime decisions open

## Validation

- `effigy test --plan`
- `effigy check:rust`
- `effigy lint:rust`
- `effigy test` — seven core tests pass
- `cargo doc --workspace --no-deps`
- crate-tree and consumer/provider coupling audit
- `git diff --check`

## Outcome

The two-crate workspace and pure Contract 003 vocabulary are in place. No
external, runtime, provider, or consumer dependencies were introduced.
