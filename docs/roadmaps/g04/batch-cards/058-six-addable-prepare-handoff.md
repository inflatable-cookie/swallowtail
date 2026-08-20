# 058 Six Addable Prepare Handoff

Status: completed
Owner: Tom
Created: 2026-08-20
Milestone: `../020-config-ref-prepare-handoff.md`
Depends on: card 057

## Goal

Prove the six addable prepare entries consume the config-ref handoff.

## Scope

1. Anthropic Messages, DeepSeek continuation, Codex app-server, Claude
   Agent ACP, Ollama attach, and llama.cpp attached.
2. Update connection-lifecycle examples and guides.
3. Stored refs feed prepare. Host still resolves values.

## Out Of Scope

- new addable routes
- overlay keying
- 047 presentation metadata
- hosted OAuth
- rewriting `public-api-0.3.3`

## Acceptance Criteria

- [x] each of the six prepares accepts the admitted identity and stored
      refs
- [x] guides no longer say stored refs do not feed prepare
- [x] public records still carry no paths, URLs, or env bodies

## Evidence

- Anthropic, DeepSeek, Codex app-server, Claude Agent ACP, Ollama attach, and
  llama.cpp attached lifecycle proofs call their route-local `from_admitted`
  constructors.
- Connection-lifecycle examples and six route guides use the same handoff.
- `cargo test -p swallowtail-adapter-anthropic -p swallowtail-adapter-deepseek
  -p swallowtail-adapter-codex -p swallowtail-adapter-claude-agent
  -p swallowtail-adapter-ollama -p swallowtail-adapter-llama-cpp
  --test connection_lifecycle`
- `effigy check:examples`
- Both required focused validation batches and `git diff --check`
- `effigy package:api`

## Validation

- `effigy validate:focused swallowtail-adapter-anthropic swallowtail-adapter-deepseek swallowtail-adapter-codex swallowtail-adapter-claude-agent`
- then `effigy validate:focused swallowtail-adapter-ollama swallowtail-adapter-llama-cpp swallowtail-runtime swallowtail-testkit`
- `git diff --check`
- `effigy package:api` if public types are added
- `effigy check:examples`

## Auto-Continuation

No. Compile g04.021 after this milestone.

## Stop Conditions

- Stop if a route still requires a parallel host target copy as the only
  path.
- Stop if values leak.
