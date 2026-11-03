# Qwen Code 0.26.3 through 0.26.6 identity

Secret-free evidence for `qwen.headless` on npm `@qwen-code/qwen-code`
`0.26.3`–`0.26.6`. The previous ceiling is `0.26.2` (Research 465). Tarballs
were downloaded, hashed, and inventoried. They were not executed and are not
in this repository.

`dist-inventory-0.26.2.json` is content-identical to the frozen
`qwen-code-0.26.2` fixture inventory. Selected mapped TypeScript, the
reviewed safety-boundary files, the `dashscope-source-config.ts` feeding
file, and the unified-loop opt-in/bound files (`core/config.ts`'s
`isUnifiedAgentLoopEnabled`, `nonInteractiveCli.ts`'s shared-loop wiring,
`settingsSchema.ts`'s `unifiedAgentLoop` entry) plus `tools/autofix-pr.ts`
are frozen under `source/` at the five tag snapshots. npm SLSA provenance
v1 binds every recorded tarball SHA-512 to the release workflow checkout;
those files are byte-identical at the GitHub tag and that checkout at every
recorded version. The identity test mutation-checks the opt-in, cancel,
permission, budget, and autofix bound snippets inside those frozen
snapshots at every recorded version.
