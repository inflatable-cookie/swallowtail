- Give OpenCode HTTP and Pi RPC structured runs a fresh cleanup grace so timeout
  can still be followed by session or process cleanup and lease release.
- Close open OpenCode activities as failed when an idle turn lacks usage, and
  cancel in-flight callback replies with their turn.
- Preserve Pi SDK sidecar cancellation on disconnect and bound OpenCode and Pi
  close waits for native acknowledgements.
