release_version=0.2.0

release_internal_patch_packages=(
  swallowtail-core
  swallowtail-host-local
  swallowtail-protocol-acp
  swallowtail-protocol-openai-chat
  swallowtail-runtime
  swallowtail-testkit
  swallowtail-transport-acp-remote
)

release_patch_args=()
for release_internal_patch_package in \
  "${release_internal_patch_packages[@]}"
do
  release_patch_args+=(
    --config
    "patch.crates-io.$release_internal_patch_package.path=\"crates/$release_internal_patch_package\""
  )
done

release_stage_1=(
  swallowtail-core
  swallowtail-protocol-acp
  swallowtail-protocol-openai-chat
)

release_stage_2=(
  swallowtail-runtime
)

release_stage_3=(
  swallowtail-host-local
  swallowtail-testkit
  swallowtail-transport-acp-remote
  swallowtail-adapter-alibaba-model-studio
  swallowtail-adapter-anthropic
  swallowtail-adapter-bedrock
  swallowtail-adapter-claude-agent
  swallowtail-adapter-codex
  swallowtail-adapter-deepseek
  swallowtail-adapter-gemini
  swallowtail-adapter-grok
  swallowtail-adapter-kimi
  swallowtail-adapter-kimi-platform
  swallowtail-adapter-llama-cpp
  swallowtail-adapter-opencode
  swallowtail-adapter-ollama
  swallowtail-adapter-openai
  swallowtail-adapter-pi
  swallowtail-adapter-qwen
  swallowtail-adapter-xai
)

release_packages=(
  "${release_stage_1[@]}"
  "${release_stage_2[@]}"
  "${release_stage_3[@]}"
)

release_consumer_packages=(
  swallowtail-adapter-codex
  swallowtail-core
  swallowtail-host-local
  swallowtail-runtime
)
