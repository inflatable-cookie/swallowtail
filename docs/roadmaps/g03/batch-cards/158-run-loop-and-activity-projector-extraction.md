# 158 Run-Loop And Activity Projector Extraction

Status: ready
Owner: Tom
Created: 2026-08-08
Milestone: `../052-shared-adapter-scaffolding.md`
Depends on: card 157

## Goal

Extract the run-loop scaffold and the ACP event-to-activity projector so the
turn, session, pump, handle, and lifecycle families and the five near-identical
ACP activity modules share one implementation.

## Scope

1. Extract a run-loop scaffold (pump, handle, cancellation, cleanup, terminal
   projection) into `swallowtail-runtime`, parameterized by the route's event
   translation and terminal rules.
2. Extract the ACP event-to-activity projection into
   `swallowtail-protocol-acp` or `swallowtail-runtime` (pair-identical at 67%
   across kimi, claude-agent, gemini, grok, cursor).
3. Migrate the run-loop family in adapter batches; migrate the ACP activity
   modules in one batch since they share the same wire.

## Out Of Scope

- activity profile or event-shape changes
- provider-specific event translation (stays adapter-local)
- public API changes

## Acceptance

- [x] the run-loop helpers have focused tests
- [x] migrated adapters pass focused and extracted-package proof with an
      unchanged public API baseline
- [x] the shared run-loop slice shrinks the measured duplication, and the
      activity-projector disposition is recorded

## Stop Conditions

- stop if a migrated adapter changes event delivery, terminal, or activity
  fidelity

## Auto-Continuation

Yes, to card 159 after acceptance.

## Validation

- focused validation per migrated adapter; `effigy package:verify-affected`
  per batch
- `effigy check:examples` and `effigy qa:routes` after activity touches

## Completion Evidence

- new `swallowtail-runtime/src/run_loop.rs` owns the byte-identical run-loop
  helpers with four focused tests: `emit` (ordered event + sequence
  increment), `emit_content` (validated content transport), `emit_activity`,
  `provider_status`, and `cleanup_result`
- nine adapter pump files across seven adapters (anthropic observations and
  managed pump, deepseek events, kimi-platform pump, ollama pump, llama.cpp
  pump, alibaba turn and run pumps, openai observations) now delegate to the
  shared helpers instead of carrying identical bodies
- the ACP activity projector was measured precisely (kimi≈claude-agent 0.99,
  grok=cursor 1.00, gemini divergent) and then recorded as staying
  adapter-local: the recorded architecture forbids a shared home, since
  `swallowtail-protocol-acp` is "without provider or runtime projection" and
  `swallowtail-runtime` is "only core, futures-core, and zeroize
  dependencies"; a shared projector would require changing that topology,
  which is a planning decision for the operator rather than this card
- the deeper pump loops (event translation, terminal rules) also stay
  adapter-local; the shared slice is the emit/sequence/status helpers
- focused validation passes for the touched packages, affected-package proof
  passes, the workspace round passes 1,506 tests, and the semantic API
  baseline is regenerated for the runtime additions
