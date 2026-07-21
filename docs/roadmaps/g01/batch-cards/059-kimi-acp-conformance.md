# 059 Kimi ACP Conformance

Status: complete
Owner: Tom
Updated: 2026-07-21
Milestone: `../018-kimi-code-acp-portability-proof.md`

## Objective

Prove Kimi as a second ACP agent without flattening its lifecycle into Gemini's
subset, then close roadmap 018.

## Scope

- provider-neutral ACP, persistent-session, and ambient-harness conformance
- local and remote-authoritative topology with explicit isolation posture
- load, any separately proven resume, replay, write callback, cancellation,
  disconnect, redaction, and cleanup
- optional installed probe only under explicit operator gating
- roadmap and front-door closeout

## Out Of Scope

- provider permission approval
- owned llama.cpp implementation
- live login as default QA

## Acceptance Criteria

- [x] common ACP transport passes against two independent agents
- [x] provider-specific capabilities remain explicit
- [x] writable callback authority, ambient execution, optional containment,
      and delegated login remain separate
- [x] full QA passes or failures are recorded honestly
- [x] the roadmap closeout selects the next coverage choice from current
      evidence

## Validation

- focused conformance tests
- `effigy qa`
- `effigy doctor`
- `git diff --check`

## Stop Conditions

- conformance would require provider identity in shared ACP or runtime code
- cards 057, 065, 066, or 058 remain incomplete

## Auto-Continuation

No. Return to the roadmap 018 planning checkpoint.

## Evidence

- one provider-neutral decoder accepts the pinned Gemini and Kimi corpora;
  Kimi-only load, resume, and write methods remain absent from Gemini claims
- a ninth synthetic profile composes persistent-session, replay, read-write
  callback, delegated-auth, ambient-authority, topology, and ordered-cleanup
  assertions without widening the baseline ACP profile
- the production Kimi fixture runs new, prompt, write, load, replay, resume,
  redaction, and joined cleanup under local and remote-authoritative host ids
- cancellation and disconnect retain explicit terminal and cleanup behavior
- `probe:kimi-installed` is ignored unless `SWALLOWTAIL_LIVE_KIMI_ACP=1`
- full repository QA passes with 271 tests; all three installed/live probes are
  ignored by default
- doctor remains at the inherited 19 findings: 12 warnings and 7 errors
- `git diff --check` passes

## Continuation

- roadmap 018 is complete
- roadmap 020 and card 067 own the SDK-native-first coverage recheck
- cards 068-069 remain in bounds only after card 067 selects one route
