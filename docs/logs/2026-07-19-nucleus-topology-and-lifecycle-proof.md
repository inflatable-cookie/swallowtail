# Nucleus Topology And Lifecycle Proof

Date: 2026-07-19
Roadmap: g01/008 Nucleus Interactive-Session Readiness
Card: 028 Nucleus Topology And Lifecycle Proof

## Realized Boundary

Runtime service registries now identify their owning execution host. Drivers
compare that identity with the immutable preflight plan before invoking any
host port. This closes the prior gap where a valid plan could be paired with a
task or process service from another host after preflight.

Interactive session handles now return a provider-neutral resume binding. It
keeps the opaque provider session reference attached to the configured-
instance, execution-host, model-route, and model identities that opened it.
Codex app-server resume validates the binding before process start and requires
the provider response to repeat the requested session. Turns reject host-
service substitution and provider events from another session.

## Topology Evidence

The same public driver path runs against distinct local and remote-
authoritative fixtures. Each fixture owns different opaque executable target
and working-resource references. The selected process port receives only those
references and the delegated environment reference; no raw client path or
secret enters a portable request.

Both topologies prove:

- open and exact-bound resume
- callback wait and native turn cancellation
- session close while a callback is pending
- callback stream abandonment and late-work prevention
- one terminal turn outcome
- joined process and task cleanup

Additional fixtures reject host, configured-instance, provider-session, turn-
host, and provider-event substitution. Unexpected disconnect reports a host
failure and performs cleanup without guessing an automatic recovery route.

## Validation

Focused topology and lifecycle integration tests, the expanded long-lived RPC
profile, runtime redaction tests, strict workspace Clippy, formatting, full
workspace tests, and Effigy docs QA provide the closeout evidence.
