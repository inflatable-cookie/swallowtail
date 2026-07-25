# 2026-07-25 Anthropic Direct Prepared Facade

Status: complete

## Changed

`swallowtail-adapter-anthropic` now exposes a prepared normal path for the
provider-supported Models and Messages route:

- `prepare_anthropic_direct` validates the exact public API access and host
  binding without provider or credential effects
- `prepare_catalogue` derives a model-free catalogue plan and request
- `prepare_inference_attempt` derives one exact route-bound Messages plan and
  structured-run request
- `list_models` and `start_run` delegate to the existing low-level roles

The prepared integration retains the configured instance, opaque endpoint
target, host, access profile and provenance, public endpoint audience, dated
facade, available host services, and low-level driver escape hatch.

## Authority

The adapter fixes only provider-owned facts: `api.anthropic.com`,
`anthropic-2023-06-01`, provider-supported API-key access, external endpoint
ownership, and the safe text-only operation policy.

The consumer still supplies the credential reference and evidence, approved
endpoint target, host, model route, content, positive output bound, deadline,
and every decision to start another attempt. Catalogue evidence selects no
route and proves no entitlement. The first direct subset exposes neither tool
calls nor direct tool continuation; Swallowtail executes no tool and starts no
follow-up attempt.

## Validation

- prepared-facade fixtures: three pass across local and remote-authoritative
  host identities
- complete Anthropic adapter suite: 40 pass
- hosted-direct and locally continued direct-session conformance: 14 pass
- compile-tested example and all-target check: pass
- warnings-denied Rust lint: pass
- full repository QA: pass, including 661 deterministic tests and four gated
  live checks ignored
- Doctor: unchanged at 19 pre-existing oversized-file findings
- public-API comparison: expected additive Anthropic adapter drift joins the
  held core, runtime, testkit, Codex, and Kimi facade drift

The held `0.1.0` public-API baseline remains unchanged. Card 036 owns the
replacement candidate and new baseline after provider-wide facade acceptance.

## Next

Card 022 adds the Ollama attached native-runtime prepared facade. It must keep
installed catalogue, running inventory, inference, model artifacts, residency,
and external server ownership separate.
