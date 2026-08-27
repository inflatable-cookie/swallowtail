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

Honest empty deliver-now set.

Exact `@google/gemini-cli@0.56.0` has a native sandbox surface (`--sandbox`,
`GEMINI_SANDBOX`, `settings.tools.sandbox`) and a closed backend vocabulary, but
the qualified ambient headless route cannot bind it process-privately or confirm
activation prompt-free. Docs claim flag-before-env precedence; tagged source
gives `GEMINI_SANDBOX` precedence over argv and settings. `start_sandbox()` is
backend start. Stream-json `init` has no sandbox field. Exit `44` is failure
only. Selection is not containment.

## Validation

```text
effigy validate:focused swallowtail-adapter-gemini  # pass; 85 tests
effigy qa:northstar                                 # pass
git diff --check                                    # pass
```

Inherited doctor baseline not re-run; record drift only if later checks diverge.

## Next

PR against current `main`. Orchestrator owns shared closeout after merge
authorization. No production binding from this branch.
