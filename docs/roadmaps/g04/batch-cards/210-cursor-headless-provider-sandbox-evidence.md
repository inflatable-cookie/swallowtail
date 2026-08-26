# 210 Cursor Headless Provider-Sandbox Evidence

Status: ready
Owner: Tom
Created: 2026-08-26
Milestone: [g04.076 Cursor Headless Provider Sandbox](../076-cursor-headless-provider-sandbox.md)
Depends on: Research 077, 087, 135, and 183; Contracts 023, 029, 033, and 037

## Goal

Determine whether any exact qualified Cursor headless build/platform/profile
row can bind `--sandbox enabled` as
`HarnessIsolation::ProviderEnforced`. Promote an honest empty set if native
containment depends on unbound ambient configuration, silent escape/approval,
backend fallback, live provider work, or platform facts unavailable to
preflight.

## Work

1. Reuse and verify the exact artifact identities for
   `2026.07.01-41b2de7`, `2026.07.23-e383d2b`,
   `2026.08.04-aaa8809`, and `2026.08.11-e8db854`. Digest every decisive CLI,
   sandbox, command, configuration, platform, and process source. Current docs
   and newer host help may corroborate only.
2. Freeze `--sandbox` parsing: `enabled|disabled`, missing/empty/invalid/
   repeated values, aliases, precedence, option placement, omission, and local
   parse failures. Prove whether CLI selection overrides persisted mode for the
   whole run.
3. Freeze exact platform/backend selection. Record supported Darwin/Linux/
   Windows mechanisms, prerequisites, environment markers, backend failure,
   fallback, disabled, degraded, and unsupported-platform behavior.
4. Trace filesystem rules: working directory, reads, writes, protected paths,
   symlinks, mounts, temporary directories, git metadata, configuration files,
   child processes, inherited descriptors, and any path outside the selected
   working resource.
5. Trace network rules: default egress, built-in domains, user/project/team
   allowlists, proxies, sockets, DNS, loopback, MCP/fetch/browser surfaces, and
   any `Allow All` or escape path.
6. Trace command classification and approvals. Determine which terminal calls
   run sandboxed, which cannot, whether print mode can request or receive an
   approval, and whether any unsupported command executes outside the sandbox
   without a new consumer operation.
7. Separate native isolation from `--mode plan`, `--trust`, `Read|ReadWrite`,
   permissions, tools, `.cursorignore`, and ambient configuration. Classify
   both access profiles independently.
8. Freeze observation truth. Record whether stream events, environment facts,
   or terminal status prove backend activation and enforcement. Do not infer
   enforcement from argv, parser acceptance, a successful command, or tool
   absence.
9. Audit prepared input, capability profile, plan/evidence, request policy,
   driver, command builder, platform facts, fixtures, guide, matrices,
   examples, and API baseline. Name the smallest exact binding or the missing
   preflight fact.
10. Prove omission retains exact no-flag argv, `AmbientHost`, and current
    configuration/retention behavior.
11. Classify every exact build/platform/access/value row as deliver now,
    evidence-gated, intentionally withheld, or not applicable. Keep calendar
    gaps and `UnverifiedNewer` points separate.
12. Promote Research 223 with the exact table or explicit empty set. Update
    milestone/card state and closeout honestly.

## Acceptance Criteria

- [ ] exact identities, decisive source digests, and platform facts are frozen
- [ ] parser, precedence, omission, invalid/repeated, and failure truth is
      settled
- [ ] filesystem, network, subprocess, configuration, approval, escape,
      fallback, and observation truth have exact dispositions
- [ ] `ProviderEnforced` is separated from access, Plan, permissions, tools,
      workspace trust, and host isolation
- [ ] production preparation, plan/evidence, driver, argv, platform, fixtures,
      docs, and API seams are audited
- [ ] Research 223 contains a non-empty exact table or honest empty set
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

Auto-continue to card 211 only when Research 223 admits a non-empty exact
`cursor-agent.headless` `ProviderEnforced` row whose full native boundary is
preflight-bindable without provider work.

## Stop Conditions

- exact source, platform applicability, boundary, approval/escape, or
  observation truth remains ambiguous
- ambient configuration can widen the claimed boundary
- backend absence or command incompatibility falls back outside containment
- deterministic proof needs login, account inspection, provider prompting,
  tool execution, paid work, config mutation, or a shared contract change

## Out Of Scope

- production binding, `disabled`, network/path policy selection, host
  isolation, live provider work, currentness, release, merge, rollover, or g04
  closure
