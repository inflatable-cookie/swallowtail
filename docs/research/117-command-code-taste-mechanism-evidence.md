# 117 Command Code Taste Mechanism Evidence

Status: draft
Owner: Tom
Date: 2026-08-09

## Question

What does Command Code's "taste" mechanism actually do, and which parts are
replicable as a provider-neutral, pluggable Swallowtail mechanism rather than
product policy?

## Method

Public docs only: taste, memory, permissions, headless, and CLI reference
pages plus the launch announcement. No installed probes: taste signals are
interactive accept/reject/edit events that headless runs do not exercise.
Harness integration itself is covered by Research 116.

## Evidence

### Observable mechanism

Five stages, all documented publicly:

1. **Signals** — every accept, reject, and post-accept edit is a training
   signal.
2. **Extraction** — a learning loop converts signals into symbolic
   constraints, stored human-readable in `.commandcode/taste/taste.md`, each
   with a confidence value in `0..1`.
3. **Application** — constraints condition generation:
   `output = LLM(prompt | taste(user))`. The constraint set acts as a
   personalized prior; docs state RAG was tried and rejected for style.
4. **Scopes and distribution** — per-project `.commandcode/taste/`,
   user-global `~/.commandcode/taste/`, and remote registry packages;
   `npx taste push/pull` merges by learning status (added / confidence
   updated / unchanged).
5. **Validation** — `npx taste lint` checks structure, headers, confidence
   ranges, and encoding.

Claims: learns patterns, not intentions; "rules decay, taste compounds";
self-published correction-loop benchmarks (no independent evidence).

### Boundaries around taste

- **Memory** (`AGENTS.md` tiers: user / project / subdirectory) is written
  instruction in the system prompt; taste is learned constraint. Both are
  injected into context and both cost tokens every turn.
- **Skills** = capability (what to do); taste = alignment (how you do it).
- **Permissions** (allow/ask/deny, modes) is enforcement; taste is
  preference. Separate engines.
- **Headless** runs use `--skip-onboarding` and have no accept/reject loop;
  the learning signal is interactive-only. Headless inherits static rules,
  not learned taste.
- **taste-1 itself is closed**; the "meta neuro-symbolic RL" description is
  the only public implementation detail.

## Assessment

Replicable as provider-neutral Swallowtail mechanism:

1. signal ingestion from existing event and tool-call transport
2. typed symbolic constraint record with confidence, provenance, scope
3. pluggable engine trait: static-rules baseline, learned backend, registry
   backend
4. injection into harness session context with budget semantics
5. push/pull merge semantics for constraint stores

Not replicable: taste-1 (closed model); the neuro-symbolic training detail.

Product policy stays host-owned per the vision: what idioms mean, which
signals matter, when to inject.

## Recommendation

Proceed to Spec 006 "Pluggable Learned Idioms". Design the mechanism with a
static-rules-first baseline and keep any learned backend (Monkey crates)
behind the engine trait. Do not absorb product policy into the mechanism.
