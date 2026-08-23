# 2026-08-23 g04.052 Mistral Vibe Maximum Turns Closeout

Status: worker complete; not merged
Owner: Tom
Milestone: g04.052

## Result

Research 199 admits a non-empty deliver-now set: exact Mistral Vibe `2.24.2`
on route `mistral-vibe.headless` exposes adapter-local caller-decreasing
`--max-turns` values `1..=8`. Caller omission keeps the current argv byte `8`.
The flag is always emitted; upstream unbounded omission stays forbidden.

`--max-turns N` is a per-child cap on completed assistant LLM turns inside one
`vibe --prompt` process. It is not a Contract 040 output-token limit, a generic
budget, or proof that the provider completed less work. Zero, negatives,
fractions, overflow, and values above eight stay withheld or unconstructable.
Zero is truthful upstream (STOP before the first assistant LLM) and not useful
on this one-prompt print route.

Native limit terminal stays process exit `1` plus stderr containing
`The configured conversation limit was reached`, mapped to
`swallowtail.mistral-vibe.headless.max_turns` / provider-failed, not
`Completed`. Last-assistant-text stderr is not promoted into that stronger
diagnostic. Cancellation and host deadline stay distinct.

Cards 145-147 bind that subset through `MistralVibeMaxTurns` on run-profile
input, prepared run, driver, and argv. No shared `Capability`, OperationPolicy
field, or Contract 029 currentness change landed here. The route remains exact
`2.24.2` only.

Worker validation passed: focused package validation (31 tests), affected-
package verification, examples, route QA, Northstar QA, research/log/roadmap/
g04/batch-card index QA, next-action QA, package API, and `git diff --check`.
Review requested a split of the new maximum-turn proofs out of
`tests/prepared_facade.rs`; they now live in
`tests/prepared_facade/max_turns.rs`. `effigy doctor` after that split
reproduces the inherited baseline: 376 god-file findings (330 warn / 46 err)
plus generated-in-src. No credentials, account state, live provider request,
or paid work was used.

PR: https://github.com/inflatable-cookie/swallowtail/pull/51
Implementation commit: `cb04d1e24d8c5a5e334711e573c3539b0205d21c`
This closeout does not claim merge.

## Shared Closeout Delta

Reserved for the orchestrator after review and merge:

- architecture and route/feature matrices record only the exact delivered
  adapter-local Mistral Vibe maximum-turn truth (`1..=8`, omission keeps `8`,
  per-child completed assistant LLM turns, native limit is provider-failed)
- Contract 029 exact `2.24.2` membership and qualified-only posture remain
  unchanged
- programme and research/log/roadmap/g04/batch-card indexes reconcile Research
  199, cards 145-147, and g04.052 at the merged head
- `CHANGELOG.md` records the unreleased `swallowtail-adapter-mistral-vibe`
  `MistralVibeMaxTurns` public API
- the sole Next Task returns to remaining per-route inventory reassessment
- g04 remains active and unrolled until explicit operator direction
