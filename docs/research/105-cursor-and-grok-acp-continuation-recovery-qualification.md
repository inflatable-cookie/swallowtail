# 105 Cursor And Grok ACP Continuation Recovery Qualification

Status: promoted
Owner: Tom
Date: 2026-08-05

## Question

Can the qualified Cursor Agent ACP or Grok Build ACP route safely map its
advertised `session/load` capability into Contract 050 continuation recovery?

## Method

The pass inspected only exact already-qualified artifacts:

- Cursor installed `2026.07.01-41b2de7` source bundle
- Cursor official ACP-registry `2026.07.23-e383d2b` archive and source bundle
- Grok Build official npm platform artifacts `0.2.114..=0.2.117`
- existing stable ACP, session-load, route, and compatibility corpora

The Cursor archive and both selected source-chunk digests match the frozen
corpus. All four decompressed Grok executables match the frozen executable
digests. No executable was authenticated. No session, prompt, model request,
provider mutation, or workspace mutation ran.

## Cursor Result

Both exact Cursor source bundles implement the same load boundary:

- `sessionId`, requested `cwd`, and requested MCP servers build the attachment
- session initialization completes before retained replay
- `replayConversationHistory` is awaited before the load response

Ordering alone is insufficient. `replayConversationHistory` catches a failed
whole-conversation read, logs it, and returns normally. It also catches each
failed turn replay, logs it, and continues. The caller then returns a successful
load response. The ACP client receives no failure or completeness evidence.

This violates Contract 017's requirement that provider or protocol failure
during replay fail load and return no usable session. Swallowtail cannot repair
the omission because it cannot distinguish complete history from silently
truncated history at the wire.

Decision: `cursor-agent.acp` continuation recovery remains blocked.

## Grok Result

All four exact Grok artifacts advertise load. Their official platform packages
contain a compressed stripped native executable, notices, package metadata,
and no inspectable control-flow source or deterministic load transcript.

Normalized embedded diagnostics show replay completion draining and an
ordering assertion, but also show paths which:

- fall back to full replay when a cursor is absent
- skip unparseable ACP or JSONL replay records
- skip delta replay after a post-replay flush failure

Those surfaces do not prove what the client receives before the successful
load response or whether omitted history is reported as failure. Binary strings
cannot substitute for exact control flow or a deterministic transcript.

Decision: `grok-build.acp` continuation recovery remains blocked.

## Negative Coverage Disposition

Neither production driver can truthfully exercise foreign replay, early
response, replay-after-readiness, malformed identity, binding drift, overflow,
cancellation, disconnect, or joined-cleanup load cases because neither route
passes the admission gate. The frozen adapter corpora enumerate every case as
unqualified so later promotion must close them rather than inheriting another
ACP agent's tests.

## Contract Result

Contracts 017 and 050 remain unchanged. Capability advertisement is still
negotiation evidence, not complete replay evidence. No production driver,
prepared facade, route capability, or public API changed.

## Promotion Gates

Cursor needs an exact maintained implementation which propagates history-read
and replay failures through `session/load`.

Grok needs exact inspectable control flow or a deterministic authenticated
load corpus proving complete ordered replay, client-visible failure,
resource/binding agreement, bounds, cancellation, disconnect, and cleanup.

## Primary Sources

- Cursor ACP registry and exact `2026.07.23-e383d2b` archive
- exact qualified Cursor source chunks recorded in the adapter corpus
- official `@xai-official/grok-darwin-arm64` npm artifacts
- Contracts 017 and 050
- Research 052, 076, 085, 087, and 104
