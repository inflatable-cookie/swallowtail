# 010 First-Proof Route Inventory

Status: planned
Owner: Tom
Created: 2026-08-20
Depends on: completed g04.009
Vision tags: consumer integration, route readiness, explicit selection
Contract refs: 011, 014, 020, 029, 032, 037, 047, 052, 057
Planning state: card 027 ready; cards 028-029 planned behind it

## Problem

The connection-lifecycle facade is realized. Contract 057 still names
first-proof routes as later implementation, not current architecture.
Anthropic Messages, Codex app-server, and Ollama attach already have
prepared facades. None of them expose addable-route descriptors, so a
consumer cannot list, admit, or sign in those routes through 057.

The hosted OAuth proof is still an evidence gate: Anthropic or Claude
subscription, whichever can be proved without extracting secrets.

## Generation Runway Goal

Prove representative hosted, installed, and local-runtime shapes and publish
a consumer path. This milestone only inventories existing adapter surfaces
against 057 and names the first implementation tranche.

## Goals

- [ ] inventory Anthropic Messages, Codex app-server, and Ollama attach
      against Contract 057 descriptors, admission, sign-in, refresh, subject,
      updates, and overlay
- [ ] record the OAuth candidate as evidence or an explicit remaining gate
- [ ] confirm the first implementation roadmap is hosted API-key Anthropic
      Messages unless inventory contradicts it
- [ ] keep adapter wiring planned until this inventory closes

## Non-Goals

- writing addable descriptors or changing prepared facades
- live provider, install, login, or billing work
- extracting harness or subscription secrets
- OpenHands production wiring
- publishing a consumer path
- putting emails, tokens, or targets into 047
- changing `Ready` / `NotReady`

## Execution Plan

### Batch 10.1 — Surface Inventory

- [ ] Execute card 027.
- [ ] map each named first-proof route's current driver, credential
      mechanism, discovery, prepared facade, 047 path, and 029/032 claim
- [ ] name what a 057 addable descriptor would require
- [ ] write a research note; do not promote architecture

### Batch 10.2 — Gap And OAuth Evidence

- [ ] Execute card 028 after card 027.
- [ ] classify each gap as reuse of a prepared facade, adapter-local
      descriptor work, live-only evidence, or still gated
- [ ] settle the OAuth candidate from evidence, or keep the gate explicit

### Batch 10.3 — Tranche Confirmation

- [ ] Execute card 029 after card 028.
- [ ] confirm g04.011 hosted API-key Anthropic Messages as the next
      implementation roadmap unless inventory contradicts it
- [ ] leave OAuth, Codex, Ollama, and Contract 052 consumer-path work
      planned behind that first proof

## Acceptance Criteria

- [ ] each named first-proof route has an existing-surface map and a 057 gap
      list
- [ ] OAuth is either a named candidate with evidence, or a remaining gate
- [ ] no adapter crate changes in this milestone
- [ ] no live OAuth or secret extraction
- [ ] only card 027 starts ready
- [ ] g04.011 stays uncompiled until this inventory closes

## Lane Runway

- previous: g04.009 overlay
- this milestone: first-proof inventory and tranche selection
- next: g04.011 hosted API-key Anthropic Messages, compiled after this
  inventory
- later: OAuth proof, Codex app-server, Ollama attach, Contract 052
  consumer path

## Decision Gates

- Stop if inventory would store raw secrets or create a Swallowtail server.
- Stop if OpenHands would gain a production route.
- Stop if OAuth is selected without a no-secret-extraction proof.
- Stop if first-proof would mutate 047 `Ready` / `NotReady`.
