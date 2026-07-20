# Synthetic Cross-Shape Conformance

Status: completed
Owner: Tom
Roadmap: 005 Async Runtime and Conformance
Updated: 2026-07-19

## Goal

Prove Contracts 008-011 across five synthetic integration shapes without real
provider dependencies.

## Scope

- one-shot structured CLI profile
- long-lived RPC or ACP profile
- hosted direct API profile
- attached self-hosted profile
- owned self-hosted profile
- composable profile runners and deliberate violation fixtures

## Out Of Scope

- Codex, Claude, OpenCode, Cursor, Pi, Kimi, GLM, Qwen, DeepSeek, xAI, Ollama,
  llama.cpp, vLLM, SGLang, or Monkey implementations
- network, process, or credential integration tests

## Acceptance Criteria

- every Contract 011 common assertion maps to a deterministic test
- hosted API profile has no process-service requirement
- attached profile never records stop
- owned profile records authorized cleanup
- deliberate violations identify the exact contract dimension
- profiles use only public core/runtime APIs

## Validation

- focused profile tests
- `effigy qa`
- `git diff --check`

## Closeout

- Testkit exposes composable runners and inspectable reports for one-shot
  structured CLI, long-lived RPC, hosted direct API, attached self-hosted, and
  owned self-hosted profiles.
- Every runner proves the 14 common Contract 011 assertions through public
  core and runtime APIs.
- Shape-specific assertions prove process and session lifecycle, the hosted
  profile's lack of process authority, attached no-stop behavior, and owned
  cleanup behavior.
- Deliberate route, access, ownership, and topology violations fail on the
  exact preflight dimension before recorded effects.
