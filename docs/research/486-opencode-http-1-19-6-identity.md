# 486 OpenCode HTTP 1.19.6 Identity

Observed 2026-11-23 for `opencode.http` on `opencode.server`. npm
`opencode-ai` latest and GitHub latest both report `1.19.6`. The two
published hops above the maintained `1.19.4` ceiling are `1.19.5` and
`1.19.6`. Both remain `UnverifiedNewer` under `AllowUnverified`.
The historical segments and the unpublished `1.18.36` gap still govern
selection. The first unpublished later stable is `1.19.7`.

## Release identity

| Version | npm publication (UTC) | GitHub publication (UTC) | Tag commit |
| --- | --- | --- | --- |
| `1.19.4` | 2026-10-22T13:03:43.704Z | 2026-10-22T13:03:41Z | `148842ef7cb1bc34192b3fff57d0f9721ab9c801` |
| `1.19.5` | 2026-10-28T06:17:11.100Z | 2026-10-28T06:17:49Z | `810c0eba1a7348036f8bdf61e6632d0c36c6876c` |
| `1.19.6` | 2026-11-17T07:22:20.263Z | 2026-11-17T07:21:32Z | `f4e363c8295fa90a03dddc451882fffe8fbce357` |

The [frozen ledger](../../crates/swallowtail-adapter-opencode/tests/fixtures/opencode-http-1.19.6/)
records the wrapper tarball digests and all four member hashes. The
launcher, postinstall script and license match across the three points.
Only the wrapper manifest changes, retaining the same 12 native targets
and pinning each to its wrapper release.

Downloaded and hashed all 12 npm native packages and matching GitHub CLI
archives per point. The npm tarballs match registry SHA-512 integrity. Each
GitHub CLI archive matches both the release API SHA-256 and the subject
digest in the release statement verified by `gh release verify`. Each
statement includes the corresponding Git tag commit and release ID. All
36 npm/GitHub executable pairs are byte-identical. This closes the
cross-channel payload identity and target inventory gaps in
[Research 464](464-opencode-http-acp-1-19-5-identity.md). The GitHub
attestation is a release association. It does not establish reproducible
source-to-binary parity or a complete SBOM. The probed npm wrapper and
Linux x64 attestation endpoints returned 404 for both newer points.

## Selected HTTP/SSE path

| Published hop | Added | Removed | Changed | Identical |
| --- | ---: | ---: | ---: | ---: |
| `1.19.4` → `1.19.5` | 4 | 13 | 83 | 6540 |
| `1.19.5` → `1.19.6` | 42 | 18 | 118 | 6491 |

These counts describe the tag trees. The first hop reproduces Research
464's archive, tree, and changed path-set hashes. The second hop's
inventory classifies every differing path. Outside the selected prompt
changes below, the deltas are workspace version metadata, removal of the
app workspace Sentry build plugin from the lockfile, desktop and app
upgrade UI, hosted app deployment, release packaging and tests. The CLI
build adds an option to skip its build-time `--version` smoke check.

OpenAPI stays byte-identical at 162 paths, version `1.0.0`. The
selected route declarations and handlers retain health, catalogue,
create, prompt, subscribe, abort, delete, list, lookup, status, replay,
permission reply and question reply/reject. The tagged callback handlers
and permission/question implementations match across both hops. These
facts fix the HTTP shape. They leave the timing and contents of selected
SSE events to the prompt and provider implementations.

### 1.19.5 GitLab boundary

`gitlab-ai-provider` moves from `6.19.0` to `6.23.1` in the OpenCode and
core package manifests and `bun.lock`. The lock adds `proper-lockfile` to
GitLab's dependencies and relocates `signal-exit` entries. The exact
dependency tarballs and embedded source map hashes match Research 464.
The Linux x64 payload contains the matching quoted version marker at
each point and the new direct-access cache environment option in both
newer points. Those literals corroborate the observed dependency change.

The unchanged OpenCode GitLab wrapper makes the dependency reachable
through the selected paths. With GitLab configured and authenticated,
`/provider` can include `discoverWorkflowModels` results. OpenCode calls
the changed `isWorkflowModel` mapping to choose the SDK model for
workflow IDs. The new mapping adds chat/workflow models and can affect
selection even though the workflow discovery source itself is identical.
Prompt dispatch uses the updated provider for workflow or agentic chat.
The dependency adds workflow defaults and namespace selection, optional
shared direct-access cache and locking, and credential refresh paths.
Changes to provider requests and response finish mappings can affect the
stream from which OpenCode derives failures, completion and usage.
Static inspection cannot establish credential audience and filesystem
effects, request routing, or the resulting catalogue, callback and
usage records in the selected configuration. The existing credential
lease covers the attached endpoint. The server owns its downstream
GitLab configuration.

### 1.19.6 history and overflow recovery

The GitLab pin and OpenCode wrapper persist. Two further changes reach
the selected `prompt_async` lifecycle.

- `packages/opencode/src/session/message-v2.ts` bounds inline images when
  building provider-bound history. When the estimated inline payload
  exceeds 25 MiB, it replaces oldest images with a text placeholder down
  to 15 MiB. The conversion changes the model context. The stored
  messages remain available for replay. Swallowtail's one-MiB per-prompt
  PNG bound does not cap accumulated or imported session history.
- `packages/llm/src/provider-error.ts` adds `request body too large` and
  `multimodal data size is too large` to overflow detection. The existing
  session processor can publish `session.error` with
  `ContextOverflowError` and return `compact` when automatic compaction
  applies. The prompt loop creates the compaction and can resume the same
  turn. When auto-compaction is disabled on an ordinary turn the
  error can still be terminal. The marker strings for both changes
  appear in the `1.19.6` Linux x64 binary and are absent at the two
  earlier points.

Swallowtail treats every named `session.error` except
`MessageAbortedError` as terminal `ProviderFailed`. Its turn pump stops
before the server's later usage and idle records. The synthetic recovery
fixture demonstrates this with the existing interactive driver. It is
source-derived adapter evidence. No upstream executable, provider
session, login, installation or live provider contact was used.

## Decision and adaptation

Keep the `opencode.http.server-window-1` ceiling at `1.19.4` on
`opencode.http-sse.surface-19`. GitLab's selected effects block the
first hop. The second also needs an overflow-recovery decision.

The version-currentness planner owns an HTTP-only adaptation under
[Contract 029](../knowledge/contracts/029-interface-version-qualification-and-compatibility.md#no-terminal-stop--2026-09-24),
continuing the HTTP proof from Research 464's lead
`5b445586-3c98-47c5-aa88-7950aace016b`. Use the frozen native digests to
qualify both hops with approved isolated fake services. Exercise the
GitLab catalogue, credential audience, cache and refresh boundaries,
and prompt/SSE usage, failure and callbacks. For `1.19.6`, exercise
both recovering and fatal overflow, history image replacement, turn
cancellation, and structured-session cleanup. Establish the terminal
signal before changing the fold. The same error name can precede both
outcomes, so a blanket rule to ignore the error would lose genuine
failures. Bring any consumer-visible lifecycle change or narrowing to
the operator.

## Sources

- [npm publication history and wrapper metadata](https://registry.npmjs.org/opencode-ai)
- [OpenCode v1.19.4](https://github.com/anomalyco/opencode/releases/tag/v1.19.4), [v1.19.5](https://github.com/anomalyco/opencode/releases/tag/v1.19.5), [v1.19.6](https://github.com/anomalyco/opencode/releases/tag/v1.19.6)
- [OpenCode 1.19.5 to 1.19.6 source comparison](https://github.com/anomalyco/opencode/compare/v1.19.5...v1.19.6)
- [GitLab provider npm package metadata](https://registry.npmjs.org/gitlab-ai-provider)
