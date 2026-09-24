# g06.015 OpenCode ACP Identity And Surface Freeze

Owner: Tom
Created: 2026-09-22
Depends on: Contract 029; Contracts 012, 017, 023, 029, 037, 041, 047, 057, 061, 063; Research 143, 158, 214, 336
Vision tags: route expansion, OpenCode ACP, identity, MCP seam

## Outcome

Freeze the identity and selected surface of OpenCode's ACP server and propose
the `opencode.acp` route shape, so the implementation lane can be compiled
against evidence instead of assumption. No claim moves here.

## Why It Matters

Bovine needs a fourth harness, and the operator names OpenCode as the most
likely one outside Codex, Claude, and Grok (direction 2026-09-22). OpenCode ACP
is an already-inventoried, deliberately unflattened candidate that was parked
only for want of a named consumer need — `docs/triage/2026-08-21-new-route-candidates.md`
records it as "sibling transport… only after a named sibling card". That need
now exists.

The route matters beyond breadth: ACP carries `mcpServers` on `session/new`, so
a client can declare an MCP server per session — the same seam that already
works for `grok-build.acp` and therefore the same path to the production MCP
through Longhorn's stdio carrier.

## Ready-State Rubric

- [x] The surface exists and is named: `opencode acp — start ACP (Agent Client
      Protocol) server`, observed on installed `opencode 1.18.18`.
- [x] The MCP seam is plausible but unproven: the literal `mcpServers` occurs in
      the installed payload, but nothing is frozen and no handshake is recorded.
- [x] A named consumer need exists (Bovine, operator direction 2026-09-22).
- [x] Scope, acceptance, validation, evidence, and stop conditions are explicit.
- [x] Review oracle is present because the result is exact, version-bound, and
      partly negative.

## Decisions

- Route id is `opencode.acp`, kept strictly separate from `opencode.http`. The
  existing ACP-versus-HTTP sibling rule stands: a claim on one transport never
  promotes the other.
- **Identity before claim.** This lane freezes evidence and proposes a claim
  shape; it moves no production claim and adds no route to the matrix.
- The consumer-declared MCP seam is the point of interest: freeze whether
  `session/new` `mcpServers` is honoured, in which transports, and whether a
  stdio entry is representable, since that decides carrier eligibility.
- No provider prompt, inference, login, install, or host update. Artifacts
  hashed and never executed; CLI help and the installed payload are readable
  evidence.
- A live ACP handshake or prompt needs separate operator authorization and is a
  typed gap here, not an inferred result.

## Dispatch manifest

- **State:** ready; evidence lane; one lane; g06.016 is the serial successor.
- **Completion:** a research record freezes official and installed identity for
  the version this route would claim, enumerates the selected ACP surface
  (entrypoint and argv, protocol revision and capabilities, `session/new`
  `mcpServers` handling per transport, prompt and update frames, permission
  callbacks, stop reasons, cancellation, exit and cleanup), names a proposed
  claim id, behavior revision, and route id, and records a typed gap for
  anything unsettleable provider-free.
- **Owned mutable paths:** the new research record and one research-index line;
  identity fixtures under
  `crates/swallowtail-adapter-opencode/tests/fixtures/opencode-acp-<version>/`;
  `PAPERCUTS.md` append only.
- **Reserved closeout surfaces:** lifecycle task record and generated
  projections; submitted-handoff deletion belongs to the repository hook.
- **Worker:** evidence-first route-identity worker; frozen-artifact and
  published-documentation analysis; no provider credentials.
- **Excluded:** any adapter, driver, or route implementation; any claim,
  matrix, or admission change; `opencode.http` and OpenCode web search;
  Contracts 060/063 semantics; release, tag, or publication; any live probe.
- **Escalation:** operator via Chatterbox for a surface that cannot be settled
  without a live session; Queue coordinator for mechanical blockers.

## Work

1. Freeze official npm/GitHub identity for the version this route would claim,
   with tarball digests and a shipped-file inventory, and record the installed
   `1.18.18` observation separately.
2. Enumerate the selected ACP surface from the frozen payload and published
   documentation: entrypoint and argv, `initialize` revision and advertised
   capabilities, session creation, prompt and update frames, permission
   callbacks, stop reasons, cancellation, and exit and cleanup behaviour.
3. Settle the `mcpServers` handoff explicitly: which transports a client may
   declare, whether a stdio entry is representable, and whether the entry is
   per session. This decides carrier eligibility and is the lane's most
   important single result.
4. Propose the route id, claim id, behavior revision, and support posture, and
   name what the implementation lane must build.
5. Stop with a typed gap where a surface cannot be settled without a live
   session or where the ACP revision diverges from the routes already qualified.

## Acceptance and review oracle

| Invariant | Adversarial counterexample | Required proof |
| --- | --- | --- |
| Identity precedes any claim | A route or matrix row gains an `opencode.acp` claim | no claim, matrix, or admission change in the diff |
| The MCP seam is settled, not assumed | Carrier eligibility asserted from the `mcpServers` literal alone | per-transport handling recorded with its evidence or a typed gap |
| HTTP and ACP stay unflattened | An `opencode.http` fact is reused as an ACP fact | every ACP claim cites an ACP-shaped artifact |
| No provider or host mutation | The CLI is executed beyond `--help`/`--version`, or artifacts run | hashes recorded; no session, prompt, or install |

## Stop conditions

Stop and ask if the ACP revision or capability set diverges from the routes
already qualified; if `mcpServers` handling cannot be settled provider-free;
if official and installed identity disagree on the selected surface; or if the
route needs a new driver shape that a contract must settle first.

## Evidence

The new research record, its frozen fixtures, and the proposed claim shape.
Downloaded artifacts are hashed and never executed.

## Next Task

Chatterbox compiles **g06.016 — implement the `opencode.acp` route** against the
frozen identity, covering the ACP driver, prepared facade, activity projection,
and the consumer-declared MCP seam, with provider-free tests. A live
qualification gate follows separately and needs its own operator authorization.
