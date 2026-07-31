# 082 Qwen Code 0.19.11 To 0.21.2 Range Corpus

Status: promoted
Owner: Tom
Date: 2026-07-31

## Question

Which exact stable Qwen Code releases from qualified baseline `0.19.11`
through installed `0.21.2` can share adapter behavior, and where is a private
behavior milestone required?

## Method

The comparison used official git tags, npm package metadata, current official
headless documentation, and the exact Swallowtail-selected source surfaces:

- non-interactive stream declarations and error shapes
- CLI flags used by the prepared command
- safe-mode and explicit tool selection
- native wall-time, tool-call, and turn budgets
- stream-JSON model catalogue control
- exact session-id resume
- session-start version, model, permission, and tool evidence

It classified only stable `0.19.11`, `0.19.12`, `0.20.0`, `0.20.1`, `0.21.0`,
`0.21.1`, and `0.21.2`. Nightlies, previews, release candidates, and named
experimental builds remain outside the candidate window.

No executable prompt, authentication, credential read, provider request, or
workspace write ran.

## Exact Source Identities

| Version | Commit | Selected behavior |
| --- | --- | --- |
| `0.19.11` | `f22cf5009ee3eb26b5c5de2eca6e1f1d0ffee0ad` | baseline stream and catalogue |
| `0.19.12` | `8dd575cc71601f61fdaaa2d0b2ca6b1527c5335c` | baseline behavior |
| `0.20.0` | `92fda5603e84ef62a1b29bf6faf4f6a8124a2bf7` | baseline behavior |
| `0.20.1` | `305b049100606fa093a14b5cd849bff3be16e31a` | baseline behavior |
| `0.21.0` | `5610eb405212f807a482214ddd28a259da7855d3` | catalogue filters image-only models |
| `0.21.1` | `41b4ee8373fb4aa324925e69e0515ca72959ec5b` | `0.21.0` behavior |
| `0.21.2` | `456fc9b02d7ed69357dd87db8fe4bcd7e2e55ac1` | `0.21.0` behavior |

The fixture records each npm SHA-512 integrity value separately from its git
commit.

## Findings

The selected stream type declarations use blob
`6c7eb0d366f36ba0965fcf6b2fe7c840691f7e71` at every stable point. The event
framing and declared error shapes therefore require no decoder split.

Every stable point retains all selected flags, the five read-only tools,
catalogue capability and request names, and exact `--resume` selection.
Swallowtail explicitly supplies its safe-mode, approval, allowlist, deny list,
budget, input, output, and partial-message arguments. Changes to unrelated
defaults, extra tools, background agents, and session administration do not
change the selected route.

At `0.21.0`, `get_available_models` begins excluding `imageOnly` entries before
returning coding-model records. This is a real catalogue capability milestone,
not wire drift. It needs a second private behavior revision:

- `qwen-code.headless.v0.19.11` for `0.19.11..=0.20.1`
- `qwen-code.headless.v0.21.0-catalogue-filter` for `0.21.0..=0.21.2`

No exact stable release needs exclusion. Versions below `0.19.11`, prereleases,
and non-stable named builds remain incompatible. A later valid stable release
may retain the existing visible unverified-newer attempt posture.

## Corpus

`swallowtail-adapter-qwen` now contains:

- `tests/fixtures/qwen-code-v0.19.11-v0.21.2/compatibility.json`
- `tests/fixtures/qwen-code-v0.19.11-v0.21.2/README.md`
- `tests/compatibility_corpus.rs`

The corpus asserts every stable point, exact source and package identity,
unchanged stream declarations, the catalogue milestone, selected command
surfaces, and the still-unchanged production claim.

## Promotion

Card 022 may implement one continuous maintained semantic-version window with
the two private behavior segments above. Runtime session-start validation must
compare the emitted `qwen_code_version` to the exact bound plan version, not a
library-wide pinned constant.

The shared contracts remain unchanged.
