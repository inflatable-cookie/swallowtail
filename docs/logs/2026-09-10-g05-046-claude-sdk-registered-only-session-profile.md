# 2026-09-10 g05.046 Claude SDK Registered-Only Session Profile

Date: 2026-09-10
Task: `../roadmaps/g05/046-claude-sdk-registered-only-session-profile.md`
No provider call, credential access, version change, release, tag, Desktop
edit, or live-acceptance inference.

## Result

Additive source/API repair after `v0.4.4`. A Claude SDK session can now bind
a non-empty qualified registered-tool selection, admit zero native SDK tools,
and carry explicit `Read` or `ReadWrite` working-resource access. Ordinary
native constructors and additive `with_registered_tools` stay byte-compatible.
Empty native admission without the registered-only binding remains an early
typed failure.

## API and wire

`ClaudeAgentSdkRegisteredOnlyBinding::new(preparation, resource_access,
permission_mode)` is the structural constructor. Preparation
`with_registered_only` / `with_registered_only_binding` and driver
`with_registered_only` propagate it. Access is the consumer's registered-route
choice; mutating `RegisteredToolEffectPosture`, MCP presence, and carrier
spellings do not elevate it. Consumer-declared MCP servers cannot share the
shape.

Open `tools[]` contains only selected carrier spellings. `allowedTools` stays
unset. `Read`, `Glob`, `Grep`, `Edit`, `Write`, `MultiEdit`, and `Bash` are
structurally disallowed. Plan, instance policy, session access policy, driver
agreement, and host lease all carry the same explicit access.

`new([])` and `from_names([])` still fail `profile.tool_set_empty`. An
extracted registered-only profile without its binding fails
`profile.registered_only_unbound` at prepare, before a lease, process, or
provider contact.

## Provider-free proof

Focused adapter tests cover `Read` and `ReadWrite`, unbound empty native,
consumer-MCP conflict, additive-path stability (mutating registered tools
keep a read-only native profile), lease mismatch, and open `tools[]`. Sidecar
asset tests prove carrier-only admission, all seven native tools disallowed,
and empty `tools[]` still `tools_invalid` before SDK construction. Zero
credential or provider contact.

Independent exact-head review `5610982422` (ready to merge, no follow-ups)
at head `ebc703f1`; PR [#309](https://github.com/inflatable-cookie/swallowtail/pull/309)
merged as `7736a3582ef1bb72a3147e38f70467a9534864be` with all-green CI.
Reserved-index closeout recorded on 2026-09-10.

## Surfaces

`crates/swallowtail-adapter-claude-agent/src/sdk/profile.rs` and registered-only
binding, preparation, driver validation, MCP open-tool rendering, sidecar
asset, focused tests, `docs/guides/claude-agent-sdk-prepared-integration.md`,
this log.

No matrix Yes/No change, range, candidate, tag, or Desktop pin follows.
