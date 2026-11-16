# ACP captured startup integration

The Rust captured preload and ACP loader now use the exact migration accepted
in SDK240 PR559 at3a9ca6b6, review5919931970. The
[integration receipt](claude-agent-acp-startup-integration.json) binds the accepted
helper, patch and source receipt, both before/after production files, fixture
sources and unchanged invocation manifest. Only those two production deltas
were integrated; the SDK task was not merged into this branch.

## Absence-only lookup

The immutable `process.getBuiltinModule` accepts exactly one string argument,
`async_hooks`, only when the captured loader exports the reviewed opt-in helper
and it returns exactly undefined. No builtin or facade is returned or forwarded.
Every other name, type and arity throws. A loader without the opt-in and an
object-returning helper still throw. Generic captured loaders keep their prior
refusal. Module resolution, raw-process boundaries, string-code-generation and
addon controls are unchanged.

This lets the selected SDK's existing module-scope absence branches choose
callback identity and the context run/getStore fallback. The frozen
[source record](claude-agent-acp-source-flow.json) retains the earlier throwing
bootstrap as historical evidence. Its SDK/Zod source bytes, invocation callsites
and standard-ESM premises have not changed. No restored vendor module is evaluated.

## Actual owned sibling proof

`validate:acp-shared-software` compiles the real Rust preload and runs the checked
LocalProcessHost route with first-party source-shaped peers. The Rust prepared
owner parents the sleep group owner and checked ACP as siblings. ACP joins the
sleep PID's group; the reviewed Python software entry is ACP's child. This is
not the SDK driver's parent-child stand-in.

The module-scope fixture makes both absence probes before ACP serves requests,
checks callback identity and run/getStore fallback, and rejects eight other
builtin argument forms. Actual missing-opt-in and object-return variants fail
during discovery before native entry or endpoint effect. Each refusal retains
its Rust child wait and helper/output-reader cleanup obligation.

The changed route exercises new/load/resume and two consecutive turns per
session with callback allow, managed denial and user denial. Permission precedes
tool reads; both denial sources omit the callback and still produce continued
output. Cancellation, checked-approval revocation, entry revocation, argv
substitution, dropped starts and controlled panic/deadline paths retain actual
software child waits. The independent diagnostic descriptor reaches EOF after
owned cleanup. These observations remain RootOnly software evidence; a vanished
PID or driver exit is never substituted for a child wait or kernel settlement.

Each owned setup emits source-graph, compound-shim, exact Rust prefix/bootstrap
recipe and combined launch fingerprints. They include its actual fixture paths,
settings and installed runtime; they are not portable admission tokens. The
reviewer receives the run evidence separately. The current installed compatible
Node is used; this transfers no standalone SDK or exact-Node qualification.

## Remaining finite shared-host delta

The [b62 invocation contract](claude-agent-acp-invocation-contract.md) remains
unchanged source/metadata evidence (review5917297533,
`invocation-manifest-coverage`, `invocation-launch-policy`,
`preentry-one-shot-model`: pass). These owned startup observations do not resolve
the original ACP option tail or provide trusted runtime callsite provenance.
Unsupported settings, extraArgs, caller/argv/env drift and unknown tails still
refuse. Native auth-status, SDK transport and bounded registry-ps remain distinct.

SDK240's remaining ACP composition is finite: bind this actual sibling actor
schema and exact updated prefix/compound loader to its same-invocation host
transaction; bind actual launch/entry/descriptor identities before release;
complete selected Python/runtime and opaque-native custody; then establish
kernel containment plus adoptee/tree settlement and original-compatible combined
permission/transport fixtures. Reuse this software topology and wait proof rather
than substituting it for kernel rights or duplicating the shared kernel boundary.
The historical f902 entry, old plan/runner bytes and consumed grants stay intact.

Original permission-without-callback, continued-turn and lifecycle qualification
still require their exact admitted native/official proof. `originalCommand` and
`actualPlan` stay null, ceiling0.100.0 and the qualification merge hold remain.
No host/kernel/canonical/workflow activation or original attempt is authorized
by this integration.

## Covering checks

The deterministic receipt verifier pins the reviewed receipt at5923c113 by
SHA256, then independently hashes the accepted shared Git files, before-source
Git files, all seven current sources, and both historical/current invocation and
source-flow records. Exact JSON comparison also protects provenance metadata,
lookup policy, history and closed gates, including missing/extra fields and
boolean/number substitutions. The candidate receipt cannot select input paths
or bless a changed source by changing its recorded hash. Missing local Git
bindings refuse; verification performs no network fetch or package evaluation.

Owned mutation tests cover each field and byte binding, coordinated source/hash
drift, missing Git inputs, duplicate keys, invalid numbers and bounded regular
files. This check detects source/receipt drift; it grants no runtime admission.

For receipt-only changes, run these selectors plus Python format, routing, docs
and retired-concept checks; preserve the unchanged Rust/owned-sibling proof:

```sh
effigy compile:acp-startup-integration
effigy check:acp-startup-integration
effigy validate:acp-startup-integration
effigy check:python-format scripts/claude-acp-startup-integration.py scripts/tests/claude-acp-startup-integration.py
effigy validate:oxide-routing
effigy qa:docs
effigy skill run northstar/retired-concepts
```

Changes to the Rust integration or owned sibling route additionally need:

```sh
effigy format:package swallowtail-host-local swallowtail-adapter-claude-agent
effigy check:acp-guard-host-environment
effigy lint:acp-startup-host
effigy lint:acp-guard-full-route
effigy validate:acp-guard-host-environment
effigy validate:acp-guard-captured-child
effigy check:acp-shared-software
effigy validate:acp-shared-software
effigy check:acp-guard-api
effigy package:verify-affected swallowtail-host-local swallowtail-adapter-claude-agent
effigy validate:oxide-routing
effigy qa:routes
effigy qa:docs
effigy skill run northstar/retired-concepts
```

Unchanged frozen-source reconstruction and invocation-model suites retain their
reviewed input scope; they are not rerun as startup proof. Northstar strict Rust
rules cover direct control flow, recoverable failures and async lock lifetimes;
MSRV1.95 and existing public API are preserved, with no new deviations.
