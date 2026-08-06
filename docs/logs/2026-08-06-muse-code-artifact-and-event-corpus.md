# Muse Code Artifact And Event Corpus

Date: 2026-08-06
Roadmap: g03.045
Card: 135

## Outcome

Froze the exact Muse Code `0.1.0-R708.1` evidence boundary before adding a Rust
package:

- mutable Bash launcher and signed versioned Mach-O payload remain distinct
- direct payload version, root help, and `exec --help` captures are exact
- the full deterministic echo stream retains session, command, run, task,
  output, and terminal correlation under normalized identities
- the authenticated Meta success is a field-minimized projection retaining
  exact `meta`, `muse-spark-1.2`, low effort, output, and terminal truth
- ten negative mutations cover malformed, oversized, reordered, foreign,
  post-terminal, and mismatched-model input
- bounded unknown payloads map only to a Muse-namespaced observation

The corpus lives under the future adapter test tree so card 136 can consume it
without relocation. No workspace package or production behavior exists yet.

## Validation

`python3 scripts/check-muse-code-corpus.py` passes five tests. The checks cover
artifact and help identity, both successful streams, strict correlation,
unknown-event authority, every negative manifest case, and private-data
hygiene.

No additional authenticated provider work ran. One local echo invocation
reconfirmed direct-payload JSONL behavior with writes, shell, web tools, and
session logging disabled.

## Next

Execute card 136. Implement exact payload discovery, compatibility, bounded
decoding, activity projection, cancellation, deadline, and joined cleanup.
