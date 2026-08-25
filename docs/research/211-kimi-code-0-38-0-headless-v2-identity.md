# 211 Kimi Code 0.38.0 Headless V2 Identity

Status: promoted
Owner: Tom
Created: 2026-08-25
Updated: 2026-08-25
Card: g04.064 / 179-180

## Question

Can exact official `@moonshot-ai/kimi-code@0.38.0` default
agent-core-v2 `runV2Print` stream-json behavior be qualified behind the
existing public `kimi-code.headless` structured-run route under a new
adapter-private revision?

## Remaining Rank

At observation on 2026-08-25, npm `latest` remained `0.38.0`. Headless
qualified ceiling was `0.37.2` under `kimi.headless.stream-json.v1`. Exact
`0.38.0` was visible `UnverifiedNewer` because naked `kimi -p` defaults to v2
unless `KIMI_CODE_LEGACY_FLAG` is truthy. Swallowtail does not set that flag.

## Method

Re-observed npm `@moonshot-ai/kimi-code@0.38.0` on 2026-08-25. Downloaded the
official tarball to disposable `/tmp` for digest cross-check. Inspected exact
public GitHub source at commit `0999454bdcb5ddd98f39bffee434dcf0a810f394` for
engine routing, `run-v2-print.ts`, and shared `prompt-render.ts`. Compared
selected JSONL grammar, stderr hooks, terminal contract, retry meta, tool
activity, and decoder assumptions against the existing v1 Swallowtail mapping.
Added a secret-free v2 corpus under
`crates/swallowtail-adapter-kimi/tests/fixtures/kimi-code-0.38.0-headless-v2/`.
No Kimi install, live prompt, OAuth, credential capture, or paid inference.

## Identity

| Surface | Observation |
| --- | --- |
| npm `latest` | `0.38.0` on 2026-08-25 |
| npm tarball SHA-256 | `d5c047dbfbbdfddf8d20030327e723ea9121af66260983a8556124580d64b549` |
| npm integrity | `sha512-O/z6sfjFdoDPPeTnoXzdsJ2U8IqP6K2gD3LsT+Nu8BAlHwdhCjdCQFkFTjIbLBun+aZT6x81ha5FiFt7trEilg==` |
| tag commit | `0999454bdcb5ddd98f39bffee434dcf0a810f394` |
| default headless dispatch | `runV2Print` when `!KIMI_CODE_LEGACY_FLAG` |
| legacy headless dispatch | v1 print only when `KIMI_CODE_LEGACY_FLAG` truthy |

## Selected Protocol

v2 `runV2Print` subscribes to agent-core-v2 `IEventBus` events and feeds the
same `PromptJsonWriter` / `PromptTranscriptWriter` classes as the legacy v1
print path. `prompt-render.ts` is byte-identical between `0.37.2` and `0.38.0`.

Source-proved stream-json shapes:

- preamble `{"role":"meta","type":"system.version","version":"<cli version>"}`
  emitted by `writeExperimentalVersion` before turn output
- assistant deltas flushed as `role":"assistant"` with optional `tool_calls`
- tool results as `role":"tool"` with `tool_call_id` and string `content`
- retry as `role":"meta","type":"turn.step.retrying"` with the existing v1 field
  names (`failed_attempt`, `next_attempt`, `max_attempts`, `delay_ms`,
  `error_name`, `error_message`, optional `status_code`)
- terminal `role":"meta","type":"session.resume_hint"` with `session_id`,
  `command`, and `content`

Fixture-only or withheld without a live prompt:

- `role":"error"` provider-failure lines project as namespaced unknown-role
  activity; terminal classification without `session.resume_hint` stays on exit
  and incomplete-stream evidence
- stderr config-warning ordering and content beyond static source inspection
- goal-mode exit codes and background `exit` / `drain` / `steer` timing
- authenticated retry/tool payloads beyond structurally empty specimens

Unmapped extras remain unmapped: reasoning effort env, experimental flag,
`KIMI_CODE_LEGACY_FLAG`, goal argv, and sibling-route controls.

## Decision

Adapter-private milestone behind the existing public structured-run lifecycle.

- keep `0.29.0..=0.37.2` qualified under `kimi.headless.stream-json.v1` as
  `Deprecated`; exact `0.38.0` is `Maintained` under
  `kimi.headless.stream-json.v2`
- keep public facade `kimi-headless-stream-json-v1`; enforce v2 preamble and
  revision at runtime without a new public facade identity
- keep synthetic `0.38.1` permitted `UnverifiedNewer` on the v2 revision
- card 180 applies the production claim edit

No new public operation, driver identity, or shared contract change is
required. Terminal success still requires `session.resume_hint` after a
successful exit; interruption and non-zero exit semantics stay on the existing
pump/terminal contract.

## Sources

- npm `@moonshot-ai/kimi-code@0.38.0`
- [Research 179](./179-kimi-code-0-38-0-identity.md)
- [Research 210](./210-kimi-code-headless-reasoning-effort-evidence.md)
- `apps/kimi-code/src/cli/prompt-render.ts` @ `0.38.0`
- `apps/kimi-code/src/cli/v2/run-v2-print.ts` @ `0.38.0`
- `apps/kimi-code/src/cli/experimental-v2.ts` @ `0.38.0`
- fixture corpus `kimi-code-0.38.0-headless-v2`
