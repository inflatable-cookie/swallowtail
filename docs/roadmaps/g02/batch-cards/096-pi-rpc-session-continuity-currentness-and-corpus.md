# 096 Pi RPC Session Continuity Currentness And Corpus

Status: completed
Owner: Tom
Created: 2026-07-28
Milestone: `../029-pi-rpc-session-continuity.md`
Depends on: card 095

## Objective

Revalidate Pi RPC persistent-session continuity and freeze the exact offline
corpus before production behavior changes.

## Scope

1. Revalidate the current maintained Pi release from official tagged source.
2. Compare the existing `0.80.10` guaranteed point with every relevant
   maintained milestone found.
3. Close the exact RPC surface around:
   - `switch_session`
   - `get_state`
   - `get_messages`
   - `get_entries`
   - prompt admission and terminal settlement
4. Decide and record:
   - the exact opaque binding source
   - which ordered projection qualifies as portable load replay
   - how replay completion precedes readiness
   - how resume proves exact attachment without exposing replay
   - whether session switches can be cancelled or redirected by extensions
5. Freeze positive and negative fixtures for exact target, host, working
   resource, access, model, session, provider-state, and version agreement.
6. Cover missing session, cancelled switch, malformed or foreign entries,
   ordering failure, duplicate ids, frame/item/byte overflow, unsolicited
   events, cancellation, deadline, disconnect, and joined cleanup.
7. Confirm Contracts 009, 017, and 038 remain sufficient. Promote only an
   evidence-required narrow delta.
8. Use no account, credential, provider request, container, or model server.

## Acceptance Criteria

- [x] current upstream and exact guaranteed milestones are recorded
- [x] load and resume phases are specified separately
- [x] replay order and completion are mechanically testable
- [x] raw session paths and ids cannot mint attachment authority
- [x] persistent provider state is separate from the existing prohibited-state
      profile
- [x] process exit remains distinct from native close
- [x] all unsafe payloads and identities stay out of stable diagnostics
- [x] implementation scope has one exact public-interface gate

## Stop Conditions

- upstream response ordering cannot prove replay completion before readiness
- a persistent session can be selected only through ambient unbound state
- required behavior exists only on an undocumented or private interface
- route selection would require a new credential, endpoint, containment, or
  product-policy decision

## Auto-Continuation

Continue to card 097 only when the exact corpus is contract-ready and no
operator decision remains.

## Outcome

Research 053 revalidates all stable releases from `0.80.10` through current
`0.82.1`. Switch response ordering, state, active-message replay, append-order
entries, prompt admission, and settlement are usable.

The stop condition applies before corpus implementation. Public RPC
`switch_session` and non-interactive CLI attachment accept the cwd stored in
the session file. They do not bind it to the host-leased working resource or
report the effective cwd. Cards 097 and 098 are paused. Contracts remain
unchanged.
