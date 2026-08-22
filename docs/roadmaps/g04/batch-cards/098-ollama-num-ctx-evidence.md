# 098 Ollama Num Ctx Evidence

Status: complete
Owner: Tom
Created: 2026-08-22
Milestone: [g04.036 Ollama Attached Context Window](../036-ollama-attached-context-window.md)
Depends on: Research 024, 035, 138, and 174

## Goal

Freeze exact secret-free evidence for Ollama native `options.num_ctx`, then
define the smallest numeric and operation-profile subset that can be bound
without a live runtime or a portable context-window abstraction.

## Method

1. Inspect exact official tagged source at the existing qualification points
   `0.14.0`, `0.18.0`, `0.30.0`, `0.32.1`, `0.32.14`, and `0.32.15`.
2. Freeze the `ChatRequest` options shape, `Options`/runner `NumCtx` JSON field,
   validation/defaulting path, and any request-time clamp, cap, reload, or
   truncation semantics relevant to a local `/api/chat` request.
3. Freeze current official FAQ and native API/OpenAPI specimens that name
   `num_ctx` as a context-window option. Keep `/api/generate`, CLI, environment,
   Modelfile, cloud, and OpenAI-compatible evidence separately classified.
4. Record exact source URLs, tag/commit identities, file digests, excerpts, and
   the existing `ollama.runtime` compatibility segment. Reuse existing fixture
   facts only after independently verifying them.
5. Confirm the selected prepared route rejects remote/cloud model detail and
   therefore cannot silently apply a local guarantee to Ollama Cloud.
6. Define the exact useful positive integer domain. Account for Rust-to-Go
   conversion and upstream minimum/cap behavior; do not turn wire dispatch into
   an effective-value claim.
7. Classify structured inference and interactive transcript replay separately.
   For sessions, inspect every-turn encoding, restoration, fixed-session
   binding, and absent-path compatibility before marking deliver-now.
8. Decide the adapter-local evidence and low-level-driver binding shape. The
   provider-neutral plan must not gain a context-window capability merely to
   carry Ollama data.
9. Freeze a deterministic corpus under a focused Ollama fixture directory and
   write/index promoted Research 184 with deliver-now, evidence-gated,
   intentionally withheld, not-applicable, and obsolete rows.

Do not start Ollama, send a prompt, inspect a local model, pull or unload a
model, authenticate, or change the host.

## Required Decisions

- `num_ctx` remains an Ollama-native runner option, not an output-token bound
  or portable context-window control.
- Structured-run and interactive-session support are independent decisions.
- Local native evidence never promotes Ollama Cloud or compatible endpoints.
- Provider acceptance does not prove the effective allocated context.
- Unknown, zero, negative, overflow, and unproved profile combinations fail
  closed or remain evidence-gated.

## Acceptance Criteria

- exact tagged-source and current official evidence is frozen without secrets
- the numeric domain and Rust/wire type conversion are explicit
- structured and interactive profile dispositions are explicit
- load, memory, truncation, cap, and external-runtime side effects are bounded
- Research 184 is promoted and indexed
- production code, claims, matrices, architecture, and changelog are unchanged
- `effigy validate:focused swallowtail-adapter-ollama` passes
- `effigy qa:northstar` passes
- `effigy qa:docs:index:research` passes
- `git diff --check` passes

Auto-continue to card 099 only when at least one useful native `/api/chat`
profile has an exact numeric domain and no contract gap.

## Stop Conditions

- the exact tagged behavior differs in a way that needs a new compatibility
  segment not covered by this milestone
- no useful positive domain can be validated before network work
- local versus cloud behavior cannot be distinguished on the prepared route
- binding needs a generic option map or provider-neutral context capability
- the value cannot survive prepared evidence and bound-driver extraction

## Out Of Scope

- production binding or dispatch
- live runtime, model, prompt, authentication, install, pull, or unload work
- version-ceiling changes
- another Ollama option or route family
