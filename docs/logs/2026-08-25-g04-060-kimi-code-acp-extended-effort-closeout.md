# 2026-08-25 g04.060 Kimi Code ACP Extended Effort Closeout

Status: complete
Owner: Tom
Milestone: g04.060
Cards: 167-169
Research: 207

## Outcome

Delivered catalogue-declared portable `xhigh` and `max` on route
`kimi-code.acp` for exact `0.29.0..=0.38.0` under existing
`kimi.acp.reasoning.declared-effort-v2`. Prepared capability admission and
option validation follow that behavior revision, so exact `0.28.1` legacy
boolean select rejects `xhigh|max` before host effects. Selection remains
new-session-only and requires current `thinking` snapshot membership plus
response `currentValue` confirmation. Foreign advertised rows may coexist;
they are not public selections. No segment split and no shared contract change.
PR 59 fast-forwarded the exact reviewed head `dc191750` to `main`.

## Evidence

- Research 207 promoted with exact source floor at `0.29.0`, preceding boundary
  `0.28.1`, and byte-identical ACP adapter construction through `0.38.0`
- focused fixtures cover advertised `xhigh|max`, narrow-snapshot rejection,
  foreign coexistence, legacy values, drift/malformed paths, and
  UnverifiedNewer visibility
- Kimi prepared guide and feature-matrix notes updated; Reasoning cell stays
  Yes

## Validation

- `cargo fmt -p swallowtail-adapter-kimi`
- `effigy validate:focused swallowtail-adapter-kimi`
- `effigy package:verify-affected swallowtail-adapter-kimi`
- remaining card-169 shared gates recorded in the worker PR

## Generation Boundary

g04.060 closes only this route-local family. g04 remains open for the next
per-route inventory reassessment unless the operator supplies a different
direction. Contract 029 currentness stays standing.
