# 2026-08-25 g04.064 Kimi Code Headless V2 Closeout

Status: complete; PR pending orchestrator re-review
Owner: Tom
Milestone: g04.064
Cards: 179-180

## Result

Research 211 admits an adapter-private milestone. Exact official
`@moonshot-ai/kimi-code@0.38.0` default agent-core-v2 `runV2Print` stream-json
qualifies under `kimi.headless.stream-json.v2` behind the existing public
`kimi-code.headless` structured-run lifecycle.

Card 179 froze exact identity, selected v2 source hashes, and a secret-free v2
decoder corpus with source-proved `system.version`, assistant, tool, retry, and
resume-hint shapes. Card 180 split the headless claim: `0.29.0..=0.37.2`
remains `kimi.headless.stream-json.v1` as `Deprecated`; exact `0.38.0` is
`kimi.headless.stream-json.v2` as `Maintained`. Public facade stays
`kimi-headless-stream-json-v1`; v2 preamble and revision are enforced at runtime.
Synthetic `0.38.1` stays permitted `UnverifiedNewer` on the v2 revision.

`prompt-render.ts` is byte-identical `0.37.2..=0.38.0`. v2 prepends
`system.version` meta before shared `PromptJsonWriter` output. Provider-error
role lines, stderr ordering, goal exit codes, and background-policy timing
remain withheld without live provider work.

ACP and local-server `0.38.0` qualifications are unchanged. Historical Research
179 and 210 text is intact; current surfaces link the correction lineage.

## Validation

- `cargo fmt --all -- --check` — passed
- `cargo fmt -p swallowtail-adapter-kimi` — passed
- `effigy validate:focused swallowtail-adapter-kimi` — passed
- `effigy package:verify-affected swallowtail-adapter-kimi` — passed
- `effigy qa:routes` — passed
- `effigy qa:northstar` — passed
- docs index and next-action gates — passed
- `git diff --check` — passed
- `headless_structured_run.rs` split: v1 module 259 lines; v2 corpus 188 lines

## Continuation

Next Task reassesses g04.063 blocked cards 177-178 for headless reasoning
effort. Contract 029 currentness stays standing. g04 remains open at operator
direction.

## Merge

None. Merge authority remains with the operator/orchestrator after PR review.
