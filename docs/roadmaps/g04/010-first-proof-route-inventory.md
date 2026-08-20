# 010 First-Proof Route Inventory

Status: completed
Owner: Tom
Created: 2026-08-20
Depends on: completed g04.009
Vision tags: consumer integration, route readiness, explicit selection
Contract refs: 011, 014, 020, 029, 032, 037, 047, 052, 057
Planning state: cards 027-029 completed

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

- [x] inventory Anthropic Messages, Codex app-server, and Ollama attach
      against Contract 057 descriptors, admission, sign-in, refresh, subject,
      updates, and overlay
- [x] record the OAuth candidate as evidence or an explicit remaining gate
- [x] confirm the first implementation roadmap is hosted API-key Anthropic
      Messages unless inventory contradicts it
- [x] keep adapter wiring planned until this inventory closes

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

- [x] Execute card 027.
- [x] map each named first-proof route's current driver, credential
      mechanism, discovery, prepared facade, 047 path, and 029/032 claim
- [x] name what a 057 addable descriptor would require
- [x] write a research note; do not promote architecture

### Batch 10.2 — Gap And OAuth Evidence

- [x] Execute card 028 after card 027.
- [x] classify each gap as reuse of a prepared facade, adapter-local
      descriptor work, live-only evidence, or still gated
- [x] settle the OAuth candidate from evidence, or keep the gate explicit

### Batch 10.3 — Tranche Confirmation

- [x] Execute card 029 after card 028.
- [x] confirm g04.011 hosted API-key Anthropic Messages as the next
      implementation roadmap unless inventory contradicts it
- [x] leave OAuth, Codex, Ollama, and Contract 052 consumer-path work
      planned behind that first proof

## Acceptance Criteria

- [x] each named first-proof route has an existing-surface map and a 057 gap
      list
- [x] OAuth is either a named candidate with evidence, or a remaining gate
- [x] no adapter crate changes in this milestone
- [x] no live OAuth or secret extraction
- [x] only card 027 starts ready
- [x] g04.011 compiled after this inventory, not before

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
