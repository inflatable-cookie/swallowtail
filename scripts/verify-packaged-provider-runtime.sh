#!/usr/bin/env bash
set -euo pipefail

release_repo_root=$(cd "$(dirname "$0")/.." && pwd)
cd "$release_repo_root"

source "$release_repo_root/scripts/release-package-set.sh"

release_tmp=$(mktemp -d)
trap 'rm -rf "$release_tmp"' EXIT

release_candidate="$release_tmp/candidate"
SWALLOWTAIL_CANDIDATE_OUTPUT="$release_candidate" \
  bash "$release_repo_root/scripts/verify-packages-local.sh"
bash "$release_repo_root/scripts/verify-release-candidate.sh" \
  "$release_candidate"
bash "$release_repo_root/scripts/verify-candidate-provider-facades.sh" \
  "$release_candidate"
bash "$release_repo_root/scripts/verify-candidate-consumers.sh" \
  "$release_candidate"

printf 'packaged_provider_candidate_env_begin\n'
sed -n \
  '/^source_commit=/p; /^source_parent_commit=/p; /^package_count=/p' \
  "$release_candidate/candidate.env"
printf 'packages_sha256_digest=%s\n' \
  "$(shasum -a 256 "$release_candidate/packages.sha256" | awk '{ print $1 }')"
cat "$release_candidate/provider-validation.env"
printf 'provider_validation_sha256=%s\n' \
  "$(shasum -a 256 "$release_candidate/provider-validation.env" | awk '{ print $1 }')"
cat "$release_candidate/consumer-validation.env"
printf 'consumer_validation_sha256=%s\n' \
  "$(shasum -a 256 "$release_candidate/consumer-validation.env" | awk '{ print $1 }')"
printf 'packaged_provider_candidate_env_end\n'
