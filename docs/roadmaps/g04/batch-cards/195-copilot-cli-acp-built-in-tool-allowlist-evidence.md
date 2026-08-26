# 195 Copilot CLI ACP Built-In Tool Allowlist Evidence

Status: complete; evidence stop
Owner: Tom
Created: 2026-08-26
Updated: 2026-08-26
Milestone: [g04.071 Copilot CLI ACP Built-In Tool Allowlist](../071-copilot-cli-acp-built-in-tool-allowlist.md)
Depends on: Research 149, Research 188; Contracts 023, 029, 033, 041

## Goal

Determine whether exact Copilot CLI `1.0.80` exposes a useful closed built-in
tool allowlist that Swallowtail can bind at ACP child startup without raw tool
strings, ambient registry inference, permission widening, or a false isolation
claim. Promote an honest empty set if any required fact remains unproved.

## Work

1. Retrieve and digest current official ACP-server, CLI command, and tool-
   permission documentation plus exact `@github/copilot@1.0.80` wrapper and
   platform package source. Record dates, identities, revisions, integrity,
   and decisive hashes in Research 218. Execute no native binary.
2. Freeze exact `--available-tools` syntax: option arity, delimiter and quoting,
   repeated flags, normalization, case, duplicates, empty input, unknown names,
   and local parser failures. Classify `--excluded-tools` precedence for
   evidence only; do not select or bind it.
3. Trace exact startup handling from parsed argv through server options,
   registry construction, tool lookup/filtering, ACP session creation, first
   and later prompts, and fresh replacement. Identify the earliest failure for
   every malformed or unsupported input.
4. Inventory exact built-in tool identifiers and aliases. Separate fixed core
   tools from model-conditioned, platform-conditioned, extension, plugin,
   skill, MCP, custom-agent, user-configured, account, and service-provided
   entries. Do not infer a stable name from documentation prose.
5. Determine whether a useful closed subset remains stable with extensions,
   MCP, user configuration, model and account state absent. Classify
   read/display/search/write/process/network effects individually; a
   read-labelled tool is not an isolation guarantee.
6. Trace permission behavior for an allowed built-in tool. Keep filter
   membership, permission request, response strength, tool invocation, effect,
   activity, and terminal outcome separate. Prove the existing route still
   observes and cancels permission requests and never selects persistent or
   one-shot approval.
7. Audit production preparation input, session profile, plan/evidence, driver,
   child command, version claim, replacement path, fixtures, guide, activity,
   cancellation, failure, and cleanup. Name the smallest safe adapter-local
   public shape; prohibit raw strings and shared provider-tool vocabulary.
8. Prove omission retains exact `copilot --acp --stdio` argv and behavior.
   Determine whether every selected profile can be validated before spawn and
   held immutable across the owned child/session and fresh replacement.
9. Classify each exact version/profile/tool row as deliver now, evidence-gated,
   intentionally withheld, or not applicable. Separate requested, dispatched,
   parser-accepted, registry-filtered, permission-requested, invoked, and
   effective truth.
10. Promote Research 218 with one exact deliver-now table or explicit empty
    set. Update the milestone/card state and reserved closeout honestly.

## Acceptance Criteria

- [x] exact official sources, package artifacts, dates, identities, integrity,
      revisions, and hashes are recorded
- [x] parser syntax, delimiter, normalization, duplicate, empty, unknown,
      precedence, omission, and failure truth is settled
- [x] fixed built-ins are separated from ambient/model/account/extension/MCP
      registry contributions
- [x] requested/dispatched/accepted/filtered/permission/invocation/effect claims
      remain distinct
- [x] production input, plan/evidence, driver, command, permission, replacement,
      activity, and lifecycle seams are audited
- [x] Research 218 contains a non-empty exact table or honest empty set
- [x] no production code, public API, shared contract/runtime, currentness,
      release, merge, rollover, or g04 closure changes
- [x] `effigy validate:focused swallowtail-adapter-copilot-cli`, `effigy
      qa:northstar`, relevant indexes, and `git diff --check` pass

## Stop Conditions

- exact package source, tool identifiers, registry membership, parser/filter
  behavior, or failures remain ambiguous
- delivery needs binary execution, login/account inspection, live prompting,
  ambient config, model inference, raw strings, generic tools/permissions,
  permission approval, or a new isolation claim

## Out Of Scope

- production binding, excluded-tools, consumer tools, MCP, extensions, custom
  agents, live access, currentness, release, merge, rollover, or g04 closure
