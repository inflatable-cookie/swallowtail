# 2026-08-06 Public API And Rustdoc Closure

## Result

Card 126 closes the first-release public Rust API review across all 27 source
packages. The supported surface is represented by 7,819 normalized semantic
API entries under pinned tooling. Documentation-only work after that baseline
did not change the inventory.

Every crate now denies missing public documentation at its root. The final
batch closes llama.cpp, Oh My Pi, and Pi without flattening unlike routes:
llama.cpp keeps attached and host-owned serving separate, while Oh My Pi and
Pi remain independently qualified installed-package RPC integrations. Their
catalogue, structured-run, interactive-session, attachment, reasoning, and
restoration limits remain explicit.

The workspace warning count fell from 5,897 to zero without crate-wide
allowances. Public documentation now fails at the defining crate instead of
reappearing as release-gate debt.

## Validation

- focused validation passed 116 tests across llama.cpp, Oh My Pi, and Pi
- warnings-denied checks passed for all three packages
- extracted package proof passed for all three archives
- the 27-package semantic API baseline remained unchanged
- all-feature workspace Rustdoc passed with missing docs denied
- all examples compiled
- broad QA passed 1,459 tests, docs and link checks, guide coverage, and route
  matrices

No authenticated provider, consumer, tag, push, GitHub Release, or registry
effect ran.

## Next

Review the frozen public surface, then execute card 127 to replace the root
chronicle with consumer-facing source-install, package-selection, support, and
release copy.
