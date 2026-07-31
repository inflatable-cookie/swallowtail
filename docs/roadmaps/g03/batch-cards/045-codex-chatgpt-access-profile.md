# 045 Codex ChatGPT Access Profile

Status: completed
Owner: Tom
Created: 2026-07-31
Milestone: `../017-prepared-facade-multi-consumer-usability.md`
Depends on: card 044

## Goal

Replace duplicated consumer construction of the fixed Codex ChatGPT-
subscription profile without asserting readiness or widening access.

## Scope

1. Add a public Codex ChatGPT-subscription audience constant and profile
   constructor.
2. Require a caller-supplied `AccessProfileId`.
3. Encode interactive OAuth, subscription allowance, and provider support.
4. Attach no credential reference and create no access status.
5. Prove API-key, enterprise-token, and public OpenAI API authority are not
   implied.

## Acceptance Criteria

- [x] both Codex drivers can receive the canonical profile through unchanged
  preparation input
- [x] access status and provenance remain separate preparation inputs
- [x] the helper performs no discovery or effect
- [x] other login and billing routes remain explicit and separate
- [x] focused Codex validation passes
- [x] card 046 becomes the sole ready and next task

## Validation

- `effigy validate:focused swallowtail-adapter-codex`
- `git diff --check`
- no authenticated Codex execution

## Auto-Continuation

Yes. Continue to card 046 after focused Codex acceptance.

## Evidence

- `codex_chatgpt_subscription_access_profile` accepts a caller-owned profile
  id and encodes only the fixed Codex ChatGPT-subscription route facts
- the constructor attaches no credential reference and creates no access
  status or provenance
- both prepared driver fixture paths accept the profile through unchanged
  `CodexPreparationInput`
- focused Codex validation: 143 passed
