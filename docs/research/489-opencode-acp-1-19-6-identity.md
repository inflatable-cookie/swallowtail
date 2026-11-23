# 489 OpenCode ACP 1.19.6 Identity

Observed 2026-11-23 for `opencode.acp` on `opencode.executable`. npm
`opencode-ai` latest and GitHub latest agree on `1.19.6`. The published
stable hops above `1.19.4` are `1.19.5` and `1.19.6`. Both remain
`UnverifiedNewer` pending ACP selected-path proof.

## Artifact identity

[Research 486](486-opencode-http-1-19-6-identity.md) binds all 36 npm
native executables to the corresponding attested GitHub CLI payloads
and release commits across `1.19.4`, `1.19.5`, and `1.19.6`. This
record reuses that ledger for executable identity. The official tag
archives were checked against its SHA-256 values. Release attestations
associate payloads with releases. The comparison is static and makes
no reproducible-build or native behavior claim.

The [ACP source inventory](../../crates/swallowtail-adapter-opencode/tests/fixtures/opencode-acp-1.19.6/source-inventory.json)
pins the shared artifact ledgers and the selected sources. The host
package on PATH belongs to the separate major line, so no installed
1.x observation or host execution was used. The first unpublished
later 1.x stable is `1.19.7`.

## Selected ACP path

All twelve ACP source files, the `acp` command, the external plugin
gate, and the selected MCP configuration, registration and handler
sources match byte for byte across both hops. The ACP SDK stays at
`0.21.0`. The tagged wire path still fixes protocol version 1, creates
one session, forwards tool and permission activity, accepts cancellation,
and aborts the backing session on close. The consumer still sends one
route-owned stdio or streamable-HTTP MCP declaration when configured.
The exact live streamable-HTTP MCP honouring evidence stays on
[`1.18.18`](349-opencode-acp-http-mcp-live-honouring.md).

The first hop's config and LLM sources only remove unused imports or an
unused markdown re-export. Deleted prompt texts have no reference in
the preceding tree. The PTY authorization change is scoped to PTY
connect, and the worker RPC fix is scoped to the TUI. The second hop's
app, packaging and test changes are outside the selected ACP path.
[Research 464](464-opencode-http-acp-1-19-5-identity.md) and Research 486
retain the complete tag-tree delta sets.

### 1.19.5 GitLab dependency

`--pure` disables external plugins. GitLab remains a built-in provider.
`session/new` loads `config.providers` from OpenCode's private backing
server and uses that catalogue to resolve a default model. With GitLab
configured and authenticated, its loader can discover workflow models.
A selected GitLab model on `session/prompt` reaches workflow or agentic
chat. The unchanged loader calls the updated `isWorkflowModel` mapping
to choose a workflow model.

The manifests and lockfile move `gitlab-ai-provider` from `6.19.0` to
`6.23.1`. Research 486's Linux x64 version and direct-access cache
literals corroborate the change. Changes to workflow mapping and
defaults, namespace selection, cache and locking, credential refresh,
and finish handling can affect model selection, provider requests,
tool and permission activity, and turn settlement. Static inspection
leaves the credential audience, filesystem effects, and resulting ACP
turn behavior unresolved. Swallowtail still uses host-owned login
state without a credential lease.

### 1.19.6 history and overflow

ACP uses the synchronous backing prompt handler, which runs the same
session prompt loop. The new history conversion bounds provider-bound
inline images above 25 MiB by replacing oldest images with text down
to 15 MiB. Swallowtail sends text prompts, but image-bearing tool
results can enter model history through the backing session. The
stored history remains intact.

Two new error matches can send a turn through the existing automatic
compaction path. The ACP event bridge handles tool and permission
events and waits for idle. It does not forward raw `session.error` as
a terminal ACP update. After the backing prompt and idle settle, ACP
sends a usage update and derives the JSON-RPC response from the final
assistant record. Swallowtail settles from that response. Its OpenCode
ACP route does not expose usage evidence. The HTTP driver's early
failure on `session.error` does not establish an ACP defect. Recovering
and terminal overflow, update ordering and usage, and cancellation
during recovery still need ACP wire proof.

## Decision and adaptation

Keep `opencode.acp.executable-window-1` at `1.19.4` on
`opencode.acp-v1.client-mcp-servers-v2`. Preserve the historical
segments, the unpublished `1.18.36` gap, and the exact MCP honouring
point. No live provider contact, login, installation or vendor
executable run was used.

The version-currentness planner owns the ACP-only adaptation under
[Contract 029](../knowledge/contracts/029-interface-version-qualification-and-compatibility.md#no-terminal-stop--2026-09-24),
continuing the ACP proof from Research 464's lead
`5b445586-3c98-47c5-aa88-7950aace016b`. Use the frozen executable
digests and approved isolated fake services to exercise `acp --pure`
on both hops. Check GitLab catalogue and model choice, credential
audience, cache and refresh boundaries, and tool, permission and MCP
activity under the resulting ACP turns. On `1.19.6`, exercise
image-bearing tool history, recovering and fatal overflow, final
usage and response ordering, cancellation, and close. Compare to the
qualified baseline before proposing a mapping or revising a claim.
Any consumer-visible narrowing or public lifecycle change needs an
operator ruling.

## Sources

- [npm publication history](https://registry.npmjs.org/opencode-ai)
- [OpenCode v1.19.4](https://github.com/anomalyco/opencode/releases/tag/v1.19.4), [v1.19.5](https://github.com/anomalyco/opencode/releases/tag/v1.19.5), [v1.19.6](https://github.com/anomalyco/opencode/releases/tag/v1.19.6)
- [GitLab provider package](https://registry.npmjs.org/gitlab-ai-provider)
