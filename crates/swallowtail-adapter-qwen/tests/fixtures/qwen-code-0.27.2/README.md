# Qwen Code 0.26.6 through 0.27.2 identity

Secret-free evidence for `qwen.headless` on npm `@qwen-code/qwen-code`
`0.26.6`–`0.27.2`. The previous ceiling is `0.26.6` (Research 475). Tarballs
were downloaded, hashed, and inventoried. They were not executed and are not
in this repository.

`dist-inventory-0.26.6.json` is content-identical to the frozen
`qwen-code-0.26.6` fixture inventory. Selected mapped TypeScript, the
reviewed safety-boundary files, the `dashscope-source-config.ts` feeding
file, the unified-loop opt-in/bound files, `tools/autofix-pr.ts`, and
`services/mod-session.ts` are frozen under `source/` at the seven recorded
versions. Snapshots come from the SLSA build checkout, which is the tree the
published tarball was built from; `source-to-shipped.json` records the tag
copy of every selected and reviewed file and proves parity. At `0.26.9` the
release workflow checked out a later `main` than its tag, so the tag copies
of the two differing reviewed-boundary files live under
`source/0.26.9-tag/`.

The identity test mutation-checks the unified-loop, safe-mode hook accessor,
Mods, autofix, and post-result memory-drain bounds inside those frozen
snapshots at every recorded version.
