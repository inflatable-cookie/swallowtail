# 2026-07-25 Kimi ACP Prepared Facade

Status: complete

## Changed

`swallowtail-adapter-kimi` now exposes a prepared normal path:

- `prepare_kimi` discovers and classifies one approved executable
- `KimiPreparedIntegration::prepare_session` derives one persistent-session
  plan and new-session request
- `KimiPreparedSession::open_session`, `load_session`, and `resume_session`
  bind the existing interactive role
- returned session and turn handles retain prompt and active-turn interruption

Preparation preserves the exact target, host, version, access provenance,
membership credential reference, isolated-state environment, configured
instance, facade, model route, and operation plan. Unverified-newer execution
remains visible and does not widen the maintained `0.28.1` and `0.29.0`
milestones.

## Authority

The profile derives explicit ambient harness configuration and `AmbientHost`
isolation. It does not claim containment. Provider- or host-enforced isolation
requires a separately qualified profile and cannot fall back to ambient
execution.

Model, workspace, reasoning, saved resume binding, prompt content, host
services, and access evidence remain explicit. Load returns provider replay.
Resume remains replay-free. Bounded writes and delegated credentials continue
through host services.

## Validation

- prepared-facade fixtures: four pass
- complete Kimi adapter and independent ACP corpus: 75 pass
- installed live probe: one gated and ignored
- compile-tested example and all-target check: pass
- warnings-denied Clippy: pass
- full repository QA: pass, including 658 deterministic tests and four gated
  live probes ignored
- Doctor: unchanged at 19 pre-existing oversized-file findings
- public-API comparison: expected additive Kimi adapter drift joins the already
  recorded core, runtime, testkit, and Codex facade drift

The held `0.1.0` candidate baseline remains unchanged. Card 036 owns the
replacement candidate and baseline after provider-wide facade acceptance.

## Next

Card 021 adds the Anthropic Models and Messages hosted-direct prepared facade.
