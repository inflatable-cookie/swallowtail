# 018 Kimi Code ACP Portability Proof

Status: completed
Owner: Tom
Updated: 2026-07-21

## Purpose

Prove ACP portability against a second maintained agent by exercising Kimi
Code lifecycle that materially differs from Gemini's new/read/cancel subset.

## Generation Runway

Advance g01 coverage expansion through persistent provider-owned sessions,
successor-supported load or resume operations, replay, filesystem-write
callbacks, delegated harness login, and explicit optional harness isolation.
The owned llama.cpp lifecycle is complete. Keep SDK bridges gated behind a
real embedding boundary.

## Contracts

- Contract 005: Integration Identity and Transport Diversity
- Contract 006: Execution Layer and Access Boundary
- Contract 009: Async Operation Lifecycle
- Contract 011: Runtime Conformance Profiles
- Contract 012: Interactive Session Options and Callback Exchange
- Contract 013: Interactive Session Access Policy
- Contract 015: ACP v1 Negotiation and Client Callbacks
- Contract 017: Provider-Owned Session Load, Replay, And Host Containment

## Goals

- [x] Revalidate current Kimi Code ACP version, implementation, capabilities,
      access, and support authority.
- [x] Promote provider-owned session, replay, write-callback, delegated-login,
      and cleanup rules before fixtures.
- [x] Freeze a deterministic second-agent ACP corpus around the promoted
      subset.
- [x] Prove and freeze the exact current Kimi Code successor corpus.
- [x] Qualify the selected execution-host containment mechanism and record the
      pinned Kimi artifact as unsupported under it.
- [x] Promote ambient harness communication as distinct from optional
      provider- or host-enforced isolation.
- [x] Implement the Kimi mapping without duplicating provider-neutral ACP wire
      behavior.
- [x] Prove portability, capability drift, topology, redaction, and joined
      cleanup through common conformance.

## Execution Plan

- [x] Currentness and persistent-session contract batch: card 055.
- [x] Protocol and capability fixture batch: card 056.
- [x] Successor currentness and fixture-delta batch: card 065.
- [x] Harness process containment qualification batch: card 057.
- [x] Optional harness isolation rebaseline: card 066.
- [x] Kimi portability driver batch: card 058.
- [x] Cross-agent conformance and closeout batch: card 059.

## Cards

- `batch-cards/055-kimi-acp-currentness-and-session-contract.md` — complete
- `batch-cards/056-kimi-acp-protocol-fixtures.md` — complete
- `batch-cards/065-kimi-code-successor-currentness-and-fixture-delta.md` — complete
- `batch-cards/057-harness-process-filesystem-containment.md` — complete with
  host-enforced Kimi marked unavailable
- `batch-cards/066-optional-harness-isolation-rebaseline.md` — complete
- `batch-cards/058-kimi-acp-portability-driver.md` — complete
- `batch-cards/059-kimi-acp-conformance.md` — complete

## Acceptance Criteria

- [x] Kimi and Gemini remain separate integration families and adapter drivers
- [x] shared ACP framing gains no Kimi identity branch
- [x] load, replay, and the presence or absence of separate resume remain
      explicit and provider-bound
- [x] filesystem callbacks use exact host and access authority
- [x] ambient process and network authority remain visible and distinct from
      provider- or host-enforced isolation
- [x] delegated login exposes no reusable provider secret
- [x] callback authority is not used as proof of provider approval or process
      containment
- [x] default QA requires no Kimi binary, account, or live credential

## Planning Checkpoint

Research 007 triggered card 057's original local-only stop condition. Research
009 records the operator's later selection of a deployment-owned native macOS
App Sandbox helper instead of a container. Research 010 repairs the intervening
currentness error: cards 055-056 already target the maintained TypeScript Kimi
Code successor at `0.28.1`. Card 065 pins its exact tag and source commit, ACP
behavior, arm64 archive and executable digests, isolated state, upstream
signature, deployment re-signing inputs, exclusions, and upgrade gate.

Roadmap 019 and card 065 are complete. Card 057 tested deployment-signing
compatibility, exact security-scoped bookmark handoff, and descendant
inheritance before creating a portable containment lease. Do not absorb
Monkey's serving, artifact, PID, readiness, or warm-lifecycle authority.

Research 011 closes that proof attempt with a split result. The bookmark and
descendant boundary works for a compatible helper, but the exact Kimi `0.28.1`
single executable crashes during V8 initialization under both tested sandbox
signature shapes. Disabling JIT only moves the failure to an extracted native
module and is not a supported deployment route. `HostEnforced` Kimi therefore
remains unavailable.

Research 012 and the operator decision settle the product ambiguity:
Swallowtail is a control plane over harnesses, not a mandatory replacement
sandbox. Card 066 makes `AmbientHost`, `ProviderEnforced`, and `HostEnforced`
explicit and reclassifies existing Gemini and OpenCode routes honestly. Card
058 now realizes the pinned ambient Kimi process with isolated state,
delegated authentication, working-resource location, callback scope, and no
bounded filesystem, descendant, or provider-tool network claim. The production
adapter holds delegated auth, resource, process, and protocol work in one
joined attachment; exposes distinct new, load-with-replay, and resume-without-
replay operations; and mediates only bounded text replacement through the
exact `ReadWrite` lease. Card 059 completes cross-agent conformance, optional
installed probing, and roadmap closeout.

Card 059 closes the lane. One shared ACP decoder now passes pinned Gemini and
Kimi corpora without provider branches. The baseline long-lived ACP profile
remains Gemini-compatible; a separate persistent ACP extension proves Kimi's
load, replay, resume, write, delegated-auth, ambient-host, topology, redaction,
and joined-cleanup claims. Production fixtures run the lifecycle under local
and remote-authoritative host identities. The installed `0.28.1` probe remains
explicitly gated and ignored by default.

Research 005's first three post-tranche choices are now complete. Roadmap 020
selects an SDK-native-first evidence recheck as the next lane. It will choose
another hosted, protocol, harness, or runtime route if no maintained Rust
embedding or explicit supported language boundary exists.

## Stop Condition

Stop if current evidence cannot distinguish load from resume, cannot authorize
write callbacks safely, would disguise ambient execution as containment, or
would require inspecting or exporting Kimi login state.
