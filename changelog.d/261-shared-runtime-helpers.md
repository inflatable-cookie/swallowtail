# 261: Shared runtime cleanup, line framing, and attachment reads

- `swallowtail-runtime` now owns the three helpers the adapters copied:
  `CleanupOutcome::merge` composes sequential releases, `scan_lines` frames
  bounded LF-delimited streams in one linear pass, and
  `read_bounded_attachment` reads one host-materialized file without exceeding
  its byte bound. Each refusal keeps the calling adapter's own code and
  message.
- `claude-agent`, `codex`, `opencode`, and `pi` adopt them: eleven private
  `merge_cleanup` copies, four hand-rolled line framers, and three
  `read_bounded_image` copies are gone. `pi`'s two wire decoders now share one
  framer.
- Inside `claude-agent`, the headless and response-only routes share pump
  mechanics, plan checks, the stream-JSON invocation prefix, and the
  `claude --version` shape check through `claude_code_pump_common`,
  `claude_code_plan_checks`, `claude_code_invocation`, and
  `claude_code_discovery`. Route labels, stage names, and failure codes stay
  distinct, so every existing diagnostic is unchanged.
- Breaking public API change:
  `ClaudeAgentSdkPackageNativePair` is now an opaque struct over one static
  maintained row, replacing the 35-variant enum. `DEFAULT`, `MAINTAINED`,
  `package_version`, `native_version`, and `with_package_native_pair` keep
  their names and meanings; a caller that named a variant moves to
  `from_package_version` or `from_versions`. The `0.6.0` public API baseline is
  updated with the reviewed removal.
