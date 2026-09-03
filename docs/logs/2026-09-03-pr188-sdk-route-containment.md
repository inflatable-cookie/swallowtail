# 2026-09-03 PR 188 SDK Route Containment

PR 188 merged as `ff7ec3d8` despite rejected exact-head review of
`6f102f83`. The review found three lifecycle blockers: refused scoped-task
relinquishment can synchronously join on drop past the caller deadline; resource
and credential leases can be released while transferred work still runs; and
the integrated reaper fixture does not retain, transfer, or join its worker.

This containment forward-reverts PR 188's tree delta from first parent
`6543c905`. That removes `claude-agent.sdk` while preserving PR 192/card 060 and
all unrelated history. `AcceptedForReap` remains transfer only, never join or
cleanup success.

Card 055 is blocked pending a provider-neutral shared-runtime
reservation/reapable-task prerequisite that is established before provider work
and owns the complete ordered cleanup continuation through terminal lease
release. This change does not compile or implement that prerequisite.

The `v0.4.0` release lane remains frozen. It cannot resume before this
containment passes independent exact-head review and a later SDK implementation
is independently accepted at its exact head.
