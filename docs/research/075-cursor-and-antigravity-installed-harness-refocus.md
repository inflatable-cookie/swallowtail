# 075 Cursor And Antigravity Installed-Harness Refocus

Status: promoted
Owner: Tom
Date: 2026-07-31

## Question

Should Swallowtail pause further Gemini CLI qualification, add Google's
Antigravity CLI, and promote Cursor from inventory evidence into the next
installed-harness implementation tranche?

## Method

This pass combined:

- the operator's installed and authenticated harness evidence
- local executable identity, version, help, account-posture, and model-list
  observations
- current official Google, Antigravity, and Cursor documentation
- the maintained ACP registry entry for Cursor
- existing Swallowtail contracts and Research 003, 022, 032, and 074

No provider prompt, model inference, workspace mutation, session creation, or
live callback ran. Local account identity and raw provider output are not
retained in repository evidence.

## Operator Evidence

- Gemini CLI accepted API-key authentication but did not return a response to
  an operator prompt.
- Cursor is installed and authenticated on a paid personal account.
- Antigravity CLI is not installed yet.
- Qwen account setup remains operator-owned and in progress.

The Gemini result is an operator-specific readiness failure. It does not prove
that Gemini's protocol is universally broken. Google's personal-account access
change nevertheless makes continued Gemini work lower value for Swallowtail's
local subscription-harness use case.

## Cursor Installed Evidence

The host exposes `cursor-agent` at an explicit executable path. Its observed
version is `2026.07.01-41b2de7`. It reports an authenticated Cursor Pro access
posture and returns an auth-aware model list without running a model.

The generic `agent` command on this host identifies as Grok, not Cursor. That
collision is material:

- automatic Cursor discovery must prefer the unambiguous `cursor-agent`
  executable name
- Swallowtail must not hardcode rejection of an explicitly host-approved
  `agent` path
- any approved path must still pass Cursor identity and version discovery
- no candidate fallback may silently cross integration families

Cursor's current CLI exposes three useful surfaces:

| Surface | Evidence | Swallowtail role |
| --- | --- | --- |
| `cursor-agent models` | authenticated local catalogue output | model-catalogue driver; catalogue membership is not invocation availability |
| `cursor-agent acp` | first-party ACP server command and maintained ACP registry entry | interactive-session driver |
| headless print mode | JSON and stream-JSON output, model selection, plan/ask modes, resume/continue, trust and optional sandbox controls | structured-run driver |

The ACP registry currently publishes Cursor `2026.07.23` with build identity
`2026.07.23-e383d2b`. The installed `2026.07.01-41b2de7` point and the registry
point are separate exact artifacts. Calendar dates do not make different build
hashes safely orderable as one continuous compatibility range.

Cursor's documented headless event stream includes initialization, user and
assistant messages, tool-call state, and a terminal result. Research 077 later
corrected this selection-stage assumption from exact installed source: plain
stream JSON also emits explicit thinking deltas and completion. Swallowtail
must project only those exact events, never infer reasoning from assistant text.

Cursor has also published a beta Agent SDK and cloud-agent APIs. They are not
selected for the first tranche. The installed CLI already proves the relevant
owned-process and shared-protocol shapes with less access, hosting, and product
policy.

## Cursor Route Selection

Add one integration family and one crate:

- family: `cursor`
- crate: `swallowtail-adapter-cursor`
- prepared facade: `prepare_cursor`

Keep three route identities:

- `cursor-agent.catalogue`
- `cursor-agent.acp`
- `cursor-agent.headless`

The first ACP claim must remain conservative. Do not claim session load,
resume, provider deletion, consumer MCP propagation, or model selection through
ACP until exact transcripts prove them. Model discovery belongs to the
catalogue route even if ACP later advertises model options.

The first headless claim should cover explicit-model structured runs, streamed
activity, usage where present, cancellation, deadlines, and bounded workspace
authority. Plan and ask modes remain provider-specific options. Optional Cursor
sandboxing is an explicit profile, not an implicit prerequisite.

Access remains provider-supported delegated local Cursor authentication.
Explicit Cursor API-key access, if added, must be a separate access profile.
Swallowtail must not extract or repackage Cursor credentials.

## Antigravity Evidence

Google announced that personal Google AI Pro, Ultra, and free access moved from
Gemini CLI to Antigravity on 2026-06-18. Gemini CLI remains relevant for
enterprise and paid API-key use, so its existing Swallowtail adapter and
qualification evidence should be retained rather than renamed or removed.

Antigravity CLI is a separate Google integration family. Official documentation
currently exposes:

- native `agy` installation and Google Sign-In backed by the system keyring
- enterprise Google Cloud access as a distinct posture
- `agy models` and explicit `--model` selection
- headless `-p` execution
- `text`, `json`, and `stream-json` output
- JSON-schema-constrained results
- step, tool, subagent, token-usage, and terminal-result events
- exact conversation continuation by conversation id
- permission settings and an optional sandbox

Invalid model selection fails rather than silently falling back. Headless tools
requiring approval are soft-denied unless already approved. The initial adapter
must not use the dangerous permission-bypass flag. Provider sandboxing remains
an optional exact capability profile.

Official pages identify CLI `1.1.8`, while the maintained repository exposes
`1.1.8` and `1.1.9` tags at the same commit. Qualification must reconcile the
published artifact, tag alias, installed identity, and selected source before
naming a guarantee.

The initial Antigravity routes should be:

- `antigravity.catalogue`
- `antigravity.headless`
- a later turn-scoped continuation route using explicit conversation ids

The documented TUI can expose richer trajectories, but no stable machine
protocol was found for consumer-mediated callbacks. Do not claim an interactive
callback session merely because the TUI is interactive.

Raw tool arguments and output can contain workspace content. The adapter must
map and bound activity evidence and keep raw payloads out of stable diagnostics.

## Gemini Disposition

Retain both existing Gemini routes and their exact guarantees. Pause roadmap
work to extend them through `0.53.0`.

Resume only when one of these is true:

- the operator explicitly requests enterprise or paid API-key qualification
- a consumer supplies reproducible evidence that the Gemini route remains
  useful for its supported access posture
- Google changes the access boundary again

There is no Gemini-to-Antigravity fallback. Selection remains explicit by
integration family, driver, transport, executable, access profile, and model.

## Cursor Exact Corpus — 2026-07-31

Card 010 froze the first exact Cursor evidence without sending a prompt or
creating a session.

| Evidence | Exact observation | Boundary |
| --- | --- | --- |
| installed executable | `2026.07.01-41b2de7`; launcher SHA-256 `eed61c52...7831`; runtime index SHA-256 `b974679f...e2e5` | first local candidate point; no production claim yet |
| maintained ACP registry | version `2026.07.23`, build `2026.07.23-e383d2b`; darwin-arm64 archive SHA-256 `f2eb2585...bbf2` | downloaded and inspected, not executed; behavior is not borrowed from the installed point |
| installed ACP initialize | wire v1; load and list advertised; image prompt input; HTTP/SSE MCP; `cursor_login` auth | capability advertisement is observation only until route-specific transcripts prove use |
| installed catalogue | 193 normalized entries; SHA-256 `e3cffea5...8585` | auth-aware and temporally dynamic; no listed-model invocation guarantee |
| installed help plus official output reference | `text`, `json`, `stream-json`; system, user, assistant, tool-call, and result events; partial assistant deltas optional | successful streams end in result; failed streams may end early and diagnose on stderr; later exact-source correction is in Research 077 |

The exact initialize probe advertised `loadSession` and session listing. Those
fields do not justify production load or list claims. No `session/new`, load,
list, prompt, authenticate, callback, model invocation, or provider mutation
ran. Card 012 must freeze route-specific transcripts before enabling each ACP
capability.

The source corpus also settles the first headless privacy rule. Cursor's public
examples include absolute working paths, prompts, tool arguments, file content,
tool results, session ids, and request ids. Stable mapping may retain opaque
correlation and bounded presentation evidence, but raw examples are not copied
into diagnostics or public events.

The normalized fixtures live in
`swallowtail-protocol-acp/tests/fixtures/acp-v1-cursor-agent-2026.07.01-41b2de7`.
They deliberately keep the two executable artifacts separate and leave
production qualification false.

## Cursor Foundation Result — 2026-07-31

Card 011 adds `swallowtail-adapter-cursor` with one catalogue driver and one
installed-discovery role. Automatic selection exposes only `cursor-agent`;
the adapter still probes any explicit host-approved executable target by
running `--version` and never rejects it by filename.

Compatibility uses the normalized release date as the ordered axis. The
opaque seven-character build revision is validated separately. The qualified
`2026.07.01` date admits only exact build `41b2de7`; an unknown same-day build
is malformed rather than inheriting qualification. Later valid dates remain
visible as unverified newer. Future milestones therefore need an ordered
release point plus an explicit qualified-build gate; hashes are not ordered.

The catalogue route runs only `models`, accepts a bounded plain-text list,
preserves model ids, display names, and the default marker, and does not infer
provider identity or invocability. Local login remains opaque
provider-supported subscription access. Deterministic local-authoritative and
remote-authoritative fixtures pass without a live provider prompt.

## Contract Result

No new shared contract is required.

- Contracts 005-006 preserve family, driver, transport, and access identity.
- Contract 015 governs ACP negotiation without granting unobserved features.
- Contract 020 governs auth-aware model catalogues.
- Contract 023 keeps provider sandboxing optional and exact.
- Contracts 029 and 032 govern exact executable qualification and discovery.
- Contract 033 represents ambient harness configuration honestly.
- Contract 037 governs prepared facades.
- Contracts 039 and 043 cover structured projection and turn-scoped
  continuation.
- Contracts 044-045 cover observable activity and child topology.

Both adapters still need route-specific deterministic corpora, compatibility
claims, discovery, and prepared-facade evidence before implementation can be
accepted.

## Selection

1. Implement Cursor first. It is installed, authenticated, high priority, and
   exercises both ACP interaction and provider-specific structured streaming.
2. Implement Antigravity second after exact artifact reconciliation and local
   installation. It replaces Gemini's personal-account role, not its identity.
3. Keep Gemini installed-route work paused without deleting existing support.
4. Reassess Qwen after the operator finishes account setup; do not couple that
   external gate to Cursor or Antigravity.

## Risks

- Cursor uses same-day and calendar-labelled builds with opaque hashes. Do not
  infer semver-style continuous ranges.
- Cursor ACP has had model-selection and capability regressions. Claim only
  exact observed negotiation.
- `agent` is an executable-name collision on the qualification host.
- Antigravity's docs/tag version discrepancy must be settled from exact
  artifacts.
- Personal, API-key, and enterprise Google access are independent authority
  surfaces.
- Neither route may expose account identity, credentials, or raw tool payloads
  through stable diagnostics.

## Primary Sources

- [Gemini CLI to Antigravity transition](https://github.com/google-gemini/gemini-cli/discussions/27274)
- [Antigravity CLI overview](https://antigravity.google/docs/cli/overview)
- [Antigravity headless mode](https://antigravity.google/docs/cli/headless)
- [Antigravity CLI reference](https://antigravity.google/docs/cli/reference)
- [Antigravity CLI repository](https://github.com/google-antigravity/antigravity-cli)
- [Cursor CLI](https://cursor.com/cli)
- [Cursor headless CLI](https://cursor.com/docs/cli/headless)
- [Cursor output formats](https://cursor.com/docs/cli/reference/output-format)
- [Cursor Agent SDK release](https://cursor.com/changelog/sdk-release)
- [ACP registry](https://cdn.agentclientprotocol.com/registry/v1/latest/registry.json)
