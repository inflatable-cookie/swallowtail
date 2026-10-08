# Research 415: Gemini CLI 0.63.0 Headless Authority Adaptation Stop

Status: promoted; no compatibility claim moves

Owner: version-currentness lane; task swallowtail#140
Date: 2026-10-08
Base: canonical main 6cd9ba506684bcdbb869cb66d46c4f83ba7d0d3e
Authority: Contract 023 Gemini Headless Currentness Adaptation; Contract 029

## Result

The official npm latest tag and GitHub latest stable release still agree on
Gemini CLI 0.63.0. Research 371 remains the frozen identity and complete hop
ledger for 0.61.0, 0.62.0, and 0.63.0. The headless claim stays maintained at
0.51.0..=0.61.0, excluding unpublished 0.56.1 and 0.59.1. The 0.62.0 and
0.63.0 points remain UnverifiedNewer; ACP is independent.

The authority-preserving adaptation stops on a source-level conflict in
0.63.0. The selected Plan Mode prompt tells a noninteractive run to draft
autonomously and use exit_plan_mode to begin implementation. The frozen Plan
policy allows that tool while noninteractive in Plan mode. Its allowed path
switches approval mode to YOLO. The separate noninteractive ASK_USER to DENY
conversion does not apply to this explicit allow.

The selected Swallowtail invocation remains noninteractive Plan Mode and has
no adapter-side exit_plan_mode guard. A fake stream through the production
adapter confirms the route accepts a successful exit transition, later tool
events, assistant output, usage, and a completed terminal result. This is
source and adapter-boundary evidence, not a live provider claim. It cannot
prove the existing no-automatic-implementation-transition contract at 0.63.0.

The brief prohibits disabling provider behavior or bypassing safety rules.
Introducing a different enforcement mechanism or changing the public
authority/lifecycle contract needs a separate ruling. No provider prompt,
live session, credential, install, host mutation, or provider artifact
execution occurred. No compatibility claim or capability changed.

## Frozen identity and source proof

Official npm metadata re-probed on 2026-10-08 reports 0.63.0; GitHub's latest
non-prerelease release is v0.63.0, published at 2026-10-06T20:38:49Z. The
exact package/source identities and all published hops after 0.61.0 are
frozen in Research 371 and the gemini-cli-0.63.0 fixture.

The tagged source tree has 3,019 paths and reproduces the frozen tree-manifest
digest
fbb5d78fd631e4a53e26a62284f8c15d5d90a1ef2743d4d79a5517218dfc6e51.
Selected authority and boundary files are pinned in the new authority
evidence fixture. The ledger test binds each digest to the complete frozen
source-tree inventory.

| Source path | Selected behavior |
| --- | --- |
| packages/core/src/prompts/snippets.ts | Noninteractive Plan Mode drafts autonomously and describes exit_plan_mode as beginning implementation. |
| packages/core/src/policy/policies/plan.toml | exit_plan_mode is allow in Plan mode when interactive = false. |
| packages/core/src/tools/exit-plan-mode.ts | An allowed noninteractive exit selects YOLO. |
| packages/core/src/policy/policy-engine.ts | The noninteractive final conversion denies ASK_USER; it leaves an explicit allow unchanged. |
| packages/cli/src/nonInteractiveCli.ts | The stream emits tool_use before the scheduler call; this is not a host-controlled permission exchange. |
| packages/core/src/tools/read-file.ts | Resolves a defensive real path before access validation and validates again before reading. |
| packages/core/src/safety/built-in.ts | .gemini configuration writes request confirmation; the noninteractive policy denies that request. |
| packages/core/src/scheduler/tool-executor.ts | Stored tool output is capped at 64 KiB. |
| packages/core/src/agents/local-executor.ts | Older function responses can be collapsed under context pressure. |

The selected invocation/options and stream formatter/types remain byte-identical
to the prior qualified point. Adapter projection still excludes tool
parameters and tool-result bodies while preserving assistant text, tool
lifecycle, and usage. The provider's internal cap and context-pressure
collapsing affect what the model receives; they are not a promise that every
tool response remains available to it.

## Remaining gate

Qualifying the current stable requires an independently reviewable way to
prevent exit_plan_mode from changing a noninteractive Plan run to YOLO without
weakening provider safety, disabling selected behavior, changing the consumer
contract, or adding an unapproved operation/lifecycle. This task has no
authority to choose among those changes. Keep the headless claim unchanged
until a separate ruling and proof settle the boundary.

Official sources: [npm package versions](https://www.npmjs.com/package/%40google/gemini-cli?activeTab=versions),
[GitHub v0.63.0](https://github.com/google-gemini/gemini-cli/releases/tag/v0.63.0).
