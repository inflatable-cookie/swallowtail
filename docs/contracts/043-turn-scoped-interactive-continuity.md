# Turn-Scoped Interactive Continuity

Status: active
Owner: Tom
Updated: 2026-08-18

## Purpose

Permit reusable interactive sessions whose provider work is scoped to one
turn at a time. Keep harness-retained continuation across restarted child
processes separate from consumer-owned transcript replay across independent
direct-inference requests.

## Continuity Modes

One interactive profile selects exactly one continuity mode:

1. **Harness-retained restarted continuation** — the harness stores
   conversation state. Swallowtail starts one owned child for each turn and
   privately supplies the exact provider session reference after the first
   successful turn.
2. **Consumer-owned transcript replay** — Swallowtail retains one bounded
   adapter-private transcript for the life of the runtime session. Every turn
   is a separate provider request carrying the committed transcript plus the
   new user message.

The modes are not interchangeable. A driver cannot switch mode after
preflight, infer one from transport framing, or replace failed continuation
with a fresh provider session.

Neither mode creates a generic prompt API. Both use the existing interactive
session and turn lifecycle from Contracts 004, 009, and 012.

## Admission And Identity

The immutable plan binds:

- exact driver, configured instance, execution host, transport, facade, model
  route, model, access profile, and interface-version assessment
- one continuity mode
- positive maximum turns and per-turn stream-record bounds
- any private-history, input, output, line, or request-body bounds used by the
  selected mode
- exact session access and provider-state policy
- required process, network, working-resource, task, time, event, and
  diagnostic services

Harness-retained continuation requires durable provider-state acceptance.
Consumer-owned replay requires provider state to be prohibited unless a
separate exact provider-cache capability is declared.

Every runtime session and turn keeps its Swallowtail identity. A provider
session reference, child process, HTTP request, stream, model tag, and consumer
thread remain separate identities.

## Harness-Retained Restarted Continuation

The first turn starts without an ambient latest-session selector. It accepts
one exact provider session reference only from the bounded provider stream.
Every later turn:

- starts a new owned child under the same host, target, working-resource,
  model, access, configuration, and provider-state bindings
- supplies only that exact provider session reference through the qualified
  private invocation
- rejects a missing, changed, malformed, or contradictory reference
- joins the child before the turn closes or another turn starts

An adapter must not use a provider's "continue latest" shortcut. It must not
scan or enumerate ambient sessions, import an arbitrary provider reference,
fork the session, or expose provider storage paths.

Private per-turn use of a provider resume flag does not implement the public
`ResumeSession` role. The interactive handle returns no resume binding unless
the adapter separately qualifies Contract 017. Provider history restored
inside the harness is not Swallowtail replay and does not become consumer
transcript output.

A completed turn with clean joined process cleanup commits the provider
reference for the next turn. Provider failure, protocol failure, reference
mismatch, cancellation, timeout, non-clean child exit, or uncertain cleanup
invalidates further turns on that runtime handle. Close preserves durable
provider state but grants no archive, restore, delete, native-close, or public
load/resume authority.

## Consumer-Owned Transcript Replay

The runtime session owns an ordered adapter-private transcript containing only
completed user and assistant messages admitted by the selected profile.
Before each request the driver:

- checks the next user message, maximum turns, message count, encoded history,
  request-body, and output bounds
- constructs one exact request from the committed transcript plus the new
  user message
- keeps model and generation fields fixed to the prepared plan
- starts one authorized inference attempt with no retry or fallback

The transcript changes transactionally. Only a successfully decoded terminal
provider response appends the user message and complete assistant message.
Partial output, provider error, malformed data, disconnect, cancellation, or
timeout leaves the committed transcript unchanged. A later retry is a new
consumer-authorized turn; Swallowtail never retries automatically.

Transcript history is ephemeral, redacted, route-bound, and unavailable as a
provider session reference or resume binding. Closing the runtime session
clears it after owned network and task work joins. It does not stop an attached
service, unload a model, restore prior residency, or claim provider deletion.

## Events, Usage, And Cost

Each turn has one ordered event stream and exactly one terminal outcome.
Provider stream records cannot become historical replay events merely because
prior context influenced the response.

Usage is scoped to the current provider attempt. A driver may emit exact token
usage supported by its corpus. It cannot infer session-wide usage, billed
cost, rate, quota, media, or tool activity from transcript length, local
compute, duration, streaming, or published prices.

## Cancellation, Deadlines, And Cleanup

Only one turn may be active.

- cancellation and deadline observation remain distinct
- cancellation stops the owned child or local request consumption through the
  exact qualified mechanism
- a timed-out terminal state requires a host deadline observation
- no failed or cancelled turn silently starts another attempt
- turn close joins all turn-owned process, connection, stream, and task work
- session close first ends any active turn, then joins session-owned work and
  releases leases in owner order
- attached services remain running

Harness-retained sessions fail closed after uncertain turn state. Consumer-
owned replay sessions may remain reusable after a failed turn only when the
transcript is unchanged and all request-scoped work joined cleanly.

## First Qwen Mapping

The first restarted-harness proof binds:

- `qwen.headless`
- Qwen Code `0.19.11`
- harness interaction over text stdin and line-delimited stream-JSON stdout
- one ambient read-intent working resource
- durable project-scoped provider transcript acceptance
- one owned child per turn
- no resume selector on the first turn
- exact private `--resume <provider-session-id>` on later turns
- explicit prohibition of `--continue`, `--fork-session`, and consumer-
  supplied `--session-id`
- a 24-turn session maximum, 4,096 stream records per turn, and 1 MiB line
  bound
- no public load, resume, archive, restore, delete, native close, consumer
  tools, write, search, sandbox, or containment claim

`--safe-mode` and the read-only tool registry reduce ambient harness behavior.
They do not prove process isolation.

## First Ollama Mapping

The first consumer-replay proof binds:

- `ollama.attached`
- native Ollama API `0.14.0..=0.32.15` through the existing four qualification
  points plus official `0.32.15`, `ollama.native-text-v1` behavior
- exact exclusions `0.32.2` and `0.32.10`
- direct inference through one approved attached loopback endpoint
- one exact operator-selected model tag and digest
- resource-free, credential-free local access
- one streaming `POST /api/chat` request per turn
- adapter-private user/assistant transcript replay
- a 24-turn maximum, 48-message maximum, 1 MiB private-history and request
  bounds, 4,096 stream records per turn, 1 MiB line bound, and the existing
  eight-token output maximum
- prohibited provider session state and no public load, resume, provider
  session reference, native close, server stop, model unload, retry, fallback,
  tool, attachment, media, or billed-cost claim

Runtime-managed model residency remains the already accepted invocation side
effect from Contract 031.

## Conformance

Deterministic adapter-local scenarios prove:

- exact mode, route, version, host, model, access, state, and bound admission
- first and continued Qwen invocation differ only by the exact private resume
  argument
- Qwen rejects ambient latest, arbitrary id, session mismatch, failed child,
  cancellation, timeout, and cleanup uncertainty without another turn
- every Qwen child joins before the next starts
- every Ollama request repeats the exact committed ordered transcript
- Ollama commits only a complete successful user/assistant pair
- Ollama failure, malformed output, disconnect, cancellation, and timeout do
  not mutate transcript history
- transcript and provider references remain redacted
- usage stays per attempt and never becomes billed cost
- close clears private history, preserves durable Qwen state, and never stops
  the attached Ollama runtime
- local and remote-authoritative host identities exercise the same public
  role seam where the selected transport supports them

## Acceptance

- restarted harness continuation and direct transcript replay stay distinct
- private continuation does not mint public load or resume capability
- every turn is bounded and has one terminal outcome
- failed-turn commit semantics are explicit
- provider state and consumer-private history remain different
- cancellation, deadline, terminal, and cleanup truth remain separate
- no provider-session management, server ownership, model selection,
  credential, write, sandbox, media, pricing, retry, or fallback authority is
  inferred
