# Cursor ACP and headless 2026.10.22 identity

Secret-free identity evidence for `cursor-agent.acp` and `cursor-agent.headless`
from the qualified `2026.10.01-14929f9` ceiling through official ACP registry
stable `2026.10.22-27dcf5f`.

Published registry hops after the ceiling are `2026.10.14-0f9edde`,
`2026.10.14-941492e`, `2026.10.16-e3cb207`, `2026.10.18-ea085cb`,
`2026.10.19-6bbf2c4`, and `2026.10.22-27dcf5f`. The claim keeps one opaque
build per calendar date, so `2026-10-14` is bound to `941492e` and
`2026.10.14-0f9edde` stays rejected. Catalogue stays at
`2026.10.01-14929f9`.

`identity.json` freezes registry commits, archive URLs, sizes, and SHA-256
digests for Darwin ARM64 and Linux x64, plus selected ACP, SDK, headless
writer, runtime index, launcher, package, and Node digests. `dist-inventory.json`
freezes complete extracted file sets and every hop's added, removed, changed,
and identical paths. `protocol.json` records the selected ACP and headless
surfaces and their comparison scope: ACP selected surfaces were compared on
Darwin at every hop and on Linux at ceiling and target only; Linux
intermediate ACP hops and headless selected argv/writer/result surfaces have
no per-hop selected-surface comparison. Writer-chunk hashes are artifact
identity.

The `cursor.com/install` script agreed on `2026.10.22-27dcf5f`. No archive
was executed. No provider prompt, credential, authenticated catalogue, or
host install was used.
