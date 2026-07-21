# 048 llama.cpp Deployment Fixtures

Status: completed
Owner: Tom
Updated: 2026-07-20
Milestone: `../015-llama-cpp-attached-runtime-proof.md`

## Objective

Freeze one attached llama.cpp deployment and protocol-facade fixture without
moving model management into Swallowtail.

## Scope

- installed server version, exact facade, health/catalogue behavior, one small
  operator-supplied model fixture, parser/template capability evidence

## Out Of Scope

- model download, conversion, quantization, storage, or Monkey behavior
- owned server lifecycle

## Acceptance Criteria

- [x] artifact, server, deployment, facade, and route identities stay separate
- [x] fixture requirements are explicit and optional for default QA
- [x] card 049 needs no fresh serving decision

## Validation

- deterministic protocol fixtures
- `git diff --check`

## Evidence

- official release, server, `/props`, server-test, and artifact-repository
  evidence rechecked on 2026-07-20
- installed `llama-server` build `9910` maps exactly to release `b9910` and
  commit `f5525f7e7a7e7cbecd386144299493ea40499bd3`; current upstream `b10069`
  is recorded but not substituted into the fixture
- one 1,185,376-byte operator-supplied Stories 260K GGUF is pinned by
  repository revision, file, and SHA-256 without bundling or downloading it
- the attached deployment fixes a host-approved loopback endpoint, explicit
  alias, ChatML/Jinja, reasoning disabled, and no Web UI
- the bounded facade contains `/health`, `/props`, `/v1/models`, and streaming
  `/v1/chat/completions`; compatibility does not import OpenAI semantics
- `/props` gates effective template and modality evidence; the first route
  claims text, streaming, usage, string content, and system role only
- eight deterministic parser and fixture tests pass without a server, model,
  credential, or network request

## Stop Conditions

- no bounded redistributable or operator-supplied fixture is available
- proof requires Swallowtail model management

## Auto-Continuation

Yes, only after card 049 is marked ready.
