# 007 ACP Range Maintenance Acceptance

Status: superseded
Owner: Tom
Created: 2026-07-31
Milestone: `../002-claude-and-gemini-acp-range-maintenance.md`
Depends on: cards 005-006

Superseded. Claude receives standalone acceptance on card 041; Gemini is
deferred outside the active generation queue.

## Goal

Close the Claude and Gemini range tranche through focused cross-adapter,
package, public-truth, and optional-probe evidence.

## Scope

1. Run shared ACP and both adapters across their claimed authoritative host
   topologies and exact compatibility boundaries.
2. Repair OpenCode's ignored live selector so it evaluates the exact observed
   server against the current claim instead of requiring `1.14.48`.
3. Compile ignored live probes without running provider effects.
4. Assemble and compile affected ACP, Claude, Gemini, and OpenCode packages.
5. Refresh route, roadmap, research, front-door, and closeout truth.

## Acceptance Criteria

- [ ] Claude and Gemini exact ranges pass boundary and milestone conformance
- [ ] stable ACP shared behavior remains unchanged
- [ ] local and remote-authoritative fixture behavior agrees where applicable
- [ ] OpenCode's optional selector accepts exact qualified or visibly
  unverified-newer posture without widening its range
- [ ] affected extracted packages assemble and compile
- [ ] public route and currentness surfaces match production claims
- [ ] no provider prompt, attached-server mutation, consumer edit, candidate
  replacement, or publication runs
- [ ] roadmap g03.002 closes with one explicit next checkpoint

## Validation

- `effigy validate:focused swallowtail-protocol-acp swallowtail-adapter-claude-agent swallowtail-adapter-gemini swallowtail-adapter-opencode`
- `effigy package:verify-affected swallowtail-protocol-acp swallowtail-adapter-claude-agent swallowtail-adapter-gemini swallowtail-adapter-opencode`
- `effigy qa:routes`
- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`
- no broad workspace suite

## Stop Conditions

- Stop if cross-host behavior differs or an extracted package fails.
- Stop if the OpenCode selector would relabel an unverified point as qualified.
- Do not start, update, prompt, authenticate, or mutate an attached harness.

## Auto-Continuation

No. Return to the g03 compatibility checkpoint.
