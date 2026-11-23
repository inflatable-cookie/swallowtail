# Pi RPC 1.6.0 evidence

[Research 485](../../../../../docs/research/485-pi-rpc-1-6-0-identity.md)
records the two qualification stops. The corpus is scoped to the published
npm CLI's fresh RPC route.

- `identity.json` records npm, GitHub and bundle identity as observed on
  2026-11-22. The provenance fields describe the decoded npm SLSA payload.
- `dist-inventory.tsv` lists all paths relative to the tarball's `package/`
  directory, in path order. Each version column holds the regular file's
  SHA-256 or `-` for absent. For a tree hash, take that version's present
  paths and hash the UTF-8 concatenation of `path<TAB>digest<LF>`. Adjacent
  columns distinguish added, removed, changed and identical paths.
- `source-correlation.tsv` lists package-relative JavaScript sourcemaps and
  repository-relative source paths. A digest is SHA-256 of the UTF-8
  `sourcesContent`, checked against the path in that version's exact tag.
  The agent-core package corroborates source for the agent loop that also
  appears in the coding-agent CLI bundle.
- The two `aborted-batch-*.jsonl` files are synthetic projections of the
  selected abort sequence from source. They omit unrelated prompt, response
  and result-message frames and exercise the Rust decoder and activity
  projection without running Pi. The abort acknowledgement is supplied by
  the unit test.

For the bundled parallel tool-loop hash, read the main chunk reached by
`dist/bundle/cli-runtime.js` and hash the UTF-8 span from
`async function executeToolCallsParallel(` to just before the next function
declaration, omitting trailing whitespace. The two loop-break sites are
present through 1.5.2 and absent in 1.6.0.

Each differing path in the complete npm inventory has one class.

| Class | Selected-path boundary |
| --- | --- |
| `bundle` | Executed CLI and compiled dependency chunks, plus alternate entry points. The selected launcher, main chunk and changed cancellation loop are traced directly. Other chunk changes are bounded by source and the selected mode and flags. |
| `cli-bootstrap`, `rpc-wire` | Entry, SDK construction, command dispatch, framing, event forwarding and shutdown. The selected wire records and stop are detailed in the research. |
| `session-usage-cleanup` | In-memory session, tool loop inputs, usage and shutdown. Stored-session operations and compaction navigation are unselected. The cache warmer is reachable and is a stop. |
| `read-tools-images` | Selected provider-owned read/search tools, internal truncation, paths and image normalization. The adapter projects bounded images and tool lifecycle, and does not interpret tool output or argument content. |
| `unselected-tools` | Shell and mutating tools stay outside `--tools read,grep,find,ls`. Catalogue uses `--no-tools`. |
| `startup-resources`, `settings-schema` | Settings, trust, resource loading and template selection. The route supplies no explicit resources and disables extensions, skills, prompt templates, themes and context files. This boundary permits ambient user settings and does not suppress the new idle default. |
| `model-auth` | Configured catalogue, explicit model selection, delegated auth and provider request plumbing. The boundary is the selected model tuple and projected assistant event/usage keys. Downstream transport, prices, auth-provider catalogues and protocol changes are not qualified independently. |
| `extension-mcp` | Extension factories, package and MCP behaviors sit behind extension loading or a distinct subcommand. The selected CLI supplies `--no-extensions` and no explicit extension path. |
| `theme-diagnostics` | Shared theme types and diagnostic reporting. The executable suppresses theme discovery and reserves stdout for RPC. |
| `other-ui`, `other-cli` | Interactive TUI, print mode, export, login/logout and package-management commands are not selected by the RPC argv. |
| `npm-package`, `map-declaration`, `docs-examples` | Package identity and pins, generated type/debug metadata, and documentation or examples. They do not add wire operations. |
