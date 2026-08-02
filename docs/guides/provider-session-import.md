# Provider Session Catalogue And Explicit Import

Use this flow when a consumer wants to browse harness-origin sessions under one
approved working resource, import one explicitly, replay its history, then
continue it. Swallowtail does not synchronize harness and consumer databases.

## Supported Routes

The complete production profile exists on exactly three routes:

| Route | Catalogue | Import | Replay | Continue |
| --- | --- | --- | --- | --- |
| `codex.app-server` | prepared cwd-scoped thread catalogue | read-before-bind exact revalidation | ordinary `load_session` | ordinary `resume_session` |
| `kimi-code.acp` | prepared resource/state-root ACP catalogue | repeat-list exact revalidation | ordinary `load_session` | ordinary `resume_session` |
| `opencode.http` | prepared directory/status HTTP catalogue | health/lookup/status exact revalidation | ordinary `load_session` | ordinary `resume_session` |

Another transport from the same provider does not inherit support. In
particular, Codex exec and Kimi headless do not become import routes.

## Prepared Flow

1. Prepare one adapter integration with explicit host, target, access, and
   exact version evidence.
2. Prepare one `ProviderSessionCatalogue` operation with an opaque catalogue
   id, one working resource, portable page/traversal/content bounds, and an
   optional deadline.
3. Call `list_sessions`. Follow only cursors created by that prepared
   catalogue. Treat every candidate as display evidence, not authority.
4. Let the consumer select one candidate whose import availability is
   `Available`.
5. Prepare the route-specific import operation with that catalogue, that
   candidate, an explicit future model route, the same working resource, and
   the intended session policy.
6. Call `import_session`. A changed, missing, active, archived, incompatible,
   cross-resource, cross-host, or cross-version candidate returns no binding.
7. Pass the returned ordinary `SessionResumeBinding` to the matching prepared
   session's `load_session` for bounded ordered replay.
8. After the consumer persists its local projection, continue through the
   loaded handle or call `resume_session` when replay is not needed.

The route-specific, compile-tested prepared examples are:

- [Codex browse, import, and load](../../crates/swallowtail-adapter-codex/examples/prepared_discovery.rs)
- [Kimi ACP browse, import, load, and resume](../../crates/swallowtail-adapter-kimi/examples/prepared_acp.rs)
- [OpenCode browse, import, load, and resume](../../crates/swallowtail-adapter-opencode/examples/prepared_opencode_attached.rs)

## Consumer Boundary

The consumer owns:

- refresh timing and user selection
- local thread creation, title, persistence, and provider-binding mapping
- replay/event deduplication and duplicate-import policy
- unsupported, stale, incomplete-history, and reauthorization presentation
- authorization for any later provider-session management action

Swallowtail owns bounded observation, exact revalidation, the imported binding,
ordered replay transport, continuation transport, diagnostics, cancellation,
deadlines, and joined cleanup.

Never construct a binding from a copied session id. Never widen a catalogue
from one working resource to an account, state root, project set, or filesystem
scan. Never treat import as archive, delete, locking, active-writer detection,
or automatic synchronization.

## Operation Separation

| Operation | Result | Does not imply |
| --- | --- | --- |
| catalogue | bounded candidates and optional cursor | attachment authority |
| import | one exact route-bound resume binding | history replay or provider mutation |
| load | bounded replay plus ready handle | background synchronization |
| resume | ready handle without replay | local persistence |
| provider management | route-specific archive, restore, or delete result | import, consumer deletion, or UI policy |

Discovery-only and attachment-only routes remain visible as partial rather
than being flattened into support. See the provider route matrix for their
exact promotion gates.

Nucleus adoption should follow the bounded
[external-thread import handoff](../releases/0.1.0-nucleus-provider-session-import-handoff.md).
It defines the in-process binding map, replay-to-live boundary, duplicate and
restart posture, deterministic fixtures, and unsupported-route UX without
moving consumer persistence into Swallowtail.
