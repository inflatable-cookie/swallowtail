# 070 Grok Build 0.2.114 Authenticated ACP Qualification

Status: promoted
Owner: Tom
Updated: 2026-07-30

## Question

Does current Grok Build support one production ACP route over an existing
subscription login without API-key fallback, interactive sign-in, or a
provider-model request?

## Method

Evidence was accessed on 2026-07-30.

- revalidated current xAI CLI, headless/ACP, enterprise authentication,
  permission, and settings documentation
- inspected npm metadata for the launcher and darwin-arm64 platform packages
- fetched the current ACP registry aggregate
- matched the installed executable to the exact public platform artifact
- verified the macOS Developer ID identity
- ran the exact installed `--no-auto-update --version` probe
- initialized ACP v1 against existing operator-provisioned Grok state
- selected only the advertised `cached_token` method
- sent `authenticate` with `_meta.headless = true`
- allocated one empty session in an empty temporary working directory
- hashed the existing authentication file before and after without reading or
  recording its contents

The live gate sent no prompt, model request, tool request, API key, login,
logout, update, installation, or direct xAI API request. Account metadata in
the authentication response was discarded and is not repository evidence.

## Current Publication Evidence

The installed direct executable reports:

- version: `0.2.114`
- source revision: `0c785038798`
- channel label: `stable`
- executable SHA-256:
  `e715f57f9018a1737c1a64ef1cb260ac2a5045dfa6a1a0e1c7a7cbe193a083b2`

That digest exactly matches the decompressed darwin-arm64 binary from
`@xai-official/grok-darwin-arm64@0.2.114`.

Artifact evidence:

| Axis | Launcher | darwin-arm64 platform |
| --- | --- | --- |
| npm version | `0.2.114` | `0.2.114` |
| tarball SHA-256 | `53915a8067f264804e61d7c5f6c776d82be3d45e9bad1c03de79072d06799775` | `de2d205c517c9fdc900146dad1a1a837cb7e696d65813c81075c3d77a2146c93` |
| npm integrity | `sha512-8eey...XPg==` | `sha512-ZTvk...Dew==` |
| compressed binary SHA-256 | not applicable | `3417664a234bd8e149dc81c8249708410d65e4e6761ac7de97c6dd9478816d8e` |

The signed executable retains team `5Y6N3AJ54S` and identifier
`xai-grok-pager`.

Registry and release channels remain separate:

- launcher `latest`: `0.2.114`
- launcher `alpha`: `0.2.116`
- platform `latest`: `0.1.220`
- platform `alpha`: `0.2.116`
- ACP registry Grok entry: `0.2.116`

Both npm packages publish matching exact points through `0.2.116`, but the
current stable launcher, platform `latest`, alpha tag, ACP registry, and local
channel observation disagree. Only exact installed `0.2.114` is qualified.

## Authenticated ACP Evidence

Exact `0.2.114` initialization returned:

- ACP wire version `1`
- agent version `0.2.114`
- load-session capability present
- one model, `grok-4.5`
- reasoning efforts `low`, `medium`, and `high`
- authentication methods `cached_token` and `grok.com`
- default method `cached_token`

The earlier exact `0.2.111` unauthenticated corpus advertised only
`grok.com`. Authentication method advertisement is therefore state-dependent
and cannot define a release-wide login route.

With the existing subscription state, one exact
`authenticate(cached_token, headless=true)` request:

- completed without browser, device-code, terminal, helper, or API-key flow
- exposed no credential bytes
- left the existing authentication file unchanged
- allowed `session/new`
- produced no stderr

The successful response carried provider-private account and entitlement
metadata. A production adapter must ignore it. It cannot expose the raw
response through stable diagnostics, infer billing authority from it, or
change the configured access profile.

## State And Lifecycle Evidence

`session/new` created one durable empty local Grok session. It also updated
Grok-owned session bookkeeping and materialized the current bundled user-guide
cache. It did not change the authentication file.

This is expected attachment state, not rollback-safe temporary state:

- provider retention: durable local
- configuration posture: ambient
- harness isolation: `AmbientHost`
- filesystem posture: provider-defined ambient authority
- sandbox: optional, not selected or implied
- deletion: not qualified

The live probe did not delete the session by path convention. Swallowtail does
not own ambient Grok state cleanup.

## Access Decision

The first route binds:

- integration family `grok-build`
- adapter driver `grok-build.acp`
- transport `acp-v1-stdio`
- execution layer `HarnessInteraction`
- operation shape `InteractiveSession`
- exact executable `0.2.114`
- exact argv `--no-auto-update agent stdio`
- `InteractiveOauth`
- `SubscriptionAllowance`
- Grok subscription endpoint audience
- provider-supported authority
- one pre-existing delegated harness credential
- adapter-private activation method `cached_token`

Activation is not login. The driver may send `authenticate` only when exact
initialization advertises `cached_token` for the bound qualified behavior
revision. Missing, renamed, rejected, or interactive behavior fails
attachment. It cannot try `grok.com`, `xai.api_key`, an external helper, or
another route.

Provider-owned refresh of the same delegated OAuth mechanism remains harness
behavior. It does not authorize account switching, credential extraction,
mechanism fallback, or API billing.

## Compatibility Decision

Qualify exact `0.2.114` as the baseline and latest guaranteed release.

- `0.2.0` and `0.2.111` remain historical incompatible evidence
- `0.2.112` and `0.2.113` remain uninspected, not inferred milestones
- later stable observations may be attempted only under explicit
  unverified-newer posture
- prerelease or alpha observations remain incompatible
- no continuous older range is claimed

The route can widen later only through exact behavior milestones. Package
semver or ACP wire version alone is insufficient.

## Contract Fit

No new provider-neutral record is justified.

- Contract 014 already separates delegated harness authentication from secret
  leases and direct API access.
- Contract 015 needs one narrow activation rule after ACP initialization.
- Contract 023 already represents ambient authority and optional sandboxing.
- Contracts 029 and 032 already cover exact observation, guaranteed support,
  and visible unverified-newer attempts.
- Contract 033 already binds ambient harness configuration.

Provisional Spec 003 can be archived after the Contract 015 rule is promoted.

## Primary Sources

- [Grok Build overview](https://docs.x.ai/build/overview)
- [Grok Build CLI reference](https://docs.x.ai/build/cli/reference)
- [Grok Build headless and ACP](https://docs.x.ai/build/cli/headless-scripting)
- [Grok Build enterprise authentication](https://docs.x.ai/build/enterprise)
- [Grok Build permissions](https://docs.x.ai/build/features/permissions)
- [Grok Build settings](https://docs.x.ai/build/settings)
- [ACP registry](https://agentclientprotocol.com/get-started/registry)
- [`@xai-official/grok`](https://www.npmjs.com/package/@xai-official/grok)
