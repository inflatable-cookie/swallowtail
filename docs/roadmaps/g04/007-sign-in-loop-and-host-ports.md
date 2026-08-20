# 007 Sign-In Loop And Host Ports

Status: completed
Owner: Tom
Created: 2026-08-19
Depends on: g04.006
Vision tags: consumer integration, route readiness, explicit selection
Contract refs: 006, 010, 014, 015, 017, 047, 057
Planning state: cards 019-021 completed

## Problem

Admission can write an instance, but required credentials still cannot be
collected or launched. Contract 008 `SignInAction` is an advertisement.
There is no host port for URL open, loopback callback, or device-code
display. ACP `authenticate` is not login.

## Generation Runway Goal

Realize library-max sign-in loops through host ports.

## Goals

- [x] add optional Contract 010 ports for URL open, loopback callback, and
      device-code display
- [x] own start, poll, complete, cancel, and timeout
- [x] fail closed when a required port is missing
- [x] materialize only opaque credential references through Contract 014
      leases
- [x] keep ACP authenticate and 017 delegated login distinct

## Non-Goals

- embedding a browser, keychain, or OAuth client secret
- live provider OAuth as a planning substitute
- extracting harness secrets for a public API
- first-proof Anthropic subscription wiring
- changing 047 to carry emails or tokens

## Execution Plan

### Batch 7.1 — Host Ports

- [x] Execute card 019 after g04.006.
- [x] optional URL, loopback, and device-code ports
- [x] spawn of an approved login helper stays process authority

### Batch 7.2 — Sign-In Loop

- [x] Execute card 020 after card 019.
- [x] start, poll, complete, cancel, timeout
- [x] interactive OAuth, device OAuth, delegated CLI login

### Batch 7.3 — Fail-Closed And API-Key Collection

- [x] Execute card 021 after card 020.
- [x] missing required port fails the loop
- [x] API-key collection through field descriptors
- [x] success materializes a `CredentialRef` for the same route and audience

## Acceptance Criteria

- [x] ports never return secret bytes to portable records
- [x] presence of a port does not start sign-in
- [x] ACP `authenticate` is not used as this loop
- [x] a mechanism, account, audience, or billing change fails closed
- [x] 047 still has no emails, tokens, or targets

## Lane Runway

- previous: g04.006 catalog and admission
- this milestone: host ports and sign-in loop
- later compile: readiness refresh, subject observation, overlay
  projection, then first-proof routes

## Decision Gates

- Stop if the loop extracts a harness secret or embeds a browser.
- Stop if login and ACP authenticate collapse.
- Stop if live provider work replaces deterministic tests.
