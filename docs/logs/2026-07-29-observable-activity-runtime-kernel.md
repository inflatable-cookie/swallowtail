# 2026-07-29 Observable Activity Runtime Kernel

## Context

Swallowtail's ordered runtime stream could not preserve provider-visible
assistant phases, reasoning summaries, plans, commands, files, tools, or
other work as stable activity. Card 119 adds the common records before any
adapter claims support.

## Changes

- Added bounded operation-local activity ids and namespaced unknown kinds.
- Added bounded opaque provider activity references in
  `swallowtail-core`.
- Added exact run or turn ownership, activity kinds, lifecycle phases,
  non-regressing status, assistant phase, disclosure strength, content
  streams, delta or replacement-snapshot semantics, and callback,
  direct-tool, or provider-request correlation.
- Added one semantic activity variant to the existing runtime event stream.
- Added ordered-buffer enforcement for identity continuity, repeated starts,
  status regression, single completion, and no post-completion observation.
- Rejected activity events that duplicate content through the legacy event
  content field.
- Kept content and opaque references out of default `Debug`, `Display`, and
  lifecycle diagnostics.

No adapter claims activity support yet. No provider effect, live
authentication, consumer edit, package publication, or release mutation
occurred.

## Validation

- `effigy format:check`
- `cargo test -p swallowtail-core` — 51 passed
- `cargo test -p swallowtail-runtime` — 78 passed
- `effigy check:rust`

The workspace check covered all targets and all existing adapters. The
activity modules were split so this batch adds no oversized-file finding.
`effigy doctor` remains at the pre-existing 111 oversized-file findings:
83 warnings and 28 errors.

## Continuation

Card 120 is ready. It adds exact activity capability constraints and immutable
prepared route profiles before any adapter projection. Card 121 remains
planned. Cards 059, 097, and 098 remain paused and in bounds.
