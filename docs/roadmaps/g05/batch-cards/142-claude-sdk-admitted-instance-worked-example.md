# 142 Claude SDK Admitted-Instance Worked Example

Status: complete; PR 291 merged at `d0f2ea3170950629dd2b73866218ae903db0e287` (reviewed head `2ec8ea8bbbf07b81b2fa77408094221a233fe61d`)
Owner: Tom
Created: 2026-09-08
Updated: 2026-09-08
Milestone: `../035-shared-harness-capability-and-producer-boundary.md`
Depends on: card 125 merged (`1cbc21ad`); the 2026-09-08 Desktop Chatterbox reference defect

## Defect

Desktop needs to assemble a `claude-agent.sdk` admitted-instance record and a
registered-tool runner to run the card 132 live gate, and Swallowtail has no
worked example of it. I pointed Desktop at
`examples/prepared_claude_agent_sdk.rs`; Desktop checked and that example
documents the inputs in prose, then accepts an already-constructed
`ClaudeAgentSdkSessionPreparation`. It never builds an `AdmittedInstanceRecord`
and never calls `ClaudeAgentSdkSessionPreparation::from_admitted`. The only
example that constructs admission is `examples/connection_lifecycle.rs`, and
that is `claude-agent.acp`, a different route with different inputs. The
guide's Explicit Inputs section names `from_admitted` correctly, so the gap is
the executable example, not the prose.

A consumer should not have to derive this composition from a packet, a guide
section, and an example for a different route.

## Scope

1. An executable example for `claude-agent.sdk` that constructs the
   `AdmittedInstanceRecord` from opaque host-owned references — the
   interpreted-script launch recipe binding the approved Node runtime and the
   source-tagged sidecar entry, the environment reference carrying
   `CLAUDE_AGENT_SDK_SIDECAR_SDK_MODULE`, `_NATIVE_BINARY`, and `_MANIFEST`,
   the delegated subscription credential reference, the model route, and the
   open deadline — then lifts it with `from_admitted`.
2. Continue it through the registered-tool path card 125 added: bind a
   `RegisteredToolPreparation`, select the mediated stdio proxy attachment
   with its proxy recipe, and open, so the example covers the exact
   composition the card 132 packet requires.
3. Keep it provider-free and non-executing against a real provider: the
   example demonstrates composition and types, as the existing examples do,
   and never spends a live route.
4. Cross-reference it from `docs/guides/claude-agent-sdk-prepared-integration.md`
   admission section and from the card 132 packet, replacing the incorrect
   pointer I gave Desktop.

## Out Of Scope

Any live run; changing the admission API; the ACP example.

## Acceptance Criteria

- [x] an executable `claude-agent.sdk` example constructs `AdmittedInstanceRecord` and calls `from_admitted`
- [x] it continues through the registered-tool binding, attachment, and proxy recipe
- [x] guide and card 132 packet point at it
- [x] provider-free; no live route spent

## Validation

- `cargo fmt -p swallowtail-adapter-claude-agent -- --check`
- `effigy validate:focused swallowtail-adapter-claude-agent`
- `effigy package:verify-affected swallowtail-adapter-claude-agent`
- `effigy qa:northstar`
- `git diff --check`

## Review Oracle

Invariant: a consumer can compose an admitted SDK instance with registered
tools from this one file. Smallest counterexample: an example that starts from
an already-constructed preparation.

## Auto-Continuation

No. Stop for exact-head review.

## Result

PR 291 was independently accepted at exact head
`2ec8ea8bbbf07b81b2fa77408094221a233fe61d` and merged into `main` as
`d0f2ea3170950629dd2b73866218ae903db0e287`. The executable example now
constructs admission, lifts it with `from_admitted`, binds the mediated
registered-tool path and proxy recipe, and shows the provider-free
open/turn/close composition. The guide and Card 132 packet point at the
worked example.

The accepted review found no required changes. Named validation passed:
`cargo fmt -p swallowtail-adapter-claude-agent -- --check`,
`cargo check -p swallowtail-adapter-claude-agent --examples`,
`effigy validate:focused swallowtail-adapter-claude-agent` (469/469),
`effigy package:verify-affected swallowtail-adapter-claude-agent`,
`effigy qa:northstar`, and `git diff --check`.

No validation failure was deferred. Live provider/consumer acceptance, the
Card 132 real-route gate, release/tag work, and consumer-repository changes
remain outside this card and retain their existing owners and authority. The
active Next Task pointer remains unchanged.
