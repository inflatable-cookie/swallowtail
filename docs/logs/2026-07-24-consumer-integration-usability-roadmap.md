# Consumer Integration Usability Roadmap

Date: 2026-07-24

## Outcome

The first `0.1.0` candidate is held. Compile-only Nucleus and Soundcheck checks
do not prove valid Codex runtime preparation.

Research 034 records the audit:

- Nucleus's five errors reduce to missing prepared version, configuration, and
  request-policy agreement
- the current Nucleus repair works but duplicates 729 lines of reusable host,
  discovery, and preflight setup
- Soundcheck duplicates another 389 lines and still compiles with runtime-
  incomplete plans
- the installed Codex `0.145.0` target and focused Swallowtail/Nucleus checks
  are healthy

Spec 005 selects a two-layer API:

- low-level records and role traits remain available
- prepared integrations bind adapter-owned facts and plan echoes
- consumer provider, host, target, access, model, resource, reasoning, tools,
  network, search, and writable authority remain explicit
- no umbrella crate, generic `send_prompt`, global executor, or fallback is
  added

## Runway

- g02.002 and cards 005-007: Contract 037, plan-derived requests, staged
  diagnostics, and joined local host composition
- g02.003 and cards 008-010: separate prepared Codex discovery, catalogue,
  interactive-session, and structured-exec paths plus conformance
- g02.004 and cards 011-012: Nucleus adoption and glue removal under Nucleus
  authority
- g02.005 and cards 013-014: Soundcheck adoption and glue removal under
  Soundcheck authority
- g02.006 and cards 015-016: packaged cross-consumer runtime proof and
  replacement `0.1.0` candidate

Only card 005 is ready. Later cards remain planned behind their contract,
implementation, validation, or repository-authority gates.

## Authority

No runtime code, consumer repository, candidate artifact, registry, tag, push,
workflow, or release state changed in this planning batch.

## Validation

- `effigy qa:docs` passes
- `effigy qa:northstar` passes
- `git diff --check` passes
- `effigy doctor` remains at the inherited 19 oversized-file findings: 12
  warnings and seven errors

## Next

Execute card 005 and promote the prepared integration boundary before
implementation.
