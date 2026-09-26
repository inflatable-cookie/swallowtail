# 003 Delegated ACP Authentication Activation

Status: promoted and archived
Owner: Tom
Updated: 2026-07-30

## Purpose

Decide whether an ACP driver may activate one already authorized,
harness-owned credential without gaining permission to sign in, switch access
mechanism, expose secrets, or choose a billing route.

## Trigger

Exact Grok Build `0.2.111` initialization advertises `grok.com` and rejects
`session/new` until the client sends an authentication method id. Current
public xAI ACP documentation instead expects `cached_token` or `xai.api_key`.

Existing delegated-auth contracts permit a host-approved harness credential
lease. They do not prove that an arbitrary ACP `authenticate` request is
activation-only. The same provider method may open login, refresh or replace
credentials, invoke an external auth helper, or select API-key access.

## Proposed Boundary

A future delegated authentication activation record would bind:

- exact driver behavior revision and executable version
- exact configured access profile and endpoint audience
- one existing delegated credential lease
- one adapter-private provider authentication method
- whether provider-owned refresh of the same credential is allowed
- sign-in, credential replacement, account switching, and mechanism fallback
  prohibited

Activation would occur only after ACP initialization and before session
allocation. Missing, renamed, ambiguous, interactive, expired, rejected, or
mechanism-changing behavior would fail the attachment and join all owned work.
It would not trigger another method, browser, device flow, terminal action,
external auth command, API key, or provider route.

Raw method metadata, token state, credential paths, account identity, and
provider payloads would remain adapter-private and absent from stable
diagnostics.

## Grok Candidate

The first candidate remains:

- exact direct executable `0.2.111`
- `--no-auto-update agent stdio`
- ACP wire version 1
- pre-existing Grok subscription OAuth
- `AmbientHost`
- ambient harness configuration
- durable local Grok state
- no bounded read-only, sandbox, or containment claim

The npm launcher, API-key access, login, logout, device auth, external auth
providers, custom endpoints, and automatic update remain excluded.

## Open Evidence

- which authentication method id appears with an existing OAuth credential
- whether one `authenticate` request can be activation-only
- whether it refreshes or rewrites the existing credential
- whether missing or expired state opens an interactive flow
- whether successful authentication plus `session/new` creates any provider
  request before a prompt
- exact state mutations and failure cleanup

## Probe Attempt

The operator authorized the narrow no-prompt probe on 2026-07-24. The exact
`0.2.111` executable is available and matches the frozen artifact, but the host
has no installed Grok command and no default Grok state directory. The
pre-existing delegated credential required by this spec is therefore absent.

The attempt stopped before agent launch, authentication, or session
allocation. No credential file was read and no login, provider request, model
request, or API-key fallback occurred. This proves the host precondition is
unavailable; it does not resolve the activation lifecycle.

## Decision Gate

Resolve by one of:

1. a separately gated, operator-authorized, no-prompt probe against an already
   authenticated exact artifact; authorization is granted, but matching state
   is not currently present
2. maintained xAI documentation that matches the exact current artifact

Do not infer the result from bundled strings, stale auth-method ids, registry
presence, or semver. Do not change to API-key access without a separate
operator decision.

## Promotion Targets

- durable activation rule to a new contract only after exact evidence
- provider-neutral records only if two adapters or one clear shared lifecycle
  justify them
- Grok private mapping and deterministic corpus to roadmap 047

## Resolution

Exact authenticated Grok Build `0.2.114` advertised `cached_token`, completed
one headless activation against existing subscription state, left the
credential file unchanged, and permitted `session/new` without login,
API-key fallback, prompt, or model request.

Research 070 records the exact evidence. Contract 015 now governs the narrow
shared activation boundary. Provider-private method selection stays inside
the adapter.
