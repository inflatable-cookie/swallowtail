# OpenCode HTTP/SSE Compatibility Corpus

Offline compatibility evidence for stable OpenCode `1.14.48` through
`1.18.10`. Captured through 2026-07-30 from maintained GitHub release tags and each
tag's `packages/sdk/openapi.json`.

The selected surface starts from six exact operations:

- `GET /global/health`
- `GET /provider`
- `POST /session`
- `POST /session/{sessionID}/prompt_async`
- `GET /event`
- `POST /session/{sessionID}/abort`

For each tag, the closure follows every transitive local JSON reference from
those operation objects. Object keys are recursively sorted before SHA-256.
The manifest records the full OpenAPI digest, closed-surface digest, component
count, event-schema count, tag commit, publication date, and behavior
revision.

Fifty-one stable releases produce 18 closed surfaces and 20 contiguous
semantic-version segments. Separate segments preserve unpublished `1.15.8`,
`1.16.1`, and cross-minor gaps. No syntactically possible but unpublished
version is inferred into the candidate claim.

`deletion.json` freezes `DELETE /session/{sessionID}` separately. The existing
six-route execution claim therefore stays unchanged before the deletion
driver exists. The delete operation plus its complete local-reference closure
has two schema revisions:

- `delete-01`: `1.14.48` through `1.15.5`
- `delete-02`: `1.15.6` through `1.18.10`

Eight semantic-version segments preserve unpublished patch and cross-minor
gaps. Two runtime evidence revisions remain separate from schema shape.
`1.14.48..=1.14.50` recursively deletes descendants without a provider busy
guard or background-job cancellation. `1.14.51..=1.18.10` adds background-job
cancellation but still has no busy guard. Swallowtail therefore keeps its
inactive-target rule.

Every tagged route returns `true` after successful provider-declared data
deletion, returns `404` for a missing target, recursively deletes provider
children, and uses the server's optional Basic authentication boundary.
Provider data deletion is not a hard-erasure claim. A server error after
dispatch leaves provider truth unconfirmed. Its body is not stable diagnostic
evidence.

`usage.sse` freezes two disjoint `step-finish` parts followed by session idle.
The required input, output, reasoning, cache-read, cache-write, and cost shape
is present at both `1.14.48` and `1.18.10` boundaries. Cost remains separate
from token usage.

`generation-controls-prompt-request.json` freezes the exact prompt `variant`
and harness-owned JSON Schema format present at both range boundaries.
OpenCode owns schema validation and retry; the route exposes no stable
per-operation output maximum. Card 086 realizes exact catalogue-gated model
variants and zero-retry schema dispatch across the qualified range and visible
unverified-newer execution.

`input-callback-corpus.json` freezes file parts plus permission and question
requests across all 51 qualified releases. Four exact closed-surface revisions
capture response-error and message-id schema changes. The portable subset
accepts only `once` or `reject`; upstream `always` remains visible but cannot
be selected. File input uses bounded host-materialized bytes encoded as a data
URL and grants no client path or arbitrary URL authority.

The small health and session envelopes are synthetic. They contain no
credential, endpoint, account, path, provider payload, model response, or user
content.

`activity-rich.sse` freezes message, reasoning, tool-state, and step lifecycle
from the selected event schema. `activity-gap-1.14.51.sse` freezes the exact
`1.14.51` segment: text delta and session close remain available, but typed
tool and reasoning part replacement do not. The 51-release
compatibility manifest is the version-coverage authority for both fixtures.

Exact `1.18.8` adds optional `iss` to an OAuth callback request outside every
selected transitive closure. `1.18.9` removes it. The full OpenAPI digests
retain that artifact delta; execution, deletion, continuity, callback, usage,
generation-control, and activity behavior stay on their existing revisions.

Sources:

- <https://github.com/anomalyco/opencode/releases>
- <https://github.com/anomalyco/opencode/tree/v1.14.48>
- <https://github.com/anomalyco/opencode/tree/v1.18.10>
- <https://opencode.ai/docs/server/>
