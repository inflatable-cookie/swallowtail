# 164 Deep Agents ACP Model Selection Evidence

Status: ready
Owner: Tom
Created: 2026-08-24
Updated: 2026-08-24
Milestone: [g04.059 Deep Agents ACP Model Selection](../059-deepagents-acp-model-selection.md)
Depends on: Research 153, 157, 159; exact route `deepagents.acp`

## Goal

Freeze exact `deepagents-acp@0.1.25` model-selection behavior and define the
smallest provider, model, access, and lifecycle set that Swallowtail can
dispatch without fallback or credential-authority expansion.

## Method

1. Freeze current official ACP/model documentation with retrieval dates and
   complete specimen digests. Mark it current, not exact `0.1.25` evidence.
2. Re-acquire or verify the exact npm `0.1.25` tarball from Research 157.
   Freeze package integrity plus full digests for the CLI parser, model
   construction, provider integration, session construction, and relevant
   error paths.
3. Record exact `--model` flag syntax, aliases, repeat handling, missing-value
   behavior, normalization, whitespace, length, provider delimiter, accepted
   provider prefixes, model suffix handling, and omission default.
4. Trace requested value through parsing, configuration, agent/model creation,
   session start, and provider call construction. Distinguish requested,
   planned, dispatched, accepted, effective, and observed states.
5. Freeze unknown provider, unknown model, malformed value, missing provider
   package, missing/wrong key, and provider rejection. Prove whether any path
   substitutes an upstream default, alias, or alternate provider.
6. Map each candidate provider/model row to exact host-owned access evidence.
   Do not inspect, copy, inject, or persist key bytes. A generic access profile
   is insufficient if provider agreement cannot be checked before spawn.
7. Determine whether initialize, `session/new`, session updates, or terminal
   events identify the selected/effective model. If no wire confirmation
   exists, decide whether exact source can support a bounded dispatch-only
   claim without fallback.
8. Classify one child/session, turn reuse, cancellation, terminal failure,
   close, and fresh context-losing restoration separately. There is no
   load/resume claim.
9. Define the smallest adapter-local provider/model type only if exact evidence
   supports it. Do not promote a live catalogue or moving alias.
10. Replace Research 206's reservation with source-backed evidence and a
    deliver-now table or honest empty set.

Current official public docs, the exact public npm artifact/source, existing
fixtures, and secret-free local parser/unit probes are authorized. Do not
install or run the ACP server, authenticate, inspect host key values, send a
provider prompt, perform external inference, or incur paid work.

## Acceptance Criteria

- [ ] official and exact artifact/source specimens have identities and full
      digests
- [ ] exact parser, provider, model, default, alias, invalid, auth, and fallback
      paths are explicit
- [ ] requested, planned, dispatched, accepted, effective, and observed states
      remain distinct
- [ ] every candidate provider/model/access/lifecycle row has a disposition
- [ ] any non-empty set has pre-spawn access agreement and no silent fallback
- [ ] Research 206 is promoted with a non-empty exact set or honest stop
- [ ] no production code, public API, matrix, contract, or currentness change
- [ ] `effigy validate:focused swallowtail-adapter-deepagents` passes
- [ ] `effigy qa:northstar` and `effigy qa:docs:index:research` pass
- [ ] `git diff --check` passes

Auto-continue to card 165 only when Research 206 admits a non-empty exact set
with bounded syntax, provider/access agreement, and no model/provider fallback.

## Stop Conditions

- exact package source cannot be frozen or diverges from the selected identity
- provider/model syntax or auth agreement is unbounded before spawn
- selection can silently fall back or substitute another provider/model
- qualification requires server execution, login, key inspection, provider
  prompt, external inference, generic configuration, contract change, or
  currentness movement

## Out Of Scope

- production binding, another Deep Agents feature/route, release, merge,
  generation rollover, or g04 closure
