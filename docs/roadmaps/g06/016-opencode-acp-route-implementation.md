# g06.016 OpenCode ACP Route Implementation

Owner: Tom
Created: 2026-09-22
Depends on: Research 337 (frozen identity); Contracts 012, 017, 023, 041, 047, 057, 061, 063
Vision tags: route expansion, OpenCode ACP, ACP driver, MCP seam

## Outcome

Build the `opencode.acp` route in `swallowtail-adapter-opencode` against the
identity frozen by g06.015: the ACP stdio driver, prepared facade, activity
projection, and the consumer-declared MCP seam for the transports a contract
admits. Provider-free tests only.

## Why It Matters

Bovine needs a fourth harness and OpenCode is the operator's named candidate.
Research 337 found something better than expected: OpenCode's ACP `session/new`
accepts a client-declared `mcpServers` entry in `stdio`, `http` and `sse` form,
advertises `mcpCapabilities: { http: true, sse: true }`, and connects an
`http`/`sse` entry with a StreamableHTTP transport that forwards declared
headers. The production MCP shape is therefore representable **without**
Longhorn's stdio carrier, which no existing route achieves.

## Ready-State Rubric

- [x] Research 337 freezes official and installed identity and enumerates the
      selected ACP surface.
- [x] The MCP seam is settled as representable, with the live-honouring gap
      named.
- [x] The route, axis, claim id, behavior revisions and segments are proposed.
- [x] Scope, acceptance, validation, evidence, and stop conditions are explicit.
- [x] Review oracle is present because the route is exact, version-bound, and
      partly negative.
- [ ] A contract admits a consumer-supplied URL-plus-header MCP placement.

## Decisions

- Route id `opencode.acp`, kept strictly unflattened from `opencode.http`.
  Axis `opencode.executable`, distinct from `opencode.server`.
- Build the ACP surface and the **stdio** MCP entry, which Contract 063 already
  admits. The `http`/`sse` placement is representable on the provider but is a
  **new placement shape no contract admits**; the driver must model it without
  wiring it into production until a contract admits it. Do not decide that
  admission here.
- Pin `--pure` unless the contract review settles otherwise: `opencode acp`
  otherwise loads host plugins, which is not part of the selected surface.
- Do not infer revision acceptance from `initialize`: OpenCode answers
  `protocolVersion` `1` regardless of the requested version.
- Bound the per-session declaration to one route-owned name, or surface the
  name-keyed last-write-wins booking honestly; the driver must not silently
  collide.
- If one behavior revision is compiled, compile for
  `opencode.acp-v1.client-mcp-servers-v2` (`1.18.31..=1.18.32`) and treat `v1`
  as accepted-but-older; the only delta is the `config_option_update` emission
  and the reasoning `messageId` binding.
- No provider prompt, login, install, host update, live ACP session, release, or
  tag.

## Dispatch manifest

- **State:** ready; implementation lane; one lane; serial after g06.015.
- **Completion:** the `opencode.acp` route builds and passes provider-free
  tests: ACP stdio launch and `initialize`, session creation, prompt and update
  mapping, permission callbacks, cancellation, stop reasons, and joined cleanup;
  the consumer-declared MCP seam accepts a stdio entry and models the
  URL-plus-header shape behind the contract gate; the prepared guide and
  affected matrices state only what is proved; independent exact-head review
  accepts the head.
- **Owned mutable paths:** `crates/swallowtail-adapter-opencode/**` for the new
  ACP route, driver, prepared facade, activity projection, and fixtures; the
  OpenCode ACP prepared guide; the affected route, activity and feature matrix
  rows; the architecture ceiling if a route count moves; `CHANGELOG.md`
  `[Unreleased]`; this task's result lines; `PAPERCUTS.md` append only.
- **Reserved closeout surfaces:** lifecycle task record and generated
  projections; submitted-handoff deletion belongs to the repository hook.
- **Worker:** Rust adapter implementer with ACP experience; provider-free
  conformance and fixture work.
- **Excluded:** any claim or admission change beyond what the contract admits;
  the `http`/`sse` placement in production; `opencode.http` and OpenCode web
  search; Contracts 060/063 semantics; Longhorn; release, tag, publication; any
  live probe.
- **Escalation:** operator via Chatterbox for the placement admission; Queue
  coordinator for mechanical blockers.

## Work

1. Add the ACP route and driver to `swallowtail-adapter-opencode` against the
   frozen identity, keeping the HTTP route untouched.
2. Map the surface: `initialize`, session creation, prompt and update frames,
   permission callbacks, cancellation, stop reasons, and joined cleanup.
3. Bind the consumer-declared MCP seam for the admitted stdio entry, and model
   the URL-plus-header shape without production wiring behind the contract gate.
4. Compile the behavior revision, claim id and segments the frozen identity
   proposes, marking the matrix cells exactly as far as the evidence reaches.
5. Run focused validation and the named docs and route gates.

## Acceptance and review oracle

| Invariant | Adversarial counterexample | Required proof |
| --- | --- | --- |
| The route is ACP, not HTTP | An `opencode.http` fixture or fact is reused | every fixture is ACP-shaped; HTTP tests unchanged |
| Only admitted placements reach production | A URL-plus-header placement is wired without a contract | the production seam accepts stdio only; the gated shape is unreachable |
| Revision claims are honest | The driver infers its requested `protocolVersion` was accepted | no acceptance inference; `1` is recorded as provider-fixed |
| Plugin loading is bounded | `opencode acp` loads host plugins on a production path | `--pure` pinned, or the omission is an explicit recorded decision |
| No unknown MCP cell flips | A matrix MCP cell says `Yes` on frozen evidence only | live-derived cells stay gated behind a live gate |

## Stop conditions

Stop and ask if the ACP surface diverges from the frozen identity, if the route
needs a public lifecycle or vocabulary change a contract must settle first, or
if the URL-plus-header placement cannot be modelled without wiring it.

## Evidence

Research 337 and its fixtures. Focused and affected-package validation for
`swallowtail-adapter-opencode`, route and matrix QA, and the named docs gates.

## Result

Accepted 2026-09-22. `opencode.acp` is a production ACP stdio route in
`swallowtail-adapter-opencode`, unflattened from `opencode.http`. Compiled
behavior is `opencode.acp-v1.client-mcp-servers-v2` for `1.18.31..=1.18.32`;
`v1` is accepted-but-older for deprecated `1.18.18..=1.18.30`. The driver pins
`opencode acp --pure`, records provider-fixed `protocolVersion` `1`, admits one
stdio MCP entry named `swallowtail-opencode-acp`, carries that declaration onto
working-state restoration, and models URL-plus-header `http`/`sse` behind the
contract gate. Every MCP matrix cell stays No. Working `0.5.1` public-API
baseline records the additive ACP facade; the working internal-dependency
graph records `swallowtail-adapter-opencode` → `swallowtail-protocol-acp`.
Named validation passed: fmt check, `validate:focused`,
`package:verify-affected`, `package:api`, `check-package-metadata.sh`,
`qa:routes`, `qa:docs`, `git diff --check`. No live ACP session, login,
install, or host update.

## Next Task

After this lands, Chatterbox reconciles the route and either compiles a live
qualification gate for the exact executable window or compiles the placement
admission, whichever the operator settles first.
