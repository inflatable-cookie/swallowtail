# Validation Latency Closeout

Date: 2026-07-30

Roadmap g02.046 is complete.

## Result

Swallowtail now has two exact-package normal-development selectors:

- `validate:focused` runs selected package tests and warnings-denied clippy
- `package:verify-affected` independently assembles and audits selected
  archives, then compiles them through one shared extracted target

Both require one to four explicit workspace package names. They reject empty,
oversized, duplicate, unknown, and option-like scope. No changed-file
inference was added.

## Measured Acceptance

- Pi and xAI: 64 focused tests plus clippy in four seconds
- four affected adapter archives: five seconds through one shared extracted
  target
- comparable card-155 archive proof: 22.4 seconds through separate targets
- affected archive improvement: about 78 percent, or 4.5 times faster

The focused path remains below its two-minute budget. The affected archive
path remains below its three-minute budget.

## Preserved Evidence

Workspace, package, candidate, consumer, MSRV, live, and release selectors
remain independent. Existing candidate and release scripts were unchanged.
Package isolation still starts with independent archive assembly and content
audit. Selector failures remain non-zero through Effigy.

Workspace check, package metadata, public-API declarations, routes, docs, and
diff gates passed. No full workspace test suite, live provider call, consumer
edit, retained-candidate replacement, or publication ran.

Doctor reported 143 warnings and one error in Kimi local-server activity
projection. That file belongs to concurrent subagent-topology work and changed
after card 155 accepted the zero-error structural baseline. This lane neither
changes nor dispositions it.

## Next

Reassess the next g02 product or provider milestone after concurrent
subagent-topology work closes. Warning-only reduction and publication remain
deferred.
