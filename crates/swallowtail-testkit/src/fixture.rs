use swallowtail_core::{
    Capability, CapabilityManifest, Diagnostic, EventEnvelope, EventKind, ExtensionNamespace,
    ProviderExtension, RunRef, SafeDiagnostic, SessionRef,
};

const INTERNAL_DETAIL: &str = "fixture-secret-internal-detail";
const EXTENSION_PAYLOAD: &[u8] = b"fixture-secret-extension-payload";

/// Canonical values covering the pure Contract 003 boundary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContractKernelFixture {
    capabilities: CapabilityManifest,
    supported_capability: Capability,
    unsupported_capability: Capability,
    session_ref: SessionRef,
    run_ref: RunRef,
    diagnostic: Diagnostic,
    event_with_extension: EventEnvelope,
}

impl ContractKernelFixture {
    #[must_use]
    pub fn canonical() -> Self {
        Self {
            capabilities: CapabilityManifest::new([Capability::StructuredRun]),
            supported_capability: Capability::StructuredRun,
            unsupported_capability: Capability::Resume,
            session_ref: SessionRef::new("fixture/session/provider-owned")
                .expect("static session fixture must be valid"),
            run_ref: RunRef::new("fixture/run/provider-owned")
                .expect("static run fixture must be valid"),
            diagnostic: Diagnostic::new(SafeDiagnostic::new(
                "swallowtail.fixture_provider_failure",
                "Fixture provider failed safely",
            ))
            .with_internal_detail(INTERNAL_DETAIL),
            event_with_extension: EventEnvelope::with_extension(
                7,
                EventKind::Progress,
                ProviderExtension::new(
                    ExtensionNamespace::new("fixture.unknown/v1")
                        .expect("static extension namespace must be valid"),
                    EXTENSION_PAYLOAD.to_vec(),
                ),
            ),
        }
    }

    #[must_use]
    pub const fn capabilities(&self) -> &CapabilityManifest {
        &self.capabilities
    }

    #[must_use]
    pub const fn supported_capability(&self) -> Capability {
        self.supported_capability
    }

    #[must_use]
    pub const fn unsupported_capability(&self) -> Capability {
        self.unsupported_capability
    }

    #[must_use]
    pub const fn session_ref(&self) -> &SessionRef {
        &self.session_ref
    }

    #[must_use]
    pub const fn run_ref(&self) -> &RunRef {
        &self.run_ref
    }

    #[must_use]
    pub const fn diagnostic(&self) -> &Diagnostic {
        &self.diagnostic
    }

    #[must_use]
    pub const fn event_with_extension(&self) -> &EventEnvelope {
        &self.event_with_extension
    }
}

impl Default for ContractKernelFixture {
    fn default() -> Self {
        Self::canonical()
    }
}
