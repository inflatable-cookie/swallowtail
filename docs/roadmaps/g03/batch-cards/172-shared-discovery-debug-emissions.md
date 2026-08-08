# 172 Shared Discovery Debug Emissions

Status: done
Closeout: 2026-08-08
Owner: Tom
Created: 2026-08-08
Milestone: `../056-cross-route-debug-observation-emissions.md`
Depends on: g03.055 / Contract 053

## Goal

Emit failure-path debug observations from shared installed-executable discovery
and plan-family host-service readiness so most installed routes gain coverage
without per-adapter wire work.

## Closeout

Added `failure_debug_observation` and `HostServices::emit_failure_debug`.
Installed discovery emits `HostProcess` / `InterfaceVersion` / `Cleanup` on
probe outcomes; plan-family readiness emits `Lifecycle` when required host
services are missing. Additive API baseline regenerated.

## Validation

- `effigy validate:focused swallowtail-runtime`: 167 passed
- `effigy package:verify-affected swallowtail-runtime`: passed
- `effigy package:api`: regenerated v0.3.0 candidate baseline
