# Swallowtail

Swallowtail is a Rust library for discovering, preparing, and driving AI model
providers and agent harnesses through explicit, testable boundaries.

It provides provider-neutral contracts, an executor-neutral runtime, local
host services, protocol codecs, and opt-in adapters. Applications keep their
prompts, tools, permissions, routing, fallback policy, persistence, billing,
and product state.

## Release Posture

`v0.1.0` is the initial supported source identity. It is distributed as one
annotated Git tag from the
[canonical repository](https://github.com/inflatable-cookie/swallowtail).
There is no crates.io publication, GitHub Release object, binary bundle, or
installer in this release line.

The canonical `v0.1.0` tag resolves to the reviewed release commit. Later
unreleased work must use an explicitly approved commit revision and must not
be presented as part of that immutable release.

All 27 packages share version `0.1.0`. The first tag establishes a pre-1.0 API
and guaranteed-behavior baseline, not an API 1.0 promise.

## Choose A Route First

Swallowtail does not choose a provider, model, credential, endpoint, executable,
billing arrangement, or fallback.

1. Choose one of the [33 production routes](docs/guides/provider-route-matrix.md).
2. Read its canonical guide through the
   [integration guide map](docs/guides/integration-guide-map.md).
3. Add only the adapter and shared packages your application imports.
4. Use the adapter's prepared facade for normal integration. Low-level runtime
   roles remain available for advanced composition.

The [feature matrix](docs/guides/provider-solution-feature-matrix.csv) compares
model catalogues, runs, sessions, reasoning, callbacks, activity, recovery,
management, and other portable features without hiding route differences.

## Install From The Source Tag

Use the same Git URL and exact tag for every direct Swallowtail dependency.
This example selects Codex and the shared types and local host services used by
a typical application:

<!-- source-install:start -->
```toml
[dependencies]
swallowtail-core = { git = "https://github.com/inflatable-cookie/swallowtail", tag = "v0.1.0" }
swallowtail-runtime = { git = "https://github.com/inflatable-cookie/swallowtail", tag = "v0.1.0" }
swallowtail-host-local = { git = "https://github.com/inflatable-cookie/swallowtail", tag = "v0.1.0" }
swallowtail-adapter-codex = { git = "https://github.com/inflatable-cookie/swallowtail", tag = "v0.1.0" }
```
<!-- source-install:end -->

Replace the Codex adapter with the route package you selected. Keep the shared
packages only when your code imports them directly. Cargo resolves their
internal workspace dependencies from the same tagged source.

Do not mix the tag with a moving branch, a local path, a crates.io placeholder,
another Swallowtail tag, or an unreviewed commit. Commit `Cargo.lock` in
applications so the selected source identity and third-party graph remain
reviewable.

## Package Map

There is no umbrella crate. Every package is independently selectable.

| Purpose | Packages |
| --- | --- |
| Portable contracts and execution | `swallowtail-core`, `swallowtail-runtime` |
| Host integration and conformance | `swallowtail-host-local`, `swallowtail-testkit` |
| Protocols and transport | `swallowtail-protocol-acp`, `swallowtail-protocol-openai-chat`, `swallowtail-transport-acp-remote` |
| Installed agent harnesses | `swallowtail-adapter-antigravity`, `swallowtail-adapter-claude-agent`, `swallowtail-adapter-codex`, `swallowtail-adapter-cursor`, `swallowtail-adapter-grok`, `swallowtail-adapter-kimi`, `swallowtail-adapter-oh-my-pi`, `swallowtail-adapter-opencode`, `swallowtail-adapter-pi`, `swallowtail-adapter-qwen` |
| Hosted APIs and SDKs | `swallowtail-adapter-alibaba-model-studio`, `swallowtail-adapter-anthropic`, `swallowtail-adapter-bedrock`, `swallowtail-adapter-deepseek`, `swallowtail-adapter-gemini`, `swallowtail-adapter-kimi-platform`, `swallowtail-adapter-openai`, `swallowtail-adapter-xai` |
| Local model runtimes | `swallowtail-adapter-llama-cpp`, `swallowtail-adapter-ollama` |

Some packages expose several explicitly separate routes. For example, the
Gemini adapter contains installed CLI and hosted Live integrations; the Kimi
adapter contains ACP, headless, and local-server routes. Package selection does
not make every route or capability in that package available.

## Integration Shape

Prepared facades follow a common sequence without becoming a generic router:

1. the host approves references, services, credentials, and runtime targets
2. the adapter discovers or validates the exact provider interface
3. the application selects an explicit configured instance and model route
4. the adapter prepares immutable evidence, a preflight plan, and a typed
   operation
5. the application dispatches the operation and drains bounded events until
   terminal and cleanup truth is known

Provider differences remain observable through capabilities and prepared
types. Do not infer support from provider prose or parse provider-native
payloads in the consumer. Start with:

- [provider selection and preparation](docs/guides/provider-selection-and-preparation.md)
- [ordinary operation lifecycle](docs/guides/ordinary-operation-lifecycle.md)
- [generation controls and input authority](docs/guides/generation-controls-and-input-authority.md)
- [observable activity](docs/guides/observable-activity.md)
- [working-state restoration](docs/guides/working-state-restoration.md)
- [portable failure handling](docs/guides/portable-failure-handling.md)

## Runtime Prerequisites

- Rust `1.90.0` or newer for every package except Bedrock
- Rust `1.94.1` or newer for `swallowtail-adapter-bedrock`
- Apple Silicon macOS is the verified `v0.1.0` target; other targets are
  unverified, not prohibited
- installed harnesses, attached services, model artifacts, authentication, and
  provider billing are external prerequisites named by each route guide

Swallowtail never searches for or installs an executable, starts an attached
service, acquires a model, or logs into a provider implicitly. The host admits
those resources explicitly.

The Claude Agent ACP route needs the separately pinned
`@agentclientprotocol/claude-agent-acp` npm sidecar. A Rust source tag does not
contain `node_modules`; follow the
[Claude Agent guide](docs/guides/claude-agent-prepared-integration.md) for the
application-local sidecar boundary. The native `claude -p` route is separate
and does not use that sidecar.

## Compatibility

Swallowtail package versions and provider-interface versions are independent.
An adapter documents exact maintained, deprecated, excluded, and sometimes
visible unverified-newer provider versions. Installing package `0.1.0` does
not guarantee every provider or harness release.

Before 1.0:

- compatible API and guaranteed-behavior changes advance the patch version
- breaking API or guaranteed-behavior changes advance the minor version
- raising an MSRV, shrinking a guaranteed provider range, changing route
  identity, or weakening lifecycle and authority truth is breaking

See [Contract 036](docs/contracts/036-crate-release-and-compatibility-boundary.md)
and the [v0.1.0 release notes](docs/releases/0.1.0.md).

## Development

The repository uses [Effigy](https://github.com/inflatable-cookie/effigy) for
task routing:

```sh
effigy tasks
effigy doctor
effigy test --plan
effigy qa
```

Live and authenticated probes are separate, opt-in tasks. Normal QA is
credential-free and must not contact providers.

Repository architecture, contracts, roadmaps, and logs start at
[docs/README.md](docs/README.md). Contribution rules are in
[CONTRIBUTING.md](CONTRIBUTING.md).

## Support And Security

Use [GitHub Issues](https://github.com/inflatable-cookie/swallowtail/issues) for
reproducible bugs and integration questions. Include the Swallowtail source
identity, route ID, qualified provider version, safe diagnostic code, target,
and smallest credential-free reproduction. Do not include credentials,
provider payloads, private prompts, or consumer data.

Read [SUPPORT.md](SUPPORT.md) for the support boundary. Report vulnerabilities
privately under [SECURITY.md](SECURITY.md), not in a public issue.

## License

Swallowtail is licensed under the [MIT License](LICENSE).
