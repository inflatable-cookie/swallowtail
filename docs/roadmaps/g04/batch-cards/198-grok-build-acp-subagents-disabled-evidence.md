# 198 Grok Build ACP Subagents-Disabled Evidence

Status: ready
Owner: Tom
Created: 2026-08-26
Updated: 2026-08-26
Milestone: [g04.072 Grok Build ACP Subagents Disabled](../072-grok-build-acp-subagents-disabled.md)
Depends on: Research 130, 163, and 204; Contracts 023, 029, 033, 045

## Goal

Determine whether exact maintained Grok Build `1.0.4..=1.0.5` applies
`--no-subagents` as a complete immutable restriction on every ACP session
owned by one child. Promote an honest empty set if parser acceptance cannot be
connected to exact effective spawn suppression without provider work.

## Work

1. Retrieve and digest current official CLI/configuration documentation plus
   exact `@xai-official/grok@1.0.4` and `1.0.5` wrapper/platform artifacts.
   Record dates, package identities, git heads, integrity, executable hashes,
   and exact-version source provenance in Research 219.
2. Freeze `--no-subagents` parsing on each exact executable: placement before
   `agent stdio`, duplicate use, conflicting aliases or settings, default,
   environment/config precedence, help/version behavior, unknown placement,
   and local failure diagnostics. Running extracted binaries for local
   help/version/parser cases is authorized; do not install or replace `grok`.
3. Trace exact flag handling from root CLI parser through configuration merge,
   ACP agent construction, tool/agent registry assembly, every subagent spawn
   entry point, and shutdown. Current public source is corroboration only unless
   tied to an exact package revision.
4. Classify operation-private structured runs, interactive new sessions, every
   later prompt, attachment recovery, child replacement, and any durable Grok
   state separately. Decide whether one child-start flag applies to all of them
   and whether a session can re-enable or inject agents through ACP `_meta`,
   rules, ambient config, plugins, or stored state.
5. Freeze observable local proof of effective suppression. Prefer exact source
   branches, deterministic constructor/registry specimens, or a secret-free
   no-prompt ACP handshake whose complete bounded response exposes the applied
   profile. Parser acceptance, help text, and binary strings alone are not
   effective proof.
6. Separate requested restriction, argv dispatch, parser acceptance,
   configuration application, subagent-tool absence, attempted spawn,
   provider behavior, and OS process containment. Do not claim an unobserved
   stage.
7. Audit production preparation input, plan/evidence, driver attachment,
   replacement, fixtures, guide, activity, permission, cancellation, failure,
   and cleanup. Name the smallest adapter-local disabled-only public shape.
8. Prove omission retains exact `--no-auto-update agent stdio` argv and current
   behavior. Determine whether every admitted row can be validated before
   spawn and held immutable for the owned child lifetime.
9. Classify exact version/profile/lifecycle rows as deliver now, evidence-gated,
   intentionally withheld, or not applicable. Do not infer inheritance for
   `UnverifiedNewer` versions.
10. Promote Research 219 with an exact deliver-now table or explicit empty set.
    Update the milestone/card state and closeout honestly.

## Acceptance Criteria

- [ ] official and exact package/source evidence is frozen with complete
      identities and decisive digests
- [ ] parser, placement, duplicate, precedence, omission, and failure truth is
      settled for every maintained exact version
- [ ] every ACP session and spawn path has an exact disposition
- [ ] dispatch, parsing, application, registry effect, spawn effect, and
      containment claims remain separate
- [ ] ambient config, stored state, plugins, session metadata, replacement, and
      later-version inheritance are explicit
- [ ] production input, plan/evidence, driver, argv, activity, permission, and
      lifecycle seams are audited
- [ ] Research 219 contains a non-empty exact table or honest empty set
- [ ] no production code, public API, shared contract/runtime, currentness,
      release, merge, rollover, or g04 closure changes
- [ ] focused Grok validation, Northstar QA, research indexes, and diff checks
      pass

## Validation

```sh
effigy validate:focused swallowtail-adapter-grok
effigy qa:northstar
effigy qa:docs:index:research
effigy qa:docs:index:logs
effigy qa:docs:index:roadmaps
effigy qa:docs:index:roadmaps:g04
effigy qa:docs:index:roadmaps:batch-cards
effigy qa:docs:next-action:roadmaps
git diff --check
```

Auto-continue to card 199 only when Research 219 admits a non-empty exact set
with source-backed effective suppression across the named ACP lifecycles.

## Stop Conditions

- exact source/applicability or complete spawn-path coverage remains ambiguous
- the flag is parser-only, advisory, TUI/headless-only, or ambiently
  overrideable
- effectiveness needs a provider prompt, account inspection, tool/subagent
  execution, paid work, generic config mutation, or a contract change

## Out Of Scope

- production binding, web-search controls, effort/model/plan/max-turn controls,
  live provider work, currentness, release, merge, rollover, or g04 closure
