# Local Process Host Service

Status: completed
Owner: Tom
Roadmap: 006 Codex Proof Drivers
Updated: 2026-07-19

## Goal

Implement the first real Contract 010 process host behind opaque executable,
environment, and working-resource references.

## Scope

- approved executable and working-resource resolution
- bounded stdio
- graceful stop, authorized force-stop, wait, and cleanup reporting
- scoped environment injection and redaction
- deterministic process fixture executable

## Out Of Scope

- Codex protocol
- arbitrary client paths or environment
- terminal emulation
- remote process transport

## Acceptance Criteria

- only host-approved references spawn
- cancellation and deadline join child cleanup
- stderr and environment secrets remain internal
- process fixture passes the Contract 011 one-shot lifecycle subset

## Validation

- deterministic local-process integration tests
- `effigy qa`
- rustdoc warnings-as-errors
- dependency and source-boundary scans
- `git diff --check`

## Closeout

- `swallowtail-host-local` implements the runtime process port without a
  provider, consumer, async executor, or transport dependency.
- Host-owned approvals resolve opaque executable, environment, and
  working-resource references. The child environment starts empty.
- Argument, stdin, stdout, and stderr limits fail on stable safe diagnostic
  dimensions. Input and output payloads remain redacted in default formatting.
- EOF provides the portable graceful-stop request. Force-stop remains explicit
  and idempotent. Wait completes only after child exit and both output readers
  join.
- Deterministic child fixtures cover approval denial, working-resource use,
  bounded round-trip I/O, overflow, cancellation cleanup, and deadline-style
  force cleanup.
