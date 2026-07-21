# 2026-07-21 Qwen Headless Driver

## Changed

- added the separately registered `swallowtail.qwen.headless` production driver
  for Qwen Code `v0.19.11`
- bound the exact executable, host-approved environment, execution host,
  working resource, delegated harness access profile, provider, model,
  deadline, durable retention, and `AmbientHost` policy before process start
- matched card 080's frozen argv exactly and transported private content only
  through stdin
- normalized bounded stream JSON into progress, output, and typed usage while
  treating unknown events as safe observations
- preserved native turn and run budgets, provider failure, malformed protocol,
  host cancellation, and host timeout as distinct terminal evidence
- force-stopped and waited for the child on cancellation, timeout, parse, or
  delivery failure; handle close joins the scoped reader task

## Evidence

- seven deterministic production-driver tests cover exact invocation, success,
  usage, unknown events, native exits 53 and 55, provider failure, malformed
  output, cancellation, timeout, redaction, pre-effect rejection, process wait,
  and task join
- all 13 Qwen adapter tests pass without a binary, credential, provider request,
  external network call, paid inference, sandbox, or container
- focused warnings-denied clippy, all-target workspace compilation, format,
  roadmap next-action, and diff checks pass
- `effigy doctor` remains at the inherited 19 findings: 12 warnings and 7
  errors, with no finding added by this batch

## Remaining Risks

- card 082 still needs provider-neutral one-shot conformance under local and
  remote-authoritative execution-host identities
- Qwen Code is not installed in this environment. Installed and authenticated
  evidence remains separately gated and may require a dated corpus delta if it
  contradicts the pinned source
- safe mode and the explicit read-only tool registry reduce provider actions;
  neither is a process, descendant, filesystem, or network containment claim
- durable project-local transcripts remain provider-owned. The driver exposes
  no resume, enumeration, deletion, or exit-proves-deletion authority

## Continuation

Card 082 is the sole ready task. Add provider-neutral one-shot structured-CLI
conformance, prove local and remote-authoritative topology plus Contract 023
isolation assertions, run full QA, and close roadmap 026.
