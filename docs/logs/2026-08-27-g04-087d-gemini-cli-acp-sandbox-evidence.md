# 2026-08-27 g04.087d Gemini CLI ACP Sandbox Evidence

Status: done
Card: 247
Research: 244

## Boundary

Evidence only. This lane updated card 247, Research 244, this log, and
Gemini-local frozen evidence under
`crates/swallowtail-adapter-gemini/tests/fixtures/gemini-cli-acp-0.56.0-sandbox/`.
Shared planning and production code stayed unchanged.

## Outcome

Honest empty deliver-now set across published `0.51.0..=0.56.0`.

Every published stable tag in the window keeps identical decisive
`sandboxConfig.ts`, precedence tests, boolean `--sandbox` argv, `gemini.tsx`
sandbox hop before `runAcpClient`, ACP stdio transport, and sandbox-free
`initialize` dispatcher bytes. `start_sandbox` may inherit stdio and drain
non-TTY stdin into `--prompt` before re-exec; `0.55.1` starter delta does not
add ACP confirmation. Unpublished
`0.54.1`/`0.54.2`/`0.54.3`/`0.55.0`/`0.56.1` are recorded. Ambient
`GEMINI_SANDBOX` still overrides argv/settings; initialize/`session/new` have
no sandbox field; selection is not containment. Research 239 used only as
headless contrast.

## Validation

```text
effigy validate:focused swallowtail-adapter-gemini  # pass; 85 tests
effigy qa:northstar                                 # pass
git diff --check                                    # pass
```

## Next

PR against current `main`. Orchestrator owns shared closeout after merge
authorization. No production binding from this branch.
