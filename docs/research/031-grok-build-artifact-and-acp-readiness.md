# 031 Grok Build Artifact And ACP Readiness

Status: promoted
Owner: Tom
Updated: 2026-07-24

## Question

Which Grok Build releases can Swallowtail qualify for the first ACP harness
route, and does the planned restrictive read-only, delegated-subscription
boundary survive exact artifact inspection?

## Method

Evidence was accessed 2026-07-24.

- snapshotted the public npm metadata for `@xai-official/grok` and
  `@xai-official/grok-darwin-arm64`
- snapshotted the live ACP registry aggregate and Grok entry
- downloaded the exact `0.2.0` and `0.2.111` launcher and darwin-arm64
  tarballs directly from the public registry
- verified tarball, launcher, compressed binary, and executable SHA-256
  digests
- verified the macOS Developer ID signature and team
- decompressed but did not install the platform executables
- ran only `--version`, help, ACP `initialize`, and an unauthenticated
  `session/new`
- used empty temporary `HOME` and `GROK_HOME`, `--no-auto-update`, and
  host-denied network access

No package manager, installer, update, ambient credential store, login,
`authenticate` request, model prompt, provider request, or paid inference was
used. Volatile host paths, hostnames, agent ids, and process-instance ids were
removed from the committed transcript.

## Publication Inventory

The launcher and darwin-arm64 platform registries both contain 111 matching
`0.2.x` points:

- `0.2.0..=0.2.47`
- `0.2.49..=0.2.111`
- `0.2.48` is absent

The launcher package marks `0.2.111` as both `latest` and `alpha`. The platform
package marks `0.1.220` as `latest` and `0.2.111` as `alpha`. Package semver,
distribution channel, platform artifact, ACP registry version, and executable
version therefore remain separate evidence.

The live ACP registry entry pins `@xai-official/grok@0.2.111` with
`agent stdio`. It labels the agent proprietary. The exact `0.2.111` npm
packages label themselves Apache-2.0. These values remain source-scoped.

This inventory corrects Research 030: the `0.2.x` points are published exact
versions, not one established stable channel or supported interval.

## Exact Artifact Evidence

### `0.2.0`

- executable output: `grok 0.2.0 (d89b1a2fa7a)`
- executable SHA-256:
  `be4db9c6dd288dce2c5d8f130421769872046e5208b6c6457679e692286dfd57`
- bundled ACP SDK: `0.6.0`
- bundled ACP schema: `0.5.0`
- ACP wire: version 1
- model: `grok-build-latest`
- `--version` created 21 files and 200381 bytes under empty `GROK_HOME`

### `0.2.111`

- executable output: `grok 0.2.111 (94172f2aa4e5)`
- executable SHA-256:
  `e1fafdfffe14f339460befaf194360e8f90bfd02efe8a4f24cfa1c7aea657ffe`
- bundled ACP SDK: `0.10.4`
- bundled schema version: not safely observable
- ACP wire: version 1
- model: `grok-4.5`
- reasoning efforts: `low`, `medium`, and `high`
- `--version` created no state under empty `GROK_HOME`

Both artifacts are signed by Developer ID team `5Y6N3AJ54S`. The exact
transition points for version-probe state, SDK, model, and reasoning behavior
are unknown. No interval can be inferred between the two inspected points.

Contract 032 discovery is viable only for the direct `0.2.111` executable and
the exact `--no-auto-update --version` command. The npm launcher is not a
discovery target: its documented code may decompress and materialize an
executable under `GROK_HOME`.

## ACP Evidence

Both exact artifacts accepted initialization with:

- ACP wire version 1
- `fs.readTextFile = true`
- `fs.writeTextFile = false`
- terminal capability false

Both advertised only the authentication method id `grok.com` when no
credential was present. Both rejected `session/new` with code `-32000`,
`Authentication required`, and `no auth method id provided`.

Current public xAI ACP documentation instead selects `xai.api_key` or
`cached_token`, then calls `authenticate`. The exact unauthenticated
`0.2.111` artifact returned neither id. This may be credential-dependent, but
proving that requires a separately authorized live delegated-auth probe.

Calling `authenticate` cannot be inferred to mean activation-only. The
bundled authentication guide says Grok may open a browser, run a device flow,
refresh OAuth, execute an external auth provider, or use an API key according
to state and configuration. Contract 017 does not authorize an ambiguous
sign-in or credential mutation merely because the harness advertised a method.

An unauthenticated `session/new` also created durable local session state
before returning its error. Failed attachment cannot imply provider-state
rollback or deletion.

## Access And Configuration Evidence

The planned bounded read-only claim does not survive exact bundled
documentation:

- plan mode permits its plan-file write
- plan mode does not inspect shell commands for writes
- subagents can bypass the parent plan-mode edit gate
- the CLI accepts `--permission-mode dontAsk`, but the bundled guide says that
  flag value does not enable the deny-first policy
- hooks may fail open
- Grok reads project `.grok`, `.claude`, and `.cursor` sources
- the built-in read-only command list is a convenience, not a security
  boundary

Provider permissions remain useful policy. They are not a bounded read-only
filesystem guarantee or sandbox.

The viable future route is an explicit `AmbientHost` relay with ambient
harness configuration and visible durable local state. Consumers may select
provider permissions or a separately supported sandbox. Swallowtail must not
claim that those controls contain the process or guarantee read-only behavior.

## Compatibility Decision

No release is qualified.

`0.2.111` is the sole current candidate because its direct short version probe
is side-effect free and its ACP initialization is inspectable. It is not an
unverified-newer point because there is no qualified baseline.

The committed corpus freezes:

- two exact artifact points
- the complete publication runs and missing `0.2.48`
- distribution-tag disagreement
- exact digest and signature evidence
- version-probe state behavior
- ACP SDK, wire, model, and reasoning differences
- initialization and authentication-required transcripts
- the absence of any qualified segment

## Required Decision

The subscription-backed route needs a narrow evidence gate before shared
contract or driver work:

1. authorize a separately gated probe against an already authenticated,
   host-approved Grok `0.2.111` state
2. send no prompt or model request
3. observe exact `initialize`, selected auth method, `authenticate`, and
   `session/new`
4. prove that the selected request activates only the existing delegated
   credential and does not launch sign-in, change mechanism, expose a token,
   or select API-key billing
5. record state mutation and cleanup without committing credential material

If that probe is not authorized, wait for maintained xAI documentation that
matches the exact artifact. Do not switch to API-key access implicitly.

## Promotion

- provisional decision: Spec 003
- delivery gate: roadmap 047, revised card 138
- operator hold: roadmap 047 and cards 138-141 are held because no Grok
  account is available
- production discovery and driver cards remain resumable
- no architecture or durable contract change yet

## Primary Sources

- [Grok Build CLI reference](https://docs.x.ai/build/cli/reference)
- [Grok Build headless and ACP](https://docs.x.ai/build/cli/headless-scripting)
- [Grok Build permissions](https://docs.x.ai/build/features/permissions)
- [Grok Build settings](https://docs.x.ai/build/settings)
- [ACP Registry](https://agentclientprotocol.com/get-started/registry)
- [`@xai-official/grok` package](https://www.npmjs.com/package/@xai-official/grok)
