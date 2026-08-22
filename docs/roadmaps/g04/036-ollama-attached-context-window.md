# g04.036 Ollama Attached Context Window

Status: complete
Owner: Tom
Created: 2026-08-22
Depends on: completed g04.035; per-route feature completion programme
Vision tags: explicit selection, provider truth, route-local controls
Contract refs: 020, 029, 031, 037, 040, 052
Research: 024, 035, 138, 174; 184 to be produced by card 098

## Problem

`ollama.attached` sends native `/api/chat` requests with explicit output and
reasoning controls, but it cannot select Ollama's request-local `options.num_ctx`
context window. Consumers therefore cannot make an exact local-runtime context
choice through the prepared facade.

`num_ctx` is an Ollama runner option with load, memory, truncation, and model
limit consequences. It is not a portable context-window capability, an output
token limit, or catalogue evidence. The milestone needs exact local-runtime
evidence and an adapter-local binding that does not claim the selected value was
accepted or applied effectively.

## Generation Runway Goal

Deliver the second route-local control family from the per-route feature
programme: typed Ollama native context-window selection with exact prepared
evidence and wire dispatch on only the qualified `/api/chat` profiles.

## Goals

- [x] freeze exact qualified-source and current official evidence for
      `options.num_ctx`
- [x] classify structured inference and interactive replay separately
- [x] define one useful positive numeric domain that fails closed before
      network work
- [x] add an adapter-local typed input only to the profiles qualified by the
      evidence card
- [x] retain the exact selection in prepared evidence and configure prepared
      dispatch from the same prepared object
- [x] encode `num_ctx` once beside `num_predict` in the native `options` object
- [x] preserve byte-identical request bodies when the control is absent
- [x] publish qualified dispatch without claiming provider acceptance,
      effective allocation, or resource feasibility

## Non-Goals

- a provider-neutral context-window capability or constraint
- a generic Ollama options map
- deriving a selectable value from model catalogue metadata
- inferring model memory fit, training context, truncation, or effective value
- Ollama Cloud, OpenAI-compatible endpoints, `/api/generate`, or embeddings
- server defaults, `OLLAMA_CONTEXT_LENGTH`, Modelfile mutation, or `ollama run`
- runtime start, stop, pull, unload, residency duration, or capacity control
- changing the Ollama Contract 029 ceiling
- another sampling, generation, or route feature

## Named Scope

The milestone is restricted to the existing local, remote-model-rejecting
`ollama.attached` native facade and its qualified `ollama.runtime` window:
`0.14.0..=0.32.15`, excluding `0.32.2` and `0.32.10`. Exact later stable
versions retain the existing visible unverified-newer posture and latest
qualified mapping; this milestone does not widen that guarantee.

Card 098 must verify `num_ctx` at the exact tagged qualification points and
settle structured-run versus interactive-session applicability. Card 099 may
implement only the resulting Research 184 deliver-now rows. The selected value
stays adapter-local in prepared evidence. Prepared dispatch configures the
low-level driver from the same prepared object; generic role dispatch exposes
`with_context_window` as caller authority and does not claim automatic
prepared-evidence agreement. The provider-neutral plan continues to advertise no
portable context-window capability.

The route claims only that the exact integer was dispatched inside the native
`options` object. Ollama may cap, raise, reject, or resource-limit the value.
Those outcomes remain provider/runtime truth.

## Execution Plan

### Batch 36.1 — Exact Native Evidence

- [x] Execute card 098.
- [ ] freeze tagged-source and official-documentation specimens
- [ ] promote Research 184 with numeric, profile, lifecycle, and claim bounds

### Batch 36.2 — Typed Binding

- [x] Execute card 099 after card 098.
- [ ] add the adapter-local value to only the admitted prepared profiles
- [ ] bind prepared evidence, low-level driver, and native request exactly
- [ ] preserve the absent path and fail before network work

### Batch 36.3 — Dispatch And Acceptance

- [x] Execute card 100 after card 099.
- [ ] prove exact native bodies and bounded lifecycle behavior
- [ ] update realized architecture, guides, matrices, programme, and changelog

## Acceptance Criteria

- [ ] consumers never construct a raw Ollama options map
- [ ] only the Research 184 numeric domain and operation profiles prepare
- [ ] `num_ctx` and `num_predict` remain independent exact integers
- [ ] prepared evidence exposes the selected value before effects
- [ ] the prepared bound driver dispatches that same value
- [ ] no portable context capability or provider-neutral alias is added
- [ ] absent selection preserves existing structured and session request bytes
- [ ] local attached ownership and residency side-effect truth remain explicit
- [ ] docs separate dispatched, accepted, effective, and observed states
- [ ] deterministic QA starts no runtime and sends no model request

## Lane Runway

- previous: g04.035 Cursor headless model parameters — complete
- this milestone: Ollama attached `num_ctx`
- next: Anthropic Messages `output_config.effort`

## Decision Gates

- Stop if exact qualified tagged source does not preserve `num_ctx` on the
  selected native `/api/chat` surface.
- Stop if a useful positive domain cannot fail closed before network work.
- Stop if the control would require a generic options map or a portable context
  capability.
- Stop if the selected value cannot remain inspectable in prepared evidence and
  immutable in the bound driver through dispatch.
- Stop before adding interactive-session support unless Research 184 proves
  exact per-turn persistence, restoration, and absent-path behavior.
- Stop before claiming provider acceptance, effective context, model fit, or
  resource feasibility without an exact confirming surface.

## Batch Cards

- [098-ollama-num-ctx-evidence.md](batch-cards/098-ollama-num-ctx-evidence.md) — complete
- [099-ollama-num-ctx-binding.md](batch-cards/099-ollama-num-ctx-binding.md) — complete
- [100-ollama-num-ctx-acceptance.md](batch-cards/100-ollama-num-ctx-acceptance.md) — complete

## References

- [Per-Route Feature Completion Programme](./per-route-feature-completion.md)
- [Advanced Route Features](../../triage/2026-08-21-advanced-route-features.md)
- [Research 024 Post-Continuation Coverage And Ollama Selection](../../research/024-post-continuation-coverage-and-ollama-selection.md)
- [Research 035 Ollama Prepared Version Posture Delta](../../research/035-ollama-prepared-version-posture-delta.md)
- [Research 138 Ollama 0.32.14 Identity](../../research/138-ollama-0-32-14-identity.md)
- [Research 174 Ollama 0.32.15 Identity](../../research/174-ollama-0-32-15-identity.md)
- [Contract 037 Prepared Consumer Integration](../../contracts/037-prepared-consumer-integration.md)
- [Contract 040 Generation-Control Application And Enforcement](../../contracts/040-generation-control-application-and-enforcement.md)
- [Ollama Attached Prepared Integration](../../guides/ollama-attached-prepared-integration.md)
