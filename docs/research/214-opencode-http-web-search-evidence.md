# 214 OpenCode HTTP Web Search Evidence

Status: reserved
Owner: Tom
Created: 2026-08-26
Card: g04.067 / 187

## Question

Which exact OpenCode HTTP versions, providers/backends, permission rules,
operation profiles, and shared search/network policy rows can admit native
`websearch` without ambient configuration authority, permission/network
conflation, or live-provider inference?

## Decision

Pending card 187. No version, provider/backend, permission action, profile, or
policy row is prequalified by this reservation.

The evidence gate must reconcile exact `v1.18.20` tool registration and
execution with provider/backend availability, environment gates, session rule
ordering, `allow|ask|deny` behavior, callback events/replies, prompt/SSE truth,
shared policy authority, omission, and claim strength before cards 188-189 may
run.
