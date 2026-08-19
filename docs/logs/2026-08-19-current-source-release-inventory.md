# Current Source Release Inventory

Date: 2026-08-19
Roadmap: `../roadmaps/g04/003-current-source-tag-before-readiness.md`
Card: `../roadmaps/g04/batch-cards/006-current-source-release-inventory.md`

## Result

The selected next coordinated source version is compatible patch `v0.3.3`.
Immutable `v0.3.2` stays 30 packages and 36 production routes. Current source
is 40 packages and 47 production routes. OpenHands remains a package without a
production route. No Spec 011 facade types are in the candidate set.

Ten additive packages: DeepSeek Harness, ZCode, Cline, Goose, Copilot CLI,
Mistral Vibe, Qoder, OpenHands, Kiro, and Deep Agents. Eleven additive routes:
Cline ACP and headless, Goose ACP, Kiro ACP, Deep Agents ACP, Copilot CLI ACP,
Mistral Vibe headless, Qoder headless, DeepSeek Harness JSON-RPC and local
server, and ZCode app-server.

Existing-package semantic APIs: 27 identical to `v0.3.2`. Claude Agent, Cursor,
and Grok gained additive public items only. Zero removals, no MSRV raise, no
range shrink, no verified-target removal. Contract 036 therefore requires
`0.3.3`, not `0.4.0`.

`[Unreleased]` still omits DeepSeek Harness and ZCode. Card 007 must add those
entries before changelog promotion.

## Validation

- `effigy package:metadata` — 40 crates at `0.3.2` and Rust `1.95`; immutable
  `v0.3.2` baseline remains 30
- `effigy package:api` — 30 immutable `v0.3.2` packages plus 13 reviewed
  unreleased API surfaces after freezing Cursor and Grok overrides
- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy qa:routes` — 47 production routes
- `git diff --check`

## Authority

No workspace version, lockfile, changelog promotion, candidate, commit, push,
tag, consumer, registry, or provider state changed in this card. Card 007 may
prepare the local `0.3.3` candidate on the worker branch.
