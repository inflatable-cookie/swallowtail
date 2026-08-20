# Key Concepts

Read this first if the route guides feel like alphabet soup. It explains the
shared vocabulary in plain English. It is a guide, not authority: the active
contracts and the provider route matrix own the exact rules. Links to both
appear throughout.

## Routes, Drivers, And Adapters

- **Provider route** — one exact, named way Swallowtail talks to a provider,
  such as `codex.exec` or `gemini.live`. A route is a specific driver plus a
  specific transport. Everything else in Swallowtail hangs off a route id, and
  no capability transfers between routes. The [provider route
  matrix](provider-route-matrix.md) is the authoritative inventory.
- **Driver** — the low-level runtime role that implements a route, such as
  `CodexExecDriver`. Drivers expose provider-neutral roles like "run" and
  "session"; they never collapse providers into one fake uniform API.
  [Contract 005](../contracts/005-integration-identity-and-transport-diversity.md)
  keeps identity separate.
- **Adapter** — a package (crate) that owns one or more routes for one
  provider, such as `swallowtail-adapter-codex`. Package selection does not
  make every route in that package available.
- **Prepared facade** — the normal, safe way to use a route. You prepare one
  immutable value (evidence + plan + request) and then call a typed operation
  on it. Think "approve everything up front, then run one thing". The
  low-level drivers behind it stay public for advanced composition.
  [Contract 037](../contracts/037-prepared-consumer-integration.md) requires
  the facade for every production driver.
- **Configured instance** — the identity of one concrete, pre-configured setup
  you want to present or select, such as `codex.local` revision `1`. It is not
  "the provider" in the abstract; it is one exact setup with its own revision.
  [Contract 047](../contracts/047-configured-provider-instance-catalogue.md)
  governs the instance catalogue.

## Connection Lifecycle

The pre-session path that admits a configured instance before the prepared
facade. See [connection lifecycle](connection-lifecycle.md).

- **Addable route** — one adapter-local `AddableRouteDescriptor` a consumer
  may offer to add. Only `anthropic.messages`, `deepseek.continuation`,
  `codex.app-server`, `claude-agent.acp`, and `ollama.attached` currently
  export one. Remaining production routes, including `codex.exec`, stay on
  the prepared-facade path. Topology is hosted, installed, or local-runtime;
  it is not `ExecutionLayer`.
- **Addable-route catalog** — the consumer-assembled list of those
  descriptors. There is no umbrella registry. Absence means the adapter was
  not linked, not that the route is unsupported.
- **Admission** — writing a configured instance through the store port.
  Admission does not prepare, select a model, or change 047 readiness.
- **Enablement** — a host preference on an admitted instance. Independent of
  access-status dimensions and of 047 `Ready` / `NotReady`.
- **Authenticated subject** — optional provider-disclosed email, login, or
  plan label, redacted by default. It is never an instance id, 047 field,
  default diagnostic, or routing key.
- **Model-presentation overlay** — hide, ordinal, consumer-default, and
  favourite markers keyed to exact catalogue identity. Overlay copies
  `Ready` / `NotReady` and cannot change it. Catalogue rows without
  `provider_id` stay unmarked.

## Access And Environment

- **Access profile** — the explicit authority a route runs under: credential
  mechanism, entitlement, endpoint audience, and support. Swallowtail never
  discovers credentials or logs in; the application picks a profile and shows
  matching evidence.
- **Evidence** — immutable records that bind identity, version, access, and
  intent. Nothing becomes durable authority without evidence, and evidence can
  be observed or explicitly asserted by the caller.
- **Execution host / host services** — the process authority the route runs
  under: approved executables, files, processes, credentials, and time. The
  application admits these explicitly. Swallowtail never searches for or
  installs an executable, starts an attached service, or acquires a model on
  its own. An optional diagnostic observer may receive restricted
  [debug observations](debug-observation.md); ordinary apps leave it off.
- **Working resource** — the exact approved workspace a harness may read or
  write. It is opaque authority bound during preparation, and it does not
  grant arbitrary filesystem access.
- **Opaque target / binding** — a value that carries exact provider or route
  identity while hiding raw ids and payloads. Use it as-is. Never parse it,
  copy a raw id out of it, or build one by hand.

## Operations

- **Structured run** — one bounded prompt-to-terminal operation with a typed
  result. `StructuredRunDriver` starts it and returns a `RunHandle`.
- **Interactive session** — a durable conversation: streamed events, tools,
  cancellation, and optional load or resume. `InteractiveSessionDriver` opens,
  loads, or resumes a session and returns an `InteractiveSessionHandle`.
- **Catalogue** — a bounded, non-authoritative inventory of models or
  provider sessions. It is display evidence for a consumer decision, never
  permission to run, hide, or select for the user.
- **Provider session history page** — a read-only newest-first window of
  provider-owned session history for UI scroll-back. It is not load readiness,
  not reconciliation, and not the consumer transcript store. See
  [provider session history pages](provider-session-history.md).
- **Terminal outcome and cleanup** — two separate truths. Terminal is the
  operation's final status; cleanup is joining its child processes, streams,
  leases, and credentials. A terminal outcome never implies cleanup success.

## Restart And Recovery

- **Reconciliation** — a read-only operation that observes what happened to an
  interrupted turn or run after the process lost its handle. It sends no
  prompt and grants no control. Its `replay_complete` flag means the
  replacement snapshot fit agreed bounds, not that older history pages exist.
- **Working-state restoration** — one prepared facade that picks the strongest
  qualified path back after restart: reconcile where supported, or a
  context-losing replacement otherwise. It never retries or replays a prompt.
- **Detachment** — stopping Swallowtail's local observation of an active
  operation without asking the provider to stop working. It is not
  cancellation and not completion.

## Safety Vocabulary

- **Fail closed** — treat missing, ambiguous, or drifted evidence as failure or
  unknown rather than assuming success. You will see this constantly; it is
  the house style for uncertainty.
- **Unverified-newer** — a stable provider version newer than the maintained
  range. Execution is allowed with visible "mileage may vary" evidence when
  the claim permits it, but it is not a support guarantee and does not widen
  the guaranteed range.
- **Capability** — a typed, evidence-backed guarantee a route advertises, such
  as `ActiveOperationDetachment`. Never infer a capability from provider
  prose, a CLI flag, or another transport from the same provider.

## Where To Go Next

- [Choose a route](provider-route-matrix.md) — the authoritative route and
  capability inventory
- [Integration guide map](integration-guide-map.md) — the guide and example
  front door
- [Quick Start](quickstart.md) — one Codex run, end to end
- [Connection lifecycle](connection-lifecycle.md) — addable catalog, admission,
  credentials, refresh, overlay, then prepare
- [Portable failure handling](portable-failure-handling.md) — what happens
  when something fails, in plain terms
