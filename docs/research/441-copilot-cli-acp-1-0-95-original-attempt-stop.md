# Research 441: Copilot CLI ACP 1.0.95 Original Attempt Stop

## Status

One reviewed production-route invocation was consumed before a prepared
session was returned. No qualification or public claim changed.

## Question

Did the approved production-route attempt on the exact frozen `1.0.95` native
artifact reach a session, correlate the requested sentinel action with its
permission request, and cancel without effect?

## Bound identity and authority

Research 440 freezes the wrapper and Darwin ARM64 package identities and every
published stable hop through `1.0.95`. The official npm `latest` tags for
`@github/copilot` and `@github/copilot-darwin-arm64` both remained `1.0.95` on
the pre-push recheck for this record. The exact target was not substituted.

The one-shot plan was reviewed at source head
`5c431e15c74e73e2d6a3967e82d020f698f5898f` under planner binding
`34ac89bbcf6d1186e0374ee89e02626b2cf42a87d41c8396b30a7c4b4d9cc60a`.
The attempt record binds plan SHA-256
`f7e00c723aeadb976decbd233c6a128bcbaca6de5b637b2205461ca553f47c76`, exact
native executable SHA-256
`35d33e040e8aa0554a02385f02648396ba3b853f026ed1db17d30d051a764e4b`, the
approved host identifier `swallowtail.copilot119.original.mac-arm64`, the
home-only environment reference `swallowtail.copilot119.home-only`, and
`--acp --stdio`. The run used the existing host-owned login and configured
default-model policy. It ran under the single planner-authorized original
allowance; this was not an artifact pre-probe or a second attempt.

## Attempt result

On 2026-10-10, the dedicated command
`effigy observe:copilot-acp-private-assessment <binding-payload>` was invoked
once with the planner-provided mode-0600 payload. It exited 1. The exclusive
mode-0600 `1.0.95-attempt-4.json` record has SHA-256
`04d229c74e74623d68a3d8bf12dd7ca479792829fdd166bf063ec7e9c372c09e`; it
records `invocation-consumed-before-prepared-open`, invocation 4, and
`original_started: true`. The mode-0600 `1.0.95-execution.json` record has
SHA-256
`787b4941d55f5a2fe12f405caf836ace01f222de53357e5af47d7afa0105d5b3`; it
records `status: preparation-failed` and `cleanup: not-started`.

The prepared assessment function returned before a prepared session was
available. The current result record does not retain the underlying safe
preparation error class. No prompt-consumption record was created, so the
third shared prompt slot remains unused. No session, permission request,
correlated action, terminal cancellation, post-run executable measurement, or
task-directory after-hash is present in the retained evidence. Therefore this
attempt proves neither permission cancellation nor absence of a sentinel
effect. No-effect must not be inferred from the missing prompt record.

## Decision

Keep the public `copilot_cli.acp` claim at exact `QualifiedOnly 1.0.80`. Leave
`1.0.81` through `1.0.95` unqualified; the fifteen native runtime hops in
Research 440 remain unresolved. Research 439's limited exact `1.0.93`
cancellation evidence does not transfer to `1.0.95`. Invocation 4 consumed
the only original allowance in this continuation. Do not retry, resend, or
run another artifact under that allowance. The repository's private plan is
restored to its disabled state.

## Follow-up

Create a bounded provider-free adaptation for the private assessment runner to
retain an allowlisted preparation-failure class and the cleanup/effect
measurements that are available at each failure boundary. Fake tests should
show the class is useful without retaining raw diagnostics or secrets. This
would clarify this stop but cannot replace the missing `1.0.95` session and
permission evidence. Any later original requires a separate explicit
allowance. Until the selected route gates close on evidence, `1.0.95` remains
unqualified.

## Sources

- [Research 440: Copilot CLI ACP Artifact Hops Through 1.0.95](./440-copilot-cli-acp-1-0-95-artifact-hop-identity.md)
- [Official wrapper npm metadata](https://registry.npmjs.org/%40github%2Fcopilot)
- [Official Darwin ARM64 npm metadata](https://registry.npmjs.org/%40github%2Fcopilot-darwin-arm64)
- Planner binding receipt and execution records for task `7823a168-9b8f-4604-8fd7-201c2998722e`; safe fields and digests only
