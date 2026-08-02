# 095 OpenCode Session Catalogue And Import Range

Status: promoted
Owner: Tom
Date: 2026-08-02

## Question

Does every maintained OpenCode HTTP/SSE server milestone expose a complete,
resource-scoped path from session listing through revalidated history replay
and continuation, and which route-specific differences must remain private?

## Method

The check downloaded the exact tagged `packages/sdk/openapi.json` for all 51
qualified published releases from `1.14.48` through `1.18.10`. For each tag it
recursively closed these operation objects and every local JSON reference:

- `session.list`
- `session.status`
- `session.get`
- `session.messages`
- `session.prompt_async`

Object keys were recursively sorted before SHA-256. Results were mapped back
to the existing exact release and semantic-gap authority in
`compatibility.json`. Current official server documentation was checked only
as corroboration. No attached server, credential, account, provider prompt, or
consumer workspace ran.

## Finding

Every one of the 51 qualified releases contains the complete five-operation
chain. The recursive closure has seven exact revisions and maps onto the same
12 published-version segments already qualified for load/resume. No partial
qualified milestone needs removal. Unpublished gaps remain gaps, and
`1.18.11` or later cannot inherit this capability as unverified newer.

All selected revisions expose directory, offset, and limit controls on
`session.list`. The response is an updated-descending array, so a private
catalogue cursor can retain the next `start` offset while common cursor
identity keeps it plan-scoped. A short page is terminal. Swallowtail selects
only explicit `directory`, positive `start`, and bounded `limit`; it does not
select project-wide, roots, path, search, or account-wide browsing.

Each session record has exact id, directory, title, version, and created/update
time fields across the range. Optional `parentID` identifies child sessions.
Children may remain visible but unavailable; the first import route does not
claim child-session authority. Foreign-directory records are rejected even if
the server returns them.

`session.status` is a directory-scoped map. `idle` is importable inactive
evidence. `busy` and `retry` are active and unavailable. Missing or malformed
status is provider-reported unavailable rather than guessed inactive. Import
repeats health, exact lookup, and status. A missing session, changed title or
update time, version mismatch, directory drift, child transition, or active
transition issues no binding.

The existing `session.messages` evidence already freezes newest-page-first
wire order, chronological item order within each page, oldest-page-first replay,
exact session ids, and page/item/byte bounds. The existing
`session.prompt_async` continuation remains unchanged. Import therefore needs
no second replay or continuation mechanism.

## Decision

Qualify the entire existing maintained OpenCode range for resource-scoped
catalogue and explicit import. Preserve seven private import surface revisions
and 12 exact published-version segments. Bind the attached endpoint, Basic-auth
credential lease, exact directory, health-observed server revision, model
route, access policy, and provider-session identity.

Keep child import, project/account scans, server ownership, deletion, sharing,
forking, renaming, revert, summarize, and automatic synchronization outside
the route. Contract 046 already governs the portable boundary; no contract
change is required.

## Sources

- [OpenCode `1.14.48` OpenAPI](https://github.com/anomalyco/opencode/blob/v1.14.48/packages/sdk/openapi.json)
- [OpenCode `1.18.10` OpenAPI](https://github.com/anomalyco/opencode/blob/v1.18.10/packages/sdk/openapi.json)
- [OpenCode server API](https://opencode.ai/docs/server/)
- [OpenCode releases](https://github.com/anomalyco/opencode/releases)
