Fix SDK session recovery when deadline-task reservation or worker creation
fails before query submission. Roll back the exact unsubmitted recipient under
the connection lock; retain all timeout drain protections. Cover both transient
refusals and genuine subsequent completion with owned regression tests.
Query commitment precedes enqueue/write and cannot be undone on send errors.
Preserve both SDK and landed ACP claims during normal main integration.
Report actual driver wait and bounded output if a prepared observer receipt is malformed; retain strict refusal.
Passed 30 refreshed original lifecycle cases across Linux and Darwin after the fix/main integration; earlier proof keeps its own scope.
Use standard macos-15 for unchanged process CI after repeated latest-runner acquisition failures.
