# OpenCode ACP Prepared Integration

Use `swallowtail-adapter-opencode` for the installed OpenCode ACP agent. The
route is `opencode.acp`; the driver ID is `swallowtail.opencode.acp`. It owns
initialize plus one bounded `session/prompt` over ACP v1 stdio on a
host-approved `opencode acp --pure` child.

This is a separate family from `opencode.http`. The ACP axis is
`opencode.executable`; the HTTP axis is `opencode.server`. Swallowtail does
not flatten them. Plugin loading is not part of the selected surface, so
`--pure` is pinned. `--port`, `--hostname`, `--mdns`, `--cors`, `login`, and
`--print-logs` stay out.

New to the shared vocabulary? Read [Key Concepts](key-concepts.md).

## Selected Boundary

Preparation requires all of the following:

- executable axis `opencode.executable`
- a permitted OpenCode package in `1.18.18..=1.18.32`, or a later stable
  visible as `UnverifiedNewer`
- host-approved `opencode` executable and isolated environment
- `opencode_acp_host_account_access_profile` with no credential reference
- working resource, plus host services for task, process, and working-resource
  ownership

The claim is `opencode.acp.executable-window-1`. Behavior
`opencode.acp-v1.client-mcp-servers-v1` covers accepted-but-older
`1.18.18..=1.18.30`. Compiled behavior
`opencode.acp-v1.client-mcp-servers-v2` covers `1.18.31..=1.18.32`. Later
stables may run as visibly unverified without extending guaranteed support.

OpenCode answers `protocolVersion` `1` regardless of the requested revision.
The driver records that provider-fixed value and does not infer that its
requested revision was accepted.

Swallowtail does not install OpenCode, search `PATH`, run `opencode auth
login`, or bind provider credentials as a Swallowtail lease. Host-owned
OpenCode login stays outside the prepared plan.

Call `prepare_opencode_acp` with `OpenCodeAcpPreparationInput` and
`OpenCodeAcpPreparationProbe`. The probe classifies the approved target only.
It does not send initialize, create a session, or prompt.

The prepared integration binds the execution host, exact target, observation,
host-account access profile, and preflight evidence. Validate that binding
before reusing it. The access profile is local and unauthenticated:
Swallowtail opens no credential lease.

Wrong axis (`opencode.server` is the HTTP route), wrong audience, or a
credential reference fails before ACP work.

## ACP Interactive Session

Create `OpenCodeAcpSessionProfileInput::new` with request identity and a
read-only working resource. There is no open deadline, model route, or
harness-mode option. Call `prepare_session`, inspect `evidence()`, `plan()`,
and `request()`, then `open_session`.

The driver owns one joined stdio child and performs this sequence:

1. spawn `opencode acp --pure` with no extra argv
2. `initialize` with host `fs` and `terminal` advertised false
3. `session/new` with `{cwd, mcpServers}` — empty, or one admitted stdio or HTTP entry
4. one bounded `session/prompt` of text blocks
5. observe permission requests and cancel; never select `allow_always`
6. `session/close`, then join connection, process, and task cleanup

Host `fs/readTextFile` and `fs/writeTextFile` callbacks are rejected. Session
deadline and `session/load` are unsupported. Unexpected initialize
`agentInfo.version` fails closed against the selected executable.

Take each turn's event stream and terminal outcome immediately and poll them
concurrently. Cancellation issues `session/cancel` and joins the active turn.
Close the turn and session separately from terminal truth. Ambient-host
isolation is not filesystem or descendant-process containment.

See the compile-tested
[`prepared_opencode_acp` example](../../crates/swallowtail-adapter-opencode/examples/prepared_opencode_acp.rs).

## Consumer-declared MCP

Contract 063 admits one stdio placement and one consumer-supplied
streamable-HTTP placement. Bind at most one of `OpenCodeAcpStdioMcpServer` or
`OpenCodeAcpRemoteMcpPlacement`, whose name is the route-owned
`swallowtail-opencode-acp`. A foreign name fails closed as a collision: OpenCode
maps per-session declarations onto a name-keyed instance-scoped registration
with last-write-wins and no unregister on close. Binding both in one session is
a typed refusal.

HTTP encoding emits ACP `type: "http"` with the consumer URL and headers
verbatim. Structure is validated only: non-empty name, absolute `http`/`https`
URL, well-formed header names. URL and header values never appear in failures,
diagnostics, activity, receipts, `Debug` output, or plan fingerprints: the
public encoder returns `OpenCodeAcpEncodedMcpServers`, whose `Debug` form
redacts those values, and the wire JSON stays crate-private. `sse` stays
modelled through `OpenCodeAcpRemoteMcpPlacement::sse` and is not emitted.

The Contract 061 placement projection names `consumer-supplied-http` when an
HTTP entry is bound. Research 348 observed live honouring of that HTTP entry
on exact `1.18.18`. That observation is not an accepted-run: the spent
attempt has no verified exact model attribution, so `client_mcp_servers`
stays No. This route does not send a model on `session/new`. The live gate
records the selected default agent's model override, else the root `model`,
after that definition and existing host auth are present. Later points in the
window stay unqualified for honouring. stdio MCP live honouring is not that
evidence.

## Restart, Failure, And Promotion

ACP exposes no public load, resume, or close binding. Cleanup still sends
`session/close` before joining. `prepare_working_state_restoration` opens a
fresh context-losing session after process loss; it does not recover the
interrupted turn or transcript. A prepared stdio or HTTP MCP declaration is carried
onto that replacement session.

Handle failures through portable classification and retain the exact
`swallowtail.opencode.acp` diagnostic for support. Do not parse stderr, ACP
data, or OpenCode account state to infer retry, auth, terminal, or cleanup
truth. Unauthorized initialize maps to
`swallowtail.opencode.acp.host_auth_required` without OpenCode policy.

Promotion of HTTP MCP honouring past exact `1.18.18`, stdio MCP live
honouring, host plugins without `--pure`, treating `protocolVersion` as
negotiated, flattening onto `opencode.http`, OpenCode login as a Swallowtail
action, or broader live qualification requires a separate card, exact version
evidence, and matrix coverage. An advertised ACP capability or CLI flag alone
is insufficient.

## Deterministic Validation

```sh
effigy validate:focused swallowtail-adapter-opencode
```

No login, install, or authenticated prompt is part of deterministic acceptance.
HTTP MCP honouring on exact `1.18.18` is Research 348 and is not a settled
`client_mcp_servers` Yes. Further live qualification stays separately gated.
