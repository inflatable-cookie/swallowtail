# 2026-07-21 Qwen Headless Conformance And Closeout

## Changed

- kept the existing provider-neutral one-shot structured-CLI profile unchanged
- added a separate Contract 023 assertion pack for ambient harness authority,
  durable retention, native-budget independence, and no transcript-deletion
  claim
- ran the production Qwen driver under local and remote-authoritative host
  identities
- proved success, native turn and run budgets, provider failure, malformed
  output, disconnect, cancellation, deadline, redaction, and enforced-
  isolation rejection before process effects
- proved every started process is waited and every scoped reader task is joined
- closed card 082 and roadmap 026

## Evidence

- 15 focused Qwen tests pass: two conformance, seven driver, and six protocol
- 41 focused testkit tests pass
- focused warnings-denied clippy passes
- full `effigy qa` passes with 360 tests; three installed/live probes remain
  ignored by default
- `effigy doctor` reports only the inherited 19 findings: 12 warnings and 7
  errors
- `git diff --check` passes
- no Qwen binary, credential, provider request, external network call, paid
  inference, sandbox, or container was used

## Remaining Risks

- Qwen Code is not installed here. Installed and authenticated evidence stays
  separately gated and may require a dated corpus delta
- safe mode, read-only tool selection, and native budgets do not contain the
  provider process, descendants, filesystem, or network
- durable project-local transcripts remain provider-owned; Swallowtail claims
  no resume, enumeration, deletion, or exit-proves-deletion authority
- direct Kimi Platform, DeepSeek, Z.AI, and Alibaba/Qwen routes may share wire
  syntax while differing in access, lifecycle, catalogue, capability, and
  evidence semantics

## Continuation

Card 083 is the sole ready task. Revalidate those four direct-provider families
against current official sources, identify the smallest honest compatible-
codec seam, and select one bounded breadth proof or one explicit prerequisite.
