# Prepared Runtime And Local Host Foundation

Date: 2026-07-24

## Outcome

Cards 006-007 and roadmap g02.002 are complete.

The shared prepared-integration foundation now has:

- explicit or plan-derived session agreement for access, provider state, and
  harness configuration
- safe typed preparation stages and one redacted causal failure chain
- observed or caller-asserted access provenance without status promotion
- per-host joined local scoped tasks
- inspectable local `HostServices` composition under one exact host identity
- explicit executable approval returning one opaque discovery target

No provider-specific type entered core or runtime. No global executor,
detached task, ambient target search, credential search, or raw path record was
added.

## API Classification

This is an intentional breaking change to the unreleased `0.1.0` Rust API:

- interactive open, load, and resume constructors require
  `SessionPlanAgreement`
- post-construction session policy setters are removed
- `OperationRequirements` no longer manufactures session access or provider
  state
- runtime adds preparation failures and access provenance
- host-local adds joined task and service-composition types plus exact target
  approval

There is no published Swallowtail release to preserve. The superseded
candidate baseline was refreshed in place. No alias or compatibility shim was
added.

## Runtime Evidence

- request-plan fixtures cover derived success, explicit access mismatch,
  provider-state mismatch, configuration mismatch, and missing plan state
- all production interactive drivers validate agreement before effects
- preparation fixtures cover all nine stages, causal ordering, stable
  formatting, and redaction
- local task fixtures cover explicit join, drop join, cancellation, deadline,
  panic, and safe cleanup failure
- local composition exposes only its exact service set and rejects a different
  remote-authoritative host identity
- executable approval keeps the host path private and passes one target to
  installed discovery
- a compile-checked public example shows the low-level local composition path

Removing implicit defaults exposed two useful fixture assumptions. Session
fixtures now declare prohibited provider state explicitly. A model-catalogue
role may retain an interactive driver shape without being forced to invent
session policy.

## Validation

- focused runtime, testkit, and eight-adapter matrix covers 333 tests; one
  timing-sensitive Gemini broken-pipe fixture passed on immediate isolated
  rerun and in the later full workspace run
- `swallowtail-host-local`: 27 tests pass
- core, DeepSeek, and testkit regression matrix: 123 tests pass
- `effigy check:rust` passes
- warnings-denied `effigy lint:rust` passes
- `effigy package:api` passes for all 23 crates after intentional baseline
  refresh
- `effigy test:rust` passes across the workspace; live authentication and
  installed-process probes remain gated
- `effigy doctor` remains at the inherited 19 oversized-file findings: 12
  warnings and seven errors

## Continuation

Card 008 is ready under active roadmap g02.003. It owns one Codex prepared
object that binds exact target, version assessment, access provenance, and
consistent plan inputs. Cards 009-010 remain in bounds behind it.

Consumer migrations, publication, tagging, pushing, releases, workflows, and
registry mutation remain out of scope.
