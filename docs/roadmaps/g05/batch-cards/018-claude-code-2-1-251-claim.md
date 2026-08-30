# 018 Claude Code 2.1.251 Claim

Status: completed
Owner: Tom
Milestone: [g05.005 Claude Code 2.1.251 Useful Newer](../005-claude-code-2-1-251-useful-newer.md)
Created: 2026-08-30
Depends on: card 017 with an admitted segment shape

## Task

Apply card 017's Contract 029 disposition to the separate Claude Code headless
and response-only axes. For compatible-extension, raise both qualified ceilings
from `2.1.241` through official `2.1.251` without mapping watcher behavior.

## Edit Set

For compatible-extension:

- keep claim ids `claude-code.headless.window-1` and
  `claude-code.response-only.window-1`
- keep `AllowUnverified`, baselines `2.1.220` and `2.1.227`, and the existing
  headless/response-only stream-JSON behavior revisions
- set both latest-qualified constants to `2.1.251`
- qualify published `2.1.242`, `2.1.243`, `2.1.245`, `2.1.246`, `2.1.247`,
  `2.1.248`, `2.1.250`, and `2.1.251`
- keep unpublished `2.1.244` and `2.1.249` incompatible
- use unpublished `2.1.252` as the synthetic later `UnverifiedNewer` point
- keep response-only exclusions and historical decoder specimens
- keep maximum-turn and other feature-specific exact version sets unchanged
  unless card 017 proves that their existing evidence already includes the
  added points

Update the Claude prepared guide, route and feature matrices, `CHANGELOG.md`,
Research 261, identity and claim logs, roadmap/log/research indexes, and the
g05 currentness closeout. Do not advertise watcher support. Leave the sole
generation pointer on the post-currentness card 010 reassessment.

## Acceptance

- exact published `2.1.251` is Qualified Maintained on both axes
- all named published intermediates are qualified on the admitted revision
- unpublished `2.1.244` and `2.1.249` remain incompatible
- `2.1.252` remains permitted `UnverifiedNewer`
- base-route qualification does not widen watcher, maximum-turn, or unrelated
  feature claims
- no provider work, install, host mutation, or watcher wiring occurs
- milestone, cards, research, logs, matrices, and currentness surfaces agree

## Validation

```sh
cargo fmt -p swallowtail-adapter-claude-agent
effigy validate:focused swallowtail-adapter-claude-agent
effigy package:verify-affected swallowtail-adapter-claude-agent
effigy qa:routes
effigy qa:northstar
effigy qa:docs:index:research
effigy qa:docs:index:logs
effigy qa:docs:index:roadmaps
effigy qa:docs:index:roadmaps:g05
effigy qa:docs:index:roadmaps:batch-cards
effigy qa:docs:next-action:roadmaps
git diff --check
```

Do not run workspace `qa`, broad `qa:docs`, live probes, MSRV, or consumer
checks.

## Stop Conditions

- card 017 records stop or new-driver-or-facade
- applying the identity disposition needs a new contract or public operation
- qualification would silently widen a feature-specific exact version set
- the official point moves before the claim is complete

## Auto-Continuation

No. Return one PR. After merge, the orchestrator reassesses card 010 against
the landed `2.1.251` route qualification. No watcher wiring or live provider
work belongs to this lane.
