# Claude Code Tool-Free Text Route

Date: 2026-08-11
Roadmap: g03.065
Contracts: 023, 029, 032-033, 037, 039, 044, 051-052
Research: 122

## Decision

Add `claude-code.response-only` as a distinct route in the existing Claude
adapter. Do not mutate `claude-code.headless`.

The new route qualifies exact Claude Code `2.1.227` only. It returns one
bounded assistant text result through local subscription authentication. It
does not advertise `StructuredOutput`, working-resource, callback, tool,
session, continuation, retry, fallback, or write capability. JSON-shaped text
remains ordinary untrusted `OperationContent`.

## Exact Boundary

The driver selects print mode, text stdin, stream JSON, empty tools, safe
mode, disabled slash commands and Chrome, disabled prompt suggestions, strict
empty MCP configuration, no session persistence, an exact caller model, and
optional qualified effort. It supplies no process working resource.

The stream must initialize exact `2.1.227` with the selected model, default
permission mode, `tools=[]`, and `mcp_servers=[]`. It admits one text-only
assistant message and one matching success result with `num_turns=1`. Any
user record, tool block, second assistant, structured output, version/model
drift, malformed envelope, bound violation, or post-terminal record fails
closed.

`ProviderSuppressed` records the exact tool/MCP configuration posture.
`AmbientHost` records that this is not an OS sandbox. The only required host
services are Task, Process, and Time.

## Authentication And Live Evidence

The installed payload was `/Users/tom/.local/share/claude/versions/2.1.227`.
`claude auth status --json` reported `claude.ai` and `max` with
`ANTHROPIC_API_KEY` absent. A cleared environment needed approved `HOME`,
`USER`, and `LOGNAME`; `HOME` alone was insufficient for the local
OAuth/keychain lookup.

The separately gated prepared-facade probe returned exactly
`CLAUDE_RESPONSE_ONLY_LIVE_OK`, then started and cancelled a second run. Both
operations cleaned and joined. The repository source-status snapshot did not
change. Direct fresh-directory evidence also recorded zero provider-created
artifacts.

Observable no-retry truth is one assistant record, zero user/tool records,
one result, and `num_turns=1`. The route makes no claim about invisible
provider HTTP recovery.

## Validation

- `effigy validate:focused swallowtail-adapter-claude-agent` — 78 passed
- `effigy package:verify-affected swallowtail-adapter-claude-agent` — passed
- `effigy qa:guides` — 36 routes, 35 examples, passed
- `effigy qa:routes` — 36 routes and 70 activity operations, passed
- `effigy check:examples` — passed
- `effigy probe:claude-code-response-only` — normal completion and live
  cancellation passed against exact `2.1.227`

`effigy qa:docs` remains blocked by the pre-existing Effigy roadmap-index
relative-link defect recorded in `PAPERCUTS.md`; link, vision, log, and
research subchecks passed before that failure. Existing doctor findings
remain the known god-file set, stale graph index, and one generated-in-source
warning.

No version bump, tag, release, registry publication, or Figmatic edit was
performed.

Figmatic should adopt implementation commit
`d8f9aae41b3604283676dc52c85b307723060f80` through the API named in card 206.
