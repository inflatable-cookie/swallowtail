# Connection Lifecycle Implementation Runway

Date: 2026-08-19
Roadmaps: `../roadmaps/g04/005-connection-lifecycle-kernel.md`,
`../roadmaps/g04/006-addable-catalog-admission-and-config-fields.md`,
`../roadmaps/g04/007-sign-in-loop-and-host-ports.md`

## Result

The first Contract 057 implementation roadmaps are compiled.

g04.005 is the kernel: core records, runtime store trait, optional
host-local in-memory and JSON-file adapters. Cards 013-015 are ready.
`PlannedConnectionRolloverPolicy` is out of scope. Additive public API must
use `public-api-unreleased` snapshots; `public-api-0.3.3` stays immutable.

g04.006 and g04.007 are planned behind that kernel: catalog and admission,
then sign-in ports and the library-max loop. Testkit fixtures only in 006.
No production Anthropic, Codex, or Ollama descriptors yet.

Refresh, subject observation, overlay projection, and first-proof routes
remain later compile work after 007.

## Next

Execute card 013. That is implementation. Dispatch a worker for g04.005
rather than editing production crates on the planning checkout.
