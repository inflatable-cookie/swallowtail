# 027 Second Installed Harness Range Selection

Status: promoted
Owner: Tom
Updated: 2026-07-23

## Question

Which remaining installed production harness should receive the second
maintained compatibility-range retrofit?

## Method

Sources were accessed 2026-07-23. Evidence includes maintained release records,
tagged source, generated schemas, official documentation, existing Swallowtail
fixtures, and local installed-version evidence.

No login, credential, provider request, installation, update, container,
consumer edit, or live model call was used. Exact executable or server release,
wire protocol, generated schema, behavior revision, configured instance, model
catalogue, and support authority remain separate.

## Current Posture

| Route | Swallowtail point | Current stable | Exact observation | Range value |
| --- | --- | --- | --- | --- |
| OpenCode HTTP | installed and frozen `1.14.48` | `1.18.4`, 2026-07-20 | unauthenticated `GET /global/health`; session response corroborates | high |
| Gemini CLI ACP | executable `0.51.0`; ACP v1 | `0.52.0`, 2026-07-22 | bounded `--version`; exact ACP `agentInfo.version` | medium |
| Kimi Code ACP | executable `0.28.1`; ACP v1 | `0.29.0`, 2026-07-22 | bounded `--version`; exact ACP `agentInfo.version` | medium |
| Qwen Code headless | package `0.19.11` | `0.20.1`, 2026-07-21 | first `system/session_start` stream event | medium-low |
| Pi RPC | package `0.80.10` | `0.81.1`, 2026-07-21 | configured binding only | medium-low |

Every current stable release remains outside its existing exact-only or
one-point claim. Release recency does not promote support.

Gemini `0.52.0` changes harness behavior including Plan Mode policy. Its two
stable points make a useful later ACP range, but provide little intermediate
range evidence.

Kimi Code `0.29.0` adds ACP thinking-effort levels and v2-only agent or tool
gating while retaining legacy thinking on/off. That is a capability milestone,
not a safe exact-version widening. It needs an ACP capability corpus before it
can extend the current `0.28.1` route. Kimi Code continues to mean the
maintained TypeScript `MoonshotAI/kimi-code` line, not the separate Python
`kimi-cli`.

Qwen `0.20.1` reports no known breaking changes, but Swallowtail observes the
release only after process start and stream work. Pi `0.81.1` adds retry
lifecycle evidence and new RPC behavior while Swallowtail has no production
runtime-version observation. Both would mix range qualification with a new
observation boundary.

## OpenCode Evidence

OpenCode is an attached HTTP/SSE harness. It is materially different from the
Codex process-stdio proof:

- the execution host approves one endpoint rather than one executable
- the external server remains operator-owned
- exact server release is available from a pre-auth health request
- provider authentication and model catalogue remain delegated to OpenCode
- ordered SSE, abort, disconnect, and attached close remain route-specific

The maintained release record contains 45 stable releases from `1.14.48`
through `1.18.4`, published 2026-05-11 through 2026-07-20. At every point,
tagged `packages/sdk/openapi.json` retains the six selected operation ids and
paths:

- `global.health`: `GET /global/health`
- `provider.list`: `GET /provider`
- `session.create`: `POST /session`
- `session.prompt_async`: `POST /session/{sessionID}/prompt_async`
- `event.subscribe`: `GET /event`
- `session.abort`: `POST /session/{sessionID}/abort`

Route presence is not a compatibility claim. Card 121 recursively followed
every local JSON reference from the six selected operations at all 45 tags.
That closed evidence corrects the first-pass projection:

- 45 exact stable releases produce 18 selected-surface revisions
- preserving unpublished patch and cross-minor gaps produces 20 contiguous
  semantic-version segments
- `1.15.8` and `1.16.1` were not published and remain outside every segment
- `1.14.52`, `1.15.14`, `1.16.3`, and `1.17.21` prove that a cross-minor gap
  is not silently treated as a release
- full OpenAPI changes at `1.18.1` / `1.18.2` do not change the recursively
  selected closure

The frozen manifest records every tag commit, publication date, full OpenAPI
SHA-256, selected-surface SHA-256, component count, event-schema count, and
private behavior revision. Unknown semantic events still fail closed.

`1.14.47`, prerelease syntax, missing or malformed versions, an unhealthy
response, and a session-version mismatch are explicit rejection points. No
interval may be published until private dispatch and cross-topology
conformance pass.

Policy supersession, 2026-07-24: the evidence still does not qualify `1.18.5`.
Contract 029 and roadmap 041 now permit that ordered stable point as
unverified-newer rather than rejecting it solely for exceeding the qualified
ceiling.

## Selection

Select OpenCode HTTP with candidate envelope `1.14.48..=1.18.4`.

The baseline preserves the already frozen and locally observed release; it
does not remove older support because no older OpenCode release is currently
claimed. The latest boundary is current stable, not an open-ended promise.
Card 121 now freezes the 18 surfaces and 20 exact segments. Card 122 owns
private dispatch and production matching. Card 123 alone may publish the
closed range after conformance.

Contract 029 already governs exact bindings, closed claims, milestones,
exclusions, and fail-closed behavior. Contract 032 does not apply: OpenCode is
an attached server endpoint, not an installed executable selected through the
process host. Existing endpoint, lifecycle, configuration, and attached
HTTP/SSE contracts are sufficient. No shared contract change is required.

The next ACP range candidate should be Kimi Code `0.28.1` through `0.29.0`,
because the latest release exercises negotiated capability evolution. That is
a later selection, not part of the OpenCode claim.

## Risks

- tagged schema continuity does not prove runtime behavior without transcripts
- an attached endpoint may change between configuration and execution; health
  must match the exact preflight-bound version before authenticated catalogue
  or session work
- provider access, entitlement, model availability, and catalogue freshness
  are not implied by server-version compatibility
- `1.18.2+` adds semantic event variants; the existing unknown-event stop rule
  must not be weakened
- moving the `1.14.48` baseline later would be a separate support-policy change

## Sources

- [OpenCode server documentation](https://opencode.ai/docs/server/)
- [OpenCode releases](https://github.com/anomalyco/opencode/releases)
- [OpenCode `1.14.48` schema](https://github.com/anomalyco/opencode/blob/v1.14.48/packages/sdk/openapi.json)
- [OpenCode `1.18.4` schema](https://github.com/anomalyco/opencode/blob/v1.18.4/packages/sdk/openapi.json)
- [Gemini CLI `0.52.0`](https://github.com/google-gemini/gemini-cli/releases/tag/v0.52.0)
- [Kimi Code `0.29.0`](https://github.com/MoonshotAI/kimi-code/releases/tag/%40moonshot-ai%2Fkimi-code%400.29.0)
- [Qwen Code `0.20.1`](https://github.com/QwenLM/qwen-code/releases/tag/v0.20.1)
- [Pi `0.81.1`](https://github.com/earendil-works/pi/releases/tag/v0.81.1)
