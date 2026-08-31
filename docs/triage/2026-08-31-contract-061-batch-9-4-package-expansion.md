# Contract 061 Batch 9.4 Package Expansion

Status: active planning evidence; candidate A complete; B-L unpromoted
Owner: Tom
Date: 2026-08-31

## Trigger

PR 131 merged the exact 36-row `codex.app-server` and 15-row
`openai.realtime` vertical at `fdd2b018`. Its ledgers intentionally claim
nothing for the other 716 rows in the reviewed Contract 061 census. Batch 9.4
must partition that remainder before another implementation tranche starts.

## Fixed Boundary

This checkpoint does not change Contract 061 or the Batch 9.1 public runtime
baseline. Every candidate must:

- own all remaining census rows for each adapter package it names;
- disposition rows by exact route, operation shape, semantic ID, lifecycle,
  value domain, omission, applicability, evidence source, and evidence
  strength;
- emit only truth admitted by exact prepared or active-observation evidence
  and withhold documentation-only or incompatible-operation rows at
  construction;
- retain separate source identities, exact access dimensions,
  consumer-mediated per-turn authority, and exact acknowledgement truth;
- keep explicit no-control route audits as negative coverage;
- use the existing consumer-supplied contribution/composer boundary, with no
  registry, runtime adapter enumeration, callback, downcast, or provider
  payload; and
- stop if `swallowtail-runtime`, `swallowtail-testkit`, `swallowtail-core`, or
  Contracts 037, 047, 057, or 061 need a new public decision.

Provider contact, live probes, currentness work, watcher work, PR 127, PR 130,
and generation closeout remain outside Batch 9.4.

## Exact Remainder

The CSV has 767 rows. Removing only the 36 `codex.app-server` and 15
`openai.realtime` rows leaves exactly 716 rows across all 31 adapter packages
and 46 route IDs. The following partition assigns every remaining row once.
Each candidate contains one to four complete adapter-package remainders and
35 to 94 rows, so normal focused validation can name its adapter packages
explicitly.

| Candidate | Complete adapter-package remainder | Exact route rows | Total | Sensitive truth retained | State |
| --- | --- | --- | ---: | --- | --- |
| A | Codex; OpenAI | `codex.exec` 35; `openai.background` 24 | 59 | prepared-only controls; background recovery descriptors | complete through card 023 / PR 133 |
| B | Alibaba Model Studio; Anthropic; xAI | `alibaba.conversations` 19; `anthropic.managed-agent` 17; `anthropic.messages` 23; `xai.responses-websocket` 17 | 76 | one consumer-mediated per-turn exchange | candidate only |
| C | Antigravity; Bedrock; Cursor | `antigravity.catalogue` 14; `antigravity.headless` 18; `bedrock.catalogue` 9; `bedrock.runtime` 10; `cursor-agent.acp` 13; `cursor-agent.catalogue` 13; `cursor-agent.headless` 17 | 94 | four explicit no-control route audits | candidate only |
| D | Claude Agent | `claude-agent.acp` 30; `claude-code.headless` 12; `claude-code.response-only` 11 | 53 | exact active-session acknowledgement | candidate only |
| E | Gemini; Grok | `gemini-cli.acp` 14; `gemini-cli.headless` 13; `gemini.live` 16; `grok-build.acp` 13 | 56 | ACP, headless, and live applicability remain distinct | candidate only |
| F | Kimi; Kimi Platform | `kimi-code.acp` 25; `kimi-code.headless` 20; `kimi-code.local-server` 31; `kimi-platform.chat` 13 | 89 | exact active-session acknowledgement | candidate only |
| G | Cline; Command Code; Copilot CLI; Goose | `cline.acp` 11; `cline.headless` 8; `command-code.headless` 11; `copilot-cli.acp` 9; `goose.acp` 9 | 48 | exact active-session acknowledgement; two no-control audits | candidate only |
| H | Deep Agents; Kiro; Qoder; Zcode | `deepagents.acp` 9; `kiro.acp` 9; `qoder.headless` 8; `zcode.app-server` 12 | 38 | three explicit no-control route audits | candidate only |
| I | DeepSeek; DeepSeek Harness | `deepseek.continuation` 19; `deepseek-harness.jsonrpc` 11; `deepseek-harness.local-server` 17 | 47 | continuation and local-server lifecycle remain distinct | candidate only |
| J | llama.cpp; Ollama | `llama-cpp.attached` 10; `llama-cpp.owned` 6; `ollama.attached` 19 | 35 | attached and owned applicability remain distinct | candidate only |
| K | Mistral Vibe; Muse; Oh My Pi; Qwen | `mistral-vibe.headless` 8; `muse-code.headless` 10; `oh-my-pi.rpc` 18; `qwen.headless` 16 | 52 | one consumer-mediated per-turn attachment row | candidate only |
| L | OpenCode; Pi | `opencode.http` 35; `pi.rpc` 15; `pi.sdk-sidecar` 19 | 69 | six per-turn rows, including observed callbacks | candidate only |
| **Total** | **31 packages** | **46 route IDs** | **716** | **all remaining census truth** | |

The nine no-control audits are assigned exactly once: four in C
(`antigravity.catalogue`, `bedrock.catalogue`, `cursor-agent.catalogue`, and
`cursor-agent.acp`), two in G (`copilot-cli.acp` and `goose.acp`), and three in
H (`deepagents.acp`, `kiro.acp`, and `qoder.headless`). The three remaining
exact active-session acknowledgement rows stay isolated in D, F, and G. The
eight remaining per-turn rows stay in B, K, and L; none may be relabelled as
session-start authority.

After candidate A, the active remainder is 657 rows across 44 route IDs and 29
adapter packages in candidates B-L. Those candidates remain planning rows.

## Promotion Rubric

A candidate becomes a numbered ready card only when all of the following are
closed for its whole package set:

1. the exact census row set and explicit no-control rows reconcile to the
   candidate total without an exception or filter list;
2. every contributing prepared or active-observation façade and its source
   identity kind are named, and documentation-only rows have an explicit
   construction-time withholding rule;
3. the work needs no new runtime/core public type, fixed maximum, composer
   failure, registry, enumeration, callback, provider payload, or contract
   amendment;
4. deterministic adapter-local ledgers prove exact emitted and withheld sets,
   source/applicability disagreement, lifecycle and authority distinctions,
   and negative coverage without contacting a provider;
5. focused validation names no more than four exact adapter packages, with
   semantic API, docs, Northstar, god-file, and diff checks added when the card
   changes or relies on those surfaces; and
6. the card stops after one reviewable package tranche and does not claim later
   candidates or the 767-row all-route audit.

Candidates B-L remain planning rows, not reserved cards or execution
authority. Promote one only after its adapter-local façade and ledger audit
passes this rubric against the then-current main branch.

## Candidate A Completion

Candidate A passed the rubric and was promoted as card 023:

- its exact set is the 35 `codex.exec` plus 24 `openai.background` rows;
- `CodexPreparedExec` and `OpenAiPreparedBackgroundRun` are the existing
  `PreparedOperationEvidence`-backed façades that may emit exact
  operation-owned truth through the already established
  `consumer_route_projection_contribution(source_id)` shape and
  `AdapterContribution` source kind;
- catalogue, route-wide, post-open, lifecycle, or other rows not proved by
  those exact prepared values are withheld at construction rather than
  widened from matrices or another route;
- the separate OpenAI Models and background-reconciliation prepared families
  are not treated as `openai.background` structured-run evidence and gain no
  new composer or observation seam;
- no new runtime/testkit/core public surface, open-result type, observer,
  acknowledgement path, or provider operation is required; and
- two exact adapter packages plus their deterministic 59-row ledgers fit one
  bounded provider-free validation tranche.

PR 133 merged exact reviewed head `fbb4b118` through `58be7122`. The proof
keeps the 35-row and 24-row ledgers independent, rejects mixed access evidence,
keeps prepared activity descriptors descriptor-only, and leaves unsupported
rows withheld at construction. No shared public or contract surface changed.

The next move is a planning-only reassessment. Resolve the open `codex.exec`
output-token census-source question, then audit candidates B-L against current
`main`. Promote at most one candidate only after the whole-candidate rubric
passes; this checkpoint itself grants no further execution authority.

## All-Route Gate

Batch 9.5 remains uncompiled. It may become ready only after candidates A-L
are completed and one audit can reconcile 767 rows, 48 route IDs, 31 adapter
packages, nine no-control routes, nine per-turn rows, and five exact
acknowledgement rows without weakening any Contract 061 boundary.

## Sources

- [Contract 061](../contracts/061-consumer-route-feature-and-control-projection.md)
- [g05.009](../roadmaps/g05/009-contract-061-consumer-projection-realization.md)
- [Batch 9.1 public baseline gate](2026-08-31-contract-061-batch-9-1-public-baseline-gate.md)
- [reviewed census](2026-08-30-consumer-route-feature-and-option-projection-census.csv)
