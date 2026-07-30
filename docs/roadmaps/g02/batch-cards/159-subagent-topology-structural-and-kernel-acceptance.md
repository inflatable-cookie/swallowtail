# 159 Subagent Topology Structural And Kernel Acceptance

Status: completed
Owner: Tom
Created: 2026-07-30
Milestone: `../047-subagent-topology-acceptance-and-consumer-handoff.md`

## Goal

Recover the realized common, Codex, and Kimi child-topology tranche into a
bounded acceptance path and remove its structural regression.

## Scope

1. Audit the committed records and profiles against Contracts 044-045.
2. Split Kimi local-server activity projection on private semantic seams.
3. Preserve exact cursor, lifecycle, actor, snapshot, and disclosure behavior.
4. Prove common bounds and redaction.
5. Prove Codex app-server, Codex exec, and Kimi local-server projection.
6. Restore doctor to warning-only without touching unrelated warnings.

## Acceptance Criteria

- [x] Kimi projection no longer produces an error-level structural finding
- [x] no new error-level structural finding appears
- [x] common identity, parent, lifecycle, bounds, duplicate rejection, and
  redaction tests pass
- [x] Codex app-server and exec topology fixtures pass
- [x] Kimi spawn, running, waiting, completion, failure, and origin mapping pass
- [x] prepared profiles retain exact observation and collaboration-action truth
- [x] public declarations and docs remain consistent
- [x] no provider call, consumer edit, candidate replacement, or publication
  occurs

## Validation

- `effigy validate:focused swallowtail-core swallowtail-runtime
  swallowtail-adapter-codex swallowtail-adapter-kimi`
- `effigy package:api`
- `effigy qa:docs`
- `effigy doctor`
- `git diff --check`

## Stop Conditions

- Stop if the split changes public APIs or provider behavior.
- Stop if the committed implementation contradicts Contract 045.
- Do not reduce warning-only findings outside the touched Kimi seam.
- Do not run the full workspace test suite.

## Auto-Continuation

Yes. Continue to card 160 after all acceptance gates pass.

## Evidence

- Kimi native child lifecycle mapping moved into one private activity module;
  the projection dropped below the error threshold without public or wire
  changes
- core, runtime, Codex, and Kimi ran 370 focused tests plus warnings-denied
  clippy in six seconds
- the 24-crate public-API declaration baseline and docs QA pass
- Doctor reports 144 warnings and zero errors
- no provider call, consumer edit, candidate replacement, or publication ran
