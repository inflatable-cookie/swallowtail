# Quick Start

This walkthrough takes you from an empty Cargo project to one prepared Codex
structured run. It uses the `codex.exec` route. Read
[Key Concepts](key-concepts.md) first if any term here is unfamiliar.

The code below is shaped like the compile-tested
[`prepared_discovery` example](../../crates/swallowtail-adapter-codex/examples/prepared_discovery.rs),
which is the source of truth for every constructor. It is excerpted for
reading, not pasted as a standalone file.

## 1. Check The Prerequisites

- Rust `1.95.0` or newer, on Apple Silicon macOS (the verified target)
- the `codex` CLI installed, with a working login state for your chosen access
  profile
- the tagged source of the packages you import (see the install block below)

Swallowtail does not install Codex, log you in, or pick a model. You supply
the executable and the access; Swallowtail verifies and uses them.

## 2. Add The Dependencies

Every Swallowtail dependency pins the exact source tag. Keep only the packages
your code imports directly:

```toml
[dependencies]
swallowtail-core = { git = "https://github.com/inflatable-cookie/swallowtail", tag = "v0.2.0" }
swallowtail-runtime = { git = "https://github.com/inflatable-cookie/swallowtail", tag = "v0.2.0" }
swallowtail-host-local = { git = "https://github.com/inflatable-cookie/swallowtail", tag = "v0.2.0" }
swallowtail-adapter-codex = { git = "https://github.com/inflatable-cookie/swallowtail", tag = "v0.2.0" }
```

## 3. Approve The Executable To The Host

Nothing runs until the execution host approves it. The local host returns an
opaque target plus a service set that owns the process boundary:

```rust
let (builder, target) = LocalProcessHost::builder(limits)
    .approve_installed_executable(executable_ref, codex_axis, executable_path);
let local = builder.build_services(execution_host_id.clone());
let services = local.services();
```

## 4. Prepare The Codex Integration

Preparation probes the approved target, classifies its exact installed
version, and returns one immutable `CodexPreparedIntegration` bound to the
selected driver. A structured exec run needs the `StructuredExec` driver; the
`AppServer` driver covers sessions, catalogues, and management instead. This
is where identity, access, and version evidence get bound:

```rust
let input = CodexPreparationInput::new(
    CodexPreparedDriver::StructuredExec,
    ConfiguredInstanceId::new("codex.local").expect("instance id is valid"),
    InstanceRevision::new("1").expect("instance revision is valid"),
    host,
    target,
    environment,
    access_profile,
    PreparedAccessEvidence::caller_asserted(access_status),
);
let probe = CodexPreparationProbe::new(
    RequestId::new("codex-prepare").expect("request id is valid"),
    ScopeId::new("codex-prepare").expect("scope id is valid"),
    deadline,
    DiscoveryCancellation::new(),
);
let prepared = prepare_codex(input, probe, services.clone()).await.expect("codex prepares");
```

## 5. Prepare And Run One Bounded Operation

Pick a named operation profile, prepare it, then execute it. A structured run
carries the prompt, an explicit model, a working resource, and explicit
network and search posture:

```rust
let exec = prepared.prepare_structured_exec(CodexExecProfileInput::new(
    RequestId::new("codex-exec").expect("request id is valid"),
    OperationContent::new("consumer-owned prompt").expect("prompt is valid"),
    model,               // one exact CodexModelSelection
    working_resource,    // bound during preparation
    ExternalNetworkPolicy::Denied,
    ExternalSearchPolicy::Disabled,
)).expect("exec prepares");
let handle = exec.start_run(services).await.expect("exec starts");
```

## 6. Drain The Run To Terminal, Then Close

A run is not a blocking call. Take the event stream and terminal future once,
drain them concurrently, then close to join process and task work:

```rust
let events = handle.take_events();
let terminal = handle.take_terminal_outcome();
// drain events and terminal concurrently...
handle.close().await.expect("run closes");
```

See [Ordinary Operation Lifecycle](ordinary-operation-lifecycle.md) for the
exact drain, cancellation, and cleanup rules. Terminal status and cleanup
result are separate truths; retain both.

## Why The Shape Feels Indirect

The steps are deliberately split so every boundary is explicit before any
provider work: the host admits the executable, the adapter verifies the exact
interface, the consumer selects the instance and model, preparation builds
immutable evidence and a plan, and execution drains to a known end. No step
logs in, picks a provider, retries, or hides provider differences.

## Validate

The full, compile-tested example lives in the Codex adapter and covers every
prepared branch:

```sh
effigy check:examples
```

Live prompts, login, and account checks are separate operator-gated probes;
normal examples compile without touching a provider.

## Next Steps

- [Key Concepts](key-concepts.md) — the shared vocabulary
- [Provider Route Matrix](provider-route-matrix.md) — all production routes, and how
  to pick one
- [Integration Guide Map](integration-guide-map.md) — the guide and example
  for your chosen route
- [Codex Prepared Integration](codex-prepared-integration.md) — the full Codex
  route guide
