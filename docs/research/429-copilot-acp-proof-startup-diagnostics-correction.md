# Research 429: Copilot ACP Proof Startup-Diagnostics Correction

Date: 2026-10-09

## Scope

Correct the fake-only permission proof exposed by [Research 428](./428-copilot-acp-host-login-permission-proof-stop.md).
This work neither repeats the consumed original `1.0.93` invocation nor
qualifies a route. The historical preflight and execution records remain
unchanged.

## Profile and identity

The original proof and the task-owned native fake now use the same verified
sandbox-profile generator bound to the original permission plan and
`permission-proof-correction-plan.json`. The profile has no fake-only runtime
root. It permits the exact executable and the existing `/System/Library` and
`/usr/lib` runtime map roots; denies repository reads and writes, home writes,
and home reads except one literal read-only path, `.copilot/config.json`; and
retains the original local default-deny CONNECT proxy and `securityd` service
lookup trust boundary. The fake tests a synthetic file and secret sentinel at
that exact path, plus adjacent home/repository reads and writes under the same
policy. It does not read the real home or keychain.

The sole path is grounded in current [GitHub Copilot CLI config-directory
documentation](https://docs.github.com/en/copilot/reference/copilot-cli-reference/cli-config-dir-reference),
which identifies `config.json` as managed configuration that can include
authentication. This source is not specific to frozen `1.0.93` and does not
establish that the binary reads the file at startup. The exact startup read
requirement therefore remains unknown. No wider home read set is declared.

The corrected harness identity is `scripts/copilot-acp-offline-proof.py`,
SHA-256 `5d5fb560c36e001b9e00e7d1e3d291aec1b444c284fc8a790c17a8ae43f317fe`.
The native fake is `scripts/copilot-acp-permission-fake.c`, source SHA-256
`15234a7e38f9be6b380e944a4d6d212ae65038bf535d0b0d92917bf8078404a6`, compiled
executable SHA-256
`1a591a1536f1df3cdfb792d63fb52c08e1b0d4526b969287e250ab7328a56828`. The
correction plan SHA-256 is
`42bead98fe78ba6a78f90ad95a48c24d0bc39b40071f9861bfa0e7f8f39f5559`; it binds
the original permission plan SHA-256
`6db73561b9607743a43fec5607a641a2fdb297cb289a4eb36c161d4f9d62da66`. The
fake-reviewed profile SHA-256 is
`cc231c5d9df414cceead6f4646212f47839845903870a19e58420bb0f6869d5d` for the
run's exact synthetic paths and local proxy port. The original target remains
the frozen Darwin ARM64 `1.0.93` executable SHA-256
`df347f272793e735629a91eea0285a736aeeb38821dd2b056234f7f47b58aef1`; it was
not staged or run in this correction.

## Diagnostics and fake results

The proof launcher drains stderr concurrently, retains no more than 4096 bytes
in memory, and caps its reported total-byte count at 65536 with an explicit cap
flag. Durable records contain only an allowlisted failure category, bounded
byte counts, truncation, and joined-reader status. Raw stderr is not persisted
or displayed. A launcher exit status is recorded separately from vendor
startup: only an ACP initialize response marks original startup as observed;
otherwise it remains `unknown`. The native fake's startup marker
identifies fake startup only.

The native fake exercises the same profile and synthetic metadata read, denied
adjacent reads and writes, repository access, shell execution, direct egress,
and inbound bind. It exercises the approved proxy destination, SNI mismatch,
and unlisted-destination rejection. The ACP fake cancels one permission
request, verifies no marker effect, and checks process, proxy, and stderr-reader
cleanup. Dedicated controls cover stderr full-pipe backpressure, malformed and
secret-bearing text, early exit, timeout/forced stop, and joined reader
cleanup. The generated v2 preflight binds the plan, correction plan, harness,
native source and compiled executable; its schema and validator reject unsafe
or unbound results. Both `--execute` and `--permission-proof` are checked to
refuse consumed `1.0.93` before staging or process start.

## Result and remaining gate

The repair is fake-only. The historical original launcher exited code 1 before
ACP initialize, its stderr was discarded, and original binary startup remains
unknown. Its single invocation remains consumed; no original command, auth
read, keychain read, network request, or prompt was used here. Permission
behavior and route qualification remain unchanged.

A future original attempt requires a new exact operator authority after review
of this correction. Its current plan is the exact frozen `1.0.93` executable,
the shared profile above, the one read-only `.copilot/config.json` exception,
the existing `securityd` trust boundary, and the bounded diagnostic policy.
Current documentation does not prove that this exact binary needs the file;
if a future run requires another path or wider authority, stop and obtain
version-specific accepted evidence and separate authority before changing the
profile. The consumed record and invocation allowance cannot be reused.

## Correction note — Research 431

[Research 431](./431-copilot-acp-offline-startup-trace-stop.md) shows that
Research 430's `sandbox-denial` result is text-marker presence, not a diagnosed
operation or cause. Existing fake startup evidence applies only to the fake;
group disappearance does not prove escaped-descendant cleanup. This note does
not alter Research 429's records, identities, or results.
