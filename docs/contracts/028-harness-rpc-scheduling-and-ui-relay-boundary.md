# 028 Harness RPC Scheduling And UI Relay Boundary

Status: active
Owner: Tom
Updated: 2026-07-22

## Purpose

Represent a long-lived harness RPC whose messages may start work, steer active
work, queue later work, or request user interaction without flattening them
into one prompt call or importing harness policy into consumers.

## Identity And Placement

The harness integration, RPC driver, local process, configured instance,
execution host, downstream model provider, model route, model, access profile,
credential mechanism, entitlement, metering, and support authority remain
separate.

A multi-provider harness cannot choose a provider or model for Swallowtail.
Preflight binds one exact provider and route before process or credential work.
The driver passes both identities explicitly and validates the harness state
before the first model operation. Missing, substituted, aliased, or fallback
identity fails closed.

JSONL RPC is a process transport, not an embedded SDK or ACP. Command ids,
harness session ids, model-operation ids, callback ids, provider request ids,
and runtime event sequences remain distinct.

## Message Scheduling

The common boundary distinguishes:

- **prompt** — starts model work only while the harness is idle
- **steering** — accepted during active work and scheduled after the current
  assistant tool phase but before the next model call
- **follow-up** — accepted during active work and scheduled only after current
  tool and steering work ends
- **abort** — requests native interruption of current model work

A correlated RPC response acknowledges command acceptance or rejection. It
does not prove that scheduled model work started, completed, or was persisted.
Runtime progress and terminal events provide that evidence.

The first proof permits one active model operation, maximum two completed
prompts, one pending steering message, and one pending follow-up. Overflow,
duplicate correlation, out-of-order completion, or work after terminal close
fails without silently dropping or reclassifying a message.

## UI Relay

Harness extension UI is callback transport, not consumer tool execution or
provider authorization.

Dialog requests carry one bounded correlation id, declared method, safe title
or prompt metadata, bounded options where applicable, and an optional provider
timeout. The consumer may answer, cancel, or abandon through the existing
callback exchange. A late answer cannot revive an expired request.

Display-only requests may update a title, status, widget, notification, or
editor suggestion without expecting a response. The driver emits bounded
observations and never fabricates acknowledgement.

Unknown methods, malformed bounds, duplicate ids, unrequested responses, and
semantic payload drift fail closed. Raw extension payloads, paths, options,
prompts, editor content, tool content, and provider diagnostics do not enter
stable diagnostics.

## Ambient Harness State

The first route accepts explicit `AmbientHost` execution. It claims no
filesystem, child-process, network, or credential containment.

The driver must suppress every customization source excluded by the selected
route. The first Pi subset disables ambient extensions, skills, prompt
templates, and context files. It also disables update checks, install/update
telemetry, package mutation, and automatic provider retry.

Harness-owned authentication may use one exact delegated access profile under
Contract 017. The lease exposes no token or state path. Login, logout, browser
or device flow, credential import, and auth-store mutation are separate
operator actions and excluded from ordinary session open.

Read-intent harness tools do not prove read containment. A route may enable
`read`, `grep`, `find`, and `ls` while still reporting `AmbientHost`. Write,
edit, shell, package, and custom tools remain disabled in the first subset.

## Retry, Cancellation, And Cleanup

Harness-native automatic retry is disabled. Host deadline, consumer
cancellation, native abort, process stop, and force-stop remain independent
terminal evidence.

Native abort acknowledgement does not prove that the downstream provider
stopped inference. The driver reports provider stop only when the harness
supplies exact evidence.

Close stops new commands, resolves or abandons pending callbacks, ends input,
joins stdout and stderr readers, observes or forces process exit, joins all
projection work, then releases delegated authentication and working-resource
authority. No reader, callback, retry timer, process, or credential task may
detach.

## First Pi Mapping

The first proof binds:

- `@earendil-works/pi-coding-agent@0.80.10`
- maintained `pi --mode rpc --no-session`
- strict LF-delimited JSON objects over piped stdin and stdout
- one exact operator-selected provider, model route, and model
- one explicit harness-owned delegated-auth access profile
- one exact working directory and `AmbientHost` isolation
- `read`, `grep`, `find`, and `ls`; no write, edit, or bash tool
- no extensions, skills, prompt templates, or context files
- offline startup, no version check, no install/update telemetry, and no
  automatic retry
- prompt, one steering message, one follow-up, abort, state observation, and
  correlated extension UI relay
- at most two completed prompts and one active model operation

The package point is the initial Contract 029 compatibility claim, not a
permanent single-version architecture. Later qualified Pi releases may reuse
the driver only after the same corpus and conformance boundary passes.

Session persistence, resume, fork, export, compaction, direct RPC bash, model
switching, commands, package installation, login, cloud execution, custom
tools, MCP, enforced sandboxing, and provider or model fallback are excluded.

## Conformance

Deterministic fixtures must prove:

- exact harness version, provider, model, access, host, resource, and ambient
  posture before process work
- strict LF framing, bounded records, correlation, unknown rejection, and
  stream closure
- prompt rejection while busy
- steering before the next model call and follow-up after all current work
- command acknowledgement distinct from operation completion
- dialog response, cancellation, timeout, late response, and display-only UI
  behavior
- automatic retry and excluded customization stay disabled
- read-intent tools create no containment claim
- native abort, cancellation, deadline, disconnect, process failure, and
  provider failure remain distinct
- raw RPC, credential, path, prompt, output, callback, and provider payloads
  remain redacted
- readers, callbacks, process work, resource authority, and delegated auth
  close in owner order

Default QA uses no Pi installation, provider account, credential, model
request, package installation, or external network.

## Acceptance

- prompt, steering, and follow-up remain separate scheduling classes
- RPC acknowledgement cannot masquerade as model completion
- UI relay does not execute consumer tools or grant provider permission
- a multi-provider harness never creates an implicit provider or model
- read-intent ambient execution remains visibly uncontained
- sandboxing remains an optional separate capability
- retry, cancellation, provider stop, and process cleanup remain independent
- all work joins before delegated authority is released
