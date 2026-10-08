# Qoder headless 1.1.65 currentness corpus

Secret-free npm artifact identity for `qoder.headless`. The previous qualified
point was `1.1.54`; stable versions `1.1.55` through `1.1.65` are published
and classified hop by hop in `dist-inventory.json`.

Each exact npm tarball matched the registry SHA-1 and SRI metadata. The
complete 30-file trees are frozen with SHA-256 digests. The public package
metadata has no source repository, so the npm artifacts are the identity
authority. No downloaded code was executed and no install or provider call
was made.

The selected invocation remains `qodercli --print --output-format
stream-json --permission-mode dont_ask --max-turns 8
--no-session-persistence --cwd DIR PROMPT`. The selected CLI/runtime markers
remain present through `1.1.65`; the adapter keeps its existing stream-json
mapping, eight-turn ceiling, permission mode and process lifecycle. At
`1.1.61` the provider runtime adds private error-code normalization for
`repeated_tool_call_denied`; the adapter keeps its generic provider-failure
projection and exposes no provider error details.

Changes to the vendored Qoder Security plugin remain outside route mapping.
Provider skills, plugin hooks and selected-run skills visibility remain
unqualified under the separate Research 256 gate.
