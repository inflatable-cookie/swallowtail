# 093 Codex Thread Catalogue And Import Range

Status: promoted
Owner: Tom
Date: 2026-08-01

## Question

Which exact releases inside Swallowtail's qualified Codex app-server range can
support resource-scoped discovery, read-only revalidation, history replay, and
continuation of harness-origin threads?

## Method

The audit inspected the official `openai/codex` tagged protocol and app-server
handler source at every relevant behavior milestone in
`0.80.0..=0.146.0`. NPM publication time and the peeled tag commit identify
each stable release. Existing Swallowtail lifecycle and session-continuity
corpora supplied the already-qualified gaps and resume segments.

The current Codex app-server documentation was checked separately on
2026-08-01. It corroborates present feasibility only. Its broader filters,
pagination, source defaults, and thread fields were not projected backward.

No provider executable, authentication flow, stored session, prompt,
filesystem mutation, or consumer repository was used.

## Required Closure

The first production profile needs all of these on one exact release:

- cursor-paginated `thread/list`
- explicit interactive-source filtering
- exact working-directory filtering
- `thread/read` without resuming
- `includeTurns: true` history
- runtime status on list and read results
- existing `thread/resume` continuation

List results are candidates, not bindings. Runtime status is required because
the first import profile must observe and reject an active or unhealthy target
before issuing attachment authority.

## Historical Boundaries

| Release | Change relevant to import | Result |
| --- | --- | --- |
| `0.80.0` | `thread/list`, cursor pagination, and `thread/resume` exist | no read-only revalidation |
| `0.91.0` | `thread/read`, `includeTurns`, and `updatedAt` appear | no source or cwd filter; no status |
| `0.92.0` | `sourceKinds` appears | no cwd filter or status |
| `0.103.0` | exact `cwd` list filter appears | no runtime status |
| `0.104.0` | selected list/read shape remains status-free | still incomplete |
| `0.105.0` | `Thread.status` plus list/read status population appear | first complete release |
| `0.107.0` | `searchTerm` appears | additive; not selected |
| `0.110.0` | maintained v2 behavior segment begins | selected profile unchanged |
| `0.123.0` | list sort direction appears | additive; not selected |
| `0.129.0` | `thread/resume.excludeTurns` appears | private resume dispatch milestone |
| `0.130.0` | v2 thread protocol splits into focused modules | wire profile unchanged |
| `0.131.0` | runtime workspace roots appear | separate existing capability segment |
| `0.139.0` | initial-turn pagination data appears | additive; not selected |
| `0.144.6` | experimental parent and ancestor filters appear | excluded from first profile |
| `0.145.0` | direct-input state appears | additive; not selected |
| `0.146.0` | pin state and filter appear | additive; not selected |

The frozen corpus records each publication time, peeled 40-character tag
commit, protocol layout, and selected feature state. `0.105.0` is not inferred
from the schema alone: its app-server handler populates statuses for
`thread/list` and resolves status again for `thread/read`.

## Guaranteed Segments

The catalogue/import operation may be guaranteed only for:

- `0.105.0..=0.107.0`
- `0.110.0..=0.128.0`
- `0.129.0..=0.130.0`
- `0.131.0..=0.146.0`

The existing excluded gaps remain `0.108.0..=0.109.0`. Versions above
`0.146.0` remain visible as unverified newer and are not hard-denied solely for
being newer.

Codex `0.80.0..=0.104.0`, excluding existing gaps, remains supported for its
already-qualified app-server operations. It simply does not advertise the
new catalogue/import capability. This is an operation-specific floor, not a
new general Codex baseline.

## Selected Wire Profile

The first list request sends a positive bounded `limit`, the prior opaque
cursor when present, exact `cwd`, `archived: false`, and explicit
`sourceKinds: ["cli", "vscode", "appServer"]`.

Explicit source selection avoids version-dependent defaults. One-shot `exec`,
subagent variants, and unknown sources are excluded. `path`, Git data, agent
metadata, ancestry, history metadata, and pin state remain adapter-private.

Portable candidate content is limited to bounded title/name, preview, update
time, source, resource, and activity state. The provider thread id remains a
private reference. `nextCursor` remains opaque and plan-bound.

Import performs `thread/read` with `includeTurns: true`, then revalidates exact
thread id, cwd, accepted source, and runtime status. Only `notLoaded` or `idle`
targets are eligible. Missing, changed, active, or unhealthy targets issue no
binding. Successful import enters the existing `thread/resume` load and resume
paths; it does not invent another continuation method.

## Current Documentation Delta

Current documentation still describes `thread/list`, `thread/read` with
optional turns, runtime status, and `thread/resume`. It also documents newer
fields and behaviors absent from parts of the qualified historical range,
including pin state, array-valued cwd, additional sort keys, experimental
ancestry filters, and narrower default interactive sources.

Those current additions remain corroboration or future evidence. The first
profile uses only the tagged-source subset frozen above.

## Decision

Select `0.105.0` as the catalogue/import floor. Preserve four exact behavior
segments so the existing resume codec can keep its pre- and post-
`excludeTurns` handling. Keep all production capability changes in card 053.

## Risks

- list metadata can become stale before read revalidation
- another Codex client can activate a thread after the status check
- history can exceed common operation bounds
- historical source defaults differ from current documentation
- later additive thread fields must not leak into portable records
- unverified-newer execution may encounter a real breaking change

## Promotion

- exact normalized evidence: Codex compatibility corpus
- operation-specific production claim: card 053
- deterministic and package acceptance: card 054
- durable portable boundaries: Contracts 017, 029, 037, and 046 remain
  unchanged

## Primary Sources

- [Codex `0.80.0` monolithic v2 protocol](https://github.com/openai/codex/blob/f0679a6ab2563135e01cc922698cb54921a1719f/codex-rs/app-server-protocol/src/protocol/v2.rs)
- [Codex `0.104.0` v2 protocol](https://github.com/openai/codex/blob/74d1f7b2b3af383bd3605344f3c842b194fd1d70/codex-rs/app-server-protocol/src/protocol/v2.rs)
- [Codex `0.105.0` v2 protocol](https://github.com/openai/codex/blob/a7eda6a29b3ee25549f385197ff109508dc49a90/codex-rs/app-server-protocol/src/protocol/v2.rs)
- [Codex `0.105.0` app-server handler](https://github.com/openai/codex/blob/a7eda6a29b3ee25549f385197ff109508dc49a90/codex-rs/app-server/src/codex_message_processor.rs)
- [Codex `0.129.0` v2 protocol](https://github.com/openai/codex/blob/2808a4deb181e5ca2b1293a1a5980938cb746861/codex-rs/app-server-protocol/src/protocol/v2.rs)
- [Codex `0.130.0` split thread protocol](https://github.com/openai/codex/blob/58573da43ab697e8b79f152c53df4b42230395a8/codex-rs/app-server-protocol/src/protocol/v2/thread.rs)
- [Codex `0.146.0` split thread protocol](https://github.com/openai/codex/blob/e363b08c9175ac1cbe5893615dd2cb9ddf95043b/codex-rs/app-server-protocol/src/protocol/v2/thread.rs)
- [Current Codex app-server documentation](https://developers.openai.com/codex/app-server/)
- [NPM package record](https://www.npmjs.com/package/@openai/codex)
- Swallowtail Research 025, 037, 052, and 092
- `app-server-thread-catalogue.json`
