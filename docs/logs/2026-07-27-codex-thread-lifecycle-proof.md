# Codex Thread Lifecycle Proof

Date: 2026-07-27

## Change

Cards 050-051 complete roadmap 016. Codex app-server is the first production
adapter for the provider-session management role.

Applicable prepared new and resumed session handles now expose an opaque
`ProviderSessionManagementBinding`. It retains exact driver, configured
instance, host, executable target, access evidence, interface compatibility,
working resource, provider thread, origin, and supported management
capabilities. The caller clones the binding and closes the runtime handle
before preparing provider lifecycle work.

The facade adds three separate operations:

- `prepare_archive_session`
- `prepare_restore_session`
- `prepare_delete_session`

There is no generic manage or prompt method. Each prepared value retains its
immutable management plan and typed request, then delegates to the same
`CodexAppServerDriver` low-level implementation.

## Exact Mapping

- archive is available from `0.80.0` and guarantees `TargetOnly`
- restore is available from `0.92.0`
- matching archive and restore notifications are expected from `0.104.0`
- best-effort descendant archive from `0.123.0` does not widen the guarantee
- delete is available from `0.140.0` as `ProviderHardDeleted` with
  `ProviderDefinedDescendants`
- unverified-newer destructive execution requires explicit facade acceptance

Unknown and repeatedly fully deleted targets remain provider failures. A
missing rollout is not projected into general already-absent success.

## Failure Truth

Plan, binding, action, deletion-strength, affected-scope, host, target,
version, and service drift stop before lifecycle dispatch.

An explicit provider rejection is failed-before-effect. Disconnect, malformed
response, cancellation, or deadline after dispatch is unconfirmed. A
successful response remains authoritative when a notification disagrees or
cleanup degrades; those failures remain separate safe diagnostics.

Every spawned app-server pump is joined. Management reuses the exact
app-server invocation, environment, working resource, process, task, deadline,
and cancellation boundaries. It does not resume a thread, discover or close an
active handle, mutate rollout files, or touch consumer thread state.

## Validation

- focused prepared lifecycle matrix: 18 tests pass
- full Codex adapter suite: 107 tests pass
- runtime unit suite: 66 tests pass
- runtime and testkit management conformance: pass
- full workspace suite: 769 tests pass, two existing leaky-test annotations,
  four tests skipped
- `effigy check:rust`, format, docs, Northstar, and diff checks: pass
- `effigy doctor`: unchanged structural-debt baseline of 25 findings
  (17 warnings, 8 errors)

## Next

Card 052 is ready. Pin the additive stable ACP v1 close/delete subset and
freeze Claude Agent tagged lifecycle behavior before production mapping.
