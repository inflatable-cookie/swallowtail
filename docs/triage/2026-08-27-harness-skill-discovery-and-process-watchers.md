# 2026-08-27 Harness Skill Discovery And Process Watchers

Status: promoted; Research 255 complete; operator decisions recorded
Owner: Tom
Source: operator direction during g04.082 execution
Updated: 2026-08-28

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

## Promotion (2026-08-28)

The operator authorized the g04-to-g05 rollover after the per-route inventory
closed. g05.001 promotes this note as an evidence-first programme: card 001
owns the prompt-free production-harness surface inventory, card 002 owns
boundary classification and the existing operator decision packet, and card
003 owns any later architecture, contract, or proof-route selection. No
watcher implementation or skill injection is ready.

## Card 002 Decision Packet

Research 255 completed the prompt-free census across 35 harness routes. It
found zero deliver-now skill-roster rows and zero complete native-watcher rows.
The evidence supports boundary choices, not architecture, public types, or a
proof-route selection.

### Evidence Classification

| Class | Evidence-shaped boundary |
| --- | --- |
| Portable candidate | A bounded skill observation vocabulary may preserve provenance, visibility scope, evidence strength, and freshness. No current route can populate a positive selected-session roster. A separate operation-local watcher vocabulary may preserve lifecycle and bounded activity once a host-owned mechanism exists. |
| Provider-local | Native skill registries, provider task or subagent ids, session ids, native activity, abort, and wait surfaces keep exact adapter semantics. None becomes portable control merely because several providers use similar names. |
| Host-owned | Watcher creation, stable operation-local watcher identity, process-tree ownership, status, wait, bounded output capture, stop, deadline, cancellation, join, and the successful-turn completion gate require one host-owned registry or an equally strong qualified native mechanism. |
| Consumer-owned | Presentation, disclosure preferences, durable projection, retention, operator authorization, and product policy for exposing a direct stop action remain downstream. |
| Unsafe | Ambient home or project scans, arbitrary PID attachment or kill, harness configuration mutation, raw or unbounded log exposure, prompt-only enforcement, foreign-process control, and ordinary watcher survival beyond the owning turn. |
| Unavailable | Prompt-free positive proof of the skills visible to the selected model/session, and a complete consumer-controllable native watcher, across the current production routes. |

Skill visibility and watcher enforcement therefore need separate specs,
contracts, and proof routes. They share provenance and session-visibility
questions only. Skill evidence cannot grant process authority. Provider
activity cannot satisfy watcher control.

### Existing Contract Coverage

| Contract | Already governs | Does not yet govern |
| --- | --- | --- |
| 013 Interactive Session Access | Preflight-bound session access, callback posture, host-monotonic deadlines, terminal cleanup, and joined provider work. | A generic watcher registry, watcher ids, or watcher operations. |
| 017 Provider-Owned Session Load | Provider-session binding, load/resume/recovery separation, containment qualification, and consumer-owned persistence. | Watcher durability or recovery. Session recovery must not imply watcher recovery. |
| 023 Harness Isolation | Exact isolation posture; separation of native budgets from host deadline, cancellation, stop, force-stop, bounded output transport, and joined cleanup. | Watcher ownership, per-watcher control, or a turn-completion gate. |
| 041 Input And Tool Admission | Exact separation and admission of consumer tools, provider-owned tools, and MCP or harness tools. | Process authority created by a skill or prompt. Any model-facing watcher tool needs its own admitted mechanism. |
| 044 Observable Activity | Operation-local activity identity, command/task lifecycle and disclosure, bounded content, and consumer-owned presentation. | Start, wait, stop, or completion authority. Provider-observed command activity is not a controllable watcher. |

### Recorded Operator Decisions

The operator recorded all four choices on 2026-08-28. These decisions settle
scope and authority for promotion work; they do not select architecture,
public types, or proof routes by themselves.

#### 1. Skill Discovery Scope

Decision: discover the effective skill set that the selected harness session
can see. Provenance does not limit scope. Distribution-bundled,
operator-installed global, and project-local skills are all included when the
harness admits them to that session. Skills deliberately installed through a
workflow such as `npx skills` are first-class evidence targets, not a deferred
class.

The exact harness surface must still prove membership and selected-session
visibility. Swallowtail must not substitute a recursive home or project scan,
infer loading from file presence, or enumerate unrelated ambient state. When
the harness exposes no dependable prompt-free roster, the route reports that
truth as unavailable or unverified rather than returning an incomplete list.
Each observation preserves provenance so consumers can distinguish bundled,
global, project-local, plugin, and unknown sources.

#### 2. Watcher Stop Authority

Decision: both model and operator receive controls through separate typed
operations against the same host-owned registry. Operator control is exposed
through the consumer-facing boundary. Accept only watcher ids owned by the
current turn. Completion-versus-stop races and repeated stop requests must be
deterministic. A PID is never authority.

#### 3. Consumer Output Exposure

Decision: expose lifecycle, status, and bounded redacted output summaries.
Keep raw or continuous logs out. A later log stream would need separate
authorization, bounds, backpressure, retention, and redaction rules.

#### 4. Active-Watcher Turn Completion

Decision: the watcher wait operation pauses the agent turn until the selected
watcher is terminal and returns its bounded result. If the model still attempts
successful completion with active watchers, reject completion and return
structured active-watcher state so it must wait or stop. Cancellation and
deadline stop and join all owned watchers before the turn fails.

The settled core invariant is that an ordinary turn cannot
complete successfully until every watcher-owned process is terminal through
completion or explicit joined stop.
