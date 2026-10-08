# Research 377: Claude Code 2.1.293 Response-Only Identity and Qualification

Status: promoted.

Question: can `claude-code.response-only` extend its `2.1.281` ceiling through
the official stable `2.1.293` while retaining its public text-only contract?

## Method and identity

Re-probed official channels at `2026-10-08T00:26:41Z`. npm `latest` and
GitHub's latest non-prerelease agree on `2.1.293`; npm `stable` remains
`2.1.285`, the documented delayed channel. Both npm's `2.1.294` version
endpoint and GitHub's `v2.1.294` release endpoint returned 404. The next
stable point was therefore `2.1.294` at this observation.

The local `claude` executable was present on `PATH`; its version and digest
were not read because invoking the self-updating CLI could mutate the host.
No downloaded package was executed. No provider prompt, session, catalogue,
credential, installation, or host update was used.

Official identity:

| Artifact | Version and identity |
| --- | --- |
| npm `latest` | `@anthropic-ai/claude-code@2.1.293`, published `2026-10-07T17:18:04.464Z`; wrapper tarball SHA-256 `a96c76dfce0fd4b0449201ac16e6ac4f6517330a9a046b0f53197088f9a1dff4` |
| GitHub latest stable | `anthropics/claude-code` tag `v2.1.293`, published `2026-10-07T18:10:20Z`, tag commit `79babc372d64101f981bd2b52c3dbe588596dc56` |
| Darwin arm64 runtime | npm platform tarball SHA-256 `0c7bbfb7c571161caa5283a61056bc6e4414222c9488e01beed77f59c9210966`; `package/claude` SHA-256 `4e21122a227857da1178aca3299700c1fd7f2b77c93f12e73c2c76db796a105e` |
| Linux x64 runtime | npm platform tarball SHA-256 `7b6d6842a9e9de1fa0f5dfc18967696b91883b9d82be333a05b35bcf21f7d70f`; `package/claude` SHA-256 `8968405e26db478af44eabc4635ab5ca557057b702a54460a59c13e1b253e978` |

The cached official wrapper and both platform packages cover every published
point `2.1.281` through `2.1.293`. Registry integrity values and a complete
per-file SHA-256/size inventory for every package and hop are in the route
fixture's [`dist-inventory.json`](../../crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-code-response-only-2.1.293/dist-inventory.json).
Each hop has no added or removed files. The wrapper's `package/package.json`
changes every hop; `sdk-tools.d.ts` changes at `2.1.284`, `2.1.285`, `2.1.290`,
and `2.1.292`; `install.cjs` changes at `2.1.288`. These are npm metadata,
SDK declarations, and installation surfaces, not response-only runtime calls.
Each Darwin and Linux package changes `package/claude` and `package/package.json`
at every hop; `LICENSE.md` and `README.md` remain byte-identical. Exact changed,
identical, added, and removed path sets and every file digest are frozen in the
inventory.

The GitHub source tag commits are recorded for each point, but the public
repository does not establish source-to-npm-runtime provenance. No source
parity is claimed; the exact published artifact tree is the runtime identity.

## Selected route review

The selected adapter invocation stays text input to one `-p` stream-JSON run,
with `--safe-mode`, empty `--tools`, strict empty MCP configuration, disabled
slash commands and Chrome, no prompt suggestions, and no session persistence.
The decoder still requires the prepared and init versions to match, empty
tools and MCP, one text-only assistant message, and one matching one-turn
result. Tool use, additional turns, malformed frames, drift, or post-terminal
events fail closed. No capability, operation, or public request shape was
added.

At `2.1.281`, the selected built-in `agents-md` hook maps ancestor
`AGENTS.md` and `CLAUDE.md` discovery through `fs.ancestors`,
`instructionFiles`, `AGENTS_NAMES`, `CLAUDE_NAMES`, and `projectDirOf` into
`prompt.context`. `session.start` is also present. The empty tool list blocks
the hook's `tool.call` Read path while `prompt.context` remains selected. Static
route-facing strings for these hooks and the selected safety flags were found
in all 26 Darwin/Linux platform runtimes from `2.1.281` through `2.1.293`; the
exact marker set and each corresponding binary hash are in
[`builtin-hook-ledger.json`](../../crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-code-response-only-2.1.293/builtin-hook-ledger.json).
These strings identify the route-facing hook surface only. They do not name
or prove an internal symlink resolver.

With no consumer `Read` working resource, v2 and v3 continue to launch from an
adapter-owned empty temporary working resource with `ResourceAccess::Read`.
An optional consumer `Read` resource remains only the child project location.
The route stays `AmbientHost`; neither path establishes filesystem
containment.

The upstream linked-instruction boundary changes within the published hop
set:

| Hop | Classification for response-only |
| --- | --- |
| `2.1.281 → 2.1.282` | **Private milestone.** The release note fixes startup instruction/rules reads reached through repository symlinks to macOS `/Network`, `/.vol`-style paths, or `/home`. Both platform `package/claude` files changed and their exact hashes are frozen. The linked project-instruction set can be smaller; the bundled resolver is opaque and no private function is inferred. |
| `2.1.282 → 2.1.283` | Prompt-audit slash commands are outside this route because slash commands are disabled. |
| `2.1.283 → 2.1.284` | External `.claude/rules` and `.claude` directory symlinks now use an approval path. This remains within the accepted linked-instruction read limit; no adapter operation or filesystem authority is added. |
| `2.1.284 → 2.1.285` | Artifact, Edit, and review changes are outside the route's empty tool set and disabled slash commands. |
| `2.1.285 → 2.1.286` | Duplicate project instruction loading for subagent worktrees is outside this route; it starts no subagents. |
| `2.1.286 → 2.1.287` | Session resume/compaction and `context: fork` slash-skill streaming are outside the route's disabled slash commands and disabled session persistence. |
| `2.1.287 → 2.1.288` | The `-p`/SDK SIGTERM fix concerns an external supervisor sending SIGCONT with SIGTERM. Response-only cancellation invokes host `ProcessHandle::force_stop` and waits; host-local Unix `force_stop` sends SIGKILL. Other host and supervisor behavior remains environment-specific. Write/Edit rule reload changes are outside the empty tool set. |
| `2.1.288 → 2.1.289` | Read-deny behavior for direct symlink reads is outside the absent Read tool. No selected stream or lifecycle change was identified. |
| `2.1.289 → 2.1.290` | Further linked `CLAUDE.md`, rules, and `AGENTS.md` reads outside working directories are blocked under the release's named read policies. This is within the same v3 linked-instruction limitation. |
| `2.1.290 → 2.1.291` | No selected response-only stream, linked-instruction, tool, permission, usage, or lifecycle change was identified. |
| `2.1.291 → 2.1.292` | Read-deny, Bash-command, directory-change, and session changes are outside the route's empty tools, disabled slash commands, and disabled persistence. |
| `2.1.292 → 2.1.293` | Nested instruction loading through Bash is outside the route's empty tool set. No selected stream or lifecycle change was identified. |

The `.282`, `.284`, and `.290` release-note changes are recorded as a
version-specific reduction of symlink-linked project-instruction reads under
the accepted Contract 023 ruling. The consumer-facing route remains one text
response with no tools or MCP. It does not promise every linked instruction is
included in `prompt.context`, restore blocked reads, or treat provider flags as
a host filesystem boundary. The exact runtime resolver remains unavailable;
the frozen evidence makes no function-level conformance claim.

## Decision

Add `claude-code.response-only.stream-json.v3` for `2.1.282..=2.1.293` as a
private behavior milestone, and qualify official latest `2.1.293`. Preserve
the `2.1.227` baseline, claim id, v1 and v2 segments, explicit denied points,
and `AllowUnverified` posture. Contract 029 derives `Deprecated` status for v1
and v2 because v3 is the newest behavior; both old segments remain supported.
`2.1.294` remains visible as `UnverifiedNewer`.

Contract 036 compatibility was assessed separately: the source change preserves
the public operation, prepared input/output, and lifecycle contract; only the
private version behavior identity and its qualification evidence change. This
does not establish a release baseline, tag, or publication authority.

## Sources

- [npm registry metadata](https://registry.npmjs.org/@anthropic-ai%2Fclaude-code)
- [GitHub `v2.1.282` release](https://github.com/anthropics/claude-code/releases/tag/v2.1.282)
- [GitHub `v2.1.284` release](https://github.com/anthropics/claude-code/releases/tag/v2.1.284)
- [GitHub `v2.1.288` release](https://github.com/anthropics/claude-code/releases/tag/v2.1.288)
- [GitHub `v2.1.290` release](https://github.com/anthropics/claude-code/releases/tag/v2.1.290)
- [GitHub `v2.1.293` release](https://github.com/anthropics/claude-code/releases/tag/v2.1.293)
- [Claude Code release channels](https://code.claude.com/docs/en/setup#configure-release-channel)
- [Contract 023 ruling](../knowledge/contracts/023-harness-operation-isolation-and-native-boundary.md#claude-code-response-only-linked-instructions)
- [Contract 029](../knowledge/contracts/029-interface-version-qualification-and-compatibility.md)
- [Contract 036](../knowledge/contracts/release.md)
- [Response-only `2.1.281` baseline evidence](348-claude-code-2-1-281-narrowed-response-only-claim.md)
- [`claude_code_response_selection.rs`](../../crates/swallowtail-adapter-claude-agent/src/claude_code_response_selection.rs)
- [`claude_code_response.rs`](../../crates/swallowtail-adapter-claude-agent/src/claude_code_response.rs)
- [`claude_code_response_pump.rs`](../../crates/swallowtail-adapter-claude-agent/src/claude_code_response_pump.rs)
- [`process_exit.rs`](../../crates/swallowtail-host-local/src/process_exit.rs)
