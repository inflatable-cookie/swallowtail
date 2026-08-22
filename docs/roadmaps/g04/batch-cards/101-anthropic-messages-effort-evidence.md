# 101 Anthropic Messages Effort Evidence

Status: ready
Owner: Tom
Created: 2026-08-22
Milestone: [g04.037 Anthropic Messages Effort](../037-anthropic-messages-effort.md)
Depends on: Research 004, 067, and 169

## Goal

Freeze exact current official evidence for Messages `output_config.effort` and
define the smallest model, value, and operation-profile subset that can map to
portable reasoning selection without a live provider request.

## Method

1. Freeze current official Messages request documentation, model support,
   effort values, defaults, exclusions, and interaction with `thinking` and
   `max_tokens`.
2. Record exact URLs, retrieval date, stable specimens, digests, and the
   existing `anthropic.messages` facade identity. Use official sources only.
3. Recheck every currently admitted Anthropic Messages model route. Do not infer
   support from provider family, catalogue presence, or another Claude product.
4. Classify `low`, `medium`, `high`, `xhigh`, and `max` separately. Unknown,
   deprecated, preview-only, model-ineligible, or account-specific values remain
   evidence-gated or withheld.
5. Classify one-attempt inference and direct-continuation sessions separately,
   including every-attempt encoding and fixed-session replay requirements.
6. Decide whether exact deliver-now values map to portable
   `ReasoningSelection` under Contract 040. Keep the provider field and model
   allowlist adapter-owned.
7. Freeze deterministic request specimens and write/index promoted Research
   185 with deliver-now, evidence-gated, withheld, not-applicable, and obsolete
   dispositions.

Do not authenticate, inspect an account, send a prompt, or infer effective
effort from response text.

## Acceptance Criteria

- exact current official evidence is frozen without secrets
- every admitted model, value, and profile has an explicit disposition
- `output_config.effort` is separated from `thinking`, Ultracode, Fast mode,
  Claude Code, and Managed Agents controls
- portable mapping and claim bounds are explicit
- Research 185 is promoted and indexed
- production code, claims, matrices, architecture, and changelog are unchanged
- `effigy validate:focused swallowtail-adapter-anthropic` passes
- `effigy qa:northstar` and `effigy qa:docs:index:research` pass
- `git diff --check` passes

Auto-continue to card 102 only when at least one exact model/value/profile
combination is deliver-now without a contract or facade-revision gap.

## Stop Conditions

- current evidence cannot identify an exact supported model/value subset
- effort requires an unresolved `thinking` interaction or new facade revision
- session support cannot keep one value fixed across every attempt
- mapping needs raw provider strings or a generic control map

## Out Of Scope

- production binding or dispatch
- Messages thinking, web-search version updates, or another feature family
- live provider, account, model catalogue, or currentness work

