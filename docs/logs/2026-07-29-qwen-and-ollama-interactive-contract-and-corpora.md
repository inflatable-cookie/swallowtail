# Qwen And Ollama Interactive Contract And Corpora

Date: 2026-07-29
Card: g02 116

## Changed

- promoted Contract 043 for turn-scoped interactive continuity
- separated Qwen harness-retained restarted continuation from Ollama
  consumer-owned transcript replay
- froze exact Qwen `0.19.11` source hashes and three interactive streams
- froze Ollama interactive requests and streams across the existing four
  qualification points and `0.32.2` exclusion
- fixed failed-turn commit, cancellation, deadline, terminal, and cleanup
  truth for both profiles
- moved the sole next-task pointer to card 117

## Key Boundary

Qwen may use exact `--resume` privately between turns inside one runtime
session. It does not implement Swallowtail's public load or resume role.
Failure or uncertainty invalidates the handle because provider transcript
mutation cannot be rolled back safely.

Ollama has no provider session. Its adapter-private transcript commits only a
complete successful user/assistant pair. Failed, malformed, disconnected,
cancelled, or timed-out turns leave history unchanged. Close clears private
history without stopping the attached runtime or unloading its model.

Neither route gains archive, restore, delete, native close, server ownership,
sandbox, media, retry, fallback, or provider-authoritative billed cost.

## Validation

- Qwen interactive corpus: 4 passed
- Ollama interactive corpus: 4 passed
- JSON and NDJSON syntax passed
- Effigy docs and route QA passed
- diff check passed

## Next

Card 117 implements the two prepared interactive profiles and exact lifecycle
scenarios. Card 118 then closes the residual matrix programme.
