#!/usr/bin/env bash
set -euo pipefail

release_repo_root=$(cd "$(dirname "$0")/.." && pwd)
cd "$release_repo_root"

source "$release_repo_root/scripts/release-package-set.sh"

release_candidate=${1:-.effigy/release-candidates/$release_version}
if [[ "$release_candidate" != /* ]]; then
  release_candidate="$release_repo_root/$release_candidate"
fi

test -f "$release_candidate/packages.sha256"
test -f "$release_candidate/swallowtail-$release_version-source.bundle"

release_tmp=$(mktemp -d)
trap 'rm -rf "$release_tmp"' EXIT

release_packages_root="$release_tmp/packages"
mkdir -p "$release_packages_root"
for release_package in "${release_packages[@]}"; do
  tar -xzf \
    "$release_candidate/packages/$release_package-$release_version.crate" \
    -C "$release_packages_root"
  mv \
    "$release_packages_root/$release_package-$release_version" \
    "$release_packages_root/$release_package"
done

release_candidate_source="$release_tmp/candidate-source"
git clone -q \
  "$release_candidate/swallowtail-$release_version-source.bundle" \
  "$release_candidate_source"

{
  printf '[workspace]\nresolver = "3"\nmembers = [\n'
  for release_package in "${release_packages[@]}"; do
    printf '  "%s",\n' "$release_package"
  done
  printf ']\n\n[patch.crates-io]\n'
  for release_package in "${release_internal_patch_packages[@]}"; do
    printf '%s = { path = "%s" }\n' \
      "$release_package" "$release_package"
  done
} > "$release_packages_root/Cargo.toml"

cp "$release_candidate_source/Cargo.lock" "$release_packages_root/Cargo.lock"

provider_facade_suites=(
  "swallowtail-adapter-alibaba-model-studio|prepared_facade"
  "swallowtail-adapter-anthropic|prepared_facade"
  "swallowtail-adapter-anthropic|managed_prepared_facade"
  "swallowtail-adapter-bedrock|prepared_facade"
  "swallowtail-adapter-claude-agent|claude_code_structured_run"
  "swallowtail-adapter-claude-agent|prepared_facade"
  "swallowtail-adapter-codex|prepared"
  "swallowtail-adapter-codex|prepared_profiles"
  "swallowtail-adapter-deepseek|prepared_facade"
  "swallowtail-adapter-gemini|prepared_facade"
  "swallowtail-adapter-gemini|headless_structured_run"
  "swallowtail-adapter-gemini|live_prepared_facade"
  "swallowtail-adapter-kimi|prepared_facade"
  "swallowtail-adapter-kimi|headless_structured_run"
  "swallowtail-adapter-kimi|local_server_corpus"
  "swallowtail-adapter-kimi|local_server_lifecycle"
  "swallowtail-adapter-kimi|local_server_binding_import"
  "swallowtail-adapter-kimi|local_server_interactive"
  "swallowtail-adapter-kimi|local_server_structured_run"
  "swallowtail-adapter-kimi-platform|prepared_facade"
  "swallowtail-adapter-llama-cpp|prepared_facades"
  "swallowtail-adapter-ollama|prepared_facade"
  "swallowtail-adapter-openai|prepared_facade"
  "swallowtail-adapter-openai|realtime_prepared_facade"
  "swallowtail-adapter-opencode|prepared_facade"
  "swallowtail-adapter-pi|prepared_facade"
  "swallowtail-adapter-qwen|prepared_facade"
  "swallowtail-adapter-xai|prepared_facade"
)

for provider_suite in "${provider_facade_suites[@]}"; do
  IFS='|' read -r provider_package provider_test <<< "$provider_suite"
  CARGO_TARGET_DIR="$release_tmp/target/provider-facades" \
    cargo test \
      --manifest-path "$release_packages_root/Cargo.toml" \
      --package "$provider_package" \
      --test "$provider_test" \
      --locked
done

provider_route_proofs=(
  "codex.exec|swallowtail-adapter-codex|prepared_profiles"
  "codex.app-server|swallowtail-adapter-codex|prepared_profiles"
  "claude-agent.acp|swallowtail-adapter-claude-agent|prepared_facade"
  "claude-code.headless|swallowtail-adapter-claude-agent|claude_code_structured_run"
  "gemini-cli.acp|swallowtail-adapter-gemini|prepared_facade"
  "gemini-cli.headless|swallowtail-adapter-gemini|headless_structured_run"
  "kimi-code.acp|swallowtail-adapter-kimi|prepared_facade"
  "kimi-code.headless|swallowtail-adapter-kimi|headless_structured_run"
  "kimi-code.local-server|swallowtail-adapter-kimi|local_server_interactive"
  "pi.rpc|swallowtail-adapter-pi|prepared_facade"
  "qwen.headless|swallowtail-adapter-qwen|prepared_facade"
  "opencode.http|swallowtail-adapter-opencode|prepared_facade"
  "anthropic.messages|swallowtail-adapter-anthropic|prepared_facade"
  "kimi-platform.chat|swallowtail-adapter-kimi-platform|prepared_facade"
  "deepseek.continuation|swallowtail-adapter-deepseek|prepared_facade"
  "alibaba.conversations|swallowtail-adapter-alibaba-model-studio|prepared_facade"
  "openai.background|swallowtail-adapter-openai|prepared_facade"
  "anthropic.managed-agent|swallowtail-adapter-anthropic|managed_prepared_facade"
  "xai.responses-websocket|swallowtail-adapter-xai|prepared_facade"
  "openai.realtime|swallowtail-adapter-openai|realtime_prepared_facade"
  "gemini.live|swallowtail-adapter-gemini|live_prepared_facade"
  "bedrock.runtime|swallowtail-adapter-bedrock|prepared_facade"
  "bedrock.catalogue|swallowtail-adapter-bedrock|prepared_facade"
  "ollama.attached|swallowtail-adapter-ollama|prepared_facade"
  "llama-cpp.attached|swallowtail-adapter-llama-cpp|prepared_facades"
  "llama-cpp.owned|swallowtail-adapter-llama-cpp|prepared_facades"
)

test "${#provider_route_proofs[@]}" -eq 26

release_provider_evidence="$release_tmp/provider-validation.env"
{
  printf 'format=swallowtail.provider-facade-candidate-validation.v1\n'
  printf 'version=%s\n' "$release_version"
  printf 'candidate_source_commit=%s\n' \
    "$(sed -n 's/^source_commit=//p' "$release_candidate/candidate.env")"
  printf 'candidate_packages_sha256=%s\n' \
    "$(shasum -a 256 "$release_candidate/packages.sha256" | awk '{ print $1 }')"
  printf 'suite_count=%s\n' "${#provider_facade_suites[@]}"
  printf 'route_count=%s\n' "${#provider_route_proofs[@]}"
  for provider_route_proof in "${provider_route_proofs[@]}"; do
    IFS='|' read -r provider_route provider_package provider_test \
      <<< "$provider_route_proof"
    printf 'route=%s package=%s test=%s\n' \
      "$provider_route" "$provider_package" "$provider_test"
  done
  printf 'live_credentials=absent\n'
  printf 'provider_calls=none\n'
} > "$release_provider_evidence"

if [[ -f "$release_candidate/provider-validation.env" ]]; then
  diff -u \
    "$release_candidate/provider-validation.env" \
    "$release_provider_evidence"
else
  cp "$release_provider_evidence" \
    "$release_candidate/provider-validation.env"
  (
    cd "$release_candidate"
    shasum -a 256 provider-validation.env > provider-validation.sha256
  )
fi

cat "$release_provider_evidence"
printf 'packaged provider facade proof passed for 23 production routes\n'
