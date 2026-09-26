# 357 Kimi Code Local Server 2.1.1 AmbientHost Claim

Status: promoted. Production claim edit.

Date: 2026-09-26

Authority: Q-004 B; Contracts 017, 023, 029, and 061; Research 282, 326, and
354; the `kimi-code.local-server` selection, prepared route, and
disabled-tool controls; the official npm and GitHub channels.

## Question

Does `kimi-code.local-server` qualify current official
`@moonshot-ai/kimi-code` `2.1.1` under Contract 023 `AmbientHost` after
Q-004 B, without pinning `disabled_tools`?

## Official latest

Re-probed npm `@moonshot-ai/kimi-code` `latest` and GitHub latest on
2026-09-26 before the claim and immediately before push. Both name `2.1.1`.
npm published `2026-09-24T07:27:15.480Z`. GitHub published
`2026-09-24T07:24:08Z`. `2.1.2` is unpublished. Research 354 identity,
blob ledger, and host observation stand. No prompt, login, catalogue, live
session, host update, or local-server start.

## Decision

**Compatible extension under Q-004 B.** Keep baseline `0.28.1` and
heartbeat-ping behavior. Raise the claim id from
`kimi.local-server.executable-window-5` to
`kimi.local-server.executable-window-6` because membership and newer-version
posture change. Restore `AllowUnverified`. Latest qualified is official
`2.1.1`.

Segments:

- keep the existing milestones through `0.35.0..=0.39.1`
- add maintained `0.40.0..=0.43.1` on the same heartbeat-ping revision
  (selected REST/WebSocket v2 unchanged; Bash `cwd` check gone and outside
  `AmbientHost` isolation)
- add maintained `2.0.0..=2.1.1` on the same revision (same-package
  major-line reset; selected REST/WebSocket v2 unchanged)

Gaps stay incompatible: unpublished `0.39.2` and `0.43.2`, the `1.x` hole,
and interior unpublished `0.40.2`, `0.41.1`, `0.42.1`, and `2.0.3`.
Synthetic later-stable `2.1.2` is `UnverifiedNewer`.

Do not pin `disabled_tools`. The route already maps that optional control
from `0.29.0`. Consumers who want no shell send exact name `Bash`. ACP and
headless stay untouched. Decoder specimens stay on
`kimi-local-server-0.28.1-0.29.0`.

Research 282 and 326's fail-closed reading is withdrawn. Their identity
ledgers stand.

## Sources

- Q-004 B in [questions](../knowledge/questions.md)
- frozen `crates/swallowtail-adapter-kimi/tests/fixtures/kimi-local-server-2.1.1/`
- [Research 354](./354-kimi-code-local-server-2-1-1-adaptation-ruling.md),
  [Research 282](./282-kimi-code-local-server-0-41-0-identity.md), and
  [Research 326](./326-kimi-code-local-server-0-43-0-containment.md)
- [Contract 023](../knowledge/contracts/023-harness-operation-isolation-and-native-boundary.md)
  and [Contract 029](../knowledge/contracts/029-interface-version-qualification-and-compatibility.md)
