# 010 Cursor Exact Artifact And Route Corpus

Status: completed
Owner: Tom
Created: 2026-07-31
Milestone: `../005-cursor-installed-dual-route-foundation.md`
Depends on: Research 075

## Goal

Freeze exact, sanitized Cursor evidence for catalogue, ACP, and headless routes
before creating a production adapter or compatibility claim.

## Scope

1. Preserve the installed `2026.07.01-41b2de7` identity and current maintained
   ACP-registry artifact as separate exact points.
2. Freeze version and model-list fixtures without account identity.
3. Freeze exact installed ACP initialization and advertised capabilities.
   Require card 012 to add route-specific session, prompt, cancellation, and
   terminal transcripts before enabling those behaviors.
4. Freeze headless JSON and stream-JSON event shapes, error behavior, tool
   state, absent usage/cancellation/schema fields, and terminal results.
5. Record absent or unproven behavior: ACP load/resume/deletion, consumer MCP,
   ACP catalogue authority, headless thinking, and implicit sandboxing.

## Acceptance Criteria

- [x] every fixture names its exact artifact and evidence source
- [x] no account identity, token, workspace content, or raw secret is retained
- [x] same-day build hashes remain distinct exact points
- [x] catalogue membership is separate from invocation availability
- [x] ACP and headless behavior groups are independently named
- [x] unsupported and unobserved surfaces are explicit
- [x] later cards have explicit evidence gates and require no unbounded product
  policy decision

## Validation

- focused deterministic corpus tests added by this card
- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`
- no broad workspace suite or live provider prompt

## Stop Conditions

- Stop if exact artifacts cannot be preserved without private account data.
- Stop if an observed route needs a missing shared contract.
- Do not send a prompt, create a provider session, or mutate Cursor state.

## Auto-Continuation

Completed. Continue to card 011; card 012 retains the explicit live-transcript
gate before it enables Cursor session behavior.
