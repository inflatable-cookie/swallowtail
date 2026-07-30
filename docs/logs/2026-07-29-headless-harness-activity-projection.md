# 2026-07-29 Headless Harness Activity Projection

## Changed

- enabled and projected Qwen Code's qualified partial-message lifecycle for
  structured runs and private turn-scoped continuation
- projected Gemini CLI assistant deltas plus completion-only correlated tool
  records
- projected Claude Code completion-only assistant, provider-tool, and safe
  unknown records
- projected Kimi Code completion-only assistant and tool records, retry
  milestones, and safe future records
- published exact prepared observable-activity profiles for all four routes
- refreshed the additive pre-1.0 public API baseline for their prepared
  evidence accessors

## Route Truth

- Qwen exposes readable thinking summaries on the selected partial-message
  stream. The other three headless routes expose no qualified readable
  reasoning channel.
- Gemini assistant identity is operation-local. Its tool ids correlate use and
  result records without exposing parameters or output.
- Claude Code supplies no qualified started or updated phase on the selected
  command. Completion-only fidelity remains explicit.
- Kimi assistant identity is operation-local. Tool ids correlate use and
  result records. Resume prose remains metadata, not activity.
- Raw provider envelopes, tool arguments, tool results, retry errors, and
  hidden reasoning remain excluded.
- Permitted newer versions inherit the last qualified profile without widening
  it.

## Validation

- complete Qwen, Gemini, Claude Agent, and Kimi adapter suites — passed
- `effigy check:rust` — passed
- `effigy lint:rust` — passed
- `effigy package:api` — passed after the intentional additive baseline
  refresh
- `cargo fmt --all -- --check` — passed

No executable, credential, account, model request, paid inference, or consumer
repository was used.

## Current State

Card 130 is complete. Card 131 is ready. Cards 131-137 remain in bounds.

## Next

Machine-check every production harness route and close exact activity gaps
before selecting direct-inference activity.
