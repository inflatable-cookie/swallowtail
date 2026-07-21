use super::*;
use crate::{BedrockCredentialProvider, BedrockRegion};
use aws_credential_types::Credentials;
use futures_channel::oneshot;
use futures_executor::block_on;
use futures_util::{SinkExt, StreamExt};
use std::num::NonZeroU64;
use std::sync::atomic::AtomicUsize;
use std::thread::{self, JoinHandle};
use std::time::Instant;
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessRequirement, AccessStatus, CapabilityProfile,
    CapabilityRequirement, ConfiguredInstance, ConfiguredInstanceId, CredentialState,
    EndpointAudience, EndpointAuthorization, EntitlementMetering, EntitlementState,
    ExecutionHostId, InstanceOwnership, InstancePolicyId, InstanceRevision, InstanceTargetRef,
    ModelId, ModelRoute, ModelRouteId, ModelRouteRevision, OperationRequirements, PreflightContext,
    ProtocolFacadeId, ProviderId, RuntimeReadiness, SupportAuthority, preflight,
};
use swallowtail_host_local::{LocalProcessHost, LocalProcessLimits};
use swallowtail_runtime::{
    BlockingWorkService, CredentialRef, CredentialService, Deadline, NetworkPolicyService,
    ScopedTaskService, TimeService,
};

include!("tests/fake.rs");
include!("tests/fixture.rs");
include!("tests/services.rs");
include!("tests/cases.rs");
