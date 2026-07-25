# 025 Pi And Qwen Harness Facades

Status: complete
Owner: Tom
Created: 2026-07-25
Milestone: `../009-remaining-harness-facades.md`

## Objective

Add prepared normal paths for Pi RPC and Qwen headless without treating them as
ACP or Codex variants.

## Governing Refs

- Contracts 023, 028-029, 032-034, and 037
- current Pi RPC and Qwen headless fixtures
- card 024

## Scope

1. Prepare exact host-approved installed targets and compatibility evidence.
2. Bind Pi prompt, steering, follow-up, abort, and UI relay separately.
3. Bind Qwen one-shot structured run with explicit prompt and budgets.
4. Preserve ambient authority, retained local state, and provider-native
   isolation truth.
5. Keep RPC scheduling and structured CLI lifecycle distinct.

## Acceptance Criteria

- [x] Pi scheduling semantics do not enter shared facade records
- [x] Qwen stdin and stream-JSON bounds remain exact
- [x] no sandbox or containment claim is inferred
- [x] provider/model identity and configuration remain explicit
- [x] low-level drivers remain callable

## Implementation

1. Add exact installed discovery and version binding where either adapter lacks
   the shared prepared prerequisite.
2. Add separate adapter-local preparation values for Pi RPC and Qwen headless.
3. Bind Pi's long-lived RPC command lifecycle without copying scheduler or UI
   protocol records into shared facade types.
4. Bind one Qwen structured-run attempt with its exact stdin, stream-JSON,
   budget, configuration, and isolation requirements.
5. Add deterministic topology, drift, cancellation, cleanup, and
   unverified-newer evidence plus public examples.

## Validation

- Pi and Qwen deterministic corpora
- harness isolation and RPC assertion packs
- local and remote-authoritative hosts
- version range and unverified-newer cases

## Stop Conditions

- stop if either adapter needs a new shared lifecycle or preparation contract
- stop if Qwen isolation evidence cannot be represented without a containment
  claim
- stop if Pi scheduling or UI relay must become provider-neutral facade state

## Auto-Continuation

Yes. Continue to card 026.

## Completion Evidence

- Pi and Qwen probe only one host-approved target with `--version`, retain the
  exact observation, qualify 0.80.10 and 0.19.11 respectively, and admit later
  stable releases as visible unverified-newer execution.
- Pi derives one provider-suppressed, ambient-host, read-only RPC session. The
  returned low-level session and turn handles continue to own prompt,
  steering, follow-up, abort, UI callback, cancellation, and cleanup.
- Qwen derives one ambient-host structured run with explicit provider, model,
  prompt, working resource, host deadline, stdin, stream-JSON output, and the
  fixed 60-second, 16-tool-call, 24-turn native bounds.
- Both adapters retain their public low-level drivers. Neither facade claims
  sandboxing or containment.
- Prepared tests pass on local and remote-authoritative host identities.
  Existing Pi RPC and Qwen native-boundary suites remain green.
- Public examples and guides compile. Full Effigy QA and the 23-crate public
  API declaration gate pass.
