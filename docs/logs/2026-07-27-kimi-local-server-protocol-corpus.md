# 2026-07-27 Kimi Local Server Protocol Corpus

## Changed

- added the separate `swallowtail.kimi.local-server` descriptor and
  `kimi-local-server-rest-ws-v2` transport identity
- left the descriptor role-free so it cannot enter production registration
- qualified exact Kimi Code `0.28.1` and `0.29.0` local-server behavior
- retained visible, permitted `UnverifiedNewer` evidence above `0.29.0`
- added bounded adapter-private REST and WebSocket v2 records and decoders
- froze secret-free selected fixtures for metadata, lifecycle, errors,
  cursors, events, resynchronization, abort, and connection close
- proved the selected session surface has archive and restore but no delete
- required exact server metadata to corroborate the executable version

## Evidence

The corpus comes from exact tagged source:

- `0.28.1` commit `efacf0452d46f5dbd67499eabc053869495d5213`
- `0.29.0` commit `8bf5bacba9e524c38fb808c0122070037ead25a8`

Selected envelope, error, metadata, session, WebSocket, and authentication
source files are byte-identical between the two commits. Fixture provenance
records the exact files and distinguishes the deprecated archive-response
alias from a delete route or effect.

No server process, endpoint, credential, provider request, workspace
mutation, or provider-session effect occurred.

## Validation

- strict Kimi adapter Clippy passes
- the full Kimi adapter suite passes with one live installed probe ignored
- documentation, Northstar, and 22-route matrix checks pass
- `git diff --check` passes

`effigy package:api` exits with the expected held-candidate diff. The additive
Kimi descriptor and claim join the unbaselined lifecycle APIs from cards
046-057. Card 059 owns candidate baseline replacement after canonical source
history exists. This fixture-only card does not rewrite release evidence from
the dirty working tree.

## Lane State

- card 061 is complete
- card 062 is ready
- cards 063-065 remain in bounds
- roadmap g02.020 remains active
- card 059 remains paused at its canonical-source gate

## Next

Execute card 062. Wire attached and owned-foreground lifecycle operations to
the frozen boundary without disabling authentication or claiming deletion.
