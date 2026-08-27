# 2026-08-27 g04.085c Gemini CLI Headless Sandbox Evidence

Status: done
Card: 240
Research: 239

## Boundary

Evidence only. This lane updated card 240, Research 239, this log, and
Gemini-local frozen evidence under
`crates/swallowtail-adapter-gemini/tests/fixtures/gemini-cli-headless-0.56.0-sandbox/`.
Shared planning and production code stayed unchanged.

## Outcome

Honest empty deliver-now set across published `0.51.0..=0.56.0`.

Every published stable tag in the window keeps identical decisive
`sandboxConfig.ts`, precedence tests, boolean `--sandbox` argv, `gemini.tsx`
re-exec lifecycle, and sandbox-free `InitEvent` bytes. `start_sandbox`
implementation changes at `0.55.1` without adding confirmation. Unpublished
`0.54.1`/`0.54.2`/`0.54.3`/`0.55.0`/`0.56.1` are recorded. Ambient
`GEMINI_SANDBOX` still overrides argv/settings; stream-json has no sandbox
field; selection is not containment.

## Validation

```text
effigy validate:focused swallowtail-adapter-gemini  # pass; 85 tests
effigy qa:northstar                                 # pass
git diff --check                                    # pass
```

Re-validated after the published-point follow-up.

## Next

PR against current `main`. Orchestrator owns shared closeout after merge
authorization. No production binding from this branch.
