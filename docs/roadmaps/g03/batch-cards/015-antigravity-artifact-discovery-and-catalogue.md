# 015 Antigravity Artifact, Discovery, And Catalogue

Status: planned
Owner: Tom
Created: 2026-07-31
Milestone: `../006-antigravity-personal-harness-foundation.md`
Depends on: card 014 and operator-installed Antigravity CLI

## Goal

Reconcile an exact Antigravity artifact, then implement identity-safe discovery
and an auth-aware model catalogue without inheriting Gemini claims.

## Scope

1. Reconcile official `1.1.8`, repository `1.1.8`/`1.1.9` tags, installed
   version output, artifact integrity, and selected source.
2. Freeze sanitized version, auth-posture, help, and model-list fixtures.
3. Create `swallowtail-adapter-antigravity` and the `antigravity` family.
4. Implement `agy` discovery, exact compatibility classification, personal
   Google access, and catalogue behavior.
5. Keep enterprise access separate and Gemini fallback absent.

## Acceptance Criteria

- [ ] the first claim names one exact authoritative artifact
- [ ] tag aliases cannot create duplicate or invented behavior ranges
- [ ] authentication remains provider-owned and secret-free
- [ ] catalogue entries do not imply invocation availability
- [ ] Gemini and Antigravity retain separate identities and access profiles
- [ ] focused discovery, compatibility, catalogue, and package tests pass

## Validation

- `effigy validate:focused swallowtail-adapter-antigravity`
- focused artifact, discovery, compatibility, catalogue, diagnostic, and
  cross-host tests
- docs and Northstar checks for the exact evidence record
- no broad workspace suite or live prompt

## Stop Conditions

- Stop if the installed binary cannot be matched to authoritative source.
- Stop if login requires Swallowtail to acquire, export, or persist credentials.
- Do not use Gemini credentials as Antigravity credentials.

## Auto-Continuation

Yes. Continue to card 016 after exact artifact evidence passes.

