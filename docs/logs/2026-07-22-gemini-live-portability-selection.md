# 2026-07-22 Gemini Live Portability Selection

## Changed

- Audited seventeen production drivers and eleven common profiles after the
  OpenAI Realtime closeout.
- Revalidated Gemini Live, Grok Build, remote ACP, Cursor, DeepSeek, Z.AI, and
  attached runtimes from current official or maintained sources.
- Selected Gemini Live raw server-to-server WebSocket as the next proof.
- Promoted Contract 027 and compiled roadmap 033 with cards 095-097.

## Decision

Gemini Live adds the most useful missing pressure: a provider-planned WebSocket
lifetime boundary where one live media session continues on a replacement
connection using a private resumption handle.

The first subset uses the provider-supported preview `v1beta` route, exact
`gemini-3.1-flash-live-preview`, one project-bound authorization API key,
project billing, explicit PCM formats, `Kore` voice, minimal thinking, manual
activity, two serial turns, and one idle-boundary rollover.

Rollover is opt-in and maximum one. It is not retry, unexpected reconnect,
stream reattachment, consumer resume, provider-managed recovery, or durable
state. Handles stay private and in memory. Cancellation closes transport and
does not claim provider stop.

## Evidence Delta

- Google now makes service-account-bound authorization keys the default and
  will reject standard Gemini API keys in September 2026. The selected access
  profile targets authorization keys only.
- ACP remote transport moved from Draft to Active, but its reference
  implementation, Rust and TypeScript SDK support, resumability, protocol
  header, origin validation, and security hardening remain incomplete.
- Cursor now exposes a public-beta TypeScript SDK for local or cloud harness
  execution. The local route repeats realized harness authority; the cloud
  route still requires repository and remote-mutation policy.
- Grok Build stable remains `0.2.106`; its useful auth, permission, persistence,
  and optional-sandbox boundaries are already represented.

## Deferred Routes

- Grok Build remains the next meaningful harness-breadth candidate.
- Remote ACP waits for maintained client and hardening evidence.
- Cursor Cloud Agents remains behind repository and remote-mutation authority.
- DeepSeek and Z.AI remain exact compatible-provider mappings.
- Additional attached runtimes remain behind information gain and serving-
  ownership checks.

## Boundaries

- Standard keys, ephemeral client tokens, browser media, automatic VAD,
  barge-in, tools, durable handles, unexpected reconnect, replay, retry, and
  fallback are excluded.
- Consumers retain capture, playback, conversion, pacing, privacy, and
  played-position truth.
- No live provider request, credential access, browser login, device, or paid
  inference occurred.

## Validation

- Documentation QA and `git diff --check` pass after promotion.
- Doctor remains at the inherited 19 findings: 12 warnings and seven errors.
  No finding was added by this documentation batch.

## Continuation

Card 095 is the sole ready task. Realize bounded-rollover records, pure
preflight, the assertion pack, and the frozen Gemini Live corpus before
production WebSocket work.
