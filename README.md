# Swallowtail

Shared Rust infrastructure for discovering, connecting to, and driving AI
models and agent harnesses across host applications.

Swallowtail owns portable integration mechanisms. Applications retain their
prompts, tools, authority, workflows, persistence, and product state.

Status: foundation. The repository is in strict Northstar posture. The
provider-neutral core, pure preflight, executor-neutral runtime, nine synthetic
conformance profiles, host-approved local process/endpoint/credential services,
and eight production transport routes across Codex, OpenCode, Anthropic, Gemini,
Kimi, llama.cpp, and xAI are validated. Codex app-server supports both unchanged read-
only sessions and one explicit host-resolved bounded workspace profile.

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

The twelve-crate Rust workspace contains `swallowtail-core`, `swallowtail-runtime`,
`swallowtail-testkit`, `swallowtail-host-local`, `swallowtail-protocol-acp`,
`swallowtail-adapter-codex`, `swallowtail-adapter-opencode`,
`swallowtail-adapter-anthropic`, `swallowtail-adapter-gemini`,
`swallowtail-adapter-kimi`, and `swallowtail-adapter-llama-cpp`, plus the production
`swallowtail-adapter-xai`. Provider behavior stays isolated in adapters.

The Soundcheck structured-run and Nucleus interactive-session lanes are
accepted in both consumers. Codex app-server
transports preflight-bound session options and declared dynamic-tool callbacks
without executing tools. Local and remote-authoritative fixtures prove
host-bound open, resume, callbacks, interruption, failure, and joined cleanup.
Soundcheck and Nucleus Agent Chat are accepted consumers. The bounded
workspace-write runtime required by Nucleus task execution is complete without
widening read-only chat. Provider expansion is now the active lane: refresh
external evidence, promote missing network and credential contracts, then
prove materially different harness and direct-inference transports. The shared
hosted foundation and attached OpenCode HTTP/SSE proof are complete. The
provider-supported Anthropic Models and Messages direct driver is complete.
ACP v1 and Gemini CLI `0.51.0` authority, fixtures, bounded protocol transport,
read-only filesystem callback, and production driver are realized. Shared
lifecycle and topology conformance passes. The exact llama.cpp b9910 attached
deployment, operator-supplied model fixture, production driver, and
self-hosted conformance proof are complete without model or server ownership.
The post-tranche checkpoint selected xAI Responses WebSocket as the next proof.
Contract 016 now governs resource-free direct sessions, connection-bound
continuation and cleanup, and exact provider-billed-cost evidence. Deterministic
xAI WebSocket fixtures freeze the exact upgrade/auth boundary, serial
`store=false` turns, private continuation, exact billed ticks, failure,
disconnect, and close behavior. The production session driver now holds one
approved endpoint and credential lease across serial turns, emits exact billed-
cost evidence, rejects parallel turns before a second frame, and invalidates
the connection on cancellation, timeout, provider lifetime failure, or loss.
Provider-neutral direct-session conformance is complete. The xAI proof passes
under local and remote-authoritative hosts with 227 repository tests. Kimi Code
`0.28.1` currentness is promoted in Research 006. Contract 017 now separates
provider-owned load replay from resume, binds persistent sessions to their
resource and access authority, and keeps write callbacks, tool approval,
delegated login, and process containment independent. The deterministic Kimi
ACP corpus now covers new, load, resume, prompt, cancellation, bounded writes,
drift, auth failure, and disconnect without changing shared ACP framing.
Kimi process isolation is now optional rather than a prerequisite for harness
communication. Research 012 confirms the common control-plane pattern: T3 Code
and similar orchestrators map provider-native permissions or sandboxes and may
otherwise relay to the harness with ambient host authority. Swallowtail now
records `AmbientHost`, `ProviderEnforced`, and `HostEnforced` separately with
no silent fallback. Research 007 found no qualifying dynamic mechanism on the
current local host, and the operator selected a native deployment-owned macOS
App Sandbox helper instead of a container as the first optional host proof.
Research 009 fixes that seamless shape: one user-selected project grant,
persisted bookmark authority, isolated Kimi state, inherited descendants, and
no broad home access. Research 010 repairs a
later currentness error: the existing `0.28.1` corpus already targets the
maintained TypeScript successor. Its exact source commit, arm64 archive and
executable digests, signature inputs, isolated state, exclusions, and upgrade
gate are now frozen without changing shared ACP behavior. The independent
host-owned ephemeral llama.cpp lane is now complete. Research 008 and Contract
018 fix the b10069 single-model proof, read-only artifact leases, loopback endpoint handoff,
readiness-before-handle, and joined owned-child cleanup without taking model
acquisition, persistent serving, or Monkey authority. Provider-neutral artifact
identity, preflight binding, read-only lease ports, start deadlines, and scoped
endpoint handoff are realized. The local host now verifies exact approved
regular-file artifacts and their SHA-256 digests, publishes only scoped
loopback serving endpoints, and invalidates those endpoints before releasing
artifact authority after child join. Owned conformance passes under local and
remote-authoritative host identities. The exact llama.cpp b10069 owned driver
adds bounded startup supervision, health/build/route readiness before handoff,
and joined graceful-or-forced teardown without a live model. Production
conformance now covers both topologies, readiness timeout, route mismatch, safe
diagnostics, and ordered lease release while the b9910 attached server remains
externally owned. Roadmap 019 is complete with 257 passing repository tests.
The native macOS proof confirms dynamic project-grant propagation and
descendant containment for a compatible helper, but the exact Kimi `0.28.1`
artifact cannot retain its V8 and native-module runtime under the documented
inherited-helper signature. That exact `HostEnforced` profile is unavailable.
The explicit ambient Kimi ACP production mapping and conformance are complete.
The driver keeps
provider-owned load replay separate from replay-free resume, binds attachments
to exact resource and access posture, mediates bounded text replacement, and
joins process, callback, resource, and delegated-credential work without a
filesystem, descendant, or provider-tool network containment claim. A separate
persistent ACP extension proves load, replay, resume, bounded writes, delegated
auth, topology, redaction, and ordered cleanup without widening Gemini's
baseline. Roadmap 018 is complete. Roadmap 020 now rechecks real SDK-native
embedding first, then selects the next highest-information provider or
transport route.
Full repository QA passes with 271 tests; three installed/live probes remain
separately gated. Doctor remains at the inherited 19 oversized-file findings.
