# 092 Pi SDK Sidecar Route Acceptance

Status: ready
Owner: Tom
Created: 2026-08-21
Milestone: `../033-pi-sdk-sidecar-route.md`
Depends on: card 091

## Goal

Admit the proved Pi SDK sidecar through consumer connection surfaces, publish
its realized truth, and record the evidence-backed disposition of `pi.rpc`.

## Scope

1. Add an adapter-local addable descriptor, admission path, and prepared
   handoff for `pi.sdk-sidecar` using opaque launch, environment, credential,
   provider-state, and resource references.
2. Add refresh, update, subject, catalogue, and Contract 047 projection only
   where the deterministic route evidence supports them.
3. Extend conformance with the complete sidecar, driver, fresh-session,
   persistent-session, lifecycle, and connection-admission proof.
4. Update realized architecture, provider route matrix, feature matrix,
   prepared integration guide, and connection-lifecycle guide.
5. Compare SDK-sidecar and RPC deployment, identity, configuration, lifecycle,
   feature, and stability evidence.
6. Retain both routes when each has a distinct useful posture, or deprecate RPC
   explicitly if the SDK route proves a safe operational superset. Never
   substitute routes silently.
7. Close Research 181, the Pi continuity triage note, g04.033, and cards
   089-092 with exact validation and merge reality.

## Out Of Scope

- live provider, package install, account, login, billing, or paid inference
- RPC protocol changes or upstream patching
- new SDK capabilities beyond the first qualified surface
- arbitrary shell/write tools, containment claims, retry, or fallback
- changing unrelated route families

## Acceptance Criteria

- a consumer can independently discover, admit, prepare, and select
  `pi.sdk-sidecar`
- portable records carry opaque references, not Node paths, session paths,
  environment values, or credentials
- matrices and guides state exact supported and unsupported capabilities
- deterministic conformance covers every accepted route claim
- RPC coexistence or deprecation is explicit and evidence-backed
- architecture describes only the route actually realized

## Validation

- `effigy validate:focused swallowtail-adapter-pi swallowtail-runtime swallowtail-host-local swallowtail-testkit`
- `effigy package:verify-affected swallowtail-adapter-pi swallowtail-runtime swallowtail-host-local swallowtail-testkit`
- `effigy qa:routes`
- `effigy qa:guides`
- `effigy qa:northstar`
- `effigy qa:docs`
- `git diff --check`
- `effigy package:api` if public API changes

## Auto-Continuation

No. Record merge reality, then run the named Gemini enterprise/API-key
currentness lane.

## Stop Conditions

- Stop if admission installs, searches for, or mutates the Node/SDK boundary.
- Stop if readiness or subject truth requires a provider probe in default QA.
- Stop if the route matrix must overstate persistent or containment support.
- Stop before deprecating RPC unless the accepted evidence supports it.
