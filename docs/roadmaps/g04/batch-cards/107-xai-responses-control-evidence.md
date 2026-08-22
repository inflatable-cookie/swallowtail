# 107 xAI Responses Control Evidence

Status: complete
Owner: Tom
Created: 2026-08-22
Milestone: [g04.039 xAI Responses Reasoning And Output Bounds](../039-xai-responses-reasoning-output-bounds.md)
Depends on: Research 004, 067, and 169

## Goal

Freeze exact current xAI Responses WebSocket reasoning and output-bound evidence
and define the smallest model/value/profile subset that fits the existing
facade and lifecycle truth.

## Method

1. Recheck official WebSocket mode, Responses request semantics, reasoning,
   current model pages, release notes, and output-bound examples.
2. Freeze exact URLs, retrieval date, stable secret-free specimens, digests,
   and the current facade identity without live provider work.
3. Classify exact model ids and aliases. Treat catalogue visibility as evidence,
   not inference-route qualification.
4. Classify `low`, `medium`, `high`, and `xhigh` per exact model. Record defaults,
   unsupported values, alias behavior, and multi-agent agent-count semantics
   separately.
5. Classify `max_output_tokens` independently for one-response runs and serial
   connection-local turns, including its positive domain and whether a model's
   lack of an intrinsic output limit changes only effectiveness wording.
6. Revalidate first turn, later turn with `previous_response_id`, failed turn,
   connection loss, and fresh replacement. One admitted selection must stay
   fixed through the operation.
7. Decide whether the current facade and Contracts 037/040 remain sufficient.
   Stop on a required revision, contract change, or empty useful subset.
8. Replace the pre-indexed Research 187 reservation with explicit deliver-now,
   evidence-gated, withheld, not-applicable, and obsolete rows. Do not edit the
   shared research index.

## Acceptance Criteria

- [x] current official evidence is frozen without secrets or provider output
- [x] models, aliases, values, controls, profiles, and continuation shapes have
      explicit dispositions
- [x] WebSocket-versus-Responses-body equivalence is explicit for each control
- [x] multi-agent effort is excluded from portable reasoning depth
- [x] current facade and contract decisions are explicit
- [x] Research 187 is promoted and indexed
- [x] production code, claims, matrices, architecture, and changelog are unchanged
- [x] `effigy validate:focused swallowtail-adapter-xai` passes
- [x] `effigy qa:northstar` and `effigy qa:docs:index:research` pass
- [x] `git diff --check` passes

Auto-continue to card 108 only when at least one useful exact control extends
the current facade without a contract or compatibility-segment change.

## Stop Conditions

- official current behavior requires a new facade revision or contract change
- no exact useful model/value/profile/control row survives
- WebSocket requests cannot carry the same control semantics as Responses create
- selection cannot remain fixed through connection-local continuation
- safe binding needs aliases, raw strings, or client-side output truncation

## Out Of Scope

- production binding or dispatch
- search, tools, multi-agent, Grok Bot, Grok Build, or another xAI route
- live provider, account, billing, release, or publication work
- shared research/log/roadmap indexes and orchestrator closeout surfaces

## Closeout

Research 187 promotes exact Grok 4.5 low/medium/high and Grok 4.6
low/medium/high/xhigh reasoning on the existing dated WebSocket facade. It
also promotes the positive `max_output_tokens` int32 range independently for
the structured-run and serial-session profiles. Grok 4.5 xhigh, aliases,
other models, and multi-agent effort remain withheld. Card 108 consumed only
these rows.
