# g04.021 Unmarked Overlay Classification

Date: 2026-08-21
Roadmap: `../roadmaps/g04/021-unmarked-overlay-rows.md`
Card: `../roadmaps/g04/batch-cards/059-unmarked-overlay-classification.md`

Worker worktree: `/Users/tom/.t3/worktrees/swallowtail/t3code-9c11615a`
Worker branch: `t3code/g04-021-unmarked-overlay-rows`

## Current Overlay Keying

`OverlayMarker` requires a `ProviderId`. `apply_model_presentation_overlay`
matches `(provider_id, model_id)` and only admits catalogue rows that
already report a provider id. Rows that omit `provider_id` still project
as overlay entries, but they cannot receive hide, ordinal,
consumer-default, or favourite. A marker that invents a provider id fails
as `UnknownModel`. Overlay copies 047 `Ready` / `NotReady` and still
refuses cross-instance copy.

## Unmarked Addable Catalogues

| Route | Catalogue identity | Current overlay |
| --- | --- | --- |
| Codex app-server | `ModelCatalogEntry` omits `provider_id` | unmarked; invented `codex` fails |
| Claude Agent ACP | lifecycle 047 snapshot has no catalogue; session-negotiated ACP models omit `provider_id` and are not 047 catalogue rows | empty overlay; invented `claude-agent` fails |
| Ollama attach | `ModelCatalogEntry` omits `provider_id` | unmarked; invented `ollama` fails |
| llama.cpp attached | `ModelCatalogEntry` omits `provider_id` | unmarked; invented `llama-cpp` fails |
| Anthropic Messages | `provider_id` `anthropic` | instance + provider + model |
| DeepSeek continuation | `provider_id` `deepseek` | instance + provider + model |

047 already rejects duplicate `(provider id, model id)`, including two
rows that both omit provider id and share a model id. Contract 020 keeps
session-negotiated models off the standalone catalogue.

## Chosen Rule

Instance-plus-model keying when `provider_id` is absent. Rows that report
a provider id still key instance, provider, and model. Overlay must not
invent a catalogue `provider_id`. Mixed gateway rows stay consumer
assembly of several catalogues. 047 `Ready` / `NotReady` is unchanged.

## Gateway Flattening

Named. Not triggered.

Contract 057 already forbids flattening gateway or cross-provider models
into another instance's catalogue. Overlay already fails closed on
cross-instance markers. Instance-plus-model keeps configured-instance in
the key, so a consumer assembling several catalogues (the mixed-gateway
shape) cannot share a marker.

The stop case would be one catalogue instance reporting mixed-provider
rows with the same model id and omitted `provider_id`. Overlay must not
invent a provider id to disambiguate that. A multi-provider gateway has
to report provider ids on the catalogue; 047 uniqueness already treats
`(None, model)` as one identity. None of the four unmarked addable
catalogues mix providers in one instance.

Claude Agent session-negotiated models stay outside this overlay until
they appear as 047 catalogue rows. Overlay still cannot invent those
rows.

## Next

Card 060 realizes instance-plus-model keying. Classification is not
forked.
