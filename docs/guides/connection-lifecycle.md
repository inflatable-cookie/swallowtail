# Connection Lifecycle

Use this flow to list addable routes, admit a configured instance, collect or
skip credentials, refresh readiness, and then reuse the existing prepared
facade. New to the shared vocabulary? Read [Key Concepts](key-concepts.md)
first.

Swallowtail stays a library. It does not become a connection server, UI,
router, secret store, or login client. Consumers assemble catalogs, own
persistence, UI, and selection policy, and keep secret bytes on the host.

## Route Applicability

Only three production routes currently export an `AddableRouteDescriptor`:

| Route | Topology | Host service for Available | Credential |
| --- | --- | --- | --- |
| `anthropic.messages` | hosted | Credential | secret API-key field `api_key`; env name `ANTHROPIC_API_KEY` is a name, not a value |
| `codex.app-server` | installed | Process | none; cached local ChatGPT login |
| `ollama.attached` | local-runtime | Network | none; local unauthenticated |

Topology grouping is hosted / installed / local-runtime. It is not
`ExecutionLayer` and must not be collapsed onto harness versus direct
inference.

Every other production route, including `codex.exec`, stays on the
[prepared-facade](provider-selection-and-preparation.md) path. Absence of a
descriptor means that adapter was not linked, not that the route is
unsupported. Hosted interactive OAuth is not a realized consumer path.

## Ordering

1. Assemble `AddableRouteCatalog` from adapter-local descriptors.
2. `admit_instance` writes an `AdmittedInstanceRecord` through the store.
3. Collect a `CredentialRef` where the descriptor advertises a field, or skip
   credentials where it does not.
4. `refresh_readiness` writes host-supplied `AccessStatus`. Optionally
   `observe_authenticated_subject`.
5. Assemble the Contract 047 snapshot from prepared evidence and project
   `apply_stored_model_presentation_overlay`.
6. Call the existing `prepare_*` entry with host target refs.

Preparation stays after admission. Enablement is not 047 `Ready` /
`NotReady`. Overlay copies readiness and cannot change it.

## Assemble The Catalog

Call the adapter-local constructor with the same `HostServices` the later
prepare will use:

- `anthropic_messages_addable_route_descriptor`
- `codex_app_server_addable_route_descriptor`
- `ollama_attached_addable_route_descriptor`

Then `AddableRouteCatalog::from_descriptors`. There is no umbrella registry
and no runtime inventory of every production route. Duplicate route ids fail
closed.

A descriptor names driver identity, topology, availability, credential-field
descriptors, config-field descriptors, and advertised `SignInAction` values.
`SignInAction` is an advertisement, not permission to execute. Listing
addable routes does not authenticate, persist, or prepare.

Availability:

- `Available` — the route can be admitted on this host
- `Unavailable(Install | Runtime | HostService)` — the descriptor can name
  what is missing
- `Unsupported` — the adapter will not offer that route on this host

A Contract 008 discovered candidate is not an addable-route row and cannot
be admitted.

## Admit An Instance

`InstanceAdmissionRequest` takes a consumer-owned `ConfiguredInstanceId`,
`IntegrationFamilyId`, and addable-route id. Optional opaque
`CredentialRef` and `ConfigFieldRef` values, `InstanceEnablement`, and an
`InstanceLabel` may be supplied. Several instances of one family remain
distinct ids.

`admit_instance` requires the route to be in the catalog and `Available`.
Unknown credential or config field ids fail closed. Admission does not
prepare, select a model, or change 047 readiness.

Config-field descriptors are `BinaryPath`, `ApiEndpoint`, or `Environment`.
Values stay host-private behind `ConfigFieldRef`. Public records do not
carry paths, URLs, or env bodies.

## Credentials And Sign-In

When a route collects an API key, the descriptor names label, secret versus
public, and an optional environment name. The host stores secret bytes.
Portable records carry only `CredentialRef`.

The library-owned loop is `start_sign_in`, `poll_sign_in`,
`submit_sign_in_credential_field`, `complete_sign_in`, `cancel_sign_in`.
API-key collection writes opaque refs and does not need URL-open, loopback,
or device-code ports. Presence of a port does not start sign-in. Missing
ports fail the loop that requires them. A loop that would change mechanism,
account, endpoint audience, or billing authority fails closed.

Codex app-server and Ollama attach advertise no credential field. Do not
extract ChatGPT tokens or invent a local-runtime secret.

Hosted interactive OAuth through URL-open and loopback is not a realized
consumer path. Do not treat Claude subscription or Codex ChatGPT cached
login as that path.

## Store Port

`ConnectionLifecycleStore` holds admitted instance records, secret
references, enablement, optional labels, and overlay markers. It never
requires raw secrets.

`MemoryConnectionLifecycleStore` and `JsonFileConnectionLifecycleStore` are
optional host-local adapters for tests and small apps. Consumers may supply
SQLite, keychain-backed, or product stores under consumer authority.

Enablement is a host preference. A disabled instance may still be 047
`Ready`. An enabled instance may be `NotReady`.

## Refresh, Subject, And Updates

`refresh_readiness` writes host-supplied `AccessStatus` onto one admitted
record: credential, entitlement, endpoint authorization, runtime readiness,
and support authority. It does not invent an aggregate ready boolean, probe
unrelated instances, or write enablement. After refresh, the consumer
replaces the 047 snapshot. Refresh is not a watcher inside 047.

`observe_authenticated_subject` is optional, redacted by default, and is
not stored on the admitted record or a 047 snapshot. Adapters report only
what the provider discloses. The three first-proofs report `Absent`. The
subject is never a configured-instance id, never a 047 selection field,
never a default diagnostic, and never a routing key.

`observe_instance_update` reuses a Contract 029 claim and optional Contract
032 installed-executable observation. It does not install, upgrade,
authenticate, or admit. Codex app-server can reuse `codex_app_server_claim`
plus a prepared 032 observation. Ollama reuses `ollama_runtime_claim`; 032
stays unobserved unless an executable is supplied.

## Overlay And Prepared Handoff

`apply_stored_model_presentation_overlay` projects hide, ordinal,
consumer-default, and favourite markers onto one bound 047 catalogue
result. Markers key to exact configured-instance, provider, and model ids.
Provider catalogue defaults stay distinct from the consumer-default marker.
Unknown models and cross-instance markers fail closed. Catalogue rows
without `provider_id` stay unmarked; do not invent a Codex or Ollama
provider id. Overlay copies `Ready` / `NotReady` and cannot make
`NotReady` selectable.

Then call the existing prepare entry with host-owned targets:

- `prepare_anthropic_direct`
- `prepare_codex` with `CodexPreparedDriver::AppServer`
- `prepare_ollama_attached`

Stored `ConfigFieldRef` values do not feed `prepare_*`. Model tag and
digest for Ollama stay prepare-time identities, not admission identity.
After prepare, continue through the route guide and
[provider selection](provider-selection-and-preparation.md).

## Consumer Responsibilities

Consumers:

- link the adapters they want and assemble the catalog
- own persistence, UI chrome, accent color, and selection policy
- keep secret bytes, paths, URLs, and env bodies on the host
- replace 047 snapshots after refresh rather than mutating them
- map a selected catalogue record back to the original prepared adapter value

Forbidden inferences:

- Swallowtail is not a server, router, fallback, or login client
- enablement is not readiness
- overlay is not a catalogue and does not change selection readiness
- subject is not an instance id or routing key
- a discovered candidate is not an addable row
- remaining production routes are not addable because three proofs exist

## Failures

Catalog, admission, sign-in, refresh, subject, overlay, and store failures
carry a redacted `SafeDiagnostic`. Use
[portable failure handling](portable-failure-handling.md). Do not parse
secret bytes, endpoint URLs, ChatGPT tokens, or provider prose in consumer
code. Terminal, cancellation, and cleanup remain on the prepared operation
after this handoff; this facade does not start a session.

## Examples And Validation

First-proof sequences:

- [Anthropic Messages](anthropic-direct-prepared-integration.md)
- [Codex app-server](codex-prepared-integration.md); `codex.exec` is not
  addable
- [Ollama attach](ollama-attached-prepared-integration.md)

Those route guides keep their existing prepared-facade examples as the
canonical route-map examples. Compiling Contract 057 examples are part of
this feature family once the guide map lists it.

Deterministic acceptance:

```sh
effigy qa:guides
effigy qa:docs
```
