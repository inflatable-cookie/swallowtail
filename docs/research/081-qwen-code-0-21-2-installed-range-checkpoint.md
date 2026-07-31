# 081 Qwen Code 0.21.2 Installed Range Checkpoint

Status: promoted
Owner: Tom
Date: 2026-07-31

## Question

After Cursor and Antigravity, should Swallowtail extend standalone Claude ACP
or close the installed Qwen Code headless range first?

## Evidence Checked

- Research 074 source comparison through Qwen Code `0.21.2`
- official Qwen Code headless, authentication, model-provider, and session
  documentation
- npm metadata for `@qwen-code/qwen-code@0.21.2`
- the installed `/Users/tom/.local/bin/qwen` launcher and package metadata
- the existing `swallowtail-adapter-qwen` claim, command, fixtures, activity,
  catalogue, restarted-continuation, and prepared-facade surfaces
- the installed Claude Code and `claude-agent-acp` versions

No prompt, model inference, credential read, authentication mutation, or
workspace write was performed.

## Exact Installed Point

The host-approved `qwen` executable reports `0.21.2`. Its local package names
`@qwen-code/qwen-code` `0.21.2`; npm records git head
`456fc9b02d7ed69357dd87db8fe4bcd7e2e55ac1` and integrity
`sha512-7XgYqIs7CN2EtLi4diwT5gOy6MKmaIrglNq1+VSZZtbFwh1t2IrqMcyCBhKCskVy9xs6Xh5UEUCEC57fsjojeA==`.

The public CLI still exposes the selected machine route:

- text input
- JSON and stream-JSON output
- partial-message streaming
- explicit model selection
- `--safe-mode`
- explicit approval mode
- native wall-time, tool-call, and turn budgets
- exact `--resume <session-id>` continuation
- model catalogue control through the existing stream protocol

The existing Swallowtail command already selects safe mode, default approval,
read-only core tools, an explicit deny list, and all three native budgets. It
does not rely on ambient `--continue`, permissive approval, or implicit
sandboxing.

## Range Delta

The production claim remains exact `0.19.11`. Stable releases above it are
currently visible as unverified newer, including the installed `0.21.2`.

Research 074 found the selected stream event types and error declarations
byte-identical between `0.19.11` and `0.21.2`. Upstream changes within the
interval affect safe-mode configuration, tool registration, and catalogue
filtering. Those are invocation and capability evidence, so the range cannot
be promoted from semver or one successful version probe alone.

Card 021 must classify every stable point after `0.19.11` through `0.21.2`,
freeze the interval boundaries and any behavior milestones, and prove that the
selected read-only command and catalogue control remain exact. Prereleases,
nightlies, and experimental package variants remain outside the maintained
window.

## Authentication Currentness

Qwen OAuth is discontinued and no longer appears in the current authentication
selector. Current maintained routes include Alibaba Cloud Coding Plan,
ModelStudio keys, supported third-party providers, and explicitly configured
compatible providers.

Swallowtail must continue to delegate authentication to the installed harness.
It must not acquire, inspect, export, or relabel provider credentials. A
qualified executable version does not prove that the operator has configured a
working provider, model, entitlement, or billing route. Live catalogue and
read-only prompt proof therefore remain separately gated.

## Selection

Select Qwen Code range closure before standalone Claude ACP maintenance.

Reasons:

1. the exact Qwen target is installed at the current candidate boundary
2. the route is already production-shaped across catalogue, structured run,
   restarted continuity, activity, and prepared facade
3. closing `0.19.11..=0.21.2` provides immediate device-version coverage
4. the installed `claude-agent-acp` is `0.63.0`, while its paused roadmap also
   needs a `0.64.0` milestone and has less immediate route breadth
5. Gemini range maintenance remains paused by operator direction

The Claude `0.62.0..=0.64.0` work remains valid and paused. This selection does
not reject or supersede it.

## Contract Result

Contracts 011, 020, 023, 029, 032, 037, 039, and 043-045 already govern the
range, process, access, catalogue, activity, facade, and continuation behavior.
No shared contract or provisional spec is missing.

Compile roadmap g03.009. Keep live provider effects in a separate accepting
card, and do not make authentication success a prerequisite for deterministic
range qualification.

## Sources

- [Qwen Code 0.21.2 release](https://github.com/QwenLM/qwen-code/releases/tag/v0.21.2)
- [Qwen Code headless mode](https://qwenlm.github.io/qwen-code-docs/en/users/features/headless/)
- [Qwen Code authentication](https://qwenlm.github.io/qwen-code-docs/en/users/configuration/auth/)
- [Qwen Code model providers](https://qwenlm.github.io/qwen-code-docs/en/users/configuration/model-providers/)
- [Qwen Code commands and session management](https://qwenlm.github.io/qwen-code-docs/en/users/features/commands/)
