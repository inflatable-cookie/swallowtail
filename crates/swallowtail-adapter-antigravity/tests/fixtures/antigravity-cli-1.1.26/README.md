# Antigravity CLI 1.1.26 identity corpus

Secret-free identity freeze for official GitHub
`google-antigravity/antigravity-cli` `1.1.17..=1.1.26`. Every
linux-x64 release tarball stayed in `/tmp`, was hashed, and was extracted
without executing the downloaded binary. Each archive contains one changed
file, `antigravity`. In-binary scans retain every selected flag, `models`, and
the per-release version literal.

The host binary was not executed. Its bytes match the official signed
`1.1.19` mac-arm64 artifact. Fresh `1.1.16` and `1.1.17` downloads recompute
the frozen `1.1.17` corpus. Fresh `1.1.17..=1.1.24` downloads plus the
`1.1.24` mac-arm64 asset match every digest parked on PR 182.

This corpus records a stop, not a claim extension. `1.1.22` changed selected
headless failure behavior to retry HTTP 502 responses. The closed artifact
publishes no bound or disable control, and Contract 023 requires separate
acceptance for provider-managed retry. Production remains qualified only
through `1.1.17`; card 072 is not admitted. `1.1.8` stays incompatible,
`1.1.27` is the first unpublished later stable, and the decoder corpus stays
at `antigravity-cli-1.1.9`.
