# Research 436: Copilot ACP Initialize Version Diagnostic

Task 170 traced the retained Copilot `1.0.93` artifacts without executing
them. It recovered the native launcher's package-selection and update-control
path, but not the ACP implementation that constructs `agentInfo.version`.
Research 435's exact mismatching value remains unknown because its record
intentionally discarded it. This task adds bounded version diagnostics and a
disabled corrected-attempt proposal; it does not authorize another original
invocation or change qualification.

## Frozen identity and static findings

The exact `1.0.93` native archive SHA-256 is
`f254651a3195e125b91d723c800e71e6541f8db3832d269854ae982254263eeb`; its
Darwin ARM64 executable SHA-256 is
`df347f272793e735629a91eea0285a736aeeb38821dd2b056234f7f47b58aef1`. The
wrapper archive is
`a8e704fb6874364af1b268aed2170bb597e0ca8086f3182b8fe5cb86ca3e43e1`; its
`package/npm-loader.js` hash is
`0ea824a86be5757533fdb092eff7050871bd7a711a46babde0ffe0e44ac5ad88`. These
identities match the retained artifact inventory.

The wrapper loader resolves the native executable and calls `spawnSync` with
the original arguments and inherited stdio. It does not inject a version or
change the environment. Static reading of the exact native executable's
embedded launcher JavaScript establishes these launcher behaviors:

- `COPILOT_AUTO_UPDATE=false` disables the update-selection path; the check is
  case-insensitive. `--no-auto-update` and `--prefer-version` also disable
  that path.
- The update path scans package-cache locations and can select a cached
  package newer than the running binary. A fresh `COPILOT_PKG_CACHE_HOME`
  alone does not prevent scanning the other locations while that path is on.
- `COPILOT_CLI_VERSION` and `COPILOT_CLI_DIST_DIR` are separate launcher
  overrides. The corrected proposal removes these two inherited entries.
- With updates disabled, the launcher selects the package matching its own
  `1.0.93` binary. If that package is absent from the selected cache, it runs
  the same executable in `--binary-version` extraction-only mode with
  `COPILOT_SEA_EXTRACT_ONLY=1` and the package-cache location, then loads the
  extracted `1.0.93` package. This is package extraction, not a re-exec into a
  newer binary.

The launcher findings explain how package selection can be pinned to the
binary's exact version for one process. They do not identify the `agentInfo`
value returned by Research 435 or prove that package selection caused its
mismatch.

## Comparison and finite gap

Research 404 initialized the same executable with `copilot --acp --stdio`
through the wrapper under synthetic isolated paths and credentials. Research
435 launched the native executable directly with `--model auto --acp --stdio`
and the inherited normal-host environment. The artifact identity matches;
the argument, wrapper, and environment tuples differ. These observations do
not isolate a cause.

The native executable's readable launcher does not expose the ACP handler
that constructs `agentInfo.version`. The packaged ACP implementation is not
recoverable as source from the retained artifact. The exact 435 version,
version-construction rule, and whether that value came from an updated package
remain unknown. No further speculative static search is required to state
this gap. The next observation that can resolve the public identity is one
separately authorized corrected initialize, retaining only a valid bounded
SemVer; an incompatible value must stop before `session/new` or prompt.

## Future diagnostics and corrected attempt

New fake and original-shaped execution records use schema v2 and may retain
`agent_version_reported` only when it is a strict SemVer no longer than 64
characters and contains no recognized secret marker. Missing versions are
classified `unknown`; malformed, non-string, overlong, path-like, arbitrary,
or secret-like values are classified `invalid` and are not retained. A valid
nonmatching version is retained and classified `mismatch`. Record validation
checks that the safe value and classification agree. The v1 reader remains
available for historical records; Research 435 and its consumed ledgers are
unchanged.

The corrected proposal binds the same frozen artifacts and direct-native
`--model auto --acp --stdio` launch. Its only launcher-policy delta is
process-scoped `COPILOT_AUTO_UPDATE=false`, a fresh private package cache, and
removal of the two version/distribution overrides. It preserves host login,
`HOME`, `COPILOT_HOME`, model `Auto`, and other inherited environment entries.
The fake-tested runner applies that environment only to the child, removes the
temporary package cache after bounded process cleanup, and rejects every
reported identity except exactly `1.0.93` before session creation. The
proposal binds one invocation, one sentinel prompt, cancellation only, no
resend or retry, 60 seconds inclusive of cleanup, and new exclusive Task 170
record paths. It is disabled and requires separate original authority. The
consumed Research 435 authority is bound to the old runner and refuses reuse.

This is not a route qualification or a change to the `1.0.80` claim. A
corrected initialize is still an original provider process and remains outside
this task's authority.

## Retained implementation

- [Corrected attempt proposal](../../crates/swallowtail-adapter-copilot-cli/tests/fixtures/copilot-cli-acp-host-permission-proof/corrected-attempt-proposal.json), SHA-256 `1e42dcf2188bedeeb8bd5b0e0505b4303c864a0063b2e1f47deb94c1b951717e`.
- Proposal schema SHA-256 `bb0d15d1251c385f68b0365180595a9c4944054d13fed958e96375819a13729c`.
- Runner SHA-256 `838beb490b903aafb46580e4ccde74d614a3f2bd326a754755548431ecf4a316`.
- The retained Research 435 attempt and observation hashes remain
  `30f16a9f2d6b0e168b0486cacb71c0f96a719bc3f7f13687807a81c11731929c` and
  `982d716e615daaa7d92612c74ae84ac13ab62c26fb19005841eb1ec09ea6ac94`.
