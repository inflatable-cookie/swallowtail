# Claude SDK Shared Lifecycle Prerequisites

Status: open
Owner: Tom
Created: 2026-09-02

PR 188 proves most of the provider-free Claude Agent SDK foundation, but its
exact-head review found two shared lifecycle gaps that cannot be repaired
inside the adapter:

- `ProcessHandle::wait` reports the root exit only. Contract 019 requires
  positive evidence that every process in the host-owned descendant tree has
  exited before close completes.
- `InteractiveSessionHandle::close` accepts neither host services nor a
  caller-selected deadline. Open and turn expiry can therefore be observed,
  but cleanup and join work after expiry can still wait forever.

The first gap can be pursued additively: extend provider-neutral process-exit
evidence and teach `swallowtail-host-local` to claim tree-empty only when its
platform mechanism proves it. PR 188 remains paused and unmerged while this
work proceeds.

The second gap changes the public pre-1.0 session-close seam. The recommended
direction is to use the v0.4.0 break window and require caller-supplied host
services plus a cleanup deadline at close, without a compatibility shim. This
requires explicit operator acceptance before its card becomes ready.

Open question: accept the breaking shared close signature, or keep the SDK
route unavailable until another caller-bounded close design is chosen.
