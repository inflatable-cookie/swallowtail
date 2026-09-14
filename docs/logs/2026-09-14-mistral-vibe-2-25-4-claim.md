# 2026-09-14 Mistral Vibe 2.25.4 Claim

g05.073 advanced the exact `QualifiedOnly` Mistral Vibe headless point from
`2.24.2` to official GitHub/PyPI `2.25.4` with the unchanged
`mistral-vibe.headless.stdio-streaming-v1` behavior revision and one
mechanical adaptation: the selected argv adds adapter-private
`--legacy-harness`, which has first upstream precedence and pins the legacy
Python harness deterministically against the `2.25.1` harness-rollout
machinery (flags, ambient GrowthBook rollout cache, native-module
availability). The internal Unified Harness module is bundled in GitHub
release zips from `2.25.1` and absent from every PyPI distribution; the
Unified Harness backend and `--smart-approve` stay unmapped.

Every other selected input is byte-stable across all nine tags: the
streaming print wire, turn-limit middleware, upstream programmatic tests,
public-history schema, builtin `plan` profile, trust and workdir authority,
missing-API-key failure, and the console script. GitHub-only `2.24.4`
remains a named PyPI packaging gap that is never qualified by adjacency.
Research 199's caller-decreasing `--max-turns` `1..=8` binding and Research
252's Plan-only profile boundary carry over unchanged.

No provider operation, prompt, login, credential use, installation, host
mutation, or downloaded-artifact execution occurred. PR review and merge
remain queue-owned.
