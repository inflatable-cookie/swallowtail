# Qwen And Ollama Interactive Contract Fit And Corpora

Status: promoted
Owner: Tom
Date: 2026-07-29

## Question

What exact shared contract and offline evidence are required to add
interactive sessions to Qwen headless and Ollama attached without flattening
their different continuity owners?

## Method

Evidence was accessed on 2026-07-29.

- checked Contracts 004, 006, 009-012, 017, 029, 031, and 037
- inspected Qwen Code tag `v0.19.11` at exact commit
  `f22cf5009ee3eb26b5c5de2eca6e1f1d0ffee0ad`
- hashed the Qwen CLI argument, session storage, and non-interactive session
  sources
- checked current Qwen headless documentation against the exact tagged source
- inspected Ollama `ChatRequest` and route source at all four existing
  qualification points
- checked the current official Ollama `/api/chat` contract
- froze bounded synthetic first-turn, continued-turn, mismatch, transaction,
  failure, cancellation, deadline, and cleanup cases

No executable, model, server, account, credential, provider request, paid
operation, container, or consumer repository was used.

## Contract Fit

The public `InteractiveSessionDriver`, `InteractiveSessionHandle`,
`TurnRequest`, event, cancellation, terminal, and cleanup roles already fit
both routes.

Contracts 017 and 030 do not.

- Qwen's `--resume` is private turn-to-turn invocation state inside one
  runtime attachment. It does not expose public load, replay, or resume.
- Ollama replays ordinary user and assistant messages. It has no provider
  continuation token and no consumer tool loop.

Contract 043 promotes the missing shared distinction:

- harness-retained restarted continuation
- consumer-owned transactional transcript replay

Existing `MaximumTurns`, `PrivateHistoryMaximumBytes`,
`StreamRecordMaximumCount`, and `OutputTokenMaximum` constraints can bind the
first profiles. No new common operation role or generic session API is needed.

## Qwen Exact Evidence

The qualified version remains exact `0.19.11`.

| Source | SHA-256 |
| --- | --- |
| `packages/cli/src/config/config.ts` | `e5decf24a473bd1095d1184c5092c45159222184eca9fe52a186f76e14af1ea4` |
| `packages/core/src/services/sessionService.ts` | `b6c7a63b5856e20ca53e0b612e0a2c8e95a1869c9948069a1034c3b83ffd4788` |
| `packages/cli/src/nonInteractive/session.ts` | `9a907a77c28bd0738ba64e5bf40168a2651d882cb46c9510dd20c1f64ea3753c` |

The exact CLI source establishes:

- chat recording defaults on unless explicitly disabled
- `--continue` selects the most recent project session
- `--resume` selects one exact session id
- the two flags are mutually exclusive
- `--session-id` cannot accompany either resume selector
- missing resumed state exits before inference

The stable Qwen profile must never use `--continue`; latest-session selection
would break immutable identity. The first turn starts normally and learns the
provider session id from the stream. Later turns use only exact private
`--resume`.

The child process exits after each headless turn. A clean completed turn
commits continuation. Any failed, cancelled, timed-out, mismatched, or
uncertain turn invalidates the runtime session because provider transcript
mutation cannot be rolled back safely.

The corpus adds:

- `interactive-session.json`
- `interactive-first-turn.jsonl`
- `interactive-continued-turn.jsonl`
- `interactive-session-mismatch.jsonl`

## Ollama Exact Evidence

The existing compatibility claim remains:

| Release | Commit |
| --- | --- |
| `0.14.0` | `02a24015968d612b418448b73cffaa1b0652d161` |
| `0.18.0` | `3980c0217d27e05a441808a446e7ee5ea7e04256` |
| `0.30.0` | `2c71d8d7ca6edbc9bdc1a312f71ce3b079c0fe56` |
| `0.32.1` | `30c390384e20333b67cadab60da5bcb669407f01` |

At every point `ChatRequest.messages` is an ordered message array explicitly
used for chat memory. The current official API still defines `messages` as
required chat history.

The session owns at most 24 committed user turns and 48 user/assistant
messages. Private encoded history and one request body are each capped at
1 MiB. A request contains the committed history plus the candidate user
message. Only a complete successful response commits that user/assistant
pair.

Provider error, malformed output, disconnect, cancellation, and timeout leave
history unchanged. A later attempt is consumer-directed; there is no retry.
The attached server and runtime-managed model residency survive session close.

The corpus adds:

- `interactive-session.json`
- `interactive-turn-1-request.json`
- `interactive-turn-1-success.ndjson`
- `interactive-turn-2-request.json`
- `interactive-turn-2-success.ndjson`
- `interactive-turn-2-error.ndjson`

The manifest repeats all four qualification points and the `0.32.2`
exclusion. One behavior corpus covers the maintained range; it does not
qualify the excluded point.

## Authority And Evidence Limits

Qwen keeps its durable project transcript. Swallowtail gains no archive,
restore, delete, enumeration, storage-path, or native-close authority.

Ollama has no provider session state. Swallowtail gains no server ownership,
model load/unload administration, credential, remote billing, or provider
session authority.

Qwen and Ollama may expose exact token usage for one turn. Neither route
reports provider-authoritative billed cost. Streaming text does not create
realtime media or history replay.

## Implementation Boundary

Card 117 is contract-ready.

It may add:

- separate prepared interactive profiles in the existing Qwen and Ollama
  adapters
- adapter-private session state and exact bound validation
- one owned Qwen child or Ollama request per turn
- deterministic lifecycle scenarios and provider-neutral conformance reuse

It must not add a shared continuity enum unless implementation proves a public
consumer need. The selected mode is currently adapter/profile-owned and
visible through expanded requirements.

## Sources

- [Qwen headless mode](https://qwenlm.github.io/qwen-code-docs/en/users/features/headless/)
- [Qwen Code `v0.19.11` configuration](https://github.com/QwenLM/qwen-code/blob/v0.19.11/packages/cli/src/config/config.ts)
- [Qwen Code `v0.19.11` session service](https://github.com/QwenLM/qwen-code/blob/v0.19.11/packages/core/src/services/sessionService.ts)
- [Qwen Code `v0.19.11` non-interactive session](https://github.com/QwenLM/qwen-code/blob/v0.19.11/packages/cli/src/nonInteractive/session.ts)
- [Ollama chat API](https://docs.ollama.com/api/chat)
- [Ollama `v0.14.0` API types](https://github.com/ollama/ollama/blob/v0.14.0/api/types.go)
- [Ollama `v0.18.0` API types](https://github.com/ollama/ollama/blob/v0.18.0/api/types.go)
- [Ollama `v0.30.0` API types](https://github.com/ollama/ollama/blob/v0.30.0/api/types.go)
- [Ollama `v0.32.1` API types](https://github.com/ollama/ollama/blob/v0.32.1/api/types.go)

## Promotion

- Added Contract 043.
- Froze exact Qwen and Ollama interactive corpora.
- Authorized card 117 without widening public load, resume, management,
  serving, media, or billing claims.
