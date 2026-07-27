# 037 Codex App-Server Lifecycle Range Evidence

Status: promoted
Owner: Tom
Date: 2026-07-27

## Question

Which Codex app-server releases in Swallowtail's qualified
`0.80.0..=0.145.0` executable range support archive, restore, and hard
deletion, and what effect truth can Swallowtail promise at each point?

## Method

Evidence came from official `openai/codex` source tags and npm publication
records.

The review:

1. scanned every stable source tag from `0.80.0` through `0.145.0` for the v2
   request variants
2. checked the adjacent absent and present tags at each lifecycle boundary
3. inspected tagged app-server handlers and tests for response, notification,
   target-state, descendant, and failure behavior
4. generated aggregate protocol schemas from source where tagged schemas were
   not published
5. hashed the four aggregate JSON schemas at every existing app-server and
   lifecycle checkpoint
6. inspected current main separately without projecting its behavior backward

The generated-schema command was:

```sh
cargo run --manifest-path codex-rs/Cargo.toml \
  -p codex-app-server-protocol --bin export -- \
  --out <output-dir>
```

The deterministic corpus records exact commits, npm dates, schema authority,
schema hashes, boundaries, exclusions, and private behavior revisions:

`crates/swallowtail-adapter-codex/tests/fixtures/compatibility/app-server-lifecycle-releases.json`

## Method Boundaries

| Behavior | Last absent | First present |
| --- | --- | --- |
| `thread/archive` | before maintained baseline | `0.80.0` |
| `thread/unarchive` | `0.91.0` | `0.92.0` |
| archive and unarchive notifications | `0.103.0` | `0.104.0` |
| spawned-descendant archive attempt | `0.122.0` | `0.123.0` |
| `thread/delete` | `0.139.0` | `0.140.0` |

Archive therefore exists across every qualified Codex app-server segment.
Restore and delete do not.

The existing gaps remain unchanged:

- `0.82.0..=0.83.0` are outside the qualified app-server claim
- `0.108.0..=0.109.0` are source tags without published npm releases
- prereleases remain rejected
- later stable releases may run only as visible unverified-newer attempts

Lifecycle qualification does not shrink the existing session range. Versions
without restore or delete remain supported for their already-qualified session
operations.

## Archive

Every qualified segment accepts:

```json
{"method":"thread/archive","params":{"threadId":"<bound-thread-id>"}}
```

Success returns `{}`.

Tagged behavior establishes:

- archive moves the target rollout from the active session store to the
  archived store
- a fresh thread whose rollout has not materialized fails
- an already archived or unknown target fails rather than returning an
  already-in-state success
- app-server can remove a loaded target before moving it
- Contract 038 still requires caller-asserted inactive evidence; Swallowtail
  must not discover, steal, or close an active handle implicitly

From `0.104.0`, success emits `thread/archived` after the response.

From `0.123.0`, app-server also attempts spawned descendants. Tagged tests
explicitly allow the root archive to succeed when a descendant archive fails
or a descendant is missing. Notifications identify only successful moves.

The portable guarantee is therefore always `TargetOnly`. Descendant archive
is a visible provider side effect, not a guaranteed
`ProviderDefinedDescendants` result.

## Restore

`thread/unarchive` begins at `0.92.0`:

```json
{"method":"thread/unarchive","params":{"threadId":"<bound-thread-id>"}}
```

Success returns the restored `thread`. From `0.104.0`, it also emits
`thread/unarchived` after the response.

Tagged behavior establishes:

- the archived rollout moves back to the active session store
- an unarchived or unknown target fails
- descendants are not recursively restored
- restore remains distinct from load and resume

The guaranteed scope is `TargetOnly`.

## Hard Delete

`thread/delete` begins at `0.140.0`:

```json
{"method":"thread/delete","params":{"threadId":"<bound-thread-id>"}}
```

Success returns `{}`, then emits one `thread/deleted` notification per removed
thread.

Tagged documentation and implementation establish:

- active persisted and archived targets are accepted
- a live persisted target may be deleted before a rollout file exists
- ephemeral targets are rejected
- spawned descendants are deleted before their parent
- rollout state and associated metadata are removed before success
- descendant deletion is strict rather than the archive path's best effort

This supports `ProviderHardDeleted` with
`ProviderDefinedDescendants` for the qualified delete segment.

### Missing-Target Correction

Research 036 repeated the upstream README phrase that missing rollout files
are treated as already deleted. Tagged implementation narrows that statement.

A missing rollout file is tolerated only after the root is otherwise known:

- a live persisted thread
- a state-database record
- or a root with known spawned descendants

A wholly unknown target fails with `thread not found`. Repeating deletion
after the root and its metadata are fully gone also fails. Codex does not
provide a general idempotent `TargetAlreadyAbsent` success.

The first production mapping must report these errors as failures. It must not
manufacture `TargetAlreadyAbsent` from a missing rollout file.

## Capability Segments

| Range | Guaranteed lifecycle capability | Private behavior revision |
| --- | --- | --- |
| `0.80.0..=0.81.0` | archive | `archive-response` |
| `0.84.0..=0.91.0` | archive | `archive-response` |
| `0.92.0..=0.103.0` | archive, restore | `archive-restore-response` |
| `0.104.0..=0.107.0` | archive, restore | `archive-restore-notifications` |
| `0.110.0..=0.122.0` | archive, restore | `archive-restore-notifications` |
| `0.123.0..=0.139.0` | archive, restore | `best-effort-descendant-archive` |
| `0.140.0..=0.145.0` | archive, restore, hard delete | `strict-descendant-hard-delete` |

The first four qualified ranges retain their existing deprecated or maintained
support status. The last three are maintained.

Cancellation remains `BeforeDispatchOnly`. The tagged protocol has no native
request-cancellation contract for these lifecycle effects. A lost response
after dispatch must remain `UnconfirmedAfterEffect`.

## Sources

- [Codex `0.80.0` archive protocol and tests](https://github.com/openai/codex/tree/rust-v0.80.0/codex-rs/app-server)
- [Codex `0.91.0` protocol](https://github.com/openai/codex/blob/rust-v0.91.0/codex-rs/app-server-protocol/src/protocol/v2.rs)
- [Codex `0.92.0` unarchive tests](https://github.com/openai/codex/blob/rust-v0.92.0/codex-rs/app-server/tests/suite/v2/thread_unarchive.rs)
- [Codex `0.104.0` archive tests](https://github.com/openai/codex/blob/rust-v0.104.0/codex-rs/app-server/tests/suite/v2/thread_archive.rs)
- [Codex `0.123.0` descendant archive tests](https://github.com/openai/codex/blob/rust-v0.123.0/codex-rs/app-server/tests/suite/v2/thread_archive.rs)
- [Codex `0.140.0` delete handler](https://github.com/openai/codex/blob/rust-v0.140.0/codex-rs/app-server/src/request_processors/thread_delete.rs)
- [Codex `0.140.0` delete tests](https://github.com/openai/codex/blob/rust-v0.140.0/codex-rs/app-server/tests/suite/v2/thread_delete.rs)
- [Codex `0.140.0` app-server contract](https://github.com/openai/codex/blob/rust-v0.140.0/codex-rs/app-server/README.md)
- [Official Codex npm package](https://www.npmjs.com/package/@openai/codex)

## Promotion

- the Codex adapter owns a separate lifecycle compatibility claim across the
  unchanged executable window
- the lifecycle fixture and focused tests freeze exact range evidence
- Contract 038 already represents target-only archive and restore,
  provider-defined descendant hard deletion, inactive caller authority,
  before-dispatch cancellation, and uncertain post-dispatch truth
- no shared contract change is required
- card 050 may map the three exact methods without inventing idempotency or a
  descendant archive guarantee
