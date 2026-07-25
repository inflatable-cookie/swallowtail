# 024 Claude And Gemini ACP Facades

Status: completed
Owner: Tom
Created: 2026-07-25
Milestone: `../009-remaining-harness-facades.md`

## Objective

Apply the accepted persistent and baseline ACP facade pattern to Claude Agent
and Gemini CLI.

## Governing Refs

- Contracts 015, 017, 023, 029, 032-035, and 037
- card 023
- current Claude and Gemini ACP fixtures

## Scope

1. Add exact-target prepared construction for both adapters.
2. Bind each adapter's supported catalogue/session operations.
3. Preserve negotiated capabilities, configuration posture, and callbacks
   while keeping optional remote ACP endpoint composition separate.
4. Keep Claude and Gemini access and version authority independent.
5. Retain adapter-private protocol mappings.

## Implementation

1. Reuse Claude's installed observation and add the missing exact Gemini
   observation boundary.
2. Add one adapter-local prepared integration and one new-session profile per
   stdio driver.
3. Keep Claude model selection explicit; keep Gemini's provider model
   observation out of route selection.
4. Delegate session open and turn lifecycle to the unchanged low-level ACP
   roles.
5. Add deterministic preparation, topology, drift, capability, and cleanup
   evidence plus public examples.

## Acceptance Criteria

- [x] neither adapter borrows Kimi-specific persistence semantics
- [x] remote ACP remains an explicit transport choice
- [x] version and configuration posture remain inspectable
- [x] unsupported negotiated capabilities fail before effects
- [x] low-level ACP roles and fixtures remain green

## Validation

- focused Claude and Gemini suites
- ACP conformance and remote-transport cases
- both host identities
- public examples

Result:

- Claude Agent prepares exact installed evidence, public API-key access,
  caller-selected model routes, and read-only ACP sessions.
- Gemini CLI has exact installed discovery, a qualified `0.51.0` baseline,
  visible unverified-newer execution, and observation-only model semantics.
- both paths bind ambient configuration, `AmbientHost`, provider-state
  prohibition, host identity, and plan-derived requests.
- remote ACP remains outside both stdio facades with no implicit fallback.
- focused suites, examples, warnings-denied lint, and full Effigy QA pass.

## Stop Conditions

- stop if either route needs Kimi load, replay, resume, or write semantics
- stop if remote ACP composition requires a new provider-authentication rule
- stop if the prepared path cannot preserve the current low-level lifecycle

## Auto-Continuation

Yes. Continue to card 025.
