release_version=0.1.0

release_patch_args=(
  --config 'patch.crates-io.swallowtail-core.path="crates/swallowtail-core"'
  --config 'patch.crates-io.swallowtail-host-local.path="crates/swallowtail-host-local"'
  --config 'patch.crates-io.swallowtail-protocol-acp.path="crates/swallowtail-protocol-acp"'
  --config 'patch.crates-io.swallowtail-protocol-openai-chat.path="crates/swallowtail-protocol-openai-chat"'
  --config 'patch.crates-io.swallowtail-runtime.path="crates/swallowtail-runtime"'
  --config 'patch.crates-io.swallowtail-testkit.path="crates/swallowtail-testkit"'
)

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
