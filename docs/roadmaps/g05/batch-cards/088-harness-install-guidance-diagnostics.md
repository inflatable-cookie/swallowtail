# 088 Harness Install Guidance Diagnostics

Status: ready; PR 242 approved as written; widened to own the `Absent` classification; completes after the `v0.4.3` tag
Owner: Tom
Created: 2026-09-04
Updated: 2026-09-06
Milestone: `../029-claude-sdk-interactive-parity.md`
Depends on: Contract 029 Install Guidance amendment (promoted with this card); `DiscoveryOutcome::Absent` in `swallowtail-core`; the Claude Code, Codex, and Grok discovery modules

## Goal

When a harness executable is absent, discovery surfaces the vendor-recommended install command for Claude Code, Codex, and Grok Build through a provider-neutral install-guidance value, so consumers can show it without inventing text.

## Scope

1. Core: an additive `InstallGuidance` value on the absent discovery outcome: harness display name, vendor command string, vendor documentation URL, and the date the guidance was frozen. Text only; discovery still never installs (Contract 029).
2. Each of the three adapters freezes its guidance from the vendor's own install page (cite the URL and the date in the source comment): Claude Code, Codex CLI, Grok Build. No package-manager guessing; if the vendor offers several, the vendor's primary one.
3. Discovery tests per adapter: absent executable carries the guidance; present executable carries none; the value is stable and free of paths or tokens.
4. Route matrix column or note for install guidance; changelog `[Unreleased]`; additive baselines for core and the three adapters.

## Out Of Scope

Installing, upgrading, or shelling out; guidance for any other harness; changing discovery classification.

## Acceptance Criteria

- [ ] a missing executable classifies `Absent` on the Claude Code, Codex, and Grok drivers and carries the guidance end to end (PR 242 review finding: `Absent` was never constructed in production; runtime and adapter probes classified a missing executable as `Failed`)
- [ ] core value additive, provider-neutral, text-only
- [ ] three adapters carry vendor-sourced guidance with URL and date
- [ ] tests for absent and present per adapter
- [ ] matrix, changelog, baselines; one PR

## Validation

- `cargo fmt -p swallowtail-core -p swallowtail-adapter-claude-agent -p swallowtail-adapter-codex -p swallowtail-adapter-grok -- --check`
- `effigy validate:focused swallowtail-core swallowtail-adapter-claude-agent swallowtail-adapter-codex swallowtail-adapter-grok`
- `effigy package:verify-affected swallowtail-core swallowtail-adapter-claude-agent swallowtail-adapter-codex swallowtail-adapter-grok`
- `effigy package:api`
- `effigy qa:northstar`
- `git diff --check`

## Review Oracle

Invariant: discovery describes and never acts. Smallest counterexample: guidance that is executed, or guidance text not traceable to the vendor page.

## Stop Conditions

A vendor publishes no stable install command (record the absence; ship the other two).

## Auto-Continuation

No. Stop for exact-head review.

## Result

- implemented additive core `InstallGuidance` on absent discovery outcomes
- attached vendor-frozen guidance to Claude Code, Codex CLI, and Grok Build
  discovery; source comments and matrix entries record URL and 2026-09-06
- added absent/present coverage and present-probe assertions proving install
  text is not executed or passed to the process service
- added the route-matrix note, changelog entry, and four additive v0.4.2 API
  baselines
