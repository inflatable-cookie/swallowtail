# 021 Bedrock Control-Plane Catalogue Proof

Status: completed
Owner: Tom
Updated: 2026-07-21

## Purpose

Prove Bedrock model discovery through its separate control-plane client without
turning catalogue presence into invocation entitlement, runtime readiness, or
a default route.

## Generation Runway

Keep g01 active. Roadmap 020 proved the SDK-native Bedrock Runtime boundary.
This lane reuses only the generic embedded-SDK rules: it must independently
bind the control-plane service, endpoint audience, permissions, generated SDK
types, and catalogue evidence.

## Contracts

- Contract 005: Integration Identity and Transport Diversity
- Contract 006: Execution Layer and Access Boundary
- Contract 008: Runtime Registration and Preflight
- Contract 010: Execution Host Services and Inputs
- Contract 011: Runtime Conformance Profiles
- Contract 014: Hosted Transport, Credential, And Evidence Boundary
- Contract 019: Embedded SDK And Cloud Client Boundary
- Contract 020: Model Catalogue Observation And Availability Boundary

Research 014 exposed and promoted the missing source-scoped observation,
availability, and non-paginated-bound rules into Contract 020.

## Goals

- [x] Revalidate the exact Bedrock control-plane SDK, endpoint, permission,
      request, lifecycle, and model-summary behavior.
- [x] Freeze a generated-type catalogue corpus with safe evidence projection.
- [x] Implement one separately registered model-catalogue driver.
- [x] Prove local and remote-authoritative hosted catalogue conformance without
      live AWS access.

## Execution Plan

- [x] Currentness and boundary decision: card 070.
- [x] Exact generated catalogue fixtures: card 071.
- [x] Production catalogue driver, conformance, and closeout: card 072.

## Cards

- `batch-cards/070-bedrock-catalogue-evidence-and-boundary.md` — completed
- `batch-cards/071-bedrock-catalogue-sdk-fixtures.md` — completed
- `batch-cards/072-bedrock-catalogue-driver-and-conformance.md` — completed

## Planning Checkpoint

Research 014 fixes `aws-sdk-bedrock = 1.148.0` and native
`ListFoundationModels` as the next bounded proof. It is one non-paginated
request through a separate AWS service client and endpoint audience. Contract
020 keeps provider name, lifecycle, modalities, streaming, inference, and
customization observations separate from IAM, entitlement, route, runtime
capability, regional invocation availability, quota, readiness, and successful
inference. The separate `bedrock-mantle` `/models` surface is outside this lane.

Card 071 realizes the provider-neutral observation records and pins the exact
generated SDK corpus. Card 072 completes the separately registered production
driver. It preserves unknown optional fields, namespaces unknown enums, bounds
the single response without truncation, and keeps provider name, ARN, route,
capability, inference access, and catalogue access separate. Local and remote-
authoritative fixtures pass without AWS access. Roadmap 022 owns the next
coverage checkpoint; no provider implementation is preselected.

## Stop Condition

Stop if the control plane cannot be bound without ambient AWS configuration,
requires a new account or routing policy, or cannot keep catalogue evidence
separate from route selection and runtime entitlement.
