use crate::host::LocalProcessHostBuilder;
use swallowtail_core::EndpointAudience;
use swallowtail_runtime::{CredentialRef, EndpointRef};

pub(crate) struct LocalEndpointApproval {
    pub(crate) audience: EndpointAudience,
    pub(crate) value: String,
}

impl LocalEndpointApproval {
    pub(crate) fn new(audience: EndpointAudience, value: String) -> Self {
        Self { audience, value }
    }
}

pub(crate) enum LocalCredentialApproval {
    Secret {
        audience: EndpointAudience,
        bytes: Vec<u8>,
    },
    Delegated {
        audience: EndpointAudience,
    },
}

impl LocalCredentialApproval {
    pub(crate) const fn audience(&self) -> &EndpointAudience {
        match self {
            Self::Secret { audience, .. } | Self::Delegated { audience } => audience,
        }
    }
}

impl LocalProcessHostBuilder {
    #[must_use]
    pub fn approve_endpoint(
        mut self,
        reference: EndpointRef,
        audience: EndpointAudience,
        value: impl Into<String>,
    ) -> Self {
        self.approvals.endpoints.insert(
            reference,
            LocalEndpointApproval::new(audience, value.into()),
        );
        self
    }

    #[must_use]
    pub fn approve_secret_credential(
        mut self,
        reference: CredentialRef,
        audience: EndpointAudience,
        bytes: impl Into<Vec<u8>>,
    ) -> Self {
        self.approvals.credentials.insert(
            reference,
            LocalCredentialApproval::Secret {
                audience,
                bytes: bytes.into(),
            },
        );
        self
    }

    #[must_use]
    pub fn approve_delegated_credential(
        mut self,
        reference: CredentialRef,
        audience: EndpointAudience,
    ) -> Self {
        self.approvals
            .credentials
            .insert(reference, LocalCredentialApproval::Delegated { audience });
        self
    }
}
