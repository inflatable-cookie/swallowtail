# 401 Antigravity Headless Static Control-Flow Evidence

Status: evidence only; no qualification or claim change.

Owner: swallowtail#147
Date: 2026-10-08
Axis: `antigravity-cli.release`
Route: `antigravity.headless`

## Outcome

This record replaces the strings-only inspection of the newer Antigravity
headless artifacts with exact Mac ARM64 function and direct-call mappings for
`1.2.11` through official stable `1.3.1`. It maps the selected
`--print --output-format stream-json` entrypoint, workspace and custom-agent
resource branches, retry-override parsing, retry executors and classifiers,
and symbolic `GEMINI_API_KEY` references.

The mappings establish several exact branches, but do not establish a complete
selected-run link from resource resolution to tool or permission decisions,
nor from `AGY_CLI_MODEL_API_MAX_RETRIES` to the newer model-request loop. The
finite missing edges are recorded below. In particular, this record does not
claim that setting the variable to `0` makes one model request on any version
after `1.2.11`. Research 359 remains the only evidence for that request-count
result. Qualification, route mappings and environment requirements stay
unchanged.

## Identity and method

Official GitHub Releases still reported stable `1.3.1` on 2026-10-08, published
`2026-10-07T03:22:02Z`, tag commit
`968f1170bd0e002e9d0914730975bc8a2cc65861`. The inspected set is the exact
`1.2.11` headless boundary plus every stable published hop through `1.3.1`:
`1.2.12`, `1.2.13`, `1.2.14`, `1.2.15`, `1.2.16`, `1.2.17`, `1.3.0`, and
`1.3.1`.

The nine official Mac ARM64 archives were acquired into task-owned scratch,
checked against their frozen SHA-256 values, and listed before extraction. Each
contains exactly one regular `antigravity` file. The extracted file's SHA-256,
byte size, tag commit, Go function count, mapped function addresses and body
digests, selected direct-call lists, and environment-literal reference PCs are
frozen in
[`control-flow.json`](../../crates/swallowtail-adapter-antigravity/tests/fixtures/antigravity-headless-static-boundary/control-flow.json).
The script that recreates it verifies each extracted-file digest and size
against the frozen `1.2.11` identity and Research 380's `1.2.12`–`1.3.1`
inventory. Research 353 and Research 380 hold the complete archive and
published-asset inventories. Adjacent public tag diffs remain exactly
`CHANGELOG.md`; that source classification is inherited from Research 380.

The analysis parses Mach-O ARM64 load commands and Go pclntab function ranges,
then decodes direct ARM64 `BL` calls and bounded `ADRP`/`ADD` references to the
two environment-name literals. Address-specific disassembly was read from the
verified files without launching them. No `agy` command, artifact execution,
provider request, auth-store access, credential read, installation or host
mutation occurred. No environment value is read or logged. This is static
evidence for macOS ARM64 only; it does not establish Linux or Windows runtime
behavior.

Recreate the frozen mapping from a directory containing one extracted file
per version with:

```sh
effigy validate:antigravity-headless-static-boundary /path/to/extracted-artifacts /path/to/control-flow.json
```

The analyzer's synthetic refusal cases cover ambiguous function names, wrong
identity, unreachable calls, and invalid binary format. These tests exercise
the analyzer only; they are not provider evidence.

## Selected headless path and resources

The adapter's exact argument prefix is `--print <prompt> --output-format
stream-json --model <model>`. It may append `--mode plan`, `--sandbox`,
`--effort`, `--json-schema`, and `--conversation`; it never adds
`--dangerously-skip-permissions` or `--continue`.

Every inspected binary has one exact `entrypoints.launchCLI` mapping and the
version-specific print runner (`printmode.Run` in `1.2.11`–`1.2.13`, then
`printmode.run`). The direct-call graph reaches the print runner,
`runStreamInput`, and `session.runTurn`. The stream path decodes input, handles
messages and reaches `runTurn`; that turn sends through
`store.Manager.SendUserMessage`. `launchCLI` also calls `resolveProjectPaths`
and registers workspace folders. `newSession` directly selects a named agent
and applies its agent mode. These bounded calls identify the adapter-selected
headless entrypoint and the exact workspace/agent subsystems present in each
build. `launchCLI` directly calls `resolveProjectPaths` and
`addWorkspaceFolders`. Custom path discovery maps `Manager.discoverPaths` →
`discoverPathsCore` → working-directory contexts and
`discoverPathsCoreWithContexts` → `discoverFromContexts`; the agent-specific
branch calls `discoverAgentCustomizations`.

The separate custom-agent RPC path maps as follows in every inspected build:
`ServerBackend.GetAgentCustomizations` → `Manager.GetAgentCustomizations` →
`Manager.getAgentCustomizations` → `getAgentScriptItem` → `loadMDAgent` or
`loadJSONAgent`. Agent path resolution derives its base from the agent file's
directory, clones the item/configuration, resolves configured resource paths
against that base, and converts results to URIs. JSON resource entries flow
through `loadConfigFile` → `resolveJSONConfig` → `resolveEntry` → `resolveDir`;
relative paths are joined to their containing base and lexically cleaned,
while absolute paths are parsed as absolute. `statCached` checks the resolved
path. `resolveProjectPaths` obtains project paths from the current working
directory, while `addWorkspaceFolders` registers those paths as workspaces.
The 1.3.0 path-handling hop is mapped to this selected entrypoint and the exact
custom-agent discovery/context chain in that artifact; `1.3.1` retains the
chain and changes the agent loader below.

Two precise resource changes are visible in the selected builds:

- Beginning at `1.2.16`, custom-manager initialization calls
  `path/filepath.EvalSymlinks` on workspace roots. This does not establish
  canonicalization of every resource leaf.
- `1.2.16` adds a `Path.Join` call in `resolveDir`; it remains present in
  `1.2.17`, `1.3.0`, and `1.3.1`. `resolveEntry` joins relative configuration
  entries to their containing base and cleans the joined path.
- In `1.3.1`, `loadMDAgent` newly calls `setDefaultAgentPath` before
  `resolveAgentPaths`; this uses the agent path URI to supply a default path.
  It is a reachable change inside the mapped custom-agent loader. The
  `1.3.1` `resolveEntry` body also differs in digest and size from `1.3.0`,
  with the same relative-join and cleanup path operations.

`fs.Path.Join` delegates to Go's lexical `path.Join`. The mappings do not show
that leaf resources are symlink-resolved or that two aliased paths canonicalize
to the same resource. Alias behavior for resource leaves therefore remains
unknown.

The mapped backend builds cascade configuration and directly calls
`populatePermissionConfig`; permission grant-store creation calls
`grantsFromConfig`, and `permissionManager.EnsurePermissions` and its callback
are both mapped. The print session directly calls agent selection and mode
application. Those mappings show where agent data and permission/tool
configuration decisions reside, but not the argument-level flow from a
resolved resource through the RPC boundary into a particular tool or
permission decision.

## Retry and auth paths

In all nine binaries, the exact literal `AGY_CLI_MODEL_API_MAX_RETRIES` has one
code reference, inside
`backend.applyModelAPIMaxRetriesOverride`. The mapped body performs environment
lookup, trims whitespace, parses base-10 unsigned 32-bit input, preserves the
existing config on absent, empty or parse-error branches, and stores a valid
parsed value (including zero) into the backend retry configuration. The direct
caller is `ServerBackend.buildCascadeConfigInternal`. This proves the loader
branch and config write for each exact build; it does not prove the resulting
request count.

The mapped retry executors are exact per build. Through `1.2.15`, the core
routine is `gemini_coder/framework/core/core.generateWithAPIRetry`; from
`1.2.16` through `1.3.1`, it is
`oneharness/internal/core/core.generateWithAPIRetry`. Both mapped variants
call their `runAttempt` and `sleepWithContext` helpers. Each build also carries
`PlannerGenerator.generateWithAPIRetry`, which calls its attempt generator,
retry predicate, error analyzer and timer. The full function addresses, byte
sizes, body hashes and selected calls are in `control-flow.json`.

`IsRetryableAPIError` and its helper call targets are mapped per artifact. The
exact direct-call inventory records these hop differences:

| Hop | Exact mapped change in retry classifier | Published note in Research 380 |
| --- | --- | --- |
| `1.2.11→1.2.12` | Adds `isGeminiAPIHardQuotaError` | Gemini API-key quota handling |
| `1.2.12→1.2.13` | Adds `isHardQuotaExhaustedMessage` and another retry-delay extraction call | Retry-delay handling |
| `1.2.13→1.2.14` | Exact body digest changes; no new helper family is inferred | Queued messages and other CLI changes |
| `1.2.14→1.2.15` | Adds `hasErrorInfoReason` | Permission, customization, MCP-auth and other changes |
| `1.2.15→1.2.16` | Retry executor package moves from `gemini_coder` to `oneharness`; classifier body digest changes | Customization and skills changes |
| `1.2.16→1.2.17` | Exact retry bodies are re-mapped; no additional class is inferred | Windows sandbox and other changes |
| `1.2.17→1.3.0` | Exact retry bodies are re-mapped; no additional class is inferred | Path handling and other changes |
| `1.3.0→1.3.1` | Exact retry bodies are re-mapped; same mapped helper families | Gemini API-key retry |

These call inventories identify the branches requiring further interpretation;
they do not make the baseline's attempt count, failure-class set, exponential
delay formula, or 30-second cap transferable to a newer build. The newer
executors call a sleep helper and the classifier references retry-delay
extraction, but this analysis did not recover an exact newer per-class
predicate-to-delay mapping, retry budget comparison, or effective cap for every
build. Research 359's `0` → one model request result remains exact to `1.2.11`.

Each build has literal-reference PCs in the following API-key functions:
`genai.defaultEnvVarProvider`, `genai.getAPIKeyFromEnv`, and
`ServerBackendConfig.chainedAuthOrDefault`. Those exact symbolic paths allow
review of present/absent branches without an absence attestation or real key.
The mapper freezes the PCs but this run does not establish, for each newer
build, which auth result reaches each retry-classifier branch or whether the
API-key-specific retry applies identically to both present and absent cases.
No new key requirement or absence rule is introduced.

## Finite missing edges

The exact mappings stop at these points:

1. The direct-call graph does not connect
   `store.Manager.SendUserMessage` to `ServerBackend.SendUserMessage`; the
   selected path crosses the backend RPC/dispatch boundary.
2. It does not connect `buildCascadeConfigInternal` to the version-specific
   API retry executor. The retry configuration field's passage into that
   executor, the attempt-budget comparison, and proof that zero means one
   request are therefore unresolved for `1.2.12`–`1.3.1`.
3. It does not connect `printmode.newSession` to
   `Manager.GetDefinedAgentByName`; agent retrieval is behind the backend
   service boundary. It also does not track resolved-resource values through
   RPC into an individual tool or permission decision.
4. `permissionManager.EnsurePermissions` reaches its callback indirectly;
   callback invocation and the resulting selected decision are not a direct
   `BL` edge.
5. Workspace roots are symlink-resolved from `1.2.16`, but leaf-resource
   canonicalization and alias equivalence are not mapped.
6. Newer retry-class predicates, per-class delay selection, retry-delay
   floors/caps, and auth-present/auth-absent effects are not fully derived for
   every exact build. The classifier, delay-extraction, API retry, Planner
   retry, key-provider and chained-auth function identities and body digests
   are retained for that bounded follow-up.
7. No Linux x64 or Windows executable control-flow evidence is included.

No exact later-version request-count, delay, cap, or alias claim is inferred
from function names, string presence, changelog text, or the older `1.2.11`
proof. Resolving these edges needs fuller static reconstruction or a separately
authorized proof method. This evidence record alone does not qualify newer
headless points and does not change the approved personal-subscription route,
resource boundary, public mapping, release, or environment policy.

## Sources

- [Official Antigravity CLI release `1.3.1`](https://github.com/google-antigravity/antigravity-cli/releases/tag/1.3.1)
- [Official CLI changelog at `1.3.1`](https://github.com/google-antigravity/antigravity-cli/blob/1.3.1/CHANGELOG.md)
- [Official headless guide](https://antigravity.google/docs/cli/headless/)
- Adapter-selected arguments in [`headless_command.rs`](../../crates/swallowtail-adapter-antigravity/src/headless_command.rs)
- Exact static mappings in [`control-flow.json`](../../crates/swallowtail-adapter-antigravity/tests/fixtures/antigravity-headless-static-boundary/control-flow.json)
  and the offline [`analyzer`](../../scripts/analyze-antigravity-headless-static-boundary.py)
- [Research 353](./353-antigravity-headless-adaptation-to-current-official.md),
  [Research 359](./359-antigravity-headless-retry-pin-evidence.md), and
  [Research 380](./380-antigravity-cli-1-3-1-catalogue-identity.md)
- [Contract 023](../knowledge/contracts/023-harness-operation-isolation-and-native-boundary.md)
  and [Contract 029](../knowledge/contracts/029-interface-version-qualification-and-compatibility.md)
