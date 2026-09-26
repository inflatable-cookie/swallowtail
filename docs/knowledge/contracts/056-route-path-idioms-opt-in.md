# 056 Route-Path Idioms Opt-In

Status: accepted
Owner: Tom
Updated: 2026-08-09

## Purpose

Let consumers opt into idioms delivery through the ordinary route path: a
one-time host registration, a portable session option bound in the prepared
plan, and a fixed runtime fold of selected idioms into the existing
developer-instructions channel — without composing product prompts or
re-wiring selection per session.

## Boundary

The opt-in is a route feature, not a default:

- absence of the session option means no idioms work anywhere: no selection,
  no folding, no provider payload change
- the runtime never invents instructions; it renders only opted-in source
  content under one fixed rule into a field the consumer already controls
- a requested opt-in without a registered source, or on a route that does
  not advertise the capability, fails closed at preflight before provider
  work
- signal recording stays consumer-owned: the product decides what an
  accept, reject, or edit means and feeds the registered recorder

## Host Ports

The execution-host service set gains two optional ports on the
`DiagnosticObserver` model (Contract 010):

- `IdiomSource::select(ctx) -> IdiomSet` — the registered selection backend
- `IdiomRecorder::record(signal)` — fail-soft signal sink; no registered
  recorder means no recording, never a failure

Both are registered once at host construction. Selection and recording
errors never change terminal status, classification, or cleanup truth.

## Session Option

`SessionOptions` gains one optional field (Contract 012):

- `idioms: Option<IdiomSessionOption>` where `IdiomSessionOption` carries
  an opaque source reference and an output maximum

The field is bound into the immutable prepared plan (Contract 037): source
identity and maximum are fixed before provider work and mismatched plans
reject before execution.

## Fold Rule

When the opt-in is bound and the source resolves a set, the runtime renders
the selected idioms into the developer-instructions field under one fixed
rule:

- one line per constraint (`Text`, `File`, `Tool`, `Command`), prefixed by
  scope and provenance labels
- total folded bytes bounded; overflow truncates with an explicit marker
- consumer-supplied developer instructions stay first; the folded idioms
  block appends after them under an explicit labeled block (settled
  2026-08-09)

The rule is deterministic and pinned by conformance fixtures. The runtime
never rewrites consumer text, never selects which idioms beyond the bound,
and never emits idioms when the option is absent.

## Runtime Dependency Floor

Realizing the host ports extends the runtime dependency floor: runtime now
depends on `swallowtail-idioms` in addition to core, `futures-core`, and
`zeroize` (amendment to Contracts 008/010/012/026 wording and the realized
architecture note). No other runtime dependency changes.

## Capability

Routes advertise an `idioms_session_option` capability. Advertising routes
accept the opt-in; others reject it at preflight. Absence of the
advertisement means unsupported, not "use developer instructions instead".

## Relationship To Contract 055

This contract is the host-gated exception to 055's "no prompt composition"
boundary: the fold is mechanism plumbing into an opted-in field under a
fixed rule, and 055's record, merge, lint, and selection semantics are
unchanged. Amendment note appended to 055.

## Conformance

Portable and route tests must cover:

- fold determinism across scopes, constraints, and provenance
- folded byte bounds and truncation marker
- consumer instructions preserved with explicit order
- fail-closed preflight: option without source, option on a
  non-advertising route, plan mismatch
- recorder no-op and failing-sink non-interference
- no idioms behavior when the option is absent

## Acceptance

- consumers opt in with one host registration and one session-option field
- Codex app-server proves the folded delivery on one deterministic fixture
- Nucleus adopts the surface on its interactive session path without
  importing product policy
- focused validation passes without live provider work
