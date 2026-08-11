# 064 Claude Code Response-Only Structured Route Disposition

Status: completed negatively
Owner: Tom
Created: 2026-08-11
Depends on: Contracts 039-040; Research 121
Vision tags: installed harness, structured output, authority boundary
Contract refs: 010, 023, 029, 033, 037, 039-040, 051-052

## Problem

Figmatic needs prompt plus inline JSON Schema to produce one structured
response through local Claude subscription access. Claude Agent ACP exposes
too much agent and filesystem behavior. The existing Claude Code headless
profile is read-only but still requires a working resource and read tools.

## Generation Runway

Advances g03's consumer-proven hardening goal by qualifying the exact upstream
boundary before implementation. The lane closes negatively because the
installed harness cannot meet the selected authority and retry contract.

## Execution Plan

- [x] card 200: capture exact `2.1.227` command, access, schema, tool, retry,
      envelope, failure, termination, and artifact evidence
- [x] card 201: apply Contracts 039-040, record the negative route and version
      disposition, keep existing route behavior unchanged, and close the lane

## Goals

- [x] distinguish tool-free text from schema-enabled execution
- [x] classify the exact structured-output enforcement behavior
- [x] prove whether zero retry and terminal structured output are guaranteed
- [x] preserve OAuth/keychain subscription access without API-key billing
- [x] leave Claude Agent ACP and current Claude Code headless behavior unchanged

## Boundaries

- no Figmatic edit or product abstraction in Swallowtail
- no prompt-based schema emulation, consumer repair, retry, or fallback
- no weakening of `claude-code.headless`
- no inferred `2.1.227` compatibility
- no version bump, tag, GitHub Release, or registry mutation

## Acceptance Criteria

- [x] exact live evidence records empty ordinary tools versus the injected
      `StructuredOutput` schema tool
- [x] retry and null-success behavior are recorded without claiming support
- [x] malformed schema, termination, cleanup, authentication, and artifact
      posture are separate evidence
- [x] route, contract, architecture, and version truth remain unchanged
- [x] the consumer gate names what exact upstream evidence can reopen the lane

## Planning Checkpoint

Return to the operator. Figmatic must not integrate Claude Code for this
boundary. Select another provider-specific route with native schema enforcement
and no model-visible tools, or explicitly revise the consumer boundary before
new Swallowtail work. Figmatic may retain `v0.3.1` for existing integrations;
there is no new Swallowtail commit or release to consume for this step.
