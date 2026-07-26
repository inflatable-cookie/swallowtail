# Release And Package Topology

Status: active
Owner: Tom
Updated: 2026-07-26
Realization: roadmap g02.001 card 003

## Boundary

Swallowtail's current 23 Rust library crates form one public package family.
Each crate remains separately consumable. There is no umbrella package,
internal release tool, or private implementation crate in the accepted set.

This architecture records the realized package structure. Current manifests
carry the registry, MSRV, resolver, package metadata, and internal-version
rules governed by Contract 036.

## Public Package Roles

### Foundations

- `swallowtail-core` — provider-neutral records and preflight vocabulary
- `swallowtail-runtime` — executor-neutral roles, handles, host ports, and
  lifecycle

### Support

- `swallowtail-host-local` — concrete host-approved local services
- `swallowtail-testkit` — public conformance fixtures and assertions

### Protocols And Transport

- `swallowtail-protocol-acp` — bounded ACP message and fixture boundary
- `swallowtail-protocol-openai-chat` — bounded compatible-chat codec
- `swallowtail-transport-acp-remote` — explicit remote ACP HTTP/SSE or
  WebSocket client

### Opt-In Adapters

- `swallowtail-adapter-alibaba-model-studio`
- `swallowtail-adapter-anthropic`
- `swallowtail-adapter-bedrock`
- `swallowtail-adapter-claude-agent`
- `swallowtail-adapter-codex`
- `swallowtail-adapter-deepseek`
- `swallowtail-adapter-gemini`
- `swallowtail-adapter-kimi`
- `swallowtail-adapter-kimi-platform`
- `swallowtail-adapter-llama-cpp`
- `swallowtail-adapter-opencode`
- `swallowtail-adapter-ollama`
- `swallowtail-adapter-openai`
- `swallowtail-adapter-pi`
- `swallowtail-adapter-qwen`
- `swallowtail-adapter-xai`

Publishing an adapter does not grant provider access, support, installation,
credential, billing, model, endpoint, topology, or sandbox authority. Those
remain runtime and provider-route concerns.

## Dependency And Publication Shape

Normal internal dependencies form three publication stages:

```text
stage 1
  swallowtail-core
  swallowtail-protocol-acp
  swallowtail-protocol-openai-chat

stage 2
  swallowtail-runtime -> swallowtail-core

stage 3
  swallowtail-host-local -> core, runtime
  swallowtail-testkit -> core, runtime
  swallowtail-transport-acp-remote -> core, runtime, protocol-acp
  adapters -> core, runtime
  ACP adapters -> core, runtime, protocol-acp
  compatible-chat adapters -> core, runtime, protocol-openai-chat
```

The ACP adapters are Claude Agent, Gemini, and Kimi. The compatible-chat
adapters are DeepSeek, Kimi Platform, and llama.cpp.

Crates within one stage have no normal internal edge between them. Development
dependencies do not add a registry publication stage.

## Version Shape

The package family uses one coordinated workspace version through pre-1.0.
The first candidate is `0.1.0`.

Internal normal and build dependencies carry both:

- a local workspace path
- an ordinary Cargo-compatible version requirement

The local path supports workspace development. The version requirement is the
registry contract. Patch releases stay compatible within the current
workspace minor. Breaking public API or guaranteed-behavior changes advance
the workspace minor.

Coordinated versions simplify one release line and consumer guidance. They do
not imply that every unchanged package must be uploaded for every patch.

## Rust And Target Shape

The initial verified Rust-version floor is:

- `1.93` for 22 packages
- `1.94.1` for `swallowtail-adapter-bedrock`

The general floor represents the accepted N-4 stable-minor support window at
the start of the release line. Bedrock carries the higher floor required by
its pinned AWS dependencies.

The workspace uses Cargo resolver 3 so dependency selection can respect
declared Rust versions. Each package must pass at its declared floor and on
current stable Rust before a release candidate.

Apple Silicon macOS is the initial verified target. Other targets may work and
are not hard-denied, but remain unverified until separately proven.

## Registry And Release Authority

crates.io is the accepted initial registry. Package names, account state,
owners, teams, credentials, and registry availability remain external state.
An absent name is not reserved.

Deterministic package preparation belongs in Effigy. Preparation, package
assembly, checksums, dry-run validation, or a version change grants no
authority to:

- upload a package
- add or remove an owner
- create or push a tag
- push a branch
- create a GitHub release
- change a consumer repository

Each external mutation requires explicit human approval against an exact
candidate.

The first external release also requires the application-scale consumer
evidence in Contract 036. Package reproducibility and isolated consumer tests
cannot substitute for a real authenticated vertical smoke through a normal
application entry point. Repeated lifecycle and failure cases belong in
deterministic adapter or consumer backend scenario harnesses unless product
startup or current provider behavior is the claim. The selected consumer owns
its product scenarios and live-effect budget; Swallowtail owns adapter
regressions and refreshed candidate evidence.

Local package checks may copy tracked and untracked source into a deterministic
root snapshot. A publishable candidate has a stricter source shape: clean
non-root `HEAD`, preserved canonical ancestry, and a bundle that reproduces
that exact commit. The candidate builder rejects dirty source state instead of
turning it into a second history. Before registry upload, that commit must be
reachable from `main` on the approved remote; `v0.1.0` targets the same commit.

Generated candidate evidence may remain outside the source commit. It cannot
change package contents or substitute for the retained source bundle.

## Compatibility Separation

Cargo package versions govern Swallowtail's Rust API and guaranteed observable
library behavior. Contract 029 separately governs Codex, Kimi, OpenCode, and
other provider-interface versions.

A package patch may add newly qualified provider versions without changing its
Rust API. It may not silently shrink a guaranteed provider range, remove a
capability, change access authority, or weaken lifecycle truth.

## Realization Evidence

Card 003 realized this architecture with:

- contract-complete metadata and crates.io publication policy across all 23
  manifests
- ordinary compatible `0.1.0` requirements on all 46 internal normal
  dependency edges
- resolver 3, Rust `1.93` for 22 packages, and Rust `1.94.1` for Bedrock
- deterministic metadata, dependency, public-declaration, documentation,
  MSRV, content, and local package-family gates through Effigy
- clean isolated source-snapshot assembly of all 23 packages in publication
  order
- extracted package-family check and test compilation without registry upload
  or any other release mutation

Card 004 produced the superseded first candidate. Card 036 produced the
provider-wide synthetic-root candidate. Roadmap g02.013 supersedes that source
shape with the final canonical-history candidate recorded in
`release-candidates/0.1.0/candidate.env`. The candidate bundle preserves
complete normal history and reproduces all 23 package archives plus audited
file lists.

Roadmap g02.014 accepted Nucleus application evidence, retained Soundcheck's
distinct structured-run integration, and refreshed the candidate without
registry or release mutation. The resulting candidate remains local for
ordinary consumer soak.
