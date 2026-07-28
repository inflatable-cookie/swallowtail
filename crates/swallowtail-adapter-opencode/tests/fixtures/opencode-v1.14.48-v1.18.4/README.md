# OpenCode HTTP/SSE Compatibility Corpus

Offline compatibility evidence for stable OpenCode `1.14.48` through
`1.18.4`. Captured 2026-07-23 from maintained GitHub release tags and each
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

Forty-five stable releases produce 18 closed surfaces and 20 contiguous
semantic-version segments. Separate segments preserve unpublished `1.15.8`,
`1.16.1`, and cross-minor gaps. No syntactically possible but unpublished
version is inferred into the candidate claim.

`deletion.json` freezes `DELETE /session/{sessionID}` separately. The existing
six-route execution claim therefore stays unchanged before the deletion
driver exists. The delete operation plus its complete local-reference closure
has two schema revisions:

- `delete-01`: `1.14.48` through `1.15.5`
- `delete-02`: `1.15.6` through `1.18.4`

Eight semantic-version segments preserve unpublished patch and cross-minor
gaps. Two runtime evidence revisions remain separate from schema shape.
`1.14.48..=1.14.50` recursively deletes descendants without a provider busy
guard or background-job cancellation. `1.14.51..=1.18.4` adds background-job
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
is present at both `1.14.48` and `1.18.4` boundaries. Cost remains separate
from token usage.

`generation-controls-prompt-request.json` freezes the exact prompt `variant`
and harness-owned JSON Schema format present at both range boundaries.
OpenCode owns schema validation and retry; the route exposes no stable
per-operation output maximum. Card 086 realizes exact catalogue-gated model
variants and zero-retry schema dispatch across the qualified range and visible
unverified-newer execution.

The small health and session envelopes are synthetic. They contain no
credential, endpoint, account, path, provider payload, model response, or user
content.

Sources:

- <https://github.com/anomalyco/opencode/releases>
- <https://github.com/anomalyco/opencode/tree/v1.14.48>
- <https://github.com/anomalyco/opencode/tree/v1.18.4>
- <https://opencode.ai/docs/server/>
