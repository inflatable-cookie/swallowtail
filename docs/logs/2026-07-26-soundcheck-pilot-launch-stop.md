# Soundcheck Pilot Launch Stop

Date: 2026-07-26
Card: `../roadmaps/g02/batch-cards/043-soundcheck-secondary-application-proof.md`

The operator approved Soundcheck card 092's ceiling of 5 provider attempts,
1 native launch, 30 minutes, subscription effects, and host-approved search.

The recorded Soundcheck, Swallowtail, Codex, and host values matched. The
16-record fixture prepared with zero attempts. Adding the offline helper binary
had made the default Effigy development command ambiguous, so the app binary
was selected explicitly. The resulting one native launch stopped before
assistant execution:

`soundcheck library schema v50 is newer than supported schema v48`

The candidate tuple was incomplete. Soundcheck consumes local Rust and
frontend path sources:

- `soundcheck-library` HEAD `e88c76a`, 51 worktree entries
- Poodle HEAD `666b985`, 16 worktree entries
- Signal HEAD `fe37838`, clean
- Swallowtail HEAD `0b0d688`, clean; runtime unchanged from `a3fbc14`

The seed and cached native app did not represent one reproducible source graph.
No provider request, external search, credential exchange, or
subscription-backed model call occurred. The attempt ledger remained empty.
The app and development runner joined, and guarded teardown removed the marked
proof root.

Soundcheck card 091 is reopened. Card 043 remains paused until dependency
owners provide clean committed checkpoints, the full local source graph is
frozen, the normal Effigy launch target is repaired, the fixture opens through
the native app offline, and a fresh live envelope is approved.
