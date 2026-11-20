# 253: Fix Codex close blocking, secret redaction, and cleanup precedence

### Fixes

- App-server shutdown without a caller deadline no longer blocks a scoped
  task-service worker. The 250 ms EOF and graceful-stop grace now runs on the
  host's blocking pool, and an EOF exit ends the wait at once. A host with
  neither a time nor a blocking service escalates straight to stop and kill,
  with no grace.
- Active `CodexExecRunHandle::close` requests cancellation before joining, so
  closing an exec run that is still running no longer waits for the child to
  exit on its own. A run that already finished is not cancelled.
- Stderr excerpts redact the value that follows a secret label. Discovery
  failure messages carry these excerpts publicly, so `Authorization: Bearer
  <secret>`, `token: <value>`, `api-key: <value>` and `password: <value>` no
  longer leak the value into a `SafeDiagnostic` message.
- Exec cleanup merges materialization cleanup with `merge_cleanup`, so a
  failed materialization release after a degraded process cleanup is reported
  as `Failed`, not `Degraded`.
- Thread history and reconciliation share the catalogue and import teardown
  and precedence: a cleanup failure or degradation replaces a projected result,
  including a projected error. Their RPC, lease and teardown boilerplate is
  consolidated in the thread-catalogue execution module.
- Stdout line buffers check the line bound before copying a chunk, so an
  oversized chunk cannot grow the pending line past the cap. Dynamic-tool and
  user-input callbacks check capacity before reserving a provider call id, so a
  capacity refusal no longer leaves a reserved id behind.
- Unsupported-input messages no longer call the app-server driver a proof
  driver.
