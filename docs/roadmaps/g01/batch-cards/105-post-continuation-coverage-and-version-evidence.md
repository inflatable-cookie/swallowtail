# 105 Post Continuation Coverage And Version Evidence

Status: completed
Owner: Tom
Updated: 2026-07-23
Milestone: `../037-post-continuation-coverage-and-compatibility-checkpoint.md`

## Objective

Refresh the realized coverage and compatibility inventory, then select and
compile the next bounded provider or transport lane only when current evidence
makes it exact.

## Readiness Gate

Roadmap 036 is complete. Twenty production routes, twelve provider-neutral
profiles, Contract 029 compatibility windows, and 489 repository tests now
cover the existing harness, direct API, ACP, SDK, realtime, background,
provider-managed, and self-hosted shapes.

## Scope

- exact inventory of production drivers, transports, access mechanisms,
  lifecycle shapes, host topologies, and shared profiles
- per-driver compatibility posture: exact-only, maintained window, baseline,
  milestones, deprecations, exclusions, and live-probe availability
- current official or maintained evidence for remote ACP, Grok tooling, Z.AI,
  Cursor, Claude local and subscription-authorized routes, and useful attached
  runtimes
- information-gain ranking against the twenty realized routes
- one evidence-backed next lane or an explicit operator checkpoint
- no provider implementation, live credential, paid inference, consumer edit,
  implicit fallback, or compatibility promise without a tested corpus

## Ordered Work

1. Inventory every realized driver and common profile without collapsing
   provider, driver, transport, access, model route, or artifact identity.
2. Classify each driver version claim and identify the smallest useful
   baseline and behavior milestones that current evidence can support.
3. Revalidate credible next candidates against current primary sources.
4. Compare new runtime pressure, authority clarity, version maintainability,
   operational weight, and duplication.
5. Promote a dated research delta and any missing contract before compiling
   implementation cards.
6. Select one exact route or stop for operator intent.

## Acceptance Criteria

- [x] all twenty production routes and twelve profiles are accounted for
- [x] exact pins and maintained ranges are distinguished honestly
- [x] no range spans untested behavior changes or inferred compatibility
- [x] candidate access and support authority are current and explicit
- [x] the recommendation is sequenced by information gain, not provider count
- [x] g01 retains one sole ready next task

## Validation

- `effigy qa:docs`
- `effigy doctor` delta review
- `git diff --check`

## Auto-Continuation

No. This is an evidence and route-selection checkpoint.

## Evidence

- Research 024 records every production route and common profile.
- Pi `0.80.10` and DeepSeek's dated facade are the only descriptor claims;
  both remain exact one-point windows.
- Remote ACP is Draft. Grok Build `0.2.111` overlaps existing ACP and JSONL
  routes. Claude subscription authority is contradictory across current
  first-party surfaces. Cursor remains public beta. Z.AI Coding Plan is
  restricted to supported tools.
- Ollama native API adds exact `/api/version`, installed/running catalogue
  observation, NDJSON, and attached-runtime residency truth without a
  container.
- Tagged source supports a text-only `0.14.0` through `0.32.1` candidate
  window with `0.18.0` and `0.30.0` intermediate qualification points.
- Contract 031 promotes the missing attached-runtime boundary.
- Roadmap 038 and cards 106-109 own records, corpus, production, and
  conformance.

## Validation

- `effigy qa:docs`
- `effigy doctor` delta review
- `git diff --check`
