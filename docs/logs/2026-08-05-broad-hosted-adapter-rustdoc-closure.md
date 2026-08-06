# 2026-08-05 Broad Hosted Adapter Rustdoc Closure

## Result

`swallowtail-adapter-alibaba-model-studio`,
`swallowtail-adapter-bedrock`, and `swallowtail-adapter-openai` now enforce
crate-root denied missing docs.

The review keeps route authority explicit. Alibaba's international model
catalogue, unstored structured response, delete-on-close conversation,
preserved replaying conversation, and inactive-session deletion remain
separate operations. Bedrock's SDK-native Runtime inference and control-plane
catalogue retain separate access profiles, endpoint audiences, SDK versions,
service revisions, drivers, and prepared branches. OpenAI's Models catalogue,
temporarily retained background Responses, observe-only reconciliation, and
manual PCM Realtime session remain distinct.

No facade gains model selection, credential discovery, provider effects,
retry, fallback, or cross-route authority. Low-level drivers remain public.
The batch removes 466 warnings, reducing the workspace from 1,883 to 1,417
without suppression. Fourteen of 27 packages now enforce denied missing docs;
the remaining warnings belong to 13 installed-harness and local-runtime
adapters.

## Validation

- focused validation passed 115 tests across the three adapter packages
- warnings-denied clippy passed for all three packages
- extracted package proof passed for all three archives
- crate-root denied-missing-doc Rustdoc passed for all three packages
- the 27-package semantic API baseline remained unchanged
- workspace all-feature Rustdoc completed with 1,417 remaining warnings

No authenticated provider, consumer, tag, push, GitHub Release, or registry
effect ran.

## Next

Continue card 126 with the remaining installed-harness and local-runtime
adapters in package-sized groups.
