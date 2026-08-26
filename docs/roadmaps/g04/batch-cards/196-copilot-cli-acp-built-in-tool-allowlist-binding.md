# 196 Copilot CLI ACP Built-In Tool Allowlist Binding

Status: blocked; Research 218 empty deliver-now set
Owner: Tom
Created: 2026-08-26
Milestone: [g04.071 Copilot CLI ACP Built-In Tool Allowlist](../071-copilot-cli-acp-built-in-tool-allowlist.md)
Depends on: card 195; promoted Research 218 with a non-empty deliver-now set

## Goal

Bind only Research 218's exact Copilot CLI built-in tool-allowlist rows through
a closed adapter-local selection, immutable prepared evidence, fail-closed
driver validation, and canonical startup argv.

## Scope

1. Add only the closed adapter-local profile or typed frozen identifier shape
   selected by Research 218. Expose no raw strings, generic provider-tool API,
   or `--excluded-tools` choice.
2. Bind the selection through session preparation input, immutable plan and
   evidence, prepared request, driver state, and exact child arguments.
3. Validate exact package, profile membership, tool identifiers, request/plan/
   evidence/driver agreement, and replacement state before spawn.
4. Emit only Research 218's canonical `--available-tools` syntax and ordering.
   Omission must preserve exact prior argv and public behavior.
5. Keep one immutable selection across initialize, session creation, every
   prompt, and fresh context-losing replacement.
6. Preserve existing permission observe-and-stop behavior. Never add yolo,
   allow-all, persistent permission, one-shot approval, or callback exchange.
7. Keep the route's `AmbientHost` isolation exact. Do not infer filesystem,
   process, network, sandbox, or read-only containment from the tool filter.
8. Preserve access, activity, cancellation, deadline, terminal, failure,
   process ownership, and joined cleanup truth. Advance only an exact private
   behavior revision selected by Research 218.

## Acceptance Criteria

- [ ] only Research 218 deliver-now rows prepare
- [ ] input, plan/evidence, driver, and exact startup argv agree
- [ ] omission retains exact prior argv and route behavior
- [ ] unknown, unsupported, duplicate, empty, drifting, and ambient rows reject
      before process work
- [ ] permission, consumer tools, MCP, extensions, network, filesystem, and
      isolation claims do not widen
- [ ] existing lifecycle and cleanup behavior remains exact

## Validation

```sh
cargo fmt -p swallowtail-adapter-copilot-cli
effigy validate:focused swallowtail-adapter-copilot-cli
effigy package:verify-affected swallowtail-adapter-copilot-cli
effigy package:api
effigy qa:northstar
git diff --check
```

Auto-continue to card 197 only when exact preparation, argv, omission,
rejection, replacement, permission, and lifecycle proof passes.

## Stop Conditions

- existing prepared state cannot express the admitted exact set
- filtering or registry truth can drift after preparation without an exact
  fail-closed boundary
- implementation needs raw strings, a denylist, generic tools/permissions,
  ambient config, shared contract change, or a breaking API

## Out Of Scope

- shared closeout selection, another Copilot feature/route, live provider work,
  currentness, release, merge, rollover, or g04 closure
