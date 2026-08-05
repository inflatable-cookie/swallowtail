# 2026-08-05 Provider-Wide Session Usability Restoration

Roadmap: `../roadmaps/g03/039-provider-wide-session-usability-restoration.md`
Cards: 107-110

## Changed

- extended Contract 050 with distinct fresh realtime replacement
- mapped Anthropic Messages, DeepSeek continuation, Ollama attached, xAI
  Responses WebSocket, and Alibaba delete-on-close sessions to fresh
  interactive replacement
- mapped OpenAI Realtime and Gemini Live to fresh realtime replacement
- preserved exact interrupted consumer turn identity in every new outcome
- kept retained Alibaba on stronger bounded continuation recovery
- reconciled restoration, provider integration, route, and feature-matrix truth

## Current State

Every prepared reusable session shape now exposes one static strongest
post-crash action. Replacement restores route usability only. It does not
recover the lost operation, settle the interrupted turn, replay prompts or
side effects, or substitute route, model, access, resource, or policy.

Anthropic and DeepSeek direct continuation require provider-private assistant
and reasoning material for exact later requests. Contract 030 keeps that
material bounded, non-serializable, and adapter-private, then zeroizes it when
the session closes. A crash therefore leaves no durable provider session and
no lawful hidden-state checkpoint to reload. Visible transcript reconstruction
would be a new consumer-authored request, not recovery of the original state.

Realtime replacement similarly returns a new connection with no audio,
transcript, response, buffer, rollover, cancellation, or terminal continuity.
Gemini's planned rollover remains an in-session transport mechanism.

No authenticated provider work, external request, prompt, paid inference, or
live media session ran.

## Validation

- focused validation passed in exact four-package groups: 255 and 171 tests
- affected-package verification passed for runtime and all seven adapters
- documentation validation passed
- route validation passed: 32 production routes, 25 solution rows
- structural scan stayed at its 224-finding baseline: 203 warnings, 21 errors
- `git diff --check` passed

## Next Move

Hold at the g03 evidence gate until new consumer evidence, material provider or
interface drift, or explicit operator promotion supplies the next roadmap
input.
