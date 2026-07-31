# 2026-07-31 OpenCode Attached Live Probe Truth

## Changed

- reconciled current local and upstream harness versions after Claude range
  acceptance
- recorded Gemini `0.53.1` as deferred evidence without reopening its lane
- confirmed no new Nucleus or Soundcheck regression was recorded
- replaced OpenCode's obsolete exact `1.14.48` live-probe assertion with its
  public compatibility claim
- added deterministic qualified, unverified-newer, incompatible, malformed,
  and unhealthy health-response coverage
- retained the separately gated network, health, OpenAPI, and selected-path
  checks

## Evidence

The feature-gated OpenCode target passed four deterministic tests with its
network test ignored. Focused OpenCode validation passed 82 tests. Docs,
Northstar, and diff-hygiene checks passed.

No provider prompt, authentication, catalogue, session, server start,
workspace mutation, consumer edit, installation, or publication ran.

## Next

Run the planned g03 prepared-facade usability reassessment against current
Nucleus and Soundcheck adoption evidence. Compile implementation only for
portable friction proven by both the facade contract and consumer use.

