# Claude Agent ACP Conformance Closeout

Date: 2026-07-24
Card: `../roadmaps/g01/batch-cards/145-claude-agent-acp-range-conformance-and-closeout.md`

## Outcome

The production Claude Agent ACP driver now passes the unchanged
`LongLivedAcpHarness` profile and an adapter-local range matrix under local and
remote-authoritative execution-host identities. No Claude identity or
provider-specific behavior entered the shared conformance profile.

Roadmap 048 and cards 142-145 are complete. The held Grok cards 138-141 and
provisional delegated-authentication spec remain intact.

## Range And Access Evidence

The matrix covers every private behavior milestone at `0.53.0`, `0.54.1`,
`0.60.0`, and `0.61.0` under both topologies. It also covers excluded,
incompatible, prerelease, latest-qualified, and stable unverified-newer
outcomes. Newer execution remains visible and does not extend guaranteed
support.

Each executable plan binds the exact `claude-agent` driver, ACP v1 stdio,
Anthropic public-API-key audience `api.anthropic.com`, one exact model,
`Ambient` configuration, `AmbientHost` isolation, and a read-only working
resource with no filesystem-boundary or sandbox claim.

Terminal-auth advertisement is rejected. Claude subscription login, provider
switching, writes, load, resume, consumer tool calls, provider network,
external search, and implicit fallback remain unavailable.

## Lifecycle And Safety

The shared profile proves session open, prompt, cancellation, deadline,
disconnect, callbacks, terminal outcome, and joined cleanup. Adapter-local
assertions cover permission rejection, model and access drift, failure
redaction, event and outcome redaction, and credential-last release.

No live account, provider request, package installation, or container was
used.

## Validation

- `cargo test -p swallowtail-adapter-claude-agent` — 14 passed
- `cargo clippy -p swallowtail-adapter-claude-agent --all-targets -- -D warnings`
  — passed
- `effigy qa` — passed
- workspace inventory — 658 tests: 654 passed, four gated probes ignored
- `effigy doctor` — unchanged inherited 19 findings: 12 warnings, seven errors
- `git diff --check` — passed before closeout edits

## Continuation

Roadmap 049 and card 146 are ready for the g01 generation-disposition
checkpoint. They may recommend generation closure, one coherent final roadmap,
or an explicit operator gate. They do not select a provider or create g02
automatically.
