# 096 Provider Session Import Route Classification

Status: promoted
Owner: Tom
Date: 2026-08-02

## Question

Which qualified harness routes can discover a harness-origin session,
revalidate it, replay bounded history, and continue it through a public
resource-bound Swallowtail binding?

## Method

This is a bounded classification delta over Research 092-095, Contracts 017,
029, 037-038, and 046, the production feature matrix, and current adapter
capabilities. It splits combined solution rows into individual transport
routes. No provider, executable, network, credential, prompt, session, or
consumer repository was used.

The classes mean:

- `supported` — production catalogue, exact revalidation, bounded replay, and
  public continuation all pass at qualified versions
- `discovery-only` — qualified read-only inventory exists, but it cannot issue
  this import binding
- `attachment-only` — qualified external load or resume exists, but no
  qualified catalogue can grant candidate authority
- `blocked` — upstream pieces exist, but a named authority or exact-evidence
  gap prevents either safe catalogue or attachment
- `not-applicable` — the selected route exposes no reusable external provider
  session identity; private operation continuity does not change that

## Route Inventory

| Route | Catalogue and exact lookup | Bounded replay | Public continuation | Resource, activity, version truth | Class | Exact promotion gate |
| --- | --- | --- | --- | --- | --- | --- |
| `codex.app-server` | qualified cwd-scoped list and read | qualified ordered thread replay | load and resume | exact host, endpoint, cwd, archive and version evidence | supported | none |
| `codex.exec` | none; operation input only | none | none | operation-private structured run | not-applicable | select a different explicit Codex transport |
| `kimi-code.acp` | qualified resource-scoped ACP list and repeated list lookup | qualified ordered ACP load replay | load and resume | exact host, executable, state root, cwd, availability and version | supported | none |
| `kimi-code.headless` | no public session identity | none | none | provider state may persist after a structured run | not-applicable | a new explicit route with public durable identity and complete evidence |
| `kimi-code.local-server` | qualified REST list and lookup surfaces | no qualified import replay path | resume only | exact server, cwd, status and version exist | attachment-only | bounded transcript replay plus list-to-resume revalidation across every qualified milestone |
| `opencode.http` | qualified directory list, status and exact lookup | qualified oldest-first message replay | load and resume | exact host, endpoint, directory, activity and server revision | supported | none |
| `claude-agent.acp` | no qualified ACP `session/list` | qualified ordered load replay | load and resume | exact executable, cwd and ACP behavior segments | attachment-only | agent-advertised stable form of `session/list`, qualified across the maintained range |
| `claude-code.headless` | none | none | none | each run disables session persistence | not-applicable | a separately selected persistent Claude transport |
| `cursor-agent.catalogue` | model and identity discovery only | none | none | no provider session exists | not-applicable | select a session-bearing Cursor transport |
| `cursor-agent.acp` | upstream initialization advertises optional list/load shapes, but production does not qualify them | unqualified | resume explicitly unsupported | exact cwd binding exists; list/load/activity/range evidence does not | blocked | exact installed-source list, lookup, replay, load/resume, resource and milestone corpus |
| `cursor-agent.headless` | no public durable session identity | none | none | provider state may persist after the run | not-applicable | a new explicit session-bearing route |
| `pi.rpc` | upstream persisted inventory and reads exist | upstream ordered reads exist | provider switching exists but public resume is rejected | stored cwd cannot be proven equal to the host-leased resource | blocked | upstream exact caller-supplied cwd attachment or observable effective cwd, then range corpus |
| `qwen.headless` | none | none | turn-scoped private continuation only | one active handle owns its private id | not-applicable | a new public durable-session route with list and history authority |
| `antigravity.catalogue` | model and identity discovery only | none | none | no provider session exists | not-applicable | select a session-bearing Antigravity transport |
| `antigravity.headless` | none | none | exact-id continuation stays private to one handle | exact cwd and version do not expose external attachment authority | not-applicable | a new public durable-session route with list and replay |
| `gemini-cli.acp` | none qualified | none qualified | no public external attachment | ACP provider state is preserved without management support | not-applicable | a separately qualified ACP list/load/resume route and supported access posture |
| `gemini-cli.headless` | qualified project-scoped `--list-sessions` exists for deletion reconciliation | no replay operation | no load or resume | exact executable and project storage identity are bound | discovery-only | exact lookup, bounded history, public load/resume, activity truth, and import revalidation |
| `grok-build.acp` | none qualified | none qualified | no public load or resume | durable local state is preserved without public identity | not-applicable | an advertised, qualified list/load/resume surface with exact resource binding |
| `anthropic.managed-agent` | operation-owned session only | operation-private output only | no reusable binding; cleanup deletes the session and environment | provider-hosted operation ownership | not-applicable | a separate provider-supported persistent managed-agent route |

## Counts

The inventory contains 19 distinct harness routes:

- supported: 3
- discovery-only: 1
- attachment-only: 2
- blocked: 2
- not applicable: 11

Combined solution rows do not alter these counts. `codex.app-server` does not
promote `codex.exec`; Kimi ACP does not promote Kimi headless or local server;
stable ACP wire support does not promote Claude, Cursor, Gemini, or Grok; and
headless private continuation does not become external attachment authority.

## Decision

Keep production provider-session catalogue/import support on exactly three
routes: Codex app-server, Kimi Code ACP, and OpenCode HTTP. Implement no route
in this card. Retain Gemini headless as discovery-only, Claude ACP and Kimi
local server as attachment-only, Cursor ACP and Pi RPC as blocked, and every
other selected harness route as not applicable until its explicit promotion
gate is met.

Card 062 may now publish separate catalogue, import, load, resume, and
management truth from this 19-route inventory.

## Promotion

- durable operation separation remains Contract 046
- public truth and extracted package acceptance: card 062
- consumer adoption boundary: card 063
