# 2026-07-19 Cross-Adapter Runtime Decisions

Status: complete
Owner: Tom

## Change

Settled the runtime semantics shared by one-shot CLI, long-lived RPC or ACP,
hosted API, SDK, and self-hosted serving drivers.

## Decision

- runtime execution is async-first and executor-neutral
- tasks, resources, processes, and connections live in joined scopes
- drivers require only the host services they use
- configured instance, model route, access profile, and ownership survive every
  operation
- structured runs and interactive sessions expose owned handles, ordered
  bounded events, cancellation, and exactly one terminal outcome
- external serving instances are never stopped by generic cleanup
- credentials, resources, attachments, schemas, and diagnostics cross the host
  boundary through scoped capabilities rather than raw paths or secrets
- schema transport remains separate from consumer validation

## Evidence

The conformance matrix covers a one-shot structured CLI, long-lived RPC or ACP
harness, hosted direct API, attached self-hosted runtime, and owned self-hosted
runtime.

## Consequence

Card 008 is complete. Card 009 can promote the accepted decision set into
testable contracts and compile implementation cards without inventing runtime
semantics.
