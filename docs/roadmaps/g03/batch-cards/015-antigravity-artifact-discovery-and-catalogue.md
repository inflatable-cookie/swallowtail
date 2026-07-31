# 015 Antigravity Artifact, Discovery, And Catalogue

Status: completed
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

- [x] the first claim names one exact authoritative artifact
- [x] tag aliases cannot create duplicate or invented behavior ranges
- [x] authentication remains provider-owned and secret-free
- [x] catalogue entries do not imply invocation availability
- [x] Gemini and Antigravity retain separate identities and access profiles
- [x] focused discovery, compatibility, catalogue, and package tests pass

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

Completed. Continue to card 016.

## Result

Research 078 reconciles the installed Google-signed macOS arm64 `agy` `1.1.9`
artifact, SHA-256
`a27bff8d7c47fe5407e6740f14ecef73e86fb65ec73fec77b0765f8849024383`,
against official documentation and the shared `1.1.8`/`1.1.9` source commit.
Only exact `1.1.9` is qualified. Later stable releases remain visible as
unverified newer; the `1.1.8` alias does not create a second behavior range.

The new `swallowtail-adapter-antigravity` package exposes a separate
`antigravity` catalogue driver, host-approved `agy --version` discovery,
provider-owned personal Google access, bounded `agy models` parsing, and safe
exit diagnostics. Model entries preserve opaque identity only and make no
invocation or entitlement claim. Gemini and enterprise access remain separate.

Focused validation passed 13 tests across two binaries plus warnings-denied
checking in one second. The 19-file package assembled and compiled from its
extracted archive in two seconds. No provider prompt, login mutation,
credential read, consumer edit, public route-count change, or publication ran.
