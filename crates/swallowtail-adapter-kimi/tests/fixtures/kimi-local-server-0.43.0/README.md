# kimi-local-server-0.43.0

Research 326 froze this corpus before any claim moved.

`identity.json` carries official npm/GitHub identity for every published
stable after the local-server ceiling (`0.39.0`, `0.39.1`, `0.40.0`,
`0.40.1`, `0.41.0`, `0.42.0`, `0.43.0`) plus the `0.38.0` revalidation point.
The `0.39.1` through `0.41.0` values reproduce Research 270, Research 282,
and Research 325; the `0.42.0`/`0.43.0` tarball, bundle, tag, commit, and
tree values reproduce Research 325 and were re-verified from the official
channels this run. The installed host stays `kimi 0.34.0` at the frozen
digest, observed only through `--version` and its digest.

`protocol.json` carries the mutation-sensitive selected-file git-blob ledger
over `0.38.0..=0.43.0`, the Bash `cwd` authority trace, the per-hop verdict,
and the containment trace. The safe prefix is `0.39.0..=0.39.1`; the
`0.40.0` Bash `cwd` widening persists byte-identical through `0.43.0` with no
restored containment, so every point above `0.39.1` fails closed.

No downloaded artifact was executed, installed, or authenticated. No prompt,
catalogue call, login, credential, host update, live session, or local-server
start occurred.
