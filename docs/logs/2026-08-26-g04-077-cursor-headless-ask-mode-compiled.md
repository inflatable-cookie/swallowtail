# 2026-08-26 g04.077 Cursor Headless Ask Mode Compiled

Status: complete
Owner: Tom

## Changed

- reassessed the remaining per-route feature inventory after g04.076
- selected Cursor headless Ask as the strongest closed route-local behavior
  candidate
- compiled g04.077 and serial cards 213-215
- reserved Research 224
- kept delivery conditional on exact immutable Ask and read-only behavior

## Why This Lane

All four exact qualified Cursor builds expose `--mode ask` beside Plan. Current
official documentation describes Ask as read-only exploration and Q&A. The
route already owns the exact child, working-resource access, explicit model,
model parameters, configuration posture, deadline, cancellation, activity,
terminal result, retention, and cleanup needed for a bounded adapter-local
selection.

Ask is stronger than the next Claude autocompact lead. Claude's documented
`CLAUDE_CODE_AUTO_COMPACT_WINDOW` environment override can supersede the CLI
selection, while the selected execution host deliberately keeps environment
values behind an opaque approved reference. Ask has a direct exact argv seam
and no known independent environment override, subject to Research 224.

## Deliberate Exclusions

Portable `HarnessMode`, raw mode strings, Agent mode, write authority,
permissions, tools, approvals, sandboxing, model-parameter changes, live
provider work, currentness, release, merge, generation rollover, and g04
closure remain outside the lane.

## Execution Shape

One serial worker lane:

1. card 213 freezes exact evidence and promotes Research 224;
2. card 214 runs only for a non-empty exact deliver-now set;
3. card 215 proves route-local acceptance and closes the lane.

The evidence card may stop the milestone honestly. Help text and parser
acceptance do not pre-authorize implementation.

## Next

Execute g04.077 cards 213-215 serially. Stop after card 213 if Research 224 is
empty or any decision gate fires. Keep g04 open. Contract 029 currentness
remains standing.
