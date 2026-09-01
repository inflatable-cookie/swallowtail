# Kimi Code ACP 0.39.0 process-authority delta

`kimi-code.acp` does not qualify official `0.39.0` or `0.39.1`. This corpus
records why, as an authority question rather than a version-label question.

## What changed

`packages/acp-server/src/acp-terminal/acpTerminalRunner.ts` previously failed
closed twice: once when the ACP client advertised no terminal capability, and
once when the invocation was not the interactive Bash tool. At `0.39.0` both
errors are replaced by `this.local.spawn(command, args, { ...options, cwd:
options?.cwd ?? this.cwd })`.

Swallowtail always advertises `clientCapabilities.terminal: false`, so
`connection.terminalEnabled` is always false and the new local-spawn path is
the one always taken. Under a `Read` resource lease the adapter also passes
`resource_io: None` and refuses `fs/write_text_file` callbacks.

The practical delta is therefore: at `0.38.0` an ACP session under
Swallowtail's advertised capabilities could not execute a host process at all;
at `0.39.0` the agent's Bash, Grep, and Glob tools execute host processes in
the leased cwd, outside ACP terminal negotiation and outside the filesystem
callback the read-only lease governs.

## Why this is a stop, not a milestone

The containment trace in `identity.json` looked for an adapter or runtime
control that constrains that spawn and found none. The route declares
`HarnessIsolation::AmbientHost`, which is explicitly "harness inherits the
ambient host environment without an isolation claim", and Contract 015 states
that "Process ownership implies neither callback authority nor filesystem
containment." Contract 015 also holds that a client which omits terminal
capability treats any terminal request as unsupported, stopping the scope.

So there is no Swallowtail-side mediation to fall back on. Contract 029 would
require a new milestone for a capability or failure change, but a new behavior
revision would assert that the changed behavior is qualified. It is not:
wire-shape stability across `0.38.0`→`0.39.1` is real and is recorded in the
`kimi-code-0.39.1` corpus, and it is not sufficient to qualify an
authority change.

## How the rejection is encoded

Exact `0.39.0` and `0.39.1` are added to the ACP claim's exclusion set.
`InterfaceCompatibilityClaim::assess` tests exclusions before the
`AllowUnverified` newer-version path, so both points classify `Incompatible`
rather than being silently admitted as unverified-newer. No new public type
was needed and no shared type was invented.

The ACP latest-qualified boundary stays `0.38.0`.

## Scope

The headless route is unaffected: `kimi -p` bootstraps its own scope and never
constructs `AcpRuntimeProviderFactory` or `AcpProcessService`, so its agent's
process authority is unchanged across `0.38.0`→`0.39.1`. Headless v2 qualifies
`0.39.1` separately. `kimi-code.local-server` is a different family and is not
touched here.

No credential, host path, account identity, session id, or provider payload
appears in this directory. No provider prompt, authentication, live probe, or
execution of a downloaded binary was involved.
