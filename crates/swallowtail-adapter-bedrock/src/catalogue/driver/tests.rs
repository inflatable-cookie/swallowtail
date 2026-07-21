use super::*;
use crate::{BedrockCredentialProvider, BedrockRegion};
use aws_credential_types::Credentials;
use futures_channel::oneshot;
use futures_executor::block_on;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessRequirement, AccessStatus, Capability, CapabilityProfile,
    CapabilityRequirement, ConfiguredInstance, ConfiguredInstanceId, CredentialState,
    EndpointAudience, EndpointAuthorization, EntitlementMetering, EntitlementState,
    ExecutionHostId, InstanceOwnership, InstancePolicyId, InstanceRevision, InstanceTargetRef,
    OperationRequirements, OperationShape, PreflightContext, ProtocolFacadeId, RuntimeReadiness,
    SupportAuthority, preflight,
};
use swallowtail_host_local::{LocalProcessHost, LocalProcessLimits};
use swallowtail_runtime::{
    BlockingWorkService, CredentialRef, CredentialService, Deadline, DeadlineObservation,
    HostServices, ModelCatalogRequest, NetworkPolicyService, RequestId, TimeService,
};

include!("tests/services.rs");
include!("tests/fake.rs");
include!("tests/fixture.rs");
include!("tests/cases.rs");
