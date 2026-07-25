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

The small health and session envelopes are synthetic. They contain no
credential, endpoint, account, path, provider payload, model response, or user
content.

Sources:

- <https://github.com/anomalyco/opencode/releases>
- <https://github.com/anomalyco/opencode/tree/v1.14.48>
- <https://github.com/anomalyco/opencode/tree/v1.18.4>
- <https://opencode.ai/docs/server/>
