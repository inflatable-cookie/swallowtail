# 2026-07-28 Pi RPC Attachment Input

## Changed

- Added one optional `image/png` input, capped at one MiB, to prepared Pi
  structured runs.
- Added explicit image-turn opt-in to prepared Pi interactive sessions.
- Bound attachment capability and host services into preflight.
- Materialized the opaque attachment reference, read it through bounded
  blocking work, and encoded Pi's exact inline base64 image record.
- Shared one lease between session and turn cleanup so rejection, terminal,
  close, and session shutdown release at most once.
- Changed only the Pi attachment matrix cell from `No` to `Yes`.

## Evidence

- Focused Pi suite: 36 passed.
- Prepared tests cover structured and interactive dispatch, exact media and
  base64 content, no materialized path on wire, and one release.
- No installed harness, credential, network, or provider access was used.

## Next

Continue card 090 with OpenCode HTTP file parts and correlated one-shot
permission and question exchange. Anthropic Messages remains the final
three-cell sub-batch.
