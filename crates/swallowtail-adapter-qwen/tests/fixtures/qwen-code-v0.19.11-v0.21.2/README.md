# Qwen Code 0.19.11 To 0.21.2 Compatibility Corpus

This fixture classifies every stable npm release after the qualified baseline
through the installed `0.21.2` boundary:

- `0.19.11`, commit `f22cf5009ee3eb26b5c5de2eca6e1f1d0ffee0ad`
- `0.19.12`, commit `8dd575cc71601f61fdaaa2d0b2ca6b1527c5335c`
- `0.20.0`, commit `92fda5603e84ef62a1b29bf6faf4f6a8124a2bf7`
- `0.20.1`, commit `305b049100606fa093a14b5cd849bff3be16e31a`
- `0.21.0`, commit `5610eb405212f807a482214ddd28a259da7855d3`
- `0.21.1`, commit `41b4ee8373fb4aa324925e69e0515ca72959ec5b`
- `0.21.2`, commit `456fc9b02d7ed69357dd87db8fe4bcd7e2e55ac1`

The selected stream declarations have one blob identity across the interval.
The only selected public behavior milestone is `0.21.0`, where the catalogue
controller starts excluding image-only models from the coding-model list.

Safe-mode, tool-registry, session-store, background-agent, and diagnostic text
changed elsewhere in the source tree. Swallowtail's exact safe-mode flags,
read-only allowlist, deny list, budgets, stream parser, and exact `--resume`
selector remain valid at every stable point. Those unrelated source changes do
not create adapter behavior revisions.

Sources:

- <https://github.com/QwenLM/qwen-code/releases>
- <https://www.npmjs.com/package/@qwen-code/qwen-code>
- <https://qwenlm.github.io/qwen-code-docs/en/users/features/headless/>

The corpus contains source identities and public command invariants only. It
contains no prompt, model response, credential, provider configuration, or
host path.
