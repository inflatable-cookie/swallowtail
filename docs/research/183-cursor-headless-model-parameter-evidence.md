# 183 Cursor Headless Model Parameter Evidence

Status: promoted
Owner: Tom
Date: 2026-08-22
Card: g04.035 / 095

## Question

Which exact Cursor headless `--model` bracket tuples can Swallowtail prepare
without a provider prompt, authenticated catalogue, or account inspection?

## Method

Re-probed installed `cursor-agent --version` and `--help`, downloaded official
darwin-arm64 archives for `2026.07.01-41b2de7`, `2026.07.23-e383d2b`, and
`2026.08.11-e8db854` into `/tmp`, reused the installed
`2026.08.04-aaa8809` host for its exact specimen, and froze current official
subagent and TypeScript SDK parameter documentation as sibling evidence.

No provider prompt, authentication, live catalogue, install, update, or
account inspection ran.

## Syntax segment

All four qualified builds publish the same `--model` help excerpt:

```text
Parameterized models accept quoted bracket overrides, e.g.
'claude-opus-4-8[context=1m,effort=high,fast=false]'
```

The selected CLI catalogue (`cursor-agent models`) returns plain model ids
without parameter descriptors. SDK `Cursor.models.list()` parameter
descriptors are a sibling surface and do not justify CLI tuple support.

## Artifact identity

| Version | Source | Archive SHA-256 | Executable SHA-256 |
| --- | --- | --- | --- |
| `2026.07.01-41b2de7` | official darwin-arm64 archive | `48cbf291c2e28d81b79fa0dcbf18ab50bf4ac7772d0e9ab0948ecbd5f5a29158` | `eed61c5224668c9236334c4c68936a16aecc37374b592f59e31eb50433817831` |
| `2026.07.23-e383d2b` | official darwin-arm64 archive | `f2eb25851f2079dcdf0558a816e06c402d187abfca93255d35167020439ebbf2` | `eed61c5224668c9236334c4c68936a16aecc37374b592f59e31eb50433817831` |
| `2026.08.04-aaa8809` | installed host | — | `eed61c5224668c9236334c4c68936a16aecc37374b592f59e31eb50433817831` |
| `2026.08.11-e8db854` | official darwin-arm64 archive | `46044d6d7bcbd7b49a0cf1cd01aa4ca79aaa2ea5f2c7a32965fc0ebe29841790` | `eed61c5224668c9236334c4c68936a16aecc37374b592f59e31eb50433817831` |

Frozen corpus:
`crates/swallowtail-adapter-cursor/tests/fixtures/cursor-agent-headless-model-parameters-2026.07.01-2026.08.11/`.

## Canonical rendering

Non-empty typed parameters render in one bracket suffix in this order:
`context`, `effort`, `fast`. Example:
`claude-opus-4-8[context=1m,effort=high,fast=false]`.

Official subagent combined examples may list `effort` before `context`; the
selected CLI help example uses `context`, then `effort`, then `fast`. This
milestone follows the CLI help order for deterministic dispatch.

## Tuple disposition

| Base model | Parameter | Value | Source | Disposition |
| --- | --- | --- | --- | --- |
| `claude-opus-4-8` | `context` | `1m` | CLI help example | deliver-now |
| `claude-opus-4-8` | `effort` | `high` | CLI help example | deliver-now |
| `claude-opus-4-8` | `fast` | `false` | CLI help example | deliver-now |
| `claude-opus-5` | `context` | `300k` | subagents model parameters | deliver-now |
| `claude-opus-5` | `effort` | `high` | subagents model parameters | deliver-now |
| `composer-2.5` | `fast` | `false` | subagents model parameters | deliver-now |
| any | `fast` | `true` | triage inventory only | evidence-gated |
| any | `effort` | `low` / `medium` | not named for a model | evidence-gated |
| `claude-opus-4-8` | `context` | `300k` | not named together | evidence-gated |
| `claude-opus-5` | `context` | `1m` | not named together | evidence-gated |
| `composer-2.5` | `context` / `effort` | any | not named | evidence-gated |
| any other base id | any | any | catalogue id alone | evidence-gated |
| any | empty `[]` | — | subagents empty-bracket pin | evidence-gated |

Qualified effort binds portable `ReasoningMode::high` only for the
`claude-opus-4-8` and `claude-opus-5` rows above. Fast and context remain
Cursor-local selected-model parameters.

## Claim boundary

Swallowtail may claim qualified dispatch for deliver-now tuples only.
Dispatch does not prove provider acceptance or effective application. Model
or account rejection of a qualified tuple is provider truth.

## Primary sources

- exact qualified CLI `--help` on all four calendar builds
- [Cursor subagents model parameters](https://cursor.com/docs/subagents.md#model-parameters)
- [Cursor TypeScript SDK model parameters](https://cursor.com/docs/sdk/typescript.md#model-parameters) (sibling surface)
