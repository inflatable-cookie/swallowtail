# 150 Provider-Reachable Expect Sweep

Status: completed
Owner: Tom
Created: 2026-08-08
Milestone: `../050-provider-reachable-panic-closure.md`
Depends on: card 149

## Goal

Sweep adapter `.expect()` sites that can see provider-observed input and
convert every provider-reachable one to a fail-closed result.

## Scope

1. Inventory the 1,816 `.expect()` sites across adapters; classify each as
   static-constant construction, lock-poisoning, or invariant/state.
2. Convert every invariant/state expect whose input can originate from
   provider wire data (parsing, version text, headers, envelope fields,
   notification phases) to `Result`/`Option` handling with a safe diagnostic.
3. Fix latent single-site risks found by the audit, including the Ollama
   activity profile binding expect
   (`adapter-ollama/src/activity/profile.rs:13`) and guarded-but-unclear
   expects such as `adapter-anthropic/src/driver/session/turn.rs:157`.
4. Record the remaining invariant expects as deliberate with a local comment
   stating the guard.

## Out Of Scope

- static-constant and lock-poisoning expects (no change)
- public API or diagnostic-code changes
- provider, transport, or consumer behavior changes

## Acceptance

- [x] no adapter `expect` is reachable from provider-observed input
- [x] invariant expects carry comments stating their guard
- [x] focused rounds pass for every adapter touched

## Stop Conditions

- stop if a conversion changes classified failure output for a qualified
  route

## Auto-Continuation

Yes, to card 151 after acceptance.

## Validation

- focused validation per touched adapter; `effigy check:examples`
- `effigy qa:routes` after any failure-mapping touch

## Completion Evidence

- inventoried the adapter `.expect()` population programmatically: roughly
  978 static-constant construction, 563 lock-poisoning, and 275
  invariant/state sites across production source
- converted every provider-reachable site found to fail-closed handling:
  - Ollama activity profile: the admitted-version behavior-revision Option
    now fails closed with `swallowtail.ollama.activity_profile_unavailable`
    instead of panicking (`adapter-ollama/src/activity/profile.rs`)
  - Anthropic and DeepSeek tool-result exchanges: `results.first().expect`
    converted to an explicit exchange-failure (`turn.rs`,
    `tool_attempt.rs`)
  - DeepSeek response parsing: an `exact_one` helper replaces the guarded
    `unwrap()`s on provider-controlled vectors (`protocol/response.rs`)
  - xAI completion: empty completed output fails closed with
    `swallowtail.xai.output_invalid` instead of panicking
    (`driver/turn/pump.rs`)
  - Muse command acceptance and Alibaba replay pagination: provider-ordering
    and page-content Options now fail closed (`events.rs`, `protocol/replay.rs`)
- verified and guard-commented the remaining provider-data-adjacent
  invariant expects (anthropic state-machine message and search identity,
  callback-map sync, deepseek final output via parser `finish()`, llama.cpp
  single-model catalogue, xAI just-assigned response identity, OpenAI
  cursor after identity consumption, and the `!delta.is_empty()`-guarded
  delta family across kimi-platform, anthropic, ollama, llama.cpp, and
  gemini transcripts)
- no qualified failure classification changed; the delta, cursor, identity,
  and output families keep their existing guarded outcomes
- focused rounds for the six touched adapters, workspace nextest (1,495
  passed across five consecutive runs), examples, route matrices, format,
  and warnings-denied clippy all pass
