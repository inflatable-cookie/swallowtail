# 084 Installed Script Launcher Portability Gap

Status: accepted
Owner: Tom
Updated: 2026-07-31

## Problem

The installed Pi `0.83.0` command is a symlink to an npm JavaScript entrypoint
with `#!/usr/bin/env node`. Direct bounded version execution succeeds. The same
path fails through `swallowtail-host-local` installed discovery because the
local process host clears the child environment and the opaque discovery target
has no host-approved launcher recipe or launcher-only environment.

This is not Pi protocol incompatibility. It is a host execution gap for
interpreted installed harnesses.

## Boundary

Do not fix this by:

- preserving ambient `PATH` for every child
- hardcoding Node, npm layouts, or Pi paths in the Pi adapter
- treating a shell wrapper as the observed harness version
- adding an adapter-side `PATH` search or executable fallback
- passing credential or provider configuration into a version probe

Contract 032 already permits the host to resolve one opaque candidate through
an explicit selection policy before discovery. Contract 010 still models the
approved executable as one program path plus adapter arguments. Neither surface
defines a host-private interpreted launcher recipe.

## Recommended Direction

Add an explicit host-approved launch recipe behind the unchanged opaque
`ExecutableRef`:

- exact interpreter or native program
- immutable prefix arguments, such as the exact script path
- bounded launcher-only environment when required for runtime bootstrap
- adapter arguments appended after the immutable prefix
- no credential, provider configuration, working resource, or model authority
- no program, path, prefix, or environment disclosure in stable records

The local host may then provide an opt-in helper that resolves a selected
script launcher into that recipe. Selection remains host policy; adapters still
cannot search `PATH` or substitute candidates. Native executables remain the
zero-prefix case.

This direction was promoted into Contracts 010 and 032 before runtime or host
implementation. It applies to npm, Python, Ruby, JVM, and similar harnesses;
provider-neutral deterministic fixtures preceded the repeated Pi proof.

## Contract Result

Contracts 010 and 032 now permit one host-private launch recipe behind the
unchanged opaque executable reference. The recipe owns one exact program,
bounded immutable prefix arguments, and optional bounded launcher bootstrap
environment. Ambient environment remains cleared. Adapter arguments follow the
prefix, and explicit request environment follows bootstrap values.

The host implementation must first prove this shape with provider-neutral
fixtures. Pi is the live acceptance target because its installed npm launcher
reproduced the gap. No Pi-specific interpreter or package-layout rule belongs
in the adapter.

## Current Evidence

- installed command: `/opt/homebrew/bin/pi`
- selected package: `@earendil-works/pi-coding-agent@0.83.0`
- launcher: `/usr/bin/env node`
- direct bounded `pi --version`: `0.83.0`, qualified
- local-host installed discovery: safe `Failed` outcome before version parse
- no provider prompt, authentication, workspace, or model effect occurred
