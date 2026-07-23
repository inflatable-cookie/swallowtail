# 026 Codex Six-Month Legacy Span Feasibility

Status: promoted
Owner: Tom
Updated: 2026-07-23

## Question

Can one Swallowtail release support Codex installations from January through
the current July release without weakening the existing isolated exec route or
inventing an app-server facade break?

## Method

Sources were accessed 2026-07-23. Evidence includes official release records,
the official npm publication history, exact tagged source, generated protocol
schemas, and current Codex documentation. No login, credential, provider
request, binary installation, container, model download, or consumer edit was
used.

The audit compares executable release, invocation, protocol facade, behavior,
configuration posture, provider retention, sandbox posture, and capability
subset independently.

## Current Point

`0.145.0`, published 2026-07-21, remains the latest stable Codex release.
Prereleases and unknown newer versions remain outside every claim.

The current maintained windows remain valid:

- exec `0.122.0..=0.145.0`
- app-server `0.110.0..=0.130.0`
- app-server with runtime workspace roots `0.131.0..=0.145.0`

Legacy work adds deprecated-but-supported segments. It does not widen these
behavior revisions by inference.

## App-Server Correction

Research 025 treated releases before `0.110.0` as a likely v1-only span. Exact
tagged source disproves that assumption.

The selected v2 methods and notifications already exist at `0.80.0`:

- `initialize`
- `model/list`
- `thread/start`
- `thread/resume`
- `turn/start`
- `turn/interrupt`
- thread, turn, item, and agent-message notifications

The v1 surface coexists in early releases, but Swallowtail does not need to use
it. The protocol facade remains `codex-app-server-v2`.

The actual milestones are private transport and capability behavior:

| Release | Published evidence | Required mapping |
| --- | --- | --- |
| `0.80.0` | 2026-01-09; selected v2 source exists | default stdio invocation; stable read-only subset |
| `0.81.0` | 2026-01-14 | same behavior |
| `0.82.0`, `0.83.0` | source tags but no stable npm publication | explicit exclusions |
| `0.84.0` | 2026-01-15 | first accepted point after the publication gap |
| `0.94.0` | 2026-02-02; generated v2 schemas enter the tag | corpus milestone, not a facade change |
| `0.99.0` | 2026-02-11 | last default-stdio point |
| `0.100.0` | 2026-02-12 | `--listen stdio://` invocation begins |
| `0.107.0` | 2026-03-02 | last published point before the next gap |
| `0.108.0`, `0.109.0` | source tags but no stable npm publication | existing explicit exclusions |
| `0.110.0` | 2026-03-05 | current base behavior begins |

The legacy app-server claim should therefore add:

- `0.80.0..=0.81.0`, deprecated, default stdio, stable read-only
- `0.84.0..=0.99.0`, deprecated, default stdio, stable read-only
- `0.100.0..=0.107.0`, deprecated, explicit stdio listener, stable read-only

Dynamic tools, observed provider requests, and bounded workspace roots remain
unavailable in these legacy segments. Exact configured-instance capabilities
must express that narrower subset. There is no v1 driver and no protocol
fallback.

## Exec Behavior

The selected JSONL events remain available from `0.80.0`. Invocation and state
behavior divide the legacy span:

| Segment | Invocation behavior | Required policy |
| --- | --- | --- |
| `0.80.0..=0.81.0` | no `--ephemeral`; legacy boolean web-search configuration | ambient configuration; durable local retention accepted |
| `0.84.0..=0.98.0` | no `--ephemeral`; top-level web-search mode | ambient configuration; durable local retention accepted |
| `0.99.0..=0.121.0` | `--ephemeral`; no config or rules suppression | ambient configuration; provider retention prohibited |
| `0.122.0..=0.145.0` | `--ephemeral`, `--ignore-user-config`, and `--ignore-rules` | suppressed configuration; provider retention prohibited |

`0.82.0` and `0.83.0` remain excluded because no stable package release was
published. Every legacy segment keeps explicit read-only provider sandbox,
approval, model, output, environment-inheritance, search, and JSONL arguments.
Ambient configuration means user or project configuration and rules may still
affect harness behavior. It does not mean those effects are Swallowtail-owned.

Using a temporary `CODEX_HOME` is not the recommended fix. Codex configuration,
authentication, history, logs, and other state share that root. Exact tagged
source binds file and keychain authentication to the selected home. Moving the
home would either lose the user's existing login or require Swallowtail or the
host to copy, expose, or re-key private authentication state. That is neither
seamless nor authorized.

## Missing Shared Contract

Current records separate harness process isolation and provider retention but
do not bind whether a harness may load ambient user or project configuration
and rules.

Add an operation-shape-neutral harness-configuration posture:

- `Ambient` — the harness may load its existing user and project configuration,
  rules, context, extensions, and provider-owned defaults
- `ProviderSuppressed` — an exact qualified invocation suppresses ambient user
  and project configuration and rules while leaving credential authority
  separate
- `HostScoped` — reserved for a separately leased host-owned configuration
  root; not required by the first legacy proof

The posture must be bound in requirements, request policy, configured-instance
capability, and preflight. It remains separate from `AmbientHost`,
`ProviderEnforced`, or `HostEnforced` process isolation; provider retention;
credential access; and working-resource policy. No failed suppressed or scoped
route may retry as ambient.

## Recommendation

Support the full six-month span without a container or copied credential home:

1. add the shared configuration-posture contract and records
2. freeze exact legacy corpora at every boundary and milestone
3. add private version dispatch inside the existing exec and app-server drivers
4. publish legacy segments as deprecated-but-supported
5. expose their narrower capabilities and policy requirements to consumers
6. keep route selection and warning policy downstream

This is not an implicit fallback. A consumer must select a configured instance
whose exact version, configuration posture, retention posture, and capability
set match its request. Unknown versions still fail before harness work.

## Proof Corpus

Default QA must cover:

- exec `0.80.0`, `0.81.0`, `0.84.0`, `0.94.0`, `0.98.0`, `0.99.0`,
  `0.100.0`, `0.110.0`, `0.121.0`, and the current `0.122.0` boundary
- app-server `0.80.0`, `0.81.0`, `0.84.0`, `0.94.0`, `0.99.0`, `0.100.0`,
  `0.107.0`, and the current `0.110.0` boundary
- exact rejection of unpublished `0.82.0`, `0.83.0`, `0.108.0`, and `0.109.0`
- exact policy rejection when ambient configuration or durable retention is
  not accepted
- unchanged current ranges through `0.145.0`

Tagged source may generate missing historical schemas, but the generated
fixtures must record the exact source commit and generation command. They must
not be treated as upstream-published artifacts.

## Risks

- app-server remains an experimental development surface
- legacy exec permits ambient configuration and rules by design
- releases before `0.99.0` may retain local transcript state
- a supported executable version does not prove authentication, entitlement,
  catalogue freshness, or model availability
- consumers decide whether deprecated legacy posture is acceptable; Swallowtail
  supplies no automatic upgrade, fallback, or default route

## Sources

- [Codex CLI reference](https://developers.openai.com/codex/cli/reference/)
- [Codex app-server protocol](https://developers.openai.com/codex/app-server/)
- [Codex npm package history](https://www.npmjs.com/package/@openai/codex?activeTab=versions)
- [Codex `0.80.0`](https://github.com/openai/codex/releases/tag/rust-v0.80.0)
- [Codex `0.94.0`](https://github.com/openai/codex/releases/tag/rust-v0.94.0)
- [Codex `0.99.0`](https://github.com/openai/codex/releases/tag/rust-v0.99.0)
- [Codex `0.100.0`](https://github.com/openai/codex/releases/tag/rust-v0.100.0)
- [Codex `0.110.0`](https://github.com/openai/codex/releases/tag/rust-v0.110.0)
- [Codex `0.121.0`](https://github.com/openai/codex/releases/tag/rust-v0.121.0)
- [Codex `0.122.0`](https://github.com/openai/codex/releases/tag/rust-v0.122.0)
- [Codex `0.145.0`](https://github.com/openai/codex/releases/tag/rust-v0.145.0)
- [Codex `0.80.0` app-server request map](https://github.com/openai/codex/blob/rust-v0.80.0/codex-rs/app-server-protocol/src/protocol/common.rs)
- [Codex `0.80.0` app-server v2 types](https://github.com/openai/codex/blob/rust-v0.80.0/codex-rs/app-server-protocol/src/protocol/v2.rs)
- [Codex `0.80.0` exec CLI](https://github.com/openai/codex/blob/rust-v0.80.0/codex-rs/exec/src/cli.rs)
- [Codex `0.99.0` exec CLI](https://github.com/openai/codex/blob/rust-v0.99.0/codex-rs/exec/src/cli.rs)
- [Codex `0.121.0` exec CLI](https://github.com/openai/codex/blob/rust-v0.121.0/codex-rs/exec/src/cli.rs)
- [Codex `0.122.0` exec CLI](https://github.com/openai/codex/blob/rust-v0.122.0/codex-rs/exec/src/cli.rs)
