# 2026-08-08 v0.3.1 Local Source Candidate

Status: closed
Owner: Tom

## Decision

Prepare compatible source candidate `v0.3.1` over immutable `v0.3.0` for
Contract 053 debug observation and Contract 054 provider-session history.

## Evidence

- workspace version and path dependency requirements advanced to `0.3.1`
- changelog promoted; release notes and consumer front door updated
- `effigy release gates`: all 11 configured gates passed on the candidate tree
  (fmt, lint, lint:no-features, test, qa, docs, metadata, api, security,
  floor, source)

## Current State

Local candidate is prepared on `main`. Remote CI and annotated tag identity
remain pending operator push and execute.

## Next Move

Push the candidate commit, prove canonical CI, then tag `v0.3.1` only after
exact acceptance. Do not rewrite `v0.3.0`.
