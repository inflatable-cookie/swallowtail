# Validation Runtime Inventory

Date: 2026-07-30

Card 156 freezes Swallowtail's current validation graph, evidence tiers,
runtime variance, duplication, and safe cache boundaries. It changes no task
or script behavior.

## Evidence Tiers And Owners

Owner means who should invoke the proof, not who owns its implementation.

| Selectors | Evidence | Tier | Invocation owner |
| --- | --- | --- | --- |
| `qa:docs:links`, `qa:docs:index:vision`, `qa:docs:next-action:roadmaps`, `qa:docs:agent-defaults`, `qa:docs` | docs links, index, sole next action, forbidden defaults | static | change author |
| `qa:northstar:spine`, `qa:northstar:agent-contract`, `qa:northstar:readme`, `qa:northstar:docs-front-door`, `qa:northstar:headings`, `qa:northstar` | strict docs posture | static | docs change author |
| `qa:routes` | route, lifecycle, feature, and activity matrices | static | route-truth change author |
| `format:check`, `package:api`, `package:metadata` | formatting, public declarations, package topology | static | affected change author |
| focused package tests and clippy | changed package behavior and warnings | focused | batch author |
| `check:rust`, `check:examples`, `lint:rust`, `test:rust`, `health`, `qa` | workspace compilation and regression | milestone | accepting card |
| `package:docs`, `package:msrv`, `package:verify-local`, `package:check` | published shape, compiler floors, all archives | package | package milestone |
| `package:candidate:prepare`, `package:candidate:verify`, `package:candidate:consumers`, `package:candidate:facades` | retained candidate, reproducibility, provider and consumer proof | release | release operator |
| `probe:gemini-installed`, `probe:opencode-installed`, `probe:kimi-installed`, `probe:ollama-installed`, `probe:claude-agent-acp-managed` | installed external interface evidence | live | evidence-lane operator |
| `bootstrap:claude-agent-acp` | managed fixture installation | setup, not validation | evidence-lane operator |

Full workspace, package, candidate, consumer, MSRV, and live selectors are not
normal batch feedback. Their evidence remains mandatory when the owning card
names it.

## Historical Runtime

Effigy history mixes earlier workspace sizes, warm and cold caches, failures,
and interrupted runs. It proves variance and routing cost; it is not a stable
benchmark.

| Selector | Successful samples | Median | 90th percentile | Maximum |
| --- | ---: | ---: | ---: | ---: |
| `qa:docs` | 512 | 0.08 s | 0.15 s | 51.1 s |
| `qa:northstar` | 301 | 0.09 s | 0.20 s | 25.9 s |
| `qa:routes` | 133 | 0.09 s | 0.15 s | 0.37 s |
| `format:check` | 241 | 0.67 s | 1.61 s | 18.9 s |
| `check:rust` | 240 | 0.84 s | 55.7 s | 12.0 min |
| `lint:rust` | 188 | 1.47 s | 46.9 s | 8.65 min |
| `test:rust` | 127 | 29.4 s | 5.10 min | 56.3 min |
| `qa` | 128 | 36.6 s | 3.88 min | 20.1 min |
| `package:verify-local` | 22 | 2.40 min | 4.01 min | 7.07 min |
| `package:check` | 7 | 3.48 min | 12.4 min | 12.4 min |
| `package:candidate:verify` | 6 | 1.82 min | 2.74 min | 2.74 min |
| `package:candidate:consumers` | 4 | 3.25 min | 11.3 min | 11.3 min |
| `package:candidate:facades` | 3 | 4.52 min | 16.7 min | 16.7 min |

The card-155 path supplies current bounded evidence:

- four-package focused tests: 10 seconds including compilation; 0.75 seconds
  execution
- four-package warnings-denied clippy: 9.3 seconds
- warm workspace all-target check: 0.18 seconds
- four affected archives compiled separately: 22.4 seconds
- bounded warm `qa:routes` measurement on this card: 0.13 seconds

No Cargo or workspace suite ran for card 156.

## Duplication

- `qa` expands docs, Northstar, routes, format, full workspace clippy, and the
  full workspace test suite. It repeats focused proof when used on every batch.
- `health` repeats Northstar and workspace compilation already present in
  milestone paths.
- `package:check` adds docs and three compiler checks, then
  `package:verify-local` assembles 24 archives, checks the extracted workspace,
  compiles every test, and executes selected suites.
- candidate verification regenerates the full package set. Provider facade,
  lifecycle, and consumer scripts each extract all packages and use separate
  targets. That isolation is intentional release evidence.
- the card-155 affected-package command used four extracted manifests without
  a shared target. Common dependencies compiled four times.
- interrupted Effigy runs left active QA and test records whose owner PID no
  longer existed. This is tooling-state evidence, not authority to delete
  state or weaken proof.

## Cache Boundaries

Safe:

- one workspace target for explicit source-package tests and clippy
- one shared temporary target for independently assembled affected archives
- one Cargo invocation with repeated explicit package arguments
- static docs, route, API, and metadata checks

Unsafe or out of scope:

- source-workspace artifacts reused as extracted-package proof
- compiler artifacts shared across MSRV toolchains
- candidate, reproduced-source, provider, consumer, or live evidence sharing
  source state
- inferred package scope from changed files before a separate trustworthy
  dependency-selection proof

## Budgets And Selected Tranche

Normal-path warm budgets:

- static docs, route, format, API, and metadata bundle: five seconds
- focused tests plus warnings-denied clippy for one to four explicit packages:
  two minutes
- independent archive assembly and shared extracted compilation for one to
  four explicit leaf packages: three minutes

Full workspace and release tiers have no artificial time limit. They run only
when their accepting card requires them. A full workspace test run should not
be repeated inside one acceptance card.

Card 157 adds two explicit-package selectors:

1. focused package tests and warnings-denied clippy
2. affected archive assembly, content audit, and shared extracted compilation

It does not change `qa`, package, candidate, consumer, MSRV, live, or release
selectors.

## Concurrent State

Card 156 started after separate observable-activity work changed core, runtime,
Codex, and Kimi files. Doctor therefore reported one new high finding in Kimi
local-server activity projection. Card 155's accepted zero-error baseline
predates that concurrent change. This card does not touch or disposition it.

## Next

Card 157 implements the two bounded selectors and their deterministic failure
evidence.
