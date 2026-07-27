# 050 Codex Thread Management Driver And Facade

Status: completed
Owner: Tom
Created: 2026-07-26
Milestone: `../016-codex-thread-lifecycle-proof.md`

## Objective

Map qualified Codex app-server thread archive, unarchive, and delete through
the shared management role and prepared facade.

## Governing Refs

- Contract 038
- Contract 037
- Research 037
- card 049
- existing Codex app-server connection and prepared integration

## Scope

1. Return a management binding for applicable new and resumed Codex sessions.
2. Build separate prepared archive, restore, and delete operations.
3. Reuse the existing app-server transport, exact target, version, access,
   host, deadlines, and joined task ownership.
4. Map notifications, empty responses, unknown and repeated-delete failures,
   hard deletion, and provider-defined descendants exactly.
5. Preserve qualified versus unverified-newer status.
6. Keep Codex exec, session resume, consumer thread state, and target selection
   unchanged.

## Acceptance Criteria

- [x] only a matching inactive management binding can dispatch
- [x] unsupported legacy segments stop before app-server work
- [x] archive and restore do not resume the session
- [x] delete reports hard deletion only where card 049 qualified it
- [x] missing rollout tolerance never becomes a general already-absent success
- [x] a lost response after dispatch remains unconfirmed
- [x] prepared and low-level paths share one lifecycle implementation

## Validation

- focused Codex protocol, range, prepared, and driver tests
- core/runtime management regressions
- `effigy check:rust`
- `effigy format:check`

## Stop Conditions

- Codex requires direct rollout-file mutation
- prepared operations need consumer persistence or confirmation
- the implementation would auto-close or discover an active runtime handle
- unverified-newer execution would be relabeled qualified

## Auto-Continuation

Yes after card 049 acceptance. Continue to card 051.

## Completion Evidence

- applicable prepared new and resumed sessions return an opaque management
  binding with exact origin, route, access, host, target, version, resource,
  and action capabilities
- separate `prepare_archive_session`, `prepare_restore_session`, and
  `prepare_delete_session` methods build immutable typed operations
- all prepared methods delegate to `ProviderSessionManagementDriver` on the
  existing app-server transport
- unsupported actions, unaccepted unverified-newer points, and binding drift
  fail during preparation before process work
- archive guarantees `TargetOnly`; qualified delete reports
  `ProviderHardDeleted` with `ProviderDefinedDescendants`
- provider rejection remains failed-before-effect; disconnect, malformed
  response, and post-dispatch control loss remain unconfirmed
- card 051 completed the production conformance and repository validation
