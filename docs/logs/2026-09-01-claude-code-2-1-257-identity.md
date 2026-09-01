# 2026-09-01 Claude Code 2.1.257 Identity

## Result

Card 046 froze official npm `@anthropic-ai/claude-code` `2.1.257` against
the `2.1.252` claim. Host `claude` is exact `2.1.257`
(`64590d7d9d9c189d33fb3dfa58c5408eaf2a10fe556bd84155d95efaab46b60e`) and
matches official darwin-arm64. Official binaries were hashed and not
executed. Wrapper installer files except `package.json` and
`sdk-tools.d.ts` are byte-identical to `2.1.252`. Official `--help` is
not byte-identical to frozen `2.1.252`
(`5ff2e7a0bca8535fb9ec097fa0a21e9d6b735ed94104fa0d1f58ac73a841d52d`);
the dump adds `--system-prompt-snapshot` and expands `--bg` resume
wording. Selected mapped stream-JSON flags stay. Changelog `2.1.257`
extras stay unmapped. Published stables after `2.1.252` contain
`2.1.257` only. Unpublished `2.1.244` and `2.1.249` stay. Hop-skipped
unpublished `2.1.253`–`2.1.256` become gaps after qualification.
Unpublished `2.1.258` is the first later stable. Watcher help/digest
stays on exact `2.1.251`; official `2.1.257` is a different help digest
and is rejected at both watcher admission seams. Production claims
stayed at `2.1.252` in this card. Decision for card 047: compatible
extension of both existing stream-JSON behaviors through `2.1.257`.

## Next

Raise both qualified ceilings on card 047. Keep watcher exact `2.1.251`.
Add hop-skipped unpublished `2.1.253`–`2.1.256` to the deny lists.
