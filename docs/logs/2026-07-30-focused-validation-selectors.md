# Focused Validation Selectors

Date: 2026-07-30

Card 157 adds two normal-development validation paths without changing
workspace, package, candidate, consumer, MSRV, release, or live gates.

## Selectors

```sh
effigy validate:focused \
  swallowtail-adapter-pi \
  swallowtail-adapter-xai

effigy package:verify-affected \
  swallowtail-adapter-pi \
  swallowtail-adapter-xai
```

Both accept one to four exact workspace package names. Empty, oversized,
duplicate, unknown, and option-like scope fails before package work. Scope is
never inferred from changed files.

`validate:focused` runs one nextest invocation and one warnings-denied
all-target clippy invocation.

`package:verify-affected`:

1. assembles each archive independently
2. checks archive size and forbidden members
3. rejects packaged path or git dependencies
4. scans extracted content for local paths and secret shapes
5. creates an offline temporary subset lock
6. compiles all selected extracted packages through one shared target

The temporary lock is required because the canonical lock describes the
24-member workspace. Repository lock and source files remain unchanged.

## Evidence

- deterministic selector tests: passed
- Bash syntax: passed
- bounded shellcheck: passed
- Pi and xAI focused proof: 64 tests plus clippy, four seconds
- Pi and xAI affected archive proof: five seconds
- four-adapter affected archive acceptance: five seconds, down from 22.4
  seconds through separate targets
- package metadata: passed
- 24-crate public-API declaration baseline: passed
- provider calls: none

The focused budget is two minutes. The affected-archive budget is three
minutes. Both representative paths are comfortably inside budget.

## Next

Roadmap g02.046 is complete. The next g02 product or provider milestone waits
for reassessment after concurrent subagent-topology work closes.
