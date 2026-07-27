# OpenCode Session Deletion Range Evidence

Status: promoted
Owner: Tom
Created: 2026-07-27
Promoted to: Contract 038, roadmap g02.018, card 055 corpus

## Question

Can Swallowtail qualify attached OpenCode session deletion across the existing
`1.14.48..=1.18.4` server range without shrinking it, projecting current
documentation backward, or claiming hard erasure?

## Method

The audit reused the 45 exact stable releases already frozen by Research 027.
For every recorded tag commit it:

1. verified the full `packages/sdk/openapi.json` SHA-256 against the existing
   six-route corpus
2. selected `session.delete`
3. followed every transitive local JSON reference
4. recursively sorted object keys and hashed the closed operation-plus-
   reference object
5. inspected the tagged route declaration, handler, session removal core,
   authorization middleware, error middleware, and deletion tests

Current server documentation was checked only as present-day corroboration.
It did not authorize historical behavior.

## Schema Result

All 45 stable tags contain:

- `DELETE /session/{sessionID}`
- operation id `session.delete`
- `200` with a Boolean success body
- `404` with `NotFoundError`
- the same directory and workspace query identity used by attached session
  routes

The complete delete closure has two revisions:

| Revision | Published releases | Change |
| --- | --- | --- |
| `delete-01` | `1.14.48..=1.15.5` | Boolean success, `BadRequestError`, `NotFoundError` |
| `delete-02` | `1.15.6..=1.18.4` | 400 becomes `BadRequest` or `InvalidRequestError`; success and missing-target behavior stay unchanged |

Eight semantic-version segments preserve unpublished patch and cross-minor
gaps. `1.15.8` and `1.16.1` remain outside the claim. `1.18.5` remains a
permitted unverified-newer example, not a guaranteed release.

The separate deletion corpus retains every tag commit, publication date, full
OpenAPI digest, existing six-route surface id, delete-closure digest,
component count, and runtime evidence revision. The original six-route corpus,
18 execution surfaces, 20 execution segments, and production behavior ids do
not change.

## Runtime Result

Every tagged route handler maps storage-not-found to `404` and returns `true`
after the session removal effect completes. Tagged route descriptions declare
permanent removal of associated messages and history. Tagged HTTP exercises
confirm `true` and subsequent session absence.

The removal core loads the target before effects. A missing target therefore
fails; repeated deletion is not already-deleted success.

The core recursively enumerates and removes child sessions before the target.
The honest scope is `ProviderDefinedDescendants`. The provider declares
session data deletion, so the strongest portable result is
`ProviderDataDeleted`. There is no secure-erasure, provider-account,
analytics, log, or backup guarantee. `ProviderHardDeleted` is unsupported.

Two runtime evidence revisions matter:

- `runtime-01`, `1.14.48..=1.14.50`: recursive descendant removal; no busy
  guard; no background-job cancellation in the removal core
- `runtime-02`, `1.14.51..=1.18.4`: recursive descendant removal plus
  background-job cancellation; still no busy guard

The provider route accepts an active target. That is not safe authority for
Swallowtail to discover or manage an active handle. Contract 038's
inactive-target requirement remains unchanged. The caller closes its runtime
handle first.

## Authentication And Failure Truth

OpenCode server password configuration applies Basic authentication to
protected routes throughout the range. Invalid or missing credentials return
`401` when authentication is enabled. The generated legacy OpenAPI deliberately
omits this middleware-level auth shape, so tagged middleware is the primary
evidence.

The selected OpenAPI declares 400 and 404 outcomes. Unexpected server failures
can produce 5xx responses. After a delete request is dispatched, a 5xx,
disconnect, cancellation, or deadline cannot prove whether provider effects
occurred. Swallowtail must return unconfirmed provider truth, must not retry,
and must not expose the raw response body as stable diagnostic evidence.

## Decision

The entire existing OpenCode range is deletion-ready. No tagged schema is
missing or contradictory. Adding deletion does not shrink the range.

Qualify the route as:

- explicit bound deletion only
- one inactive session target
- `ProviderDataDeleted`
- `ProviderDefinedDescendants`
- 404 as provider rejection, not already-deleted success
- no archive, restore, resume, hard-erasure, retry, server ownership, or
  active-handle management claim
- visible unverified-newer execution under Contract 029

Card 056 may implement the low-level and prepared paths against this corpus.

## Sources

- [OpenCode server documentation](https://opencode.ai/docs/server/)
- [OpenCode `1.14.48` OpenAPI](https://github.com/anomalyco/opencode/blob/v1.14.48/packages/sdk/openapi.json)
- [OpenCode `1.15.5` OpenAPI](https://github.com/anomalyco/opencode/blob/v1.15.5/packages/sdk/openapi.json)
- [OpenCode `1.15.6` OpenAPI](https://github.com/anomalyco/opencode/blob/v1.15.6/packages/sdk/openapi.json)
- [OpenCode `1.18.4` OpenAPI](https://github.com/anomalyco/opencode/blob/v1.18.4/packages/sdk/openapi.json)
- [OpenCode `1.14.48` session route](https://github.com/anomalyco/opencode/blob/v1.14.48/packages/opencode/src/server/routes/instance/httpapi/groups/session.ts)
- [OpenCode `1.18.4` session route](https://github.com/anomalyco/opencode/blob/v1.18.4/packages/opencode/src/server/routes/instance/httpapi/groups/session.ts)
- [OpenCode `1.14.48` deletion core](https://github.com/anomalyco/opencode/blob/v1.14.48/packages/opencode/src/session/session.ts)
- [OpenCode `1.14.51` deletion core](https://github.com/anomalyco/opencode/blob/v1.14.51/packages/opencode/src/session/session.ts)
- [OpenCode `1.18.4` deletion core](https://github.com/anomalyco/opencode/blob/v1.18.4/packages/opencode/src/session/session.ts)
- [OpenCode `1.14.48` HTTP session tests](https://github.com/anomalyco/opencode/blob/v1.14.48/packages/opencode/test/server/httpapi-session.test.ts)
- [OpenCode `1.18.4` HTTP session tests](https://github.com/anomalyco/opencode/blob/v1.18.4/packages/opencode/test/server/httpapi-session.test.ts)
- [OpenCode `1.14.48` authorization middleware](https://github.com/anomalyco/opencode/blob/v1.14.48/packages/opencode/src/server/routes/instance/httpapi/middleware/authorization.ts)
- [OpenCode `1.18.4` authorization middleware](https://github.com/anomalyco/opencode/blob/v1.18.4/packages/opencode/src/server/routes/instance/httpapi/middleware/authorization.ts)
