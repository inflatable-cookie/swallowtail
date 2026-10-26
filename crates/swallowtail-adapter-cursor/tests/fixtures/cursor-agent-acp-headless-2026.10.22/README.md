# Cursor ACP and headless 2026.10.22 identity

Secret-free identity evidence for `cursor-agent.acp` and `cursor-agent.headless`
from the qualified `2026.10.01-14929f9` ceiling through official ACP registry
stable `2026.10.22-27dcf5f`.

Published registry hops after the ceiling are `2026.10.14-0f9edde`,
`2026.10.14-941492e`, `2026.10.16-e3cb207`, `2026.10.18-ea085cb`,
`2026.10.19-6bbf2c4`, and `2026.10.22-27dcf5f`. The claim keeps one opaque
build per calendar date, so `2026-10-14` is bound to `941492e` and
`2026.10.14-0f9edde` stays rejected. Catalogue is independently qualified
through `2026.10.22-27dcf5f` by
[Research 451](../../../../../docs/research/451-cursor-agent-2026-10-22-catalogue-identity.md).
This fixture does not extend the catalogue claim.

`identity.json` freezes registry commits, archive URLs, sizes, and SHA-256
digests for Darwin ARM64 and Linux x64, plus selected ACP, SDK, headless
writer, runtime index, launcher, package, and Node digests. `dist-inventory.json`
freezes complete extracted path sets and every hop's added, removed, changed,
and identical paths, with selected-file digests and path-set digests. It does
not freeze a digest for every archive member. `protocol.json` records the
selected ACP and headless control-flow extracted from those retained archives
on both platforms at every hop: initialize/auth/permission/cancel/replay
literals, SDK `0.14.1` / Zod packaging, and headless argv/mode/writer result
fields, with SHA-256 of the byte-identical cores. Session cleanup is host-owned
and has no vendor literal. Writer-chunk hashes remain artifact identity.
The elided system/init and result summaries are field descriptions, not
byte-identical shipped excerpts, and carry no literal digest.
Archives were not executed.

The `cursor.com/install` script agreed on `2026.10.22-27dcf5f`. No archive
was executed. No provider prompt, credential, authenticated catalogue, or
host install was used.
