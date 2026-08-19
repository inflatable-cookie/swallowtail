# g04.001 Inventory And g04.002 Contract Targets

Date: 2026-08-19

## Decision

Research 168 maps the Poodle/T3 connection lifecycle onto existing Swallowtail
records. Contract 047 stays a selection snapshot. The lifecycle in front of it
needs a new contract after 056.

Existing pieces: `DriverDescriptor`, per-driver `DiscoveryDriver`,
`AccessStatus` dimensions, opaque `CredentialRef` leases, installed-executable
classification, prepared facades, and 047 readiness chips. `SignInAction` is
advertised in core and unused in production. Discovery cannot list addable
routes. ACP `authenticate` is not login. There is no browser, loopback, or
device-code host port.

Crate placement: records in core, roles and store trait in runtime, optional
simple store adapters in host-local, addable descriptors adapter-local.

First-proof set after the tag: Anthropic Messages, an OAuth/subscription
loop that can be proved without extracting secrets, Codex app-server, Ollama
attach.

Seam amendments only: 006, 008, 010, 014, 015, 017, 029, 032, 037, 047.

## Validation

- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`

## Next

Define the current-source release inventory for the pre-facade tag. Facade
implementation stays planned until that tag exists.
