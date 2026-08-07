# Prepared Facade Authoring

Use this pattern when adding the adapter-local normal path required by
Contract 037. The facade removes repeated low-level assembly. It does not
choose consumer intent or flatten provider behavior. New to the shared
vocabulary? Read [Key Concepts](key-concepts.md) first.

## Two Phases

Every facade has two visible phases:

1. prepare one exact integration
2. prepare and execute one native operation shape

Integration preparation binds configured-instance identity, host, opaque
target, access profile and provenance, facade, and exact compatibility
evidence where the route exposes a version. It may observe the selected target
when that observation is required. It performs no model inference.

Operation preparation accepts the remaining consumer choices, derives
adapter-owned facts and immutable plan echoes, runs preflight, and returns a
typed prepared value. Execution delegates that plan and request to the
unchanged low-level role.

Do not add an adapter-spanning constructor, provider registry, generic
`send_prompt`, or operation enum.

## Shared Records

Use `PreparedAccessEvidence` for access status and provenance.

Use `PreparedOperationEvidence` for facts common to every prepared operation:

- driver identity and role
- execution layer and operation shape
- configured instance and revision
- execution host and opaque target
- protocol facade
- immutable plan
- access evidence
- exact interface bindings and compatibility assessments

Do not add provider session IDs, native model tags, environment references,
credentials, wire revisions, inventory, or provider-resource ownership to
these shared records. Keep them in adapter-local evidence.

## Adapter-Local Types

Each adapter owns:

- one explicit preparation input
- a separate probe input when preparation performs bounded observation
- one prepared integration
- one operation input and prepared value per supported low-level role
- adapter-private evidence required by those operations

Name operations for their real role. `prepare_catalogue`, `prepare_inventory`,
`prepare_inference_attempt`, and `prepare_session` are intentionally different.
Execution methods such as `list_models`, `observe_inventory`, `start_run`,
`open_session`, `load_session`, and `resume_session` retain native lifecycle
semantics.

Prepared operation values expose `evidence`, `plan`, `request`,
`low_level_driver`, and an ownership-preserving `into_parts` path where those
parts exist. The low-level API stays public.

## Preparation Effects

Preparation effect shape follows the route:

| Route shape | Preparation behavior |
| --- | --- |
| Installed harness | Probe one host-approved executable, parse one exact version, classify compatibility, join the probe. |
| Hosted direct | Validate explicit host, endpoint target, access profile, and evidence locally when no provider observation is required. |
| Attached runtime | Authorize one host-approved endpoint and observe the exact runtime and bounded native state required for safe invocation. |

Async preparation is required only when the selected observation is async.
Do not manufacture an async boundary for pure local validation.

## Intentional Differences

The representative proofs fix these differences:

| Route | Native evidence | Prepared operations | Authority that remains absent |
| --- | --- | --- | --- |
| Kimi Code ACP | executable observation, environment, delegated credential reference | new, load, resume, prompt, interruption | containment, provider-state ownership, implicit login |
| Anthropic direct | public endpoint and API access binding | catalogue, one Messages attempt | retry, tool execution, continuation, endpoint or model fallback |
| Ollama native | runtime version, installed/running inventory, selected detail | inventory observation, one chat attempt | install, pull, unload, model artifact, process, server lifecycle |

These are not variants of one shared session or prompt abstraction.

## Failure Rules

Map preparation failures to the shared safe stages:

- target approval or binding: `TargetSelection`
- process start: `ProcessSpawn`
- bounded probe or observation: `BoundedOutput`
- process completion: `ProcessExit`
- exact version decoding: `VersionParse`
- support classification: `CompatibilityClassification`
- access-profile or provenance mismatch: `AccessEvidence`
- capability, plan, or request construction: `Preflight`
- joined release failure: `Cleanup`

Not every route uses every stage. Preserve the earliest meaningful failure and
keep cleanup failure visible. Stable diagnostics contain no target material,
credentials, account identity, prompt, output, or raw provider payload.

## Version Rules

Ordered version claims preserve the guaranteed range and exact assessment.
Known exclusions and prereleases fail. A later exact stable version may
proceed only when the claim permits visibly unverified execution. It must use
the latest qualified behavior and pass the same drift and protocol checks.

Opaque hosted facades, protocol revisions, SDK versions, and executable
versions remain separate axes.

## Conformance Checklist

Before rollout:

- prove local and remote-authoritative host identity where the driver supports
  both
- prove target, instance, access, and version drift fail before operation
  effects
- prove plan-derived request agreement
- prove cancellation, deadline, terminal outcome, and joined cleanup through
  the low-level role
- prove stable diagnostics redact host and provider material
- prove guaranteed and unverified-newer compatibility remain distinct
- compile one public example
- retain the adapter's complete low-level fixture suite
- run the shared prepared-evidence assertions
- classify the public API change against the held release baseline

Kimi ACP, Anthropic direct, and Ollama native satisfy this pattern without a
new shared lifecycle or access rule. The remaining adapter rollout can reuse
the pattern while keeping each route's native operations and evidence local.

## Worked Shape

A facade is two thin layers over one unchanged low-level role. The shape below
is illustrative, not a compile target; the compile-tested examples linked
below are the ground truth:

```rust
// Phase 1: one prepare function for the whole integration.
pub async fn prepare_<route>(
    input: <Route>PreparationInput,
    probe: <Route>PreparationProbe,
    services: HostServices,
) -> Result<<Route>PreparedIntegration, PreparationFailure> {
    // observe the target only when the route requires it
    // bind identity, host, target, access evidence, version evidence
    // return one prepared value
}

// Phase 2: one operation per low-level role.
impl <Route>PreparedIntegration {
    pub fn prepare_structured_exec(
        &self,
        input: <Route>ExecProfileInput,
    ) -> Result<<Route>PreparedExec, PreparationFailure> {
        // derive the immutable plan, preflight, and request
    }
}

impl <Route>PreparedExec {
    pub async fn start_run(
        &self,
        services: HostServices,
    ) -> Result<Box<dyn RunHandle>, RuntimeFailure> {
        // delegate to the public low-level driver
    }
}
```

The three representative compile-tested examples show the real shape:

- [Kimi ACP](../../crates/swallowtail-adapter-kimi/examples/prepared_acp.rs)
- [Anthropic direct](../../crates/swallowtail-adapter-anthropic/examples/prepared_direct.rs)
- [Ollama native](../../crates/swallowtail-adapter-ollama/examples/prepared_attached.rs)

## Validation

Follow the conformance checklist above, then prove the public surface
deterministically:

```sh
effigy check:examples
effigy validate:focused swallowtail-adapter-<route>
effigy qa:docs
effigy qa:routes
```

Live provider probes remain separately operator-gated and never replace the
fixture and example evidence.
