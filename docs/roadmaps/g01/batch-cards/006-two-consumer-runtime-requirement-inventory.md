# Two-Consumer Runtime Requirement Inventory

Status: completed
Owner: Tom
Updated: 2026-07-19

## Goal

Build an evidence-backed requirement matrix for Nucleus interactive sessions
and Soundcheck structured runs without promoting either consumer's product
policy into Swallowtail.

## Governing References

- Contract 002: Repository Authority
- Contract 003: Portable Contract Kernel
- `docs/specs/001-foundation-and-extraction.md`
- `docs/roadmaps/g01/003-integration-landscape-and-runtime-boundary.md`

## Scope

- inspect Nucleus `live_runtime`, `codex_runtime`, `live_registry`, and
  `local_codex_chat/runtime` boundaries
- inspect Soundcheck `app_settings` and `assistant_tagging` boundaries
- separate shared mechanisms from consumer-owned intent, authority, and state
- compare session, run, streaming, cancellation, timeout, tool, schema,
  progress, error, and resume requirements
- record topology requirements for local and remote execution hosts
- promote durable findings into Swallowtail architecture or contracts

## Out Of Scope

- moving or editing consumer code
- choosing a final Rust trait shape
- implementing `swallowtail-runtime`
- provider protocol or process extraction
- changing consumer dependencies

## Acceptance Criteria

- each requirement cites concrete consumer evidence
- common requirements and shape-specific requirements are distinct
- Nucleus task, goal, memory, and tool policy remains downstream
- Soundcheck taxonomy, tagging, schema content, and repair policy remains
  downstream
- gaps and conflicts become explicit inputs for card 007
- research findings are promoted rather than left as an unowned memo

## Validation

- source-path and symbol evidence checks
- Swallowtail docs QA and all Markdown links
- `git diff --check` in every touched repository

## Stop Condition

Stop if a consumer requirement cannot be distinguished from product policy.
Record the ambiguity for operator or card 007 review; do not guess ownership.

## Outcome

The evidence is recorded in
`docs/research/001-two-consumer-runtime-requirements.md`, promoted into
`docs/architecture/consumer-runtime-evidence.md`, and made durable in Contract
004. Shared mechanisms and downstream consumer policy are distinct. Eight
implementation-shape decisions remain explicit for card 007.
