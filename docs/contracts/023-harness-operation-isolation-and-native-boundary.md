# 023 Harness Operation Isolation And Native Boundary

Status: active
Owner: Tom
Updated: 2026-07-21

## Purpose

Represent local harness isolation honestly for every operation shape and keep
provider-native permissions, budgets, retention, and sandbox features separate
from Swallowtail lifecycle authority.

## Operation-Shape-Neutral Isolation

`HarnessIsolation` applies whenever a local harness process performs an
interactive session or structured run:

- `AmbientHost` — the harness process and descendants execute with the ambient
  authority of the selected execution host
- `ProviderEnforced` — the provider harness enforces a documented boundary
- `HostEnforced` — the execution host enforces a separately qualified boundary

The exact posture is selected in operation requirements, carried in runtime
policy, and compared during pure preflight. A direct-inference operation cannot
claim harness isolation. A driver cannot infer an enforced posture from a
binary name, settings file, platform, or installation method.

Existing interactive-session access policy remains valid. New structured-run
harness drivers must bind isolation explicitly before process work. Older
drivers may migrate without a compatibility alias; no route may silently
change its realized claim during migration.

## Permission And Tool Boundary

Provider approval modes, tool allowlists or denylists, plan modes, safe modes,
and disabled extensions limit harness behavior. They do not contain the
harness process, its descendants, or unmediated filesystem and network access.

Swallowtail may require exact provider flags as part of a driver contract. It
reports only the isolation posture independently proven by the configured
route. A read-only tool posture under `AmbientHost` remains ambient.

## Native Sandbox Boundary

Provider-native sandboxing is optional. A configured route may select
`ProviderEnforced` only when the exact provider version, invocation, platform,
and deployment satisfy its documented boundary and deterministic conformance
evidence.

Container, VM, App Sandbox, Landlock, or another host mechanism is never an
implicit prerequisite for harness communication. `AmbientHost` remains a valid
explicit posture. There is no fallback from `ProviderEnforced` or
`HostEnforced` to `AmbientHost` when setup, startup, or qualification fails.

## Native Budget Boundary

Harness-native wall-time, turn, tool-call, retry, and output bounds are
provider behavior. They may be exact required invocation inputs and may produce
typed provider failures or progress evidence. Their documented exemptions and
scope remain provider-specific.

They do not replace:

- the Swallowtail monotonic deadline
- consumer cancellation
- process stop and force-stop authority
- bounded event and output transport
- joined task and process cleanup

A driver may use both layers. It must keep terminal causes distinct and must
not report one provider-native bound as proof that another bound or host
deadline fired.

Provider-managed retry is disabled unless separately accepted. A harness flag
that retries indefinitely is not enabled by a generic deadline or CI context.

## Configuration And Retention

Harness configuration, credential state, runtime transcript state, and working
resources are distinct authorities.

- delegated harness authentication may use an exact host-approved environment
  without exposing stored credentials
- a config-suppression mode does not imply isolated credential or runtime state
- local harness transcript retention requires explicit temporary or durable
  provider-retention acceptance
- process exit does not prove transcript deletion
- retained state does not grant Swallowtail resume, load, enumerate, mutate, or
  delete authority

If a later route uses operation-scoped harness runtime state, its host lease and
cleanup must be contracted separately. No adapter deletes ambient harness state
by path convention.

## First Qwen Mapping

The Qwen Code `v0.19.11` headless proof uses:

- `HarnessInteraction` plus `StructuredRun`
- exact `AmbientHost` isolation
- explicit safe mode, approval posture, tool exclusions, model route, and
  native wall/tool/turn bounds
- a host deadline and process cancellation independent of native budgets
- explicit durable local harness retention with no resume or deletion claim
- no provider sandbox, container, persistent retry, background work, or route
  fallback

Qwen `--sandbox` remains a later `ProviderEnforced` profile. It is not enabled
or required by the ambient proof.

## Conformance

Applicable fixtures prove:

- missing or mismatched structured-run isolation fails before process work
- direct inference rejects a harness-isolation requirement
- provider tool restrictions cannot satisfy an enforced isolation requirement
- provider-native budget, host deadline, cancellation, and process failure
  remain distinct terminal causes
- durable local retention is explicit and creates no resume or deletion claim
- an unavailable enforced profile does not retry as ambient
- public diagnostics expose no raw configuration, credential, transcript path,
  argv prompt, stdout, or stderr payload

The existing one-shot structured-CLI profile remains the execution profile.
This contract adds an isolation and native-bound assertion pack rather than a
new transport profile.

## Exclusions

This contract does not standardize provider tool names, approval modes, budget
vocabulary, sandbox implementations, configuration files, transcript formats,
or credential stores. It does not authorize repository writes, provider
fallback, or consumer routing policy.
