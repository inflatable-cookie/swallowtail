# 245: Claude Agent adapter Drop, lease, scope, and NDJSON framing

- The Claude Code pump awaits watcher completion and cleanup. Dropping an
  unclosed binding hands cleanup off without waiting for a pending completion
  gate. A panicking gate leaves its lease available for cleanup, and a refused
  reap task falls back to detached cleanup.
- A refused task spawn releases the response-only launch-directory lease
  instead of leaving the operation-scoped resource held.
- The ACP turn, run, and run-cleanup scopes are derived and bounded before the
  first provider side effect, so an overlong turn id can neither leave an
  active turn bound to the connection nor leave a prompt on the wire.
- `ClaudeCodeRunHandle::close` cancels a run that is still active and reports
  the pump's `CleanupOutcome::Failed(..)` instead of always returning clean.
- The three NDJSON framers check their length bound before copying, scan each
  chunk once, and reject an oversized trailing partial line.
