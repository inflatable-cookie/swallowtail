# 2026-08-27 g04.083a Claude Code Headless Fast-Mode Evidence

Status: complete
Card: 232
Research: 233

## Boundary

Evidence only. The worker may update this file, card 232, Research 233, and new
Claude-local frozen evidence. Shared planning and production code stay unchanged.

## Outcome

Research 233 promoted with an honest empty deliver-now set.

Exact package evidence at `2.1.220` and `2.1.241` freezes:

- boolean `fastMode` schema on `--settings`
- print-mode activation encoding via launch-time
  `--settings '{"fastMode": true}'` (`sdk_opt_in_required` without it)
- stream-json `fast_mode_state`, `fast_mode_disabled_reason`, and
  `usage.speed: "standard"` while off
- unchanged omission argv on `claude-code.headless`

Withheld because effective Fast mode, model/access membership, usage-credit and
org-enablement truth, billing, and latency cannot be closed without login,
account inspection, or successful provider work authorized out of scope for this
lane. Unsupported models did not reject before init under unauthenticated
probes. Current `--setting-sources user,project,local` does not prove full
process-private precedence.

Frozen corpus:
`crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-code-2.1.241/headless-fast-mode.json`.

No production code, guide, matrix, or shared index changes.

## Validation

```sh
effigy validate:focused swallowtail-adapter-claude-agent
effigy qa:northstar
git diff --check
```

## Next Move

Orchestrator review of the evidence PR. Shared g04.083 closeout and Next Task
updates remain orchestrator-owned after merge.
