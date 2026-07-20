# Swallowtail

Shared Rust infrastructure for discovering, connecting to, and driving AI
models and agent harnesses across host applications.

Swallowtail owns portable integration mechanisms. Applications retain their
prompts, tools, authority, workflows, persistence, and product state.

Status: foundation. The repository is in strict Northstar posture. The
provider-neutral core, pure preflight, executor-neutral runtime, five synthetic
conformance profiles, host-approved local process service, and separate Codex
exec and app-server drivers are validated. App-server supports both unchanged
read-only sessions and one explicit host-resolved bounded workspace profile.

## Start Here

```sh
effigy tasks
effigy doctor
effigy test --plan
```

Then read [docs/README.md](docs/README.md).

## Current Direction

- provider-neutral identities, capabilities, model catalogs, references,
  events, and diagnostics first
- interactive sessions and bounded structured runs as distinct execution
  shapes
- provider-specific behavior exposed honestly through capabilities
- multiple transport-specific drivers per integration family where needed
- harness control, direct model APIs, SDKs, CLIs, protocols, and local runtimes
  treated as distinct routes
- credential mechanism, entitlement, endpoint audience, and support authority
  kept explicit per driver instance
- local and remote execution hosts treated as equal topologies
- Nucleus and Soundcheck as initial consumers, not Swallowtail authorities

The Rust workspace contains `swallowtail-core`, `swallowtail-runtime`,
`swallowtail-testkit`, `swallowtail-host-local`, and
`swallowtail-adapter-codex`. Provider behavior stays isolated in the adapter.

The Soundcheck structured-run and Nucleus interactive-session readiness lanes
are complete with bounded downstream adoption plans. Codex app-server
transports preflight-bound session options and declared dynamic-tool callbacks
without executing tools. Local and remote-authoritative fixtures prove
host-bound open, resume, callbacks, interruption, failure, and joined cleanup.
Soundcheck and Nucleus Agent Chat are accepted consumers. The bounded
workspace-write runtime required by Nucleus task execution is complete without
widening read-only chat. Nucleus consumer adoption is the current feedback
lane; provider expansion follows the multi-consumer proof.
