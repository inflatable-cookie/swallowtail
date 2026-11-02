# Ollama selected llama.cpp pin corpus

This corpus closes the selected dependency and renderer gates that Research
462 recorded as the `0.40.12` stop. It freezes the two `LLAMA_CPP_VERSION`
pins that the selected GGUF text runner fetches — `b11845` at Ollama
`0.40.11` and `b11901` at `0.40.12` — resolved to immutable commits and trees
in `ggml-org/llama.cpp`, and the classified 263-file delta between them.

`llama/server/CMakeLists.txt` is byte-identical across the Ollama hop and is
frozen as `llama-server-cmake.txt`; its Git blob hash must equal
`9153c9bf019548f7015305b773a9fed2a61f3fcc`. It pins the fetch to
`https://github.com/ggml-org/llama.cpp.git` at the version-file tag and names
the source override axes. `llama/compat` (the load-time compat shim and its
hook patch) is byte-identical across the hop, so the whole selected delta is
the pin movement itself plus the independent Ollama renderer/parser changes.

`identity.json` freezes the pin identities, the pin consumer, the compat
layer, the byte-identical Go selected surface, and the host observation.
`delta-inventory.json` classifies every changed file between the pins along
the selected GGUF text path and its unselected neighbors.

The claim ceiling stays `0.40.11`; nothing here changes a claim. See
Research 466.

No local runtime was installed or started. No vendor process ran. No
provider prompt, inference, model download, or model-store write occurred.
