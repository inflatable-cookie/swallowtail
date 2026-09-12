//! Live one-shot probe for the exact installed Grok Build `1.0.25`
//! authenticated non-inference model catalogue.
//!
//! This test never runs by default. It requires:
//!
//! - `SWALLOWTAIL_LIVE_GROK_CATALOGUE=1`
//! - an installed `grok` `1.0.25` on `PATH`
//! - an authorized Grok home. `SWALLOWTAIL_GROK_CATALOGUE_HOME`, or
//!   `GROK_HOME`, selects it. With neither set, an empty private directory is
//!   created and the delegated `~/.grok/auth.json` is copied into it when
//!   present, so the child authenticates through the provider's own store.
//!
//! The catalogue is an authenticated, prompt-free metadata command: exact
//! `--no-auto-update models` under `HarnessConfigurationPosture::Ambient`. It
//! may refresh authentication or catalogue metadata. It must not send a
//! prompt, open a model session, invoke inference or a tool, update the
//! harness, retry, or retain provider state beyond the bounded operation. The
//! probe writes the redacted capsule before any assertion and does not retry.

use futures_executor::block_on;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use swallowtail_adapter_grok::{
    GROK_BUILD_ACP_AXIS, GROK_BUILD_CATALOGUE_VERSION, GrokCatalogueListing,
    GrokCatalogueProfileInput, GrokPreparationInput, GrokPreparationProbe,
    grok_build_subscription_access_profile, prepare_grok_build,
};
use swallowtail_core::{
    AccessStatus, ConfiguredInstanceId, CredentialRef, CredentialState, EndpointAuthorization,
    EntitlementState, ExecutionHostId, InstanceRevision, InterfaceVersionAxis,
};
use swallowtail_host_local::{LocalProcessHost, LocalProcessLimits};
use swallowtail_runtime::{
    Deadline, DiscoveryCancellation, EnvironmentRef, ExecutableRef, MonotonicInstant,
    PreparedAccessEvidence, RequestId, RuntimeFailure, ScopeId,
};

const MAXIMUM_RUNTIME_NANOS: u64 = 180_000_000_000;

#[test]
#[ignore = "requires SWALLOWTAIL_LIVE_GROK_CATALOGUE=1 and an authorized Grok home"]
fn configured_grok_catalogue_is_bounded_authenticated_metadata() {
    assert_eq!(
        std::env::var("SWALLOWTAIL_LIVE_GROK_CATALOGUE").as_deref(),
        Ok("1"),
        "live Grok catalogue probe requires its explicit gate"
    );
    let started = Instant::now();
    let installed =
        std::fs::canonicalize(installed_path("grok").expect("Grok Build is installed on PATH"))
            .expect("Grok Build executable resolves exactly");
    let executable_sha256 = sha256(&installed);
    let home = AuthorizedHome::acquire();
    assert!(
        home.path().is_dir(),
        "the authorized Grok home must already exist"
    );

    let execution_host_id = ExecutionHostId::new("live.grok-catalogue.host").expect("host");
    let executable = ExecutableRef::new("live.grok-catalogue.installed").expect("executable");
    let environment = EnvironmentRef::new("live.grok-catalogue.environment").expect("environment");
    let (builder, target) = LocalProcessHost::builder(LocalProcessLimits::default())
        .approve_installed_executable(
            executable,
            InterfaceVersionAxis::new(GROK_BUILD_ACP_AXIS).expect("axis"),
            &installed,
        );
    let local = builder
        .approve_environment(
            environment.clone(),
            [(
                "GROK_HOME".into(),
                home.path().to_path_buf().into_os_string(),
            )],
        )
        .build_services(execution_host_id.clone());

    let now = local
        .services()
        .time()
        .expect("local host has time service")
        .now();
    let deadline = Deadline::at(MonotonicInstant::from_ticks(
        now.ticks().saturating_add(MAXIMUM_RUNTIME_NANOS),
    ));
    let credential = CredentialRef::new("live.grok-catalogue.delegated").expect("credential");
    let access = grok_build_subscription_access_profile(credential);
    let status = AccessStatus::new(
        access.id().clone(),
        CredentialState::Ready,
        EntitlementState::Available,
        EndpointAuthorization::Allowed,
        swallowtail_core::RuntimeReadiness::Ready,
        swallowtail_core::SupportAuthority::ProviderSupported,
    );
    let prepared = block_on(prepare_grok_build(
        GrokPreparationInput::new(
            ConfiguredInstanceId::new("live.grok-catalogue.instance").expect("instance"),
            InstanceRevision::new("1").expect("revision"),
            execution_host_id.clone(),
            target,
            environment,
            access,
            PreparedAccessEvidence::caller_asserted(status),
        ),
        GrokPreparationProbe::new(
            RequestId::new("live.grok-catalogue.probe").expect("request"),
            ScopeId::new("live.grok-catalogue.probe").expect("scope"),
            deadline,
            DiscoveryCancellation::new(),
        ),
        local.services().clone(),
    ))
    .expect("installed Grok Build discovery prepares");
    assert_eq!(
        prepared.observation().version().version().as_str(),
        GROK_BUILD_CATALOGUE_VERSION
    );
    let catalogue = prepared
        .prepare_catalogue(GrokCatalogueProfileInput::new(
            RequestId::new("live.grok-catalogue.models").expect("request"),
        ))
        .expect("the exact 1.0.25 catalogue prepares");
    assert_eq!(
        catalogue.plan().harness_configuration_posture(),
        Some(swallowtail_core::HarnessConfigurationPosture::Ambient)
    );

    // Exactly one bounded listing. Persist the redacted capsule before any
    // assertion so a failure still leaves counts, digests, and the outcome.
    let outcome = block_on(catalogue.list_models_recorded(local.services().clone()));
    let capsule_path = std::env::var_os("SWALLOWTAIL_GROK_CATALOGUE_CAPSULE")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("tests/fixtures/grok-1.0.25-model-catalogue/live-capsule.json")
        });
    write_redacted_capsule(
        &capsule_path,
        &outcome,
        &executable_sha256,
        home.path().join("auth.json").is_file(),
        started.elapsed().as_millis() as u64,
    );

    let listing = outcome.expect("one bounded catalogue listing completes");
    let model_ids: Vec<String> = listing
        .entries()
        .iter()
        .map(|model| model.id().as_str().to_owned())
        .collect();
    assert_eq!(model_ids, ["grok-4.6", "grok-4.5"]);
    assert!(listing.entries()[0].metadata().is_default());
    assert!(
        listing.entries()[1..]
            .iter()
            .all(|model| !model.metadata().is_default())
    );
    assert_eq!(listing.evidence().stderr_bytes(), 0);
}

/// Writes the redacted live capsule before any assertion.
fn write_redacted_capsule(
    path: &Path,
    outcome: &Result<GrokCatalogueListing, RuntimeFailure>,
    executable_sha256: &str,
    delegated_credential_present: bool,
    duration_ms: u64,
) {
    let mut capsule = serde_json::json!({
        "schema": "swallowtail.grok.catalogue.live-capsule.v3",
        "task": "g05.053",
        "boundary": "authenticated non-inference metadata operation",
        "cli": {
            "version": GROK_BUILD_CATALOGUE_VERSION,
            "executable_sha256": executable_sha256,
            "executable_path": "<host-private>",
        },
        "argv": ["--no-auto-update", "models"],
        "configuration_posture": "Ambient",
        "isolated_home": "<host-private>",
        "delegated_credential_present": delegated_credential_present,
        "prompt_sent": false,
        "session_opened": false,
        "inference_performed": false,
        "tool_dispatched": false,
        "stdin_closed": true,
        "process_joined": true,
        "duration_ms": duration_ms,
        "notes": [
            "live listing owns membership, order, and default",
            "the frozen exact-1.0.25 embedded document supplements matching ids only",
            "authentication refresh and bounded catalogue metadata activity are permitted",
            "no retry was performed",
        ],
    });
    match outcome {
        Ok(listing) => {
            capsule["outcome"] = serde_json::json!("success");
            capsule["stdout_bytes"] = serde_json::json!(listing.evidence().stdout_bytes());
            capsule["stderr_bytes"] = serde_json::json!(listing.evidence().stderr_bytes());
            capsule["stdout_sha256"] = serde_json::json!(listing.evidence().stdout_sha256());
            capsule["stderr_sha256"] = serde_json::json!(listing.evidence().stderr_sha256());
            capsule["model_ids"] = serde_json::json!(
                listing
                    .entries()
                    .iter()
                    .map(|model| model.id().as_str().to_owned())
                    .collect::<Vec<_>>()
            );
            capsule["default_model"] = serde_json::json!(
                listing
                    .entries()
                    .iter()
                    .find(|model| model.metadata().is_default())
                    .map(|model| model.id().as_str().to_owned())
            );
        }
        Err(error) => {
            capsule["outcome"] = serde_json::json!("failure");
            capsule["failure"] = serde_json::json!({
                "code": error.diagnostic().code(),
                "message": error.diagnostic().message(),
            });
        }
    }
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("capsule directory");
    }
    std::fs::write(
        path,
        format!(
            "{}\n",
            serde_json::to_string_pretty(&capsule).expect("capsule serializes")
        ),
    )
    .expect("capsule is written");
}

/// Authorized home that cleans up any directory it created.
struct AuthorizedHome {
    path: PathBuf,
    owned: Option<PathBuf>,
}

impl AuthorizedHome {
    fn acquire() -> Self {
        for variable in ["SWALLOWTAIL_GROK_CATALOGUE_HOME", "GROK_HOME"] {
            if let Some(value) = std::env::var_os(variable) {
                return Self {
                    path: PathBuf::from(value),
                    owned: None,
                };
            }
        }
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time follows epoch")
            .as_nanos();
        let root = std::env::temp_dir().join(format!(
            "swallowtail-grok-catalogue-live-{}-{nanos}",
            std::process::id()
        ));
        std::fs::create_dir_all(&root).expect("authorized Grok home is created");
        if let Some(host_home) = std::env::var_os("HOME") {
            let delegated = PathBuf::from(host_home).join(".grok/auth.json");
            if delegated.is_file() {
                let copied = root.join("auth.json");
                std::fs::copy(&delegated, &copied).expect("delegated auth is copied privately");
                let mut permissions = std::fs::metadata(&copied)
                    .expect("copied auth metadata")
                    .permissions();
                std::os::unix::fs::PermissionsExt::set_mode(&mut permissions, 0o600);
                std::fs::set_permissions(&copied, permissions).expect("copied auth is private");
            }
        }
        Self {
            path: root.clone(),
            owned: Some(root),
        }
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for AuthorizedHome {
    fn drop(&mut self) {
        if let Some(root) = self.owned.take() {
            let _ = std::fs::remove_dir_all(root);
        }
    }
}

fn sha256(path: &Path) -> String {
    let output = Command::new("shasum")
        .args(["-a", "256"])
        .arg(path)
        .output()
        .expect("shasum is available");
    assert!(output.status.success(), "installed executable digests");
    String::from_utf8(output.stdout)
        .expect("shasum emits utf-8")
        .split_whitespace()
        .next()
        .expect("shasum emits a digest")
        .to_owned()
}

fn installed_path(command: &str) -> Option<PathBuf> {
    std::env::var_os("PATH")
        .into_iter()
        .flat_map(|path| std::env::split_paths(&path).collect::<Vec<_>>())
        .map(|directory| directory.join(command))
        .find(|candidate| candidate.is_file())
}
