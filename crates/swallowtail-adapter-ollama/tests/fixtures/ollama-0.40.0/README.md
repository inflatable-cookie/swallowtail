# Ollama currentness corpus through 0.40.0

This corpus freezes official stable GitHub tags `v0.35.0`, `v0.35.1`, and
`v0.40.0` after the qualified `0.34.4` ceiling. The complete recursive Git
trees, archive digests, and per-hop file classifications are in
`dist-inventory.json`. The previous Research 342 and Research 350 records stay
unchanged.

`0.35.0` changes the `typical_p` comment and adds an unselected SystemOne route.
The adapter does not send that option or select that route. `0.35.1` adds
decision-model capability advertisement; the existing selected-model decoder
rejects `decision`, and this task does not map decision models.

`0.40.0` is the current official stable release. Its selected chat path starts
a background local compatibility migration for supported legacy GGUF models.
That migration can write converted model blobs and manifest-list entries in
the attached runtime's store; later `/api/tags` responses can expose a row per
child runner. This exceeds the qualified attached-runtime lifecycle and
catalogue contract, so the claim advances only through exact stable `0.35.1`
points. `0.40.0` remains `UnverifiedNewer` pending an operator ruling and an
adapter adaptation decision.

The source-date fields for the `v0.40.0` release record are preserved as
observed. No local runtime was reachable or started; no provider prompt,
inference, model download, or host mutation occurred.
