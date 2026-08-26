# 201 Cline Headless Plan-Mode Evidence

Status: complete
Owner: Tom
Created: 2026-08-26
Updated: 2026-08-26
Milestone: [g04.073 Cline Headless Plan Mode](../073-cline-headless-plan-mode.md)
Depends on: Research 147 and 190; Contracts 012, 023, 029, 033, 034

## Goal

Determine whether exact qualified Cline `3.0.55` applies `--plan` to the
selected one-run JSON route with behavior equivalent to portable
`HarnessMode::Plan`. Promote an honest empty set if the fixed argument cannot
be tied to complete immutable Plan behavior without provider work.

## Work

1. Retrieve and digest the published `cline@3.0.55` wrapper, annotated tag
   `cli-v3.0.55`, exact commit
   `ad442cbb6a81d21773ceabc1398ea5eb58170718`, and decisive tagged sources.
   Reconcile identities with Research 147; current docs/main may corroborate
   but cannot amend the exact package.
2. Freeze `-p` / `--plan` parsing: canonical spelling, duplicates, conflicts
   with hidden `--act`/`--yolo`/`--zen`, option placement relative to
   `--json`, `--auto-approve false`, `-c`, and positional prompt, invalid
   forms, omission, and local parse failures.
3. Trace explicit Plan through `commanderToParsedArgs`, `resolveStartupMode`,
   persisted global settings, `Config.mode`, system-prompt construction,
   mode-tagged user input, one-run session creation, terminal result, cleanup,
   and any retained manifest or transcript state. Prove explicit CLI selection
   wins every ambient setting that could otherwise choose Act.
4. Freeze exact Plan behavior: system-prompt instructions, tool preset,
   extension registration, pre-approval command guard, built-in and extension
   tools, shell executor, file edits, writes outside the working resource,
   network/process tools, subagents/teams, and any other route that could mutate
   state. Keep behavioral restrictions separate from containment.
5. Trace every Plan-to-Act seam, including `switch_to_act_mode`, interactive
   mode-switch code, automatic continuation, queued/steered turns, and session
   rebuild. Determine whether any can run during the selected one-prompt JSON
   operation and whether a model can authorize its own behavior widening.
6. Freeze the selected JSON wire's mode evidence. Record whether `run_start`
   requires unselected `--verbose`, what it reports, and whether source-level
   deterministic evidence proves application without synthesizing an
   effective-value observation.
7. Reconcile Plan with existing `--auto-approve false`, read-only
   working-resource policy, `AmbientHost`, `Ambient`, local-account access,
   provider retention, host deadline, cancellation, activity, failure, and
   joined cleanup. Do not relabel any independent boundary.
8. Audit production prepared input, capability profile, plan/evidence,
   request, driver, command builder, fixtures, guide, feature matrix, example,
   and API baseline. Name the smallest exact `HarnessMode::Plan` binding.
9. Prove omission retains exact current argv and behavior. Determine whether
   selected Plan can be validated before spawn and held immutable for the
   entire one-child run.
10. Classify exact route/version/value rows as deliver now, evidence-gated,
    intentionally withheld, or not applicable. Keep ACP, model/thinking,
    later versions, and `UnverifiedNewer` inheritance separate.
11. Promote Research 220 with an exact deliver-now table or explicit empty
    set. Update the milestone/card state and closeout honestly.

## Acceptance Criteria

- [x] exact artifact/source identities and decisive digests are frozen
- [x] parser, precedence, omission, duplicate/conflict, and failure truth is
      settled
- [x] prompt, tool, guard, Plan-to-Act, output, retention, and lifecycle paths
      have exact dispositions
- [x] behavioral Plan equivalence is separated from isolation, permission,
      resource, shell, process, network, and account authority
- [x] requested, planned, dispatched, accepted, applied/effective, and observed
      states are not conflated
- [x] production preparation, plan/evidence, driver, argv, fixtures, docs, and
      API seams are audited
- [x] Research 220 contains a non-empty exact table or honest empty set
- [x] no production code, public API, shared contract/runtime, currentness,
      release, merge, rollover, or g04 closure changes
- [x] focused Cline validation, Northstar QA, research indexes, and diff checks
      pass

## Validation

```sh
effigy validate:focused swallowtail-adapter-cline
effigy qa:northstar
effigy qa:docs:index:research
effigy qa:docs:index:logs
effigy qa:docs:index:roadmaps
effigy qa:docs:index:roadmaps:g04
effigy qa:docs:index:roadmaps:batch-cards
effigy qa:docs:next-action:roadmaps
git diff --check
```

Auto-continue to card 202 only when Research 220 admits a non-empty exact
`cline.headless` `3.0.55` `HarnessMode::Plan` row with complete behavior proved
without a provider prompt.

## Stop Conditions

- exact source/applicability, behavioral equivalence, or complete mode/tool
  coverage remains ambiguous
- Plan is parser-only, advisory-only, ambiently overrideable, or can widen to
  Act inside the one-run operation
- deterministic proof needs login, account inspection, provider prompting,
  arbitrary tool execution, paid work, config mutation, or a contract change

## Out Of Scope

- production binding, ACP, model/thinking/timeout/permission changes, live
  provider work, currentness, release, merge, rollover, or g04 closure
