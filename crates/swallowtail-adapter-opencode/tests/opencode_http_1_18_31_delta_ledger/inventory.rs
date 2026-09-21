use serde_json::Value;

use super::{INVENTORY, assert_exact_object_keys, assert_exact_strings, json};

#[test]
fn artifact_tree_delta_ledger_is_mutation_sensitive() {
    let inventory = json(INVENTORY);
    assert_exact_strings(&inventory["compared"], &["1.18.30", "1.18.31"]);
    assert_eq!(inventory["npm_package_file_counts"]["1.18.30"], 4);
    assert_eq!(inventory["npm_package_file_counts"]["1.18.31"], 4);
    assert_eq!(
        inventory["implementation_source_file_counts"]["1.18.30"],
        408
    );
    assert_eq!(
        inventory["implementation_source_file_counts"]["1.18.31"],
        408
    );
    assert_eq!(inventory["repository_file_counts"]["1.18.30"], 6561);
    assert_eq!(inventory["repository_file_counts"]["1.18.31"], 6566);
    assert_eq!(inventory["repository_symlink_counts"]["1.18.30"], 60);
    assert_eq!(inventory["repository_symlink_counts"]["1.18.31"], 60);
    assert_eq!(inventory["repository_symlink_targets_identical"], true);

    assert_exact_object_keys(
        inventory["source_deltas"].as_object().unwrap(),
        &["1.18.30_to_1.18.31"],
    );
    assert_delta(
        &inventory,
        "source_deltas",
        "1.18.30_to_1.18.31",
        &[],
        &[
            "packages/opencode/src/acp/config-option.ts",
            "packages/opencode/src/acp/event.ts",
            "packages/opencode/src/acp/service.ts",
            "packages/opencode/src/cli/cmd/tui.ts",
            "packages/opencode/src/plugin/github-copilot/models.ts",
            "packages/opencode/src/server/routes/instance/httpapi/middleware/error.ts",
        ],
    );
    assert_delta(
        &inventory,
        "bundled_workspace_source_deltas",
        "1.18.30_to_1.18.31",
        &[],
        &[],
    );
    assert_exact_strings(
        &inventory["repository_deltas"]["1.18.30_to_1.18.31"]["added"],
        &[
            "packages/console/app/src/component/go-models.ts",
            "packages/console/app/src/component/limits-graph.css",
            "packages/console/app/src/routes/api/support/actions/block-workspaces.ts",
            "packages/console/app/src/routes/api/support/actions/unblock-workspaces.ts",
            "packages/stats/core/src/r2-sql.test.ts",
            "packages/ui/src/assets/icons/provider/merge-gateway.svg",
        ],
    );
    assert_exact_strings(
        &inventory["repository_deltas"]["1.18.30_to_1.18.31"]["removed"],
        &["packages/app/e2e/regression/legacy-new-session.spec.ts"],
    );
    assert_eq!(
        inventory["repository_deltas"]["1.18.30_to_1.18.31"]["changed_count"],
        101
    );
    assert_exact_strings(
        &inventory["repository_changed_files"],
        &[
            "bun.lock",
            "infra/console.ts",
            "nix/hashes.json",
            "packages/app/e2e/regression/session-timeline-projection.spec.ts",
            "packages/app/package.json",
            "packages/cli/package.json",
            "packages/codemode/package.json",
            "packages/console/app/package.json",
            "packages/console/app/src/component/limits-graph.tsx",
            "packages/console/app/src/i18n/ar.ts",
            "packages/console/app/src/i18n/br.ts",
            "packages/console/app/src/i18n/da.ts",
            "packages/console/app/src/i18n/de.ts",
            "packages/console/app/src/i18n/en.ts",
            "packages/console/app/src/i18n/es.ts",
            "packages/console/app/src/i18n/fr.ts",
            "packages/console/app/src/i18n/it.ts",
            "packages/console/app/src/i18n/ja.ts",
            "packages/console/app/src/i18n/ko.ts",
            "packages/console/app/src/i18n/no.ts",
            "packages/console/app/src/i18n/pl.ts",
            "packages/console/app/src/i18n/ru.ts",
            "packages/console/app/src/i18n/th.ts",
            "packages/console/app/src/i18n/tr.ts",
            "packages/console/app/src/i18n/uk.ts",
            "packages/console/app/src/i18n/zh.ts",
            "packages/console/app/src/i18n/zht.ts",
            "packages/console/app/src/lib/inference-proxy.ts",
            "packages/console/app/src/lib/language.ts",
            "packages/console/app/src/routes/go/index.css",
            "packages/console/app/src/routes/go/index.tsx",
            "packages/console/app/src/routes/workspace/[id]/go/lite-section.tsx",
            "packages/console/app/src/routes/zen/go/v1/models.ts",
            "packages/console/app/src/routes/zen/go/v1/usage.ts",
            "packages/console/app/src/routes/zen/util/handler.ts",
            "packages/console/app/src/routes/zen/v1/models.ts",
            "packages/console/core/package.json",
            "packages/console/core/src/workspace.ts",
            "packages/console/function/package.json",
            "packages/console/mail/package.json",
            "packages/console/support/package.json",
            "packages/core/package.json",
            "packages/desktop/package.json",
            "packages/effect-drizzle-sqlite/package.json",
            "packages/effect-sqlite-node/package.json",
            "packages/enterprise/package.json",
            "packages/function/package.json",
            "packages/http-recorder/package.json",
            "packages/llm/package.json",
            "packages/opencode/package.json",
            "packages/opencode/src/acp/config-option.ts",
            "packages/opencode/src/acp/event.ts",
            "packages/opencode/src/acp/service.ts",
            "packages/opencode/src/cli/cmd/tui.ts",
            "packages/opencode/src/plugin/github-copilot/models.ts",
            "packages/opencode/src/server/routes/instance/httpapi/middleware/error.ts",
            "packages/opencode/test/acp/config-option.test.ts",
            "packages/opencode/test/acp/event.test.ts",
            "packages/opencode/test/acp/service-session.test.ts",
            "packages/opencode/test/cli/acp/config-options.test.ts",
            "packages/opencode/test/server/httpapi-error-middleware.test.ts",
            "packages/plugin/package.json",
            "packages/sdk/js/package.json",
            "packages/server/package.json",
            "packages/session-ui/package.json",
            "packages/slack/package.json",
            "packages/stats/app/package.json",
            "packages/stats/core/package.json",
            "packages/stats/core/src/domain/inference.test.ts",
            "packages/stats/core/src/domain/inference.ts",
            "packages/stats/core/src/domain/model-normalization.ts",
            "packages/stats/core/src/r2-sql.ts",
            "packages/stats/core/src/stat-sync.ts",
            "packages/stats/server/package.json",
            "packages/tui/package.json",
            "packages/tui/src/app.tsx",
            "packages/tui/src/util/error.ts",
            "packages/tui/test/app-lifecycle.test.tsx",
            "packages/ui/package.json",
            "packages/ui/src/components/provider-icons/sprite.svg",
            "packages/ui/src/components/provider-icons/types.ts",
            "packages/web/package.json",
            "packages/web/src/content/docs/ar/go.mdx",
            "packages/web/src/content/docs/bs/go.mdx",
            "packages/web/src/content/docs/da/go.mdx",
            "packages/web/src/content/docs/de/go.mdx",
            "packages/web/src/content/docs/es/go.mdx",
            "packages/web/src/content/docs/fr/go.mdx",
            "packages/web/src/content/docs/go.mdx",
            "packages/web/src/content/docs/it/go.mdx",
            "packages/web/src/content/docs/ja/go.mdx",
            "packages/web/src/content/docs/ko/go.mdx",
            "packages/web/src/content/docs/nb/go.mdx",
            "packages/web/src/content/docs/pl/go.mdx",
            "packages/web/src/content/docs/pt-br/go.mdx",
            "packages/web/src/content/docs/ru/go.mdx",
            "packages/web/src/content/docs/th/go.mdx",
            "packages/web/src/content/docs/tr/go.mdx",
            "packages/web/src/content/docs/zh-cn/go.mdx",
            "packages/web/src/content/docs/zh-tw/go.mdx",
            "sdks/vscode/package.json",
        ],
    );

    assert_exact_strings(
        &inventory["npm_identical_through_1.18.30_to_1.18.31"],
        &["LICENSE", "bin/opencode.exe", "postinstall.mjs"],
    );
    assert_eq!(
        inventory["openapi_sha256"]["1.18.30"],
        "00502bd13e9c86f3ca9e765e99a57e06fa9f434ca16f2a714766d1444f8d37f3"
    );
    assert_eq!(
        inventory["openapi_sha256"]["1.18.31"],
        "00502bd13e9c86f3ca9e765e99a57e06fa9f434ca16f2a714766d1444f8d37f3"
    );
    assert_eq!(
        inventory["npm_invariant_hashes"]["LICENSE"],
        "625f0f619133f89bbbb2abe37369613dfa1885eba1e50d02170deb62bb42cb6b"
    );
    assert_eq!(
        inventory["npm_invariant_hashes"]["bin/opencode.exe"],
        "21c366f53283d5b5e1cdbbec2aafc98286b4c1075a4c3d5afd5ce1f5c9bf46dd"
    );
    assert_eq!(
        inventory["npm_invariant_hashes"]["postinstall.mjs"],
        "5a7c990fe552e76b16422cdba3f4b0550590c7a487f7932c773362f74317c87b"
    );
    assert_eq!(
        inventory["npm_changed_hashes"]["package.json"]["1.18.30"],
        "1db4d5007a35b9bd666df943532111f0c9fec151a15496dc24076eed550de592"
    );
    assert_eq!(
        inventory["npm_changed_hashes"]["package.json"]["1.18.31"],
        "955f7ecba06e5f152edf409f743e8c281212089f35ecd704b703aaf0fc5481b6"
    );
    assert_eq!(
        inventory["changed_source_hashes"]
            ["packages/opencode/src/server/routes/instance/httpapi/middleware/error.ts"]
            ["1.18.30"],
        "88acd9ee60e81213bf60836ac624f96168d0cc972d30e5c3c5de32041ea778f1"
    );
    assert_eq!(
        inventory["changed_source_hashes"]
            ["packages/opencode/src/server/routes/instance/httpapi/middleware/error.ts"]
            ["1.18.31"],
        "f29c2a49d43e3aabe9472d2c9c5c8f912749d7c211a39ca9ae81e91088d676a7"
    );
    assert_eq!(
        inventory["changed_source_hashes"]["packages/opencode/src/acp/service.ts"]["1.18.31"],
        "238750d52e220080db7b7f4992432fbfc7fb7ec5a15a6175460f8f0832a2c8de"
    );
    assert_eq!(
        inventory["changed_source_hashes"]["packages/opencode/src/plugin/github-copilot/models.ts"]
            ["1.18.31"],
        "1988a50c13ca51821df0806d0f50c1fe8950a58141991c24faa964446c59b992"
    );
}

fn assert_delta(inventory: &Value, group: &str, hop: &str, added: &[&str], changed: &[&str]) {
    let delta = &inventory[group][hop];
    assert_exact_strings(&delta["added"], added);
    assert_exact_strings(&delta["changed"], changed);
    assert_exact_strings(&delta["removed"], &[]);
}
