# 231: Copilot prepared-open cleanup proof

- Settle fake preparation under its own finite budget before starting the
  measured session-open clock, so real-child discovery latency under
  parallel QA scheduling can no longer starve the intended
  `session-open-failed` / `session-open-timeout` oracle.
- Add a deterministic slow-discovery regression (`2s` fixed fixture sleep)
  proving the intended failure and joined cleanup are still reached.
- Surface the allowlisted failure stage/class/diagnostic and elapsed time
  when the run reports an unexpected status.
