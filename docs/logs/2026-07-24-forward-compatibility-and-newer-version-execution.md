# Forward Compatibility And Newer-Version Execution

Date: 2026-07-24
Cards:
`../roadmaps/g01/batch-cards/124-forward-compatibility-policy-correction.md`,
`../roadmaps/g01/batch-cards/125-unverified-newer-core-assessment.md`,
`../roadmaps/g01/batch-cards/126-codex-opencode-newer-version-dispatch.md`,
`../roadmaps/g01/batch-cards/127-forward-compatibility-conformance-and-closeout.md`

## Outcome

Qualified version windows now mean guaranteed tested support, not a hard upper
execution ceiling.

- each ordered claim explicitly chooses qualified-only or unverified-newer
  posture
- `supports` remains the qualified-support answer
- `permits` includes an exact stable newer version only when the claim opts in
- unverified evidence carries the exact version, latest-qualified boundary,
  and latest qualified private behavior revision
- installed discovery returns newer stable executables as discovered but
  unverified
- preflight retains the exact version and permits the forward attempt
- consumer warning, confirmation, or rejection policy remains downstream

Codex exec and app-server now permit stable releases above `0.145.0` through
their latest qualified private behavior. OpenCode HTTP permits stable releases
above `1.18.4` through surface 18. Synthetic `0.146.0` and `1.18.5` fixtures
prove these paths without adding either release to guaranteed support.

Below-baseline versions, in-range gaps, exact exclusions, prereleases,
malformed values, missing or ambiguous bindings, qualified-only claims, and
runtime version drift remain incompatible. No provider, model, endpoint,
credential, route, or topology fallback was added.

## Validation

- focused core, testkit, Codex, and OpenCode conformance: 209 passed; one gated
  OpenCode probe skipped
- full repository QA: 583 inventoried, 579 passed, four gated probes ignored
- workspace all-target check, warnings-denied clippy, formatting, docs, and
  `git diff --check`: passed
- doctor: inherited 19 findings, 12 warnings and seven errors

The first doctor pass rose to 22 findings. Compatibility assessment, discovery
records, and Codex discovery coverage were split or consolidated, restoring
the inherited count without changing behavior.

## Risks

- a newer upstream release may break despite passing version assessment;
  runtime protocol validation remains the stopping boundary
- unverified execution is not a compatibility promise and must stay visible to
  consumers
- only Codex and OpenCode currently opt in; other claims remain
  qualified-only
- opaque interfaces cannot infer a newer ordering

## Continuation

- roadmap 041 and cards 124-127 are complete
- roadmap 042 and card 128 resume provider-coverage selection
- Kimi Code remains a candidate, not a selected policy
- card 128 may compare Kimi range evidence with a materially different missing
  transport or lifecycle proof
