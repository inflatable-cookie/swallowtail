# 267: Round 3 sweep 1B: runtime, Codex, OpenCode, and Pi cleanup and lifecycle fixes

- `swallowtail-runtime` delegated CLI sign-in keeps its pending process wait
  across polls instead of re-polling once with a no-op waker and dropping the
  future.
- Breaking (pre-1.0): `swallowtail-runtime` `complete_sign_in` and
  `cancel_sign_in` return `(SignInFailure, SignInSession)` on failure, through
  the `SignInFinishFailure` alias. When host cleanup returns `Poll::Pending` the
  session comes back with its in-flight loopback release and process stop, so
  `complete_sign_in_async` or `cancel_sign_in_async` finishes them instead of
  the cleanup being abandoned. Map the error with
  `.map_err(|(failure, _)| failure)` to keep the old shape.
- `swallowtail-runtime` admission gate deregisters a cancelled waiter's waker
  when its future is dropped, and wakes one waiter on release instead of every
  queued waiter. A waiter dropped after being woken passes the wake-up on.
- `swallowtail-runtime` installed-executable discovery starts its deadline and
  cancellation watchers before spawning the probe, races stdin close against
  them, and bounds child teardown so a stopped child that never settles reports
  `cleanup_failed` instead of stalling discovery.
- `swallowtail-adapter-codex` bounds the `initialize` handshake of model
  discovery and thread-catalogue, import, history, and reconciliation
  operations by the caller deadline, and by cancellation where the caller has
  it. Cancelled or timed-out handshakes report
  `swallowtail.codex.app_server.initialize_cancelled` or
  `initialize_timed_out`. Session open, load, and resume keep their existing
  handshake.
- `swallowtail-adapter-codex` bounds `model/list` pagination to 64 pages and
  rejects a repeated `nextCursor` with
  `swallowtail.codex.app_server.catalog_cursor_repeated` or
  `catalog_page_limit`.
- `swallowtail-adapter-codex` turn close finishes the turn when the provider
  refuses `turn/interrupt`, so the deadline watcher exits instead of waiting for
  the full turn deadline, and reports the refusal as a cleanup failure.
- `swallowtail-adapter-opencode` closes the SSE subscription before returning
  when the turn task spawn is refused, instead of leaving the worker detached.
- `swallowtail-adapter-opencode` and `swallowtail-adapter-pi` merge task,
  attachment, and cancellation failures into the `CleanupOutcome` returned by
  `close()` instead of discarding them. Pi session and sidecar close report a
  deadline task that fails to join.
- `swallowtail-adapter-opencode` session-history cancellation and timeout report
  `swallowtail.opencode.session_history.{cancelled,timed_out}` instead of the
  `session_reconciliation.*` codes.
- `swallowtail-adapter-opencode` SSE event boundaries split at the earliest of
  an LF or CRLF boundary, so a CRLF frame followed by an LF frame in one buffer
  no longer merges into one event.
- `swallowtail-adapter-pi` structured-run close escalates to a forced stop when
  a native abort is refused, so the run can join. A refused abort is no longer
  acknowledged, and a retry uses a fresh command id.
