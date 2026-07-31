# Currentness Evidence And Gap Classification

Date: 2026-07-31

## Outcome

g03 card 002 revalidated the bounded installed-harness source set. Research
074 now records exact current releases, tagged commits, selected source
comparisons, classifications, and a ranked card 003 candidate set.

## Classification

- Codex `0.146.0` and stable ACP v1/schema `v1.20.0` remain current and need
  no implementation
- Claude Agent `0.62.0..=0.64.0` contains qualified-range candidates with
  distinct tool, subagent, steering, and elicitation milestones
- Gemini CLI `0.53.0` is a compatible-extension candidate for its separate
  ACP and headless claims
- Pi `0.83.0` and Qwen `0.21.2` need provider-specific milestone corpora
  before range qualification
- Pi session continuity remains externally blocked because its cwd-binding
  source is unchanged
- OpenCode remains at qualified `1.18.10`; only its optional live selector is
  stale

No selected source requires a new exclusion, hard upper denial, architecture
change, or contract delta. Unqualified stable points remain visibly
`UnverifiedNewer`.

## Evidence Boundary

Evidence came from official Codex documentation, maintained GitHub releases
and tagged source, npm registry metadata, and crates.io metadata. No installed
executable, authentication, provider request, model invocation, consumer
repository, or production range changed.

## Validation

- source and publication metadata cross-check — passed
- `effigy qa:docs` — passed
- `effigy qa:northstar` — passed
- `git diff --check` — passed

## Next

Execute g03 card 003. Select one coherent fixture-first maintenance tranche
from Research 074's ranked candidates.
