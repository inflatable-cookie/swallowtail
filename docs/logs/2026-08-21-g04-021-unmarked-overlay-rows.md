# g04.021 Unmarked Overlay Rows

Date: 2026-08-21
Roadmap: `../roadmaps/g04/021-unmarked-overlay-rows.md`
Cards: `../roadmaps/g04/batch-cards/059-unmarked-overlay-classification.md`,
`../roadmaps/g04/batch-cards/060-unmarked-overlay-rule.md`,
`../roadmaps/g04/batch-cards/061-unmarked-addable-overlay-proof.md`

Worker worktree: `/Users/tom/.t3/worktrees/swallowtail/t3code-9c11615a`
Worker branch: `t3code/g04-021-unmarked-overlay-rows`

## Result

Overlay keys instance plus model when a catalogue row omits
`provider_id`. Rows that report a provider id still match that provider
id. Overlay does not invent a catalogue provider id. Mixed gateway rows
stay consumer assembly of several catalogues. 047 `Ready` / `NotReady`
is copied unchanged.

`OverlayMarker::without_provider` is the unmarked constructor.
`provider_id()` now returns `Option<&ProviderId>`. JSON-file store omits
`provider_id` for unmarked markers. Codex, Ollama, and llama.cpp attached
catalogue rows can receive instance-plus-model markers. Claude Agent ACP
still has no 047 catalogue; instance-plus-model cannot invent those
rows, and an invented `claude-agent` provider id still fails closed.
Anthropic and DeepSeek still key `anthropic` / `deepseek`.

Additive API is in `public-api-unreleased` for core. `public-api-0.3.3`
is unchanged.

## Validation

- `effigy qa:docs:index:logs` — passed.
- `effigy validate:focused swallowtail-core swallowtail-runtime
  swallowtail-testkit` — 395 tests passed.
- `effigy validate:focused swallowtail-adapter-codex
  swallowtail-adapter-claude-agent swallowtail-adapter-ollama
  swallowtail-adapter-llama-cpp` — passed.
- `effigy validate:focused swallowtail-host-local` — 48 tests passed.
- `effigy package:api` — passed; only `public-api-unreleased` changed.
- `git diff --check` — passed.

## Next

Open the worker PR against `main`. Do not merge. g04.022 stays planned.
Hosted OAuth stays parked.
