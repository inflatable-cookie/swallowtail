# Repository Authority Map

Status: active
Owner: Tom
Updated: 2026-07-20

## Ownership

| Repository | Owns | Does not own |
| --- | --- | --- |
| Swallowtail | Portable adapter contracts, runtime mechanisms, provider integrations, conformance fixtures | Consumer prompts, tools, workflows, product persistence |
| Nucleus | Projects, tasks, goals, memory, operator authority, agent workflow, UI and server state | Shared provider contracts after adoption |
| Soundcheck | Plugin taxonomy, tagging prompts and schemas, review workflow, product state | Shared Codex/process connector mechanics after adoption |
| Monkey | Local model execution and model-serving behavior | Swallowtail's cross-provider contract vocabulary |

## Evidence Flow

Existing Nucleus and Soundcheck code is source evidence. Findings must be
promoted into Swallowtail specs, contracts, architecture, or logs before they
become Swallowtail decisions.

## Dependency Direction

- Nucleus and Soundcheck may depend on released or pinned Swallowtail crates.
- Swallowtail must not import consumer crates or use consumer repositories as
  runtime configuration.
- Monkey may later implement or back a Swallowtail provider route; neither
  project becomes a submodule of the other by default.

Consumer migration plans remain in each consumer. Shared API and adapter plans
remain here.

## Bounded Task-Execution Seam

Swallowtail owns the provider-neutral interactive access-policy vocabulary,
preflight enforcement, host-resource resolution contract, Codex sandbox and
callback translation, deadlines, terminal outcomes, and joined cleanup.

Nucleus owns task and Goal admission, the execution host and resource choice,
the selected exact policy, prompts, mandates, task/work-item identities,
provider-reference persistence, waiting/recovery projection, checkpoints,
diffs, review, receipts, and UI.

An observed provider approval or user-input request crosses the seam only as a
bounded correlated runtime observation. It grants neither repository authority
nor product mutation authority.
