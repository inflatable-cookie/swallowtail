# 061 Unmarked Addable Overlay Proof

Status: ready
Owner: Tom
Created: 2026-08-20
Milestone: `../021-unmarked-overlay-rows.md`
Depends on: card 060

## Goal

Prove the chosen overlay rule on the unmarked addable catalogues.

## Scope

1. Codex app-server, Claude Agent ACP, Ollama attach, llama.cpp attached.
2. Anthropic and DeepSeek overlay keying stays unchanged.
3. Update connection-lifecycle guides.

## Out Of Scope

- new addable routes
- 047 presentation metadata
- hosted OAuth

## Acceptance Criteria

- [ ] unmarked addable catalogues follow the chosen rule
- [ ] Anthropic and DeepSeek still key `provider_id` rows
- [ ] 047 `Ready` / `NotReady` is unchanged

## Validation

- `effigy validate:focused swallowtail-adapter-codex swallowtail-adapter-claude-agent swallowtail-adapter-ollama swallowtail-adapter-llama-cpp`
- `git diff --check`

## Auto-Continuation

No. Compile g04.022 after this milestone.

## Stop Conditions

- Stop if a test invents a catalogue provider id to make overlay work.
