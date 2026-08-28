# 2026-08-27 Harness Skill Discovery And Process Watchers

Status: open
Owner: Tom
Source: operator direction during g04.082 execution

## Why This Note Exists

Two related harness abilities need research and product shaping after the
current per-route evidence wave:

1. discover the skills shipped by each harness distribution that the selected
   model can actually see
2. optionally inject a Swallowtail watcher skill that gives agents one
   dependable process-watcher workflow and publishes its state through the
   application-facing interface

This is triage, not execution authority. It does not interrupt g04.082 or alter
the per-route feature inventory count.

The 2026-08-28 per-route remainder audit transfers original item 51, Deep
Agents `--skills` / `--memory`, into this note. Host path, lifetime, provenance,
and model-visibility authority belong here; item 51 no longer remains in the
per-route generation-control queue.

## A. Harness Skill Discovery

### Operator Intent

For every harness-based route, identify and list the skills that are part of
that harness distribution and visible to the model in the selected operation.
Consumer applications should not have to guess from a product name or inspect
an unrelated host installation.

### Required Distinctions

- shipped by the exact harness distribution
- installed or configured on the host
- visible to the selected model/session
- eligible under the current route, version, access, resource, and mode
- selected or invoked during an operation
- provider tool, MCP tool, prompt instruction, agent profile, and skill
- observed discovery fact versus immutable prepared capability

Distribution membership does not prove model visibility. Files present on the
host do not prove that the child process loads them. A model mentioning a skill
does not prove discovery. Ambient personal, project, plugin, or enterprise
configuration must not be flattened into distribution-bundled skills.

### Candidate Consumer Shape

A future observation may need:

- route, driver, exact harness distribution identity, and version
- stable provider-local skill id or qualified adapter-minted id
- bounded display name and description
- provenance: bundled, host-configured, project-local, plugin, or unknown
- visibility scope: run, session, model, agent profile, or installation
- discovery strength: static distribution evidence, process observation, or
  session-confirmed
- freshness and whether later operation preparation must revalidate it

This is not yet a public type proposal. Exact harnesses may expose less or
different truth.

### Research Questions

- Which production routes are harness-based and which exact versions have an
  official prompt-free skill-listing or manifest surface?
- Does listing start a process, load account/project state, mutate caches, or
  invoke a model?
- Can discovery run without credentials, prompts, persistence, or arbitrary
  host filesystem enumeration?
- Do ACP initialize/session responses expose skills, or only tools and model
  options?
- Which harnesses bundle skills but hide their list from the running model?
- Can a prepared observation prove model visibility, or only installation
  membership?
- Should host-configured and project-local skills ever be included, and if so,
  what explicit resource and privacy authority permits their names and
  descriptions to cross the boundary?
- Where does this belong: route readiness, a new harness-capability catalogue,
  prepared session evidence, or another observation surface? It must not be
  placed in the model catalogue merely because both are lists.

### Initial Safety Boundary

Prefer exact prompt-free official listing or a frozen distribution manifest.
Do not recursively scan user homes, infer skills from arbitrary files, invoke a
model, mutate harness configuration, install plugins, or claim that a bundled
skill is visible when the selected route cannot confirm it.

## B. Swallowtail Process Watcher Skill

### Operator Intent

Swallowtail may optionally inject a skill into eligible harness sessions. The
skill teaches one consistent way to start, wait for, inspect, and stop
background processes. Swallowtail publishes consolidated watcher state through
its application-facing interface so consumers can present background process
activity consistently.

The immediate consumer evidence is T3 Code: the operator reports that only the
Claude route currently exposes useful background-process state. Treat this as
a research lead, not a portable capability claim.

### Settled Behavioral Aim

- a watcher-owned process has a stable operation-local identity
- the agent can check whether it is still running
- the agent can request bounded status/output updates
- the agent can stop a watcher-owned process
- the consumer can observe start, running, completion, failure, and stop state
- an ordinary turn must not complete successfully while a watcher-owned process
  is still running
- the agent waits or polls until completion, or explicitly stops/cancels the
  process and reaches a joined terminal state

"Background" therefore means concurrent with other work inside the same turn,
not detached beyond the turn. Durable daemon/service management is a different
problem.

### Dependability Boundary

Prompt text alone cannot enforce the workflow. The injected skill can teach the
model how to use it, but dependable watcher behavior requires a bound mechanism:

- a host-owned process registry or exact harness-native watcher registry
- stable watcher ids that cannot alias arbitrary PIDs
- ownership limited to processes started through the watcher mechanism
- process-tree, cancellation, deadline, stop, and join semantics
- bounded output buffering, backpressure, redaction, and disclosure
- a turn-completion gate that refuses successful completion while owned
  watchers remain active
- explicit behavior when the agent ignores the skill, the watcher channel
  fails, or the harness/process dies

Swallowtail must not grant arbitrary process-kill authority, attach to foreign
processes, trust PID identity across reuse, or present instruction compliance
as runtime enforcement.

### Candidate Watcher Lifecycle

The exact public shape is unresolved, but research should cover:

1. start accepted or rejected before process work
2. process started with stable watcher and owning turn identities
3. running updates with bounded status and optional output summary
4. explicit status check and wait operation
5. completed, failed, cancelled, timed out, or stopped terminal state
6. process tree joined and resources released
7. root turn allowed to complete only after every watcher it owns is terminal

Multiple concurrent watchers need bounded count, ordering, independent state,
and one root-turn completion rule. Cancellation and deadline must say whether
all owned watchers are stopped and joined. Session load/resume and process loss
must not imply durable watcher recovery unless separately proved.

### Activity And Tool Boundaries

Contract 044 already has `command execution` activity and operation-local
identity, but provider-observed command activity is not automatically a
controllable watcher. Research must decide whether watcher state extends that
activity detail or needs a new exact typed detail.

Contract 041 keeps native client tools, provider-owned tools, and MCP/harness
tools distinct. A watcher skill does not itself grant execution authority. The
mechanism may be:

- a Swallowtail/consumer-owned tool callback
- an operation-scoped MCP bridge
- an exact harness-native watcher API
- a wrapped shell/process tool whose registry and turn gate are host-enforced

These are different architectures and must not be silently combined. The
consumer-facing display projection also remains separate from model-facing
tool instructions and from raw process output.

### Research Questions

- What exact mechanism gives Claude/T3 Code its present background-process
  visibility, and which truth comes from Claude versus T3 Code?
- Which other production harnesses expose native background task ids, status,
  output retrieval, wait, stop, and completion events?
- Can Swallowtail inject a skill independently of user/project skill folders,
  or does injection require ambient configuration mutation?
- Is the watcher tool executed by the consumer, Swallowtail host services,
  MCP, or the harness? Who owns process authority and cleanup?
- May the consumer stop a watcher directly, or only request that the agent do
  so? What authorization and race semantics apply?
- What output can be shown without exposing commands, arguments, environment,
  paths, secrets, or unbounded logs?
- How are remote/container processes represented when no local PID is useful?
- What happens if a watcher finishes between status and stop, emits output
  after terminal, loses its transport, or leaves descendants?
- Does turn cancellation always stop watchers, and can any opt-in process
  survive a turn? The current operator intent says ordinary watchers do not.
- Which proof route should come first after architecture and contracts: Claude
  as the existing evidence route, or a route with a simpler host-owned process
  seam?

## Relationship Between The Two Ideas

Skill discovery can reveal whether a harness already ships a watcher-like
skill, but discovery does not authorize or standardize it. Watcher injection
also needs to declare its provenance and model visibility, so a future skill
observation surface may describe the injected skill. Runtime watcher authority,
activity, and turn gating remain separate contracts.

## Likely Promotion Path

1. survey exact harness skill-discovery and background-process surfaces
2. settle vocabulary, ownership, privacy, activity, and turn-completion rules
3. promote architecture and contracts before public API or route cards
4. select one proof route for discovery and one for watcher enforcement
5. add per-route qualification only after the shared boundary is testable

Do not compile implementation cards from this note yet.

## Open Operator Decisions

- Skill discovery scope: distribution-bundled only, or also explicitly
  configured/project-local skills that the exact session confirms visible?
- Watcher stop authority: model only, consumer only, or both through separate
  typed operations?
- Consumer presentation: status only, bounded output summaries, or a separately
  authorized bounded log stream?
- Failure posture when the model attempts to finish with active watchers:
  refuse completion and return control, automatically wait, or stop and fail?

The operator has already selected the core semantic invariant: successful turn
completion waits for watcher-owned process completion or explicit joined stop.
