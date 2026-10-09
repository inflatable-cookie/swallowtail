# Research 425: Copilot ACP Authenticated Proof Preparation

Date: 2026-10-09

## Scope

Prepare the separately approved bounded permission proof for exact Copilot CLI
ACP artifacts `1.0.80`, `1.0.81`, and `1.0.93`. This work adds offline fake
delegation and cancellation evidence, a secret-free plan schema, and a
reviewable continuation plan. It does not read real credentials, run an
original artifact, contact an authenticated endpoint, or qualify a route.

The evidence starts from [Research 404](./404-copilot-cli-acp-offline-authentication-stop.md)
and its existing exact package identities and offline harness. The selected
route remains `copilot --acp --stdio`.

## Exact artifact mapping

The existing frozen inventory binds the three Darwin ARM64 native executable
hashes. This preparation downloaded those public pinned packages into task
scratch for static inspection and checked each extracted native file against
its already recorded digest. It did not repeat the full package inventory or
launch an artifact.

For all three versions, the four-file npm wrapper inventory is unchanged. The
hash-matched `npm-loader.js` selects the platform package and forwards
`process.argv.slice(2)` to its native executable with `spawnSync`. It does not
select a model, configure authentication, or mediate ACP permission behavior.
Its exact SHA-256 is
`0ea824a86be5757533fdb092eff7050871bd7a711a46babde0ffe0e44ac5ad88` for each
version. The ACP and provider control flow is inside the native payload.

A static strings scan of the exact native files did not establish a selected
ACP model configuration path, a trustworthy observation of the underlying
Auto model, a credential delegation mechanism, a complete endpoint allowlist,
or a finite provider retry/fallback/tool-attempt bound. Bundled runtime strings
and a model-option string containing `auto` do not identify selected control
flow. These are unresolved for each version.

Current official CLI documentation describes global model options, including
Auto, and generic authentication methods. It is not pinned to these artifacts
and does not establish their ACP configuration path or the account's active
entitlement and credential mechanism. The [CLI command reference](https://docs.github.com/en/copilot/reference/copilot-cli-reference/cli-command-reference),
[programmatic reference](https://docs.github.com/en/copilot/reference/copilot-cli-reference/cli-programmatic-reference),
and [authentication guide](https://docs.github.com/en/copilot/how-tos/copilot-cli/set-up-copilot-cli/authenticate-copilot-cli)
are context only.

| Exact question | Preparation result | Gate owner | Smallest next proof |
| --- | --- | --- | --- |
| Does each artifact accept Auto through the selected ACP session configuration? | Unresolved; no exact selected path was recoverable from the wrapper or static native strings. | Frozen artifact maintainer | Show the exact version's ACP configuration control flow; otherwise stop before prompt if Auto cannot be bound. |
| Can the selected underlying model be observed before the action? | Unresolved; no exact observation field or reliability guarantee was established. | Frozen artifact maintainer | Identify an exact selected-path observation and demonstrate mismatch fails before action. |
| Which credential mechanism and authenticated audiences does the exact CLI use? | Unresolved; generic current docs cannot establish this older artifact's path. | Account owner, host authentication owner, and frozen artifact maintainer | Attest the existing login mechanism and exact audience set without revealing a secret; prove the fake egress policy rejects any other audience. |
| Are provider retries, model fallbacks, permission requests, and tool attempts finite? | Unresolved; the static scan did not establish a selected control or cap. | Frozen artifact maintainer | Show an exact shipped bound or disabling control for each selected path; if a bound is unavailable, do not start that original artifact. |
| Does the existing account have the needed entitlement and policy? | The operator-reported login and account reference are recorded; entitlement and organization policy remain unattested. | Tom, as account owner | Provide a secret-free entitlement and policy attestation. |
| Can host containment allow the proven audiences without exposing host home or keychain? | Fake deny-all containment passed; the future exact-audience profile is only proposed. | Host containment owner | Review an exact-audience proxy profile with host-home/keychain denial, task-scratch-only writes, and bounded joined cleanup. |

The reported local Gemma 12B selection is a separate path and cannot satisfy
this GitHub-hosted proof. Auto remains only a candidate until its exact
selection and model identity are bound.

## Fake delegation and permission proof

The offline harness now validates the committed plan against its JSON Schema
and exercises a fake host-owned broker. The broker returns a synthetic
one-use capability over an inherited private pipe. The child receives no
credential environment variable; the capability is not written to a file,
log, or record. The fake agent advertises `copilot-login` and accepts the
ACP `authenticate` exchange by method ID after it consumes the synthetic
capability. The fake process runs with network denied, host-home reads denied,
keychain access denied, and writes restricted to task scratch.

The accepted fake case binds the account reference, fake entitlement, pinned
fake model/policy, credential request, and one fake audience. Six negative
cases fail before process start and prompt: wrong account, audience, model,
missing entitlement, unexpected credential request, and unexpected provider
traffic. The valid fake records durable pre-execution data before process
start, observes one permission request, cancels it, abandons the pending
permission wait, rejects a late approval, sends `session/cancel`, and joins the
child. The action marker is absent before the prompt and after stop. The fake
completes within its five-second test ceiling.

This demonstrates the harness shape only. It does not show that any frozen
Copilot binary accepts the pipe, advertises or uses `copilot-login` through
ACP `authenticate`, selects Auto, or follows the fake's provider and
permission behavior. The original three-attempt allowance was not used.

## Reusable execution plan

[`authenticated-proof-plan.json`](../../crates/swallowtail-adapter-copilot-cli/tests/fixtures/copilot-cli-acp-offline-proof/authenticated-proof-plan.json)
is validated by
[`authenticated-proof-plan.schema.json`](../../crates/swallowtail-adapter-copilot-cli/tests/fixtures/copilot-cli-acp-offline-proof/authenticated-proof-plan.schema.json).
It contains one pre-execution record template per exact version, the account
access reference, entitlement state, Auto candidate and model policy,
credential mechanism status, audience list status, containment, benign action,
and exact budgets. Each future original invocation requires a fresh record
that is fsynced before process start and then updated with a result correlated
to the selected provider/model, permission response, cancellation, marker,
network audience observations, retry/effect counts, and joined cleanup.

The machine status is `prepared_not_ready_for_live_execution` and
`execution_authorized` is `false`. The separate `missing_owner_attestations`
array names every unresolved owner and required evidence. Per version, the
future cap is one prompt and one invocation, 60 seconds total, zero harness
retries or resends, zero Auto fallback, at most one permission request and
tool attempt, zero effects, and zero reviewer live attempts. The marker action
is never approved. The planner must review and bind all exact prerequisites
before dispatching any live continuation.

## Result

Preparation and fake proof are complete; live execution is not ready. The
exact `1.0.80` qualification and all existing route claims remain unchanged.
No live provider attempt, credential access, authenticated network request,
provider installation, or consumer change occurred.
