# Deep Agents ACP 0.1.34 identity evidence

This fixture freezes official npm `deepagents-acp` `0.1.30..=0.1.34` and each exact `deepagents` runtime pin. Artifact tarballs were downloaded to a fresh temporary directory, integrity-checked against npm SHA-512 metadata, fully inventoried, and inspected statically. No artifact was installed or executed; no provider prompt, login, credential, or live ACP session was used.

The ACP CLI and entrypoints are byte-identical across the five points. The four package hops update only `package.json` and pin exact `deepagents` releases. Runtime changes are classified in `protocol.json`; full npm package trees, per-hop file sets, hashes, and embedded-source hashes are in `dist-inventory.json`. The `0.1.31` read-result text changes within the existing ACP text-content slot. Other upstream changes are scoped to inactive defaults or model-internal behavior. At runtime `deepagents@1.14.2` also raises its `@langchain/core` peer floor from `^1.2.9` to `^1.2.14`; the host-resolved peer tree is outside this claim.

Registry metadata omits `gitHead`, so the exact official artifact tree and shipped source maps are the source authority. The ACP SDK remains a declared `^1.1.0` range; this evidence does not claim a host-specific resolved peer tree.
