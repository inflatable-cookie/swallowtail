# 121 Claude Code Response-Only Structured Route

Status: promoted
Owner: Tom
Date: 2026-08-11

The schema-enforced boundary closes negatively here. Research 122 supersedes
only the later consumer disposition by selecting a distinct plain-text route
with no structured-output claim.

## Question

Can installed Claude Code `2.1.227` provide one subscription-authenticated
schema-constrained response with no model-visible tools, filesystem authority,
session retention, retry, or fallback?

Figmatic supplied the consumer evidence. It needs prompt plus inline JSON
Schema to yield one typed proposal while the consumer keeps compilation,
writes, validation, repair, and acceptance.

## Existing Route

`claude-code.headless` is not this boundary. Its exact `2.1.220` profile
requires a read-only filesystem working resource, Plan mode, ambient
configuration, and `Read`, `Glob`, and `Grep`. Contract 039 forbids silently
weakening that profile or emulating structured output in the prompt.

The candidate was therefore assessed as an additive response-only profile.
Contracts 039 and 040 require exact structured-output capability, visible
enforcement source, preflight-bound attempt authority, one terminal structured
result, and fail-closed behavior before such a profile can be implemented.

## Exact Executable Evidence

- executable: `/Users/tom/.local/bin/claude`
- resolved payload: `/Users/tom/.local/share/claude/versions/2.1.227`
- version: `2.1.227 (Claude Code)`
- access: `loggedIn=true`, `authMethod=claude.ai`, `subscriptionType=max`
- `ANTHROPIC_API_KEY`: absent

The safe candidate invocation used print mode, stream JSON, an explicit model
and effort, `--tools ""`, safe mode, disabled slash commands and Chrome,
strict empty MCP configuration, no session persistence, and no fallback model.
It ran from a fresh empty launch directory. Every probe left that directory
empty.

## Live Results

### Tool-free text

Without `--json-schema`, the init envelope reported:

```text
tools=[] mcp_servers=[] permissionMode=default
```

The run completed in one turn with text `OK` and no `structured_output`.

### Valid schema

Adding `--json-schema` changed the init envelope to:

```text
tools=["StructuredOutput"] mcp_servers=[] permissionMode=default
```

The assistant emitted a `tool_use`; the harness returned a `tool_result`; the
terminal envelope reported `num_turns=2`, the JSON string in `result`, and the
typed object in `structured_output`.

This is harness-owned schema-tool validation, not provider-native enforcement.
The schema option makes a tool model-visible even when the built-in tool list
is explicitly empty.

### Unsatisfiable schema

A syntactically valid schema with `minLength: 2` and `maxLength: 1` was
accepted for execution. The stream then contained four `StructuredOutput`
tool calls and four tool results across six turns. Claude Code exited `0` with
subtype `success` and `structured_output:null`.

The CLI exposes no option in `2.1.227` to bind the structured-output retry
count to zero. The observed harness retry loop also converts exhausted schema
validation into terminal success without a structured value.

### Malformed schema and termination

- malformed schema JSON failed locally with exit `1` and no launch-directory
  artifact
- external `SIGTERM` produced exit `143`, no child remained, and no
  launch-directory artifact remained

Swallowtail's existing process host still owns cancellation, deadline, forced
stop, wait, and joined task cleanup. No candidate profile was implemented, so
the live termination observation does not widen current route guarantees.

## Decision

Do not add the response-only profile on Claude Code `2.1.227`.

The exact candidate fails three required boundaries:

1. schema transport injects a model-visible `StructuredOutput` tool
2. schema-invalid calls retry without a caller-bound zero-attempt budget
3. retry exhaustion can exit successfully with no structured result

Prompt schema emulation, consumer repair, post-hoc relabelling, and accepting
the null success would violate Contracts 039 and 040. A distinct route
identity would not repair the upstream behavior.

## Version And Capability Result

- `claude-code.headless` keeps exact qualified version `2.1.220`
- `2.1.227` remains visible as unverified-newer for that existing read-only
  route; this study does not infer compatibility
- no `StructuredOutput` capability or enforcement source is advertised for a
  Claude Code response-only profile
- Claude Agent ACP remains separate and too broad for the consumer boundary

## Reopen Gate

Reassess only when an exact Claude Code version exposes a native response
surface that simultaneously proves:

- no model-visible tool, including a schema pseudo-tool
- no filesystem, callback, MCP, session, retry, or fallback authority
- schema-invalid or missing structured output fails terminally
- caller-bound zero retry
- local subscription authentication without API-key billing
- cancellation, deadline, redacted failure, and joined cleanup

## Consumer Disposition

Figmatic has no response-only integration task against current Swallowtail
source. It may retain approved release `v0.3.1` for existing integrations, but
must not use `claude-agent.acp` or `claude-code.headless` for the responsive
proposal step. No current commit or release identity carries the requested
capability.
