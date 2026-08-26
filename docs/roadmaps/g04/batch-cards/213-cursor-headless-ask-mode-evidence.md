# 213 Cursor Headless Ask-Mode Evidence

Status: ready
Owner: Tom
Created: 2026-08-26
Updated: 2026-08-26
Milestone: [g04.077 Cursor Headless Ask Mode](../077-cursor-headless-ask-mode.md)
Depends on: Research 077, 087, 135, 183, and 223; Contracts 010, 023, 029, 033, 034, and 037

## Goal

Determine whether any exact qualified Cursor headless build admits a useful,
closed Ask selection for `ResourceAccess::Read`. Promote an honest empty set
if exact read-only Q&A behavior, precedence, immutable dispatch, or version
membership cannot be proved without provider work or wider authority.

## Work

1. [ ] Reuse and verify the exact artifact identities for
   `2026.07.01-41b2de7`, `2026.07.23-e383d2b`,
   `2026.08.04-aaa8809`, and `2026.08.11-e8db854`. Digest every decisive CLI,
   parser, mode, configuration, command, tool, and output source. Current
   official docs and newer host help may corroborate only.
2. [ ] Freeze parser truth for `--mode ask`: missing/empty/invalid/case values,
   `--mode plan`, `--plan`, repeated values, mixed flag forms, placement,
   omission, exit status, diagnostics, and whether any other value reaches the
   selected headless path.
3. [ ] Freeze precedence across CLI flags, persisted user/project/team
   settings, environment, workspace trust, and any session or server default.
   Prove the selected value cannot drift after immutable preparation.
4. [ ] Trace the exact Ask selection from parse through run construction, mode
   dispatch, prompt handling, command/tool registry, permission decisions,
   output framing, and terminal result. Distinguish label, selected enum,
   applied behavior, and effective behavior.
5. [ ] Prove the read-only boundary. Inventory filesystem reads/writes,
   terminal commands, MCP/browser/fetch tools, approvals, settings or transcript
   writes, child processes, and any path that can mutate the selected working
   resource or broader host. Keep provider behavior separate from process
   containment and callback mediation.
6. [ ] Compare Ask with the current Plan mapping and default Agent behavior.
   Determine whether Ask is valid only with `ResourceAccess::Read`, whether
   `ReadWrite` must reject, and whether mode selection can grant or imply
   resource, tool, permission, approval, network, or isolation authority.
7. [ ] Freeze observation truth. Record whether stream JSON or terminal state
   reports requested, selected, applied, or effective mode. Do not infer mode
   from the absence of writes, tools, or plan text.
8. [ ] Audit prepared input, prepared result, low-level driver state, immutable
   plan/evidence, command builder, exact-version assessment, fixtures, guide,
   matrices, examples, and API baseline. Name the smallest closed binding or
   the missing preflight fact.
9. [ ] Prove the existing default paths remain exact: `Read` dispatches
   `--mode plan`; `ReadWrite` omits `--mode`; both retain `--trust`, explicit
   model, ambient configuration, `AmbientHost`, durable retention, and the
   one-child lifecycle.
10. [ ] Compose candidate Ask with every Research 183 model-parameter tuple.
    Prove mode placement and rendered `--model` remain independent.
11. [ ] Classify every exact build/access/value row as deliver now,
    evidence-gated, intentionally withheld, or not applicable. Keep
    `UnverifiedNewer` behavior and calendar gaps separate.
12. [ ] Promote Research 224 with the exact table or explicit empty set. Update
    milestone/card state and close out honestly.

## Acceptance Criteria

- [ ] exact identities, decisive source digests, and parser/precedence facts
      are frozen
- [ ] Ask's behavioral and read-only boundaries have exact dispositions
- [ ] requested, prepared, dispatched, parser-accepted, applied, effective,
      and observed state remain distinct
- [ ] Ask is separated from access authority, working-resource containment,
      isolation, trust, permissions, tools, approvals, and configuration
- [ ] production preparation, driver, argv, fixtures, docs, and API seams are
      audited
- [ ] Research 224 contains a non-empty exact table or honest empty set
- [ ] no production code, public API, shared contract/runtime, currentness,
      release, merge, rollover, or g04 closure changes

## Validation

```sh
effigy validate:focused swallowtail-adapter-cursor
effigy qa:northstar
effigy qa:docs:index:research
effigy qa:docs:index:logs
effigy qa:docs:index:roadmaps
effigy qa:docs:index:roadmaps:g04
effigy qa:docs:index:roadmaps:batch-cards
effigy qa:docs:next-action:roadmaps
git diff --check
```

Auto-continue to card 214 only when Research 224 admits a non-empty exact Ask
row whose closed selection and claimed behavior are deterministic without
provider work.

## Stop Conditions

- exact source, precedence, read-only behavior, or version membership remains
  ambiguous
- ambient configuration can change or widen the selected behavior
- Ask and ReadWrite compose, or Ask can grant write/approval/tool authority
- deterministic proof needs login, account inspection, provider prompting,
  tool execution, paid work, config mutation, or a shared contract change

## Out Of Scope

- production binding, portable `HarnessMode`, raw provider modes, live provider
  work, currentness, release, merge, rollover, or g04 closure
