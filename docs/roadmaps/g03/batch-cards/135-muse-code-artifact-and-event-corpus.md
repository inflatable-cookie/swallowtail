# 135 Muse Code Artifact And Event Corpus

Status: completed
Owner: Tom
Created: 2026-08-06
Milestone: `../045-muse-code-headless-foundation.md`
Depends on: card 134

## Goal

Freeze exact Muse Code `0.1.0-R708.1` artifact, command, JSONL, correlation,
terminal, and failure evidence before production Rust behavior exists.

## Scope

1. Record sanitized launcher and signed versioned-payload identity.
2. Freeze exact `--version`, root help, `exec --help`, echo success, Meta
   success, and bounded negative JSONL fixtures.
3. Define strict envelope, stream, sequence, causation, model, task, output,
   terminal, and unknown-event rules.
4. Prove direct payload invocation bypasses launcher update mutation.
5. Name the exact qualified-only compatibility and protocol-facade revisions.

## Acceptance

- [x] fixtures contain no credentials, account identifiers, private paths,
      provider request ids, or unredacted reasoning
- [x] random identities are consistently sanitized without weakening
      correlation evidence
- [x] the exact payload and launcher remain distinct artifacts
- [x] malformed, oversized, reordered, cross-session, post-terminal, and
      mismatched-model records fail safely
- [x] unknown bounded payload types remain namespaced observations and do not
      gain semantic authority

## Validation

- focused package-independent fixture/parser tests introduced by this card
- `effigy qa:northstar`

## Stop Conditions

- stop if direct payload execution needs launcher-private mutable state
- stop if JSONL output cannot bind one command, session, run, and terminal
- stop if sanitized fixtures cannot retain exact lifecycle meaning

## Auto-Continuation

Completed. Continue to card 136; the shared fixture tree is ready for the Rust
driver and no further provider capture is required.

## Completion

The exact launcher, signed versioned payload, direct command surface, complete
echo stream, sanitized Meta success projection, bounds, correlation rules, and
ten negative mutations are frozen under the future adapter test tree. The
package-independent validator passes five tests. No additional authenticated
provider work ran.
