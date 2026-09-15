# 2026-09-15 Antigravity 1.2.2 Identity

Research 323 froze official GitHub `google-antigravity/antigravity-cli`
identity for the five stable hops `1.1.27`, `1.1.28`, `1.2.0`, `1.2.1`, and
`1.2.2` and classified the selected surfaces per claim under Tom's
2026-09-15 operator ruling that the official release notes are the
behavioural authority for g05.075. Exhaustive binary scanning stopped at
the ruling; the hashes collected by the pre-ruling probe are preserved in
the frozen corpus.

Every linux-x64 and mac-arm64 tarball SHA-256 matches GitHub's declared
asset digest exactly, each archive holds only the single `antigravity`
binary with its own version literal, and re-downloading `1.1.21`, `1.1.22`,
and `1.1.26` reproduced the Research 283 digests byte-for-byte, so the
frozen boundary stands. Host `agy` remains the byte-identical official
`1.1.19` mac-arm64 build and was never executed.

The decision is a per-claim split: the catalogue claim advances to
maintained `1.1.9..=1.2.2` because the release notes name no selected-path
change to `agy models` in any hop, while the headless claim keeps its
`1.1.17` ceiling because the `1.1.22` provider-managed HTTP 502 retry
stands and `1.1.28` ("much longer" exponential retry) and `1.2.1`
(broader automatic retry of 502/503/504, per-minute 429, and mid-stream
interruptions) publish no finite bound or disable control. `1.1.18..=1.2.2`
stay the named unqualified gap; `1.1.28`'s partial-output print-timeout
expiry and `1.2.0`'s content-filter stop reason are classified inside that
gap. No downloaded binary was executed and no provider, prompt, login,
credential, installation, or host-mutation operation occurred. PR review
and merge remain queue-owned.
