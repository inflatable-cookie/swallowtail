# Selected SDK startup absence probe

Status: source/fake adaptation; independent review pending, not consumable.

The [source receipt](claude-agent-sdk-startup-source.json) binds ACP239's
committed source analysis at81c27b9 to the four selected SDK files. Their hashes
match the SDK's frozen selected-flow ledger. Both uncaught module-scope
`globalThis.process?.getBuiltinModule?.("async_hooks")` probes run before query:

| SDK | Member-expression byte offsets | Missing-API behavior |
| --- | --- | --- |
|0.3.318|176698,260441|callback identity; `run`/`getStore` fallback|
|0.3.319|177342,262549|same|
|0.3.321|177972,268137|same|
|0.3.322|177972,268137|same|

Each member expression is36 bytes. The surrounding frozen contexts include the
literal call argument and fallback. These are retained source data, never
executed fixture code. A present throwing replacement is called by optional
chaining, so the existing guards prevent either fallback from being selected.

## Private contract

The selected SDK route uses
[`sdkStartupBuiltinLookup`](../../../scripts/claude-agent-sdk-startup-lookup.mjs):
exactly one argument, exactly the primitive string `async_hooks`, returns
`undefined`. No builtin is looked up, forwarded or exposed. Every other name,
arity and type throws without coercing the argument. The process property stays
non-writable and non-configurable. Existing code-generation, native, module,
dynamic-edge, environment, custody, revocation, permission and cleanup rules stay
binding.

This deliberately selects the frozen SDKs' own missing-API branches. It does not
provide AsyncResource, AsyncLocalStorage or a replacement implementation. Owned
source-shaped fixtures verify callback identity, `run` execution and empty
`getStore`; actual prepared/shipped-sidecar fakes exercise callback denial,
pre-callback managed denial/continued turn and cleanup under the same guard.
No SDK/ACP/vendor package was evaluated for this proof.

## Shared migration for ACP239

The [exact migration](claude-agent-sdk-startup-migration.patch) is unapplied here:
its Rust/ACP source exists on the preserved ACP branch. Its base is
`captured_source.rs` SHA-256
`e75caa2819f12d2653a97153c276259c7ca3f0d961241007ec15372a57f50dbf`.
Before/after loader, Rust, shared helper and patch hashes live in the receipt.

The generic Rust preload keeps denial by default. It permits this absence probe
only when its exact source-bound loader exports the shared helper, the request
matches the exact string/arity, and the helper returns `undefined`. An object
result refuses. The ACP loader delta appends the same pure helper export; it
adds no ambient builtin, public Rust API, loader edge or runtime grant. The
ordinary SDK fake bootstrap installs the shared helper directly for that route.

ACP239 must bind the new loader/shim/recipe fingerprints, integrate the Rust
delta, compile and exercise its actual Rust sibling topology, and obtain normal
independent review. Retained snapshot f902 and prior fingerprints remain
immutable historical evidence; they cannot name the new source. Rust recipe
construction/compilation is not proved by this branch's generated-preload tests.
Until that integration/review, this is a concrete migration candidate, not a
consumable shared milestone.

The source census also records a guarded Lodash `binding("util")` path. This
repair establishes no generic raw-binding prohibition or opaque-native closure.
Actual original startup, shared-host/kernel admission and native qualification
remain separate closed gates. The qualification merge hold stays in place.

## Proof

`validate:claude-agent-sdk-startup-lookup` checks the exact source receipt and
executes only rewritten owned fixtures against the exact original/proposed
first-party preload strings. The original and non-opted-in guards refuse before
the fixture effect; opted-in probes select fallback and unauthorized requests
remain denied. `validate:claude-agent-sdk-spawn-proposal` covers real owned
prepared/sidecar peers and the permission, cancellation, revocation and cleanup
regressions. Cover compile, formatting, routing, docs and retired concepts through
the named Effigy selectors; no original activation is part of them.
