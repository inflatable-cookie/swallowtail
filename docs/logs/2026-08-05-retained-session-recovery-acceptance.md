# 2026-08-05 Retained Session Recovery Acceptance

Roadmap: `../roadmaps/g03/037-retained-session-recovery-promotion.md`
Card: `../roadmaps/g03/batch-cards/101-retained-session-recovery-acceptance.md`

## Changed

- mapped the separate Alibaba retained conversation profile into
  `PreparedWorkingStateRestoration`
- selected `ProviderSessionContinuationRecovery` before provider work
- reused exact resource-free binding validation and the qualified bounded load
  path
- preserved exact interrupted-turn identity, complete replay, one live session,
  and separate management authority
- published Alibaba load and provider-session deletion as supported while
  retaining unsupported archive, restore, replay-free resume, and native close
- kept the ordinary Alibaba profile delete-on-close
- kept Pi RPC blocked because public session switching still cannot bind and
  corroborate effective cwd
- left Gemini ACP replay readiness and private headless continuation outside
  production recovery
- returned the sole roadmap pointer to the g03 evidence gate

## Validation

- `effigy validate:focused swallowtail-runtime swallowtail-adapter-pi swallowtail-adapter-alibaba-model-studio`
  — 216 tests passed; focused package checks passed
- `effigy package:verify-affected swallowtail-adapter-alibaba-model-studio`
  — extracted package compiled
- `effigy qa:docs` — passed
- `effigy qa:routes` — 32 routes, 25 solution rows, and 63 activity operations passed
- `cargo fmt --all -- --check`
- `git diff --check`

No authenticated provider work, external request, paid inference, conversation
mutation, or remote deletion ran.

## Next Move

Hold at the g03 evidence gate until a consumer-reproduced portable defect,
material non-deferred provider or interface drift, or explicit operator
promotion supplies the next roadmap input.
