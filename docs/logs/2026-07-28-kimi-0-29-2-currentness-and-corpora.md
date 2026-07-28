# 2026-07-28 Kimi 0.29.2 Currentness And Corpora

## Changed

- audited exact Kimi Code `0.29.1` and `0.29.2` release source against
  `0.29.0`
- extended ACP declared-effort compatibility through `0.29.2`
- added a local-server `0.29.1..=0.29.2` behavior revision for global event
  fan-out and configured-catalogue filtering
- made the local WebSocket pump ignore recognized events for foreign sessions
  while retaining strict bound-session parsing
- froze exact metadata, catalogue, WebSocket, and default headless JSONL
  corpora
- executed ACP, reasoning, catalogue, archive, restore, and interactive
  compatibility checks at the newly qualified releases

## Current State

Kimi ACP and local-server routes guarantee exact `0.28.1` plus
`0.29.0..=0.29.2` where their behavior segments apply. Later releases remain
visible and executable as unverified-newer, not denied.

The default Kimi headless surface is frozen for a separate structured driver.
The experimental v2 prompt engine remains excluded. Kimi session deletion
remains unsupported.

## Evidence

- Research 046 records exact tags, commits, selected source blobs, and delta
  dispositions.
- `cargo test -p swallowtail-adapter-kimi` passes all deterministic tests; the
  one installed live probe remains separately gated and ignored.
- no installed Kimi binary, credential, account, provider request, inference,
  or live local server was used.

## Next

Card 078 adds separate Kimi headless and retained local-server structured-run
roles using the qualified `0.29.2` evidence.
