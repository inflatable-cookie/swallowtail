#!/bin/sh
set -eu

if [ "$#" -ne 1 ] || [ -z "$1" ]; then
    echo "usage: effigy observe:copilot-acp-private-assessment <mode-0600-binding-payload.json>" >&2
    exit 64
fi

export SWALLOWTAIL_COPILOT_ACP_BINDING_PAYLOAD="$1"
exec cargo nextest run \
    --locked \
    --profile ci \
    --test-threads 1 \
    --run-ignored ignored-only \
    -p swallowtail-adapter-copilot-cli \
    --lib \
    -E 'test(/assessment_tests::runner::invoke_reviewed_original_entry/)'
