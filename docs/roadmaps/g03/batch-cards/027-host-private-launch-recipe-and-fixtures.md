# 027 Host-Private Launch Recipe And Fixtures

Status: completed
Owner: Tom
Created: 2026-07-31
Milestone: `../011-host-approved-interpreted-executable-launch.md`
Depends on: Contracts 010 and 032; Research 084

## Goal

Realize the provider-neutral local-host launch recipe behind one opaque
executable reference and prove its authority and bounds deterministically.

## Scope

1. Add one redacted local-host launch recipe for an exact program, immutable
   prefix arguments, and optional bootstrap environment.
2. Make native approvals use the same shape with no prefix or bootstrap values.
3. Compose prefix and driver arguments under the existing argument limits.
4. Bound bootstrap environment and apply explicit request environment last.
5. Add deterministic fixtures for ordering, environment precedence, limits,
   redaction, and unchanged native behavior.

## Acceptance Criteria

- [x] recipes remain private to `swallowtail-host-local`
- [x] adapters and portable runtime records still receive only `ExecutableRef`
- [x] ambient environment remains cleared
- [x] prefix arguments precede driver arguments and share their limits
- [x] bootstrap environment is bounded and request environment has explicit
  final precedence
- [x] formatting exposes only safe counts
- [x] native process behavior remains unchanged
- [x] focused host-local validation passes

## Validation

- `effigy validate:focused swallowtail-host-local`
- `cargo fmt --all --check`
- `git diff --check`
- no broad workspace suite

## Auto-Continuation

Yes. Continue directly to card 028 when deterministic proof passes.

## Evidence

- 29 host-local tests passed, including interpreted argv ordering, composed
  limits, bootstrap precedence, redaction, and unchanged native fixtures
- package compilation and focused checks completed in two seconds
- no provider binary, credential, prompt, network, or workspace effect ran
