# 2026-07-21 Claude Managed Agent Coverage Selection

## Outcome

Selected Claude Managed Agents as the next high-information proof. Roadmap 024
and card 076 are complete. Contract 022 is active. Roadmap 025 is active; card
077 is ready.

## Evidence

- the first-party beta exposes versioned agent definitions, reusable
  environment templates, provider sessions, fresh cloud sandboxes, and
  persisted events as distinct resources
- sessions retain history until deletion and sandbox checkpoints for a
  documented period; the route cannot claim prohibited or merely temporary
  provider retention
- provider `rescheduling` can perform provider-managed retry after transient
  failure and cannot be flattened into one Swallowtail inference attempt
- persisted events are authoritative; connection-local previews are best
  effort, unreplayable, and never terminal truth
- custom tools pause sessions at `requires_action`; native interrupt,
  cumulative token usage, and resource deletion expose new lifecycle pressure
- Cursor remains policy-bound by repository, GitHub, provider-VM mutation,
  artifact, and durable-agent authority
- remote ACP remains behind incomplete transport, reconnect, resumability, and
  security authority

No provider account, credential, repository integration, remote resource,
external inference request, or paid work participated.

## Decisions

- the first proof is a resource-free structured harness run, not a repository
  agent or direct inference request
- one operator-owned agent is pinned to one exact version and model; the driver
  never mutates it
- the driver owns one limited-network environment and one session and deletes
  them in that order before credential release
- durable retention and provider-managed recovery require explicit opt-in
- authoritative persisted events drive output and terminal truth; preview
  deltas remain excluded
- the subset grants no files, provider built-ins, external sandbox network,
  MCP, skills, memory, multiagent, GitHub, schedules, webhooks, or resume
- no local container or host sandbox is required

## Changed

- added Research 016 and promoted Contract 022
- completed roadmap 024 and card 076
- added roadmap 025 and cards 077-079 inside g01
- made card 077 the sole ready continuation
- updated research, contracts, roadmaps, batch cards, generation, long-term,
  log, and project front doors

## Validation

- `effigy qa:docs` passes
- `effigy qa:northstar` passes
- `git diff --check` passes
- `effigy doctor` remains at the inherited 19 findings: 12 warnings and 7
  errors

## Remaining Risks

- the beta surface and header can drift
- exact limited-network environment creation without external hosts must be
  proven by the dated fixture gate before production
- provider internal attempt count during rescheduling is unknown
- remote deletion can fail or remain ambiguous; degraded cleanup must preserve
  that truth
- token usage and running-session time are separate billing dimensions; local
  elapsed time is not exact provider cost
- live entitlement, quota, billing, request acceptance, and remote behavior
  remain separately gated

## Continuation Record

Card 077 is ready. Add only Contract 022's minimum common records and freeze
the exact Managed Agents REST/SSE corpus. Cards 078-079 remain in bounds after
the fixture gate proves the selected limited environment and lifecycle.
