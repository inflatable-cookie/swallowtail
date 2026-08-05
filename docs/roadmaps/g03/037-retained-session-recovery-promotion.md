# 037 Retained Session Recovery Promotion

Status: active
Owner: Tom
Created: 2026-08-05
Depends on: g03.036
Vision tags: restart continuity, retained sessions, exact resource ownership
Contract refs: 009, 017, 025, 029, 037-038, 050
Planning state: card 098 complete; Pi card 099 superseded; card 100 ready;
card 101 gated

## Problem

Pi RPC exposes persisted-session switching and ordered history but cannot prove
that stored cwd matches the host-leased working resource. Alibaba Conversations
can retrieve, list, and continue a conversation, but the selected route owns
and deletes it during cleanup. Neither route can enter working-state recovery
under its current binding and ownership contract.

## Generation Runway Goal

Promote retained-session recovery only after exact resource attachment and
provider-state ownership are separately proven.

## Goals

- [x] refresh Pi and Alibaba currentness without weakening existing gates
- [x] retain or close Pi's exact cwd-binding blocker from public evidence
- [x] define a separate retained Alibaba conversation profile if supported
- [ ] implement and publish only independently qualified routes

## Execution Plan

- [x] card 097: revalidate both candidates and record independent promotion
      decisions
- [x] card 098: promote the retained Alibaba ownership and replay contract if
      its evidence passes
- [x] card 099: close Pi implementation negatively because exact cwd
      attachment remains unprovable
- [ ] card 100: implement Alibaba retained-conversation recovery only after its
      separate contract passes
- [ ] card 101: reconcile route truth, facade mappings, package proof, and all
      remaining blocked gates

## Boundaries

- no copied path, conversation id, session id, process cwd, or provider prose
  grants attachment authority
- no widening of Pi's existing ephemeral session profile
- no widening of Alibaba's existing operation-owned delete-on-close profile
- no implementation card runs for a candidate whose gate remains closed
- no native close, archive, restore, delete, retry, or fallback claim is
  inferred from continuation
- no authenticated provider work unless a later card explicitly requires and
  the operator authorizes it

## Acceptance Criteria

- [ ] every candidate ends as supported or blocked with one exact reason
- [ ] Pi requires caller-bound cwd plus corroborated effective-resource truth
- [ ] Alibaba retention uses a separate explicit provider-state and cleanup
      profile
- [ ] load replay completes before readiness; replay-free resume stays distinct
- [ ] facade mappings change only for routes with complete bindings and corpus
- [ ] public and package truth retain every unsupported route honestly

## Lane Runway

Card 098 is complete. The contract and corpus keep resource-free retained load,
ordinary preservation, and separately authorized cleanup distinct from the
existing delete-on-close profile. Card 100 is ready to implement that exact
prepared route. Card 101 remains gated for final route truth.
