# 2026-07-25 llama.cpp Prepared Facades

## Changed

- added separate attached-runtime and owned-serving preparation constructors
- added exact opaque interface evidence for b9910/f5525f7e7 attached and
  b10069/178a6c449 owned runtime subsets
- added attached catalogue and one-attempt inference bound operations
- added an owned serving selection coupling the approved GGUF artifact and
  exact model route
- added bound owned start through the unchanged serving lifecycle driver
- added public examples and llama.cpp integration guidance

## Boundary

Attached preparation has no serving-lifecycle method. It cannot stop the
external server.

Owned preparation accepts one approved executable and one exact artifact
binding. It does not acquire models, select endpoints, search for executables,
create persistent serving, or absorb Monkey ownership. The returned handle is
available only after loopback endpoint publication and exact readiness. Stop
joins the process, releases endpoint authority, then releases the artifact.

## Evidence

- focused check and Clippy pass with warnings denied
- all 32 llama.cpp adapter tests pass
- local and remote-authoritative identities pass attached and owned facade
  fixtures
- attached build drift stops before catalogue inventory
- owned start request repeats the prepared artifact binding
- readiness precedes handle return
- endpoint release precedes artifact release
- full `effigy qa` passes on the final source state
- the 23-crate public API declaration baseline passes after the expected
  additive llama.cpp hash update
- `effigy doctor` retains the pre-existing 19 oversized-file findings:
  12 warnings and 7 errors

## Continuation

Milestone g02.011 is complete. Card 034 is ready to publish the exact 22-route
matrix and compile-tested provider-wide guidance. Cards 035-036 remain in
bounds for packaged facade proof and the replacement unpublished candidate.
