# Backlog: Grok Build Maintained ACP Range

Status: completed through g02.043 and g03.012
Priority: closed
Estimated effort: one evidence gate plus four implementation batches
Source: `g01.047`

## Problem

Exact Grok Build `0.2.111` requires an ACP authentication request before
session allocation. Current maintained documentation does not establish that
the artifact's advertised method can activate an existing Grok subscription
credential without opening sign-in, changing mechanism, or selecting another
billing route.

The operator provisioned an authenticated current Grok installation on
2026-07-30. Exact `0.2.114` passed the activation-only gate. Roadmap g03.012
later extends the maintained range through installed exact `0.2.117` with a
separate private task-control behavior revision.

## Preserved Evidence

- [source roadmap g01.047](../g01/047-grok-build-maintained-acp-range.md)
- [card 137 exact corpus](../g01/batch-cards/137-grok-build-range-corpus.md)
- [card 138 authentication gate](../g01/batch-cards/138-grok-delegated-authentication-and-access-qualification.md)
- [card 139 discovery and dispatch](../g01/batch-cards/139-grok-installed-discovery-and-dispatch.md)
- [card 140 production driver](../g01/batch-cards/140-grok-acp-production-driver.md)
- [card 141 conformance](../g01/batch-cards/141-grok-range-conformance-and-closeout.md)
- [archived Spec 003](../../specs/archive/003-delegated-acp-authentication-activation.md)

## Proposed Approach

Resume card 138 first. Prove activation-only delegated subscription access
against the frozen exact artifact without login, API-key fallback, prompt, or
model request. Promote a narrow shared contract only if the evidence supports
one. Keep ambient configuration, `AmbientHost`, durable harness state, and the
absence of sandbox or read-only containment claims explicit.

Cards 139-141 remain ordered behind that gate.

## Promotion Trigger

One of:

1. the operator independently provisions exact `0.2.111` subscription state
   and authorizes the existing no-prompt probe
2. maintained xAI documentation matches the exact artifact and settles the
   activation lifecycle
3. the operator explicitly selects a different access route and authorizes new
   research

Promotion also requires a currentness refresh because Grok releases and
authentication behavior change frequently.

## Promotion Result

- Research 070 freezes exact installed, artifact, registry, authentication, and
  state evidence.
- Contract 015 now permits exact activation of one existing delegated harness
  credential.
- Spec 003 is promoted and archived.
- Roadmap g02.043 and cards 142-145 replace the held g01 execution plan.
- Research 085 and roadmap g03.012 maintain exact `0.2.114..=0.2.117` without
  widening access or task-control authority.

## Success Criteria

- [x] one exact release is qualified without inferring compatibility from
      package semver
- [x] activation remains machine-distinct from login and mechanism switching
- [x] no API-key, endpoint, billing, model, or provider fallback occurs
- [x] permissions remain distinct from isolation and containment
- [x] production mapping passes local and remote-authoritative conformance

## Risks

- authentication may remain interactive or mutate provider state
- current artifacts may supersede the frozen candidate before promotion
- a future route may need a different access contract
