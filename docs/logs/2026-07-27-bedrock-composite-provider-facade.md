# Bedrock Composite Provider Facade

Date: 2026-07-27

## Change

Amazon Bedrock now has one adapter-local provider facade:

- `prepare_bedrock` binds the execution host and explicit cloud-client config
- `runtime` prepares the Runtime inference route
- `catalogue` prepares the control-plane model-catalogue route

The facade shares no target, access profile, configured-instance identity,
version assessment, plan, model selection, or operation request between the
branches.

## Preserved Boundaries

- Runtime and catalogue remain separate registered drivers.
- Each branch has a separate endpoint audience and access profile.
- Catalogue results do not select or authorize inference routes.
- Route-specific prepared constructors and low-level drivers remain public.
- There is no ambient AWS configuration, credential discovery, fallback, or
  provider-wide operation enum.

## Evidence

- focused prepared-facade tests pass for local and remote-authoritative hosts
- facade preparation rejects an execution-host mismatch before host effects
- existing cross-route access rejection remains covered
- the combined public example compiles through the normal example check
- strict Bedrock Clippy, formatting, route-matrix, and docs checks pass
- `effigy package:api` retains the expected held-candidate diff; this additive
  Bedrock API is not promoted into the frozen `0.1.0` release baseline
