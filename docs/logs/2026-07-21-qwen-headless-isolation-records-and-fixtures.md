# 2026-07-21 Qwen Headless Isolation Records And Fixtures

## Decision

Select stable Qwen Code `v0.19.11` headless as the next bounded provider proof.
Remote ACP remains an Active RFD, Cursor Background Agents would establish
repository and remote-mutation policy, and `qwen serve` remains experimental.

The first Qwen route is a read-only attached structured harness run. It uses
text stdin and stream-JSON stdout with explicit safe mode, approval mode,
registry-level tool allowlisting, native bounds, durable local transcript
acceptance, and `AmbientHost`. Qwen sandboxing stays opt-in and is not required.

## Realized

- Research 017, Contract 023, roadmap 026, and cards 080-082
- optional exact `HarnessIsolation` on common operation requirements and
  runtime operation policy
- pure direct-inference rejection and request/preflight mismatch validation
  before provider effects
- fixture-only `swallowtail-adapter-qwen` crate pinned to full source commit
  `f22cf5009ee3eb26b5c5de2eca6e1f1d0ffee0ad`
- exact invocation, access, model-route, tool, isolation, retention, stream,
  budget, exit, malformed-output, unknown-event, and redaction fixtures

Safe mode, approval mode, tool restrictions, and native budgets reduce harness
behavior. None proves process containment. Process exit also does not prove
deletion of Qwen's project-scoped JSONL state.

## Validation

110 focused tests pass across core, runtime, testkit, and the Qwen fixture
crate. Focused warnings-denied clippy, workspace all-target compile, Effigy
docs QA, formatting, and diff checks pass. No Qwen binary, credential,
provider request, paid inference, or container was used. Doctor returns to the
inherited 19 oversized-file findings after splitting the touched common
records; the batch adds no structural-debt finding.

## Continuation

Card 081 is ready: implement the production driver against the frozen corpus.
Card 082 remains in bounds for local and remote-authoritative conformance and
roadmap closeout. After that, return to the direct-provider compatible-codec
checkpoint covering Kimi Platform, DeepSeek, Z.AI, and Alibaba/Qwen.
