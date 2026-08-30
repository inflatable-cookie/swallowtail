# 2026-08-30 g05.007 Card 020 Pre-Contact Stop

Status: complete; pre-contact evidence stop
Owner: Tom
Card: 020
Milestone: g05.007
Contracts: 044, 059, 060
Worker head: `348090bf`

## Result

No provider request was made. The card 020 authorization envelope did not hold
in the worker environment, so the single authorized Claude Code turn was never
opened. The operator's one-turn authority is unconsumed, not spent.

Three independent pre-contact gates failed. Any one is a card 020 stop
condition on its own.

## Envelope Checks

| Check | Result |
| --- | --- |
| installed Claude Code on `PATH` | present |
| `claude --version` | `2.1.251 (Claude Code)` — exact match |
| frozen native SHA-256 | **mismatch** |
| `ANTHROPIC_API_KEY` absent | absent — pass |
| source tree clean and unchanged from pushed `main` | clean — pass |

The worker host is `linux-x86_64`. Its installed native binary digest is
Research 261's official `linux-x64` `2.1.251` value
`fd5f10ff0eb58daec04900466b143ea98aab50abf208a422bc008eaec13f61f7`. The card
freezes one digest,
`625869b01e0050f260b2980fac248fd9cef9e462612bded4ec9d3d49ff8969a5`, which
Research 261 records as the official `darwin-arm64` `2.1.251` binary.

The installed binary is therefore official exact `2.1.251` for its own
platform, but the envelope's single frozen digest is platform-specific and
does not describe it. `live_watcher_probe.rs` hard-codes the `darwin-arm64`
constant and asserts equality before preparation, so a live run would abort at
that assertion. The envelope precondition fails either way. Correcting it is a
probe change, which card 020 places out of scope before contact.

## Pre-Contact Validation

| Row | Result |
| --- | --- |
| `cargo fmt -p swallowtail-runtime -p swallowtail-host-local -p swallowtail-adapter-claude-agent -p swallowtail-testkit -- --check` | pass |
| `effigy validate:focused` (same four packages) | pass, 4 packages |
| `effigy package:verify-affected` (same four packages) | **fail** |
| `effigy package:api` | **fail** |
| `cargo test -p swallowtail-adapter-claude-agent --features live-probes --test live_watcher_probe --no-run` | pass, compiles |
| `git diff --check` | pass |

`effigy probe:claude-code-watcher-live` was not run.

`package:verify-affected` fails while resolving the extracted package offline:
`chacha20 0.10.1` is now yanked upstream, and `Cargo.lock` pins that exact
version through `rand` → `quinn-proto` → `quinn` → `reqwest` →
`swallowtail-transport-acp-remote`. The workspace still builds because the
lockfile is honored in place; only fresh re-resolution fails. This is registry
drift against unchanged pushed `main`, not a worker change.

`package:api` fails because `cargo-public-api 0.52.0` is not installed in this
environment. Installing it is a setup need, which is a card 020 stop condition.

`effigy doctor` matched the inherited baseline exactly: 390 god-file findings
(341 warnings, 49 errors) and one generated-in-source warning. `effigy test
--plan` reports the broad workspace `cargo nextest run --workspace` plan, which
was not executed.

## Privacy

No prompt text, provider payload, endpoint, bearer, credential, path,
command, argument, environment, PID, or watcher output is retained. The two
digests named here are already-published official artifact values from
Research 261.

## Current State

Card 020 is stopped before contact. g05.007 is stopped. Every watcher
capability, matrix, guide, and version-range claim stays withheld. Card 011 and
g05.003 remain unchanged evidence stops. The repaired card 019 oracle remains
credential-free green and unrun against a live provider.

The one authorized provider turn is still available. Nothing in this stop
consumes it.

## Next

Orchestrator decision, not worker repair. Three findings return to planning:

- **planning-change:** the authorization envelope freezes one platform's
  native digest. Either re-authorize against the `linux-x64` digest and widen
  the probe constant to a per-platform set, or dispatch the run on a
  `darwin-arm64` host that matches the frozen value.
- **validation-gap:** `Cargo.lock` pins yanked `chacha20 0.10.1`, so
  `package:verify-affected` cannot pass on any card until the lock moves.
- **validation-gap:** `cargo-public-api 0.52.0` is absent, so `package:api`
  cannot pass in this environment.

No second authorization, probe edit, or rerun follows from this log.
