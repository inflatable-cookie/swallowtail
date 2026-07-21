use super::failure::{CatalogueFailureKind, provider_failure};
use crate::binding::{BedrockCredentialProvider, BedrockRegion};
use crate::failure::failure;
use crate::sdk_config::catalogue_one_attempt_retry_config;
use aws_sdk_bedrock::config::{BehaviorVersion, Region};
use aws_sdk_bedrock::operation::list_foundation_models::{
    ListFoundationModelsError, ListFoundationModelsOutput,
};
use swallowtail_runtime::{BoxFuture, RuntimeFailure};
use tokio::sync::watch;

#[derive(Clone)]
pub(super) struct CatalogueInvocation {
    pub(super) endpoint: String,
    pub(super) region: BedrockRegion,
    pub(super) provider: BedrockCredentialProvider,
}

pub(super) trait CatalogueSdkExecutor: Send + Sync {
    fn execute(
        &self,
        invocation: CatalogueInvocation,
        cancelled: watch::Receiver<bool>,
    ) -> BoxFuture<'static, Result<ListFoundationModelsOutput, RuntimeFailure>>;
}

#[derive(Default)]
pub(super) struct AwsCatalogueSdkExecutor;

impl CatalogueSdkExecutor for AwsCatalogueSdkExecutor {
    fn execute(
        &self,
        invocation: CatalogueInvocation,
        mut cancelled: watch::Receiver<bool>,
    ) -> BoxFuture<'static, Result<ListFoundationModelsOutput, RuntimeFailure>> {
        Box::pin(async move {
            let config = aws_sdk_bedrock::Config::builder()
                .behavior_version(BehaviorVersion::latest())
                .region(Region::new(invocation.region.as_str().to_owned()))
                .endpoint_url(invocation.endpoint)
                .credentials_provider(invocation.provider.as_sdk_provider())
                .retry_config(catalogue_one_attempt_retry_config())
                .build();
            let client = aws_sdk_bedrock::Client::from_conf(config);
            let send = client.list_foundation_models().send();
            tokio::pin!(send);
            tokio::select! {
                biased;
                changed = cancelled.changed() => {
                    let _ = changed;
                    Err(failure(
                        "swallowtail.bedrock.catalogue_cancelled",
                        "Bedrock catalogue work was cancelled",
                    ))
                }
                response = &mut send => response.map_err(|error| {
                    error.as_service_error().map_or_else(
                        || failure(
                            "swallowtail.bedrock.catalogue_transport_failed",
                            "Bedrock catalogue transport failed",
                        ),
                        |service| provider_failure(classify_failure(service)),
                    )
                }),
            }
        })
    }
}

pub(super) fn classify_failure(error: &ListFoundationModelsError) -> CatalogueFailureKind {
    if error.is_access_denied_exception() {
        CatalogueFailureKind::PermissionDenied
    } else if error.is_validation_exception() {
        CatalogueFailureKind::InvalidRequest
    } else if error.is_throttling_exception() {
        CatalogueFailureKind::RateLimited
    } else if error.is_internal_server_exception() {
        CatalogueFailureKind::ProviderUnavailable
    } else {
        CatalogueFailureKind::ProviderFailed
    }
}
