# 2026-07-31 Pi RPC Installed Range Acceptance

## Changed

- selected Pi ahead of standalone Claude ACP because installed/current Pi is
  exact `0.83.0`, while installed Claude Agent ACP is `0.63.0` against target
  `0.64.0`
- froze all six stable Pi packages from `0.80.10` through `0.83.0`
- promoted five maintained behavior segments without moving baseline
  `0.80.10`
- added explicit summarization-retry drift handling under the disabled-retry
  policy
- added one bounded ignored installed-version selector
- kept direct bash, extensions, persisted load/resume, and sandboxing outside
  the route

## Current State

Pi RPC now guarantees exact published `0.80.10`, `0.81.0`, `0.81.1`,
`0.82.0`, `0.82.1`, and `0.83.0`. Unpublished gaps remain incompatible.
Later stable versions remain permitted and visible as unverified newer.

The installed `0.83.0` npm launcher classifies as qualified through the bounded
live selector. Provider authentication and a model prompt were not needed.

The live pass also exposed a separate local-host launcher gap. The installed
command uses `#!/usr/bin/env node`; environment-cleared host discovery cannot
resolve that interpreter. Research 084 owns the provider-neutral follow-up.
This does not weaken the Pi range claim, but it blocks a seamless local-host
prepared path for this installed npm shape until the host launcher boundary is
promoted and implemented.

## Validation

- 41 focused Pi tests passed with warnings denied
- extracted 85-file Pi package compiled
- installed bounded `0.83.0` selector passed
- no broad workspace suite ran
- no provider prompt, credential mutation, workspace write, or persisted
  session operation ran

## Continuation

- promote Research 084 into Contracts 010 and 032
- compile provider-neutral interpreted-launcher work as g03.011
- keep standalone Claude ACP, Gemini range maintenance, Pi load/resume, and
  Qwen live access paused
