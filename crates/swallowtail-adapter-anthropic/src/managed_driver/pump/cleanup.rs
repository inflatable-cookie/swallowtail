async fn cleanup_resources(
    transport: &ManagedCurlTransport,
    scope: &ScopeId,
    endpoint: &str,
    credential: &[u8],
    resources: &mut OwnedResources,
    services: &HostServices,
) -> (
    RemoteResourceDeletionOutcome,
    RemoteResourceDeletionOutcome,
    CleanupOutcome,
) {
    let session = match resources.session_id.take() {
        Some(id) => {
            delete_resource(
                transport,
                scope,
                endpoint,
                credential,
                services,
                Request::delete_session(&id),
                &id,
                OwnedRemoteResourceKind::Session,
            )
            .await
        }
        None => RemoteResourceDeletionOutcome::Unconfirmed,
    };
    let environment = match resources.environment_id.take() {
        Some(id) => {
            delete_resource(
                transport,
                scope,
                endpoint,
                credential,
                services,
                Request::delete_environment(&id),
                &id,
                OwnedRemoteResourceKind::Environment,
            )
            .await
        }
        None => RemoteResourceDeletionOutcome::Unconfirmed,
    };
    let cleanup = if session == RemoteResourceDeletionOutcome::Confirmed
        && environment == RemoteResourceDeletionOutcome::Confirmed
    {
        CleanupOutcome::Clean
    } else {
        CleanupOutcome::Degraded(swallowtail_core::SafeDiagnostic::new(
            "swallowtail.anthropic.managed.remote_deletion_unconfirmed",
            "Anthropic Managed Agents could not confirm all owned resource deletions",
        ))
    };
    (session, environment, cleanup)
}

#[allow(clippy::too_many_arguments)]
async fn delete_resource(
    transport: &ManagedCurlTransport,
    scope: &ScopeId,
    endpoint: &str,
    credential: &[u8],
    services: &HostServices,
    request: Request,
    id: &str,
    kind: OwnedRemoteResourceKind,
) -> RemoteResourceDeletionOutcome {
    let response = transport
        .request(
            scope.clone(),
            endpoint.to_owned(),
            credential.to_vec(),
            request,
            services,
            Arc::new(AtomicBool::new(false)),
        )
        .await;
    match response {
        Ok(response) if require_success(&response, "resource deletion").is_ok() => {
            crate::managed::parse_deletion(&response.body, id, kind)
                .unwrap_or(RemoteResourceDeletionOutcome::Unconfirmed)
        }
        _ => RemoteResourceDeletionOutcome::Unconfirmed,
    }
}
