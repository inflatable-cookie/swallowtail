#[allow(clippy::too_many_arguments)]
async fn provision(
    transport: &ManagedCurlTransport,
    scope: &ScopeId,
    agent: &ProviderAgentBinding,
    agent_version: u64,
    model: &str,
    request: &StructuredRunRequest,
    deadline: Deadline,
    services: &HostServices,
    endpoint: &str,
    credential: &[u8],
    resources: &mut OwnedResources,
    headers: &mut Vec<BTreeMap<String, String>>,
) -> Result<(ManagedSubscription, Arc<AtomicBool>), RuntimeFailure> {
    let response = request_before_deadline(
        transport,
        scope,
        endpoint,
        credential,
        Request::agent(agent.id().as_str(), agent_version),
        deadline,
        services,
    )
    .await?;
    require_success(&response, "agent retrieval")?;
    headers.push(response.headers);
    crate::managed::validate_agent(
        &response.body,
        agent.id().as_str(),
        agent_version,
        model,
    )?;

    let response = request_before_deadline(
        transport,
        scope,
        endpoint,
        credential,
        Request::create_environment("swallowtail-managed-run"),
        deadline,
        services,
    )
    .await?;
    require_success(&response, "environment creation")?;
    headers.push(response.headers);
    resources.environment_id = Some(crate::managed::parse_environment(&response.body)?);

    let tools = request.tools().collect::<Vec<_>>();
    let response = request_before_deadline(
        transport,
        scope,
        endpoint,
        credential,
        Request::create_session(
            agent.id().as_str(),
            agent_version,
            model,
            resources.environment_id.as_deref().expect("environment exists"),
            &tools,
        )?,
        deadline,
        services,
    )
    .await?;
    require_success(&response, "session creation")?;
    headers.push(response.headers);
    resources.session_id = Some(crate::managed::parse_session_with_tools(
        &response.body,
        resources.environment_id.as_deref().expect("environment exists"),
        agent.id().as_str(),
        agent_version,
        model,
        &tools,
    )?);

    let response = request_before_deadline(
        transport,
        scope,
        endpoint,
        credential,
        Request::message(
            resources.session_id.as_deref().expect("session exists"),
            request.content(),
        ),
        deadline,
        services,
    )
    .await?;
    require_success(&response, "message submission")?;
    headers.push(response.headers);

    let connection = Arc::new(AtomicBool::new(false));
    let subscription = transport.subscribe(
        scope.clone(),
        endpoint.to_owned(),
        credential.to_vec(),
        Request::stream(resources.session_id.as_deref().expect("session exists")),
        services,
        Arc::clone(&connection),
    )?;
    Ok((subscription, connection))
}

async fn request_before_deadline(
    transport: &ManagedCurlTransport,
    scope: &ScopeId,
    endpoint: &str,
    credential: &[u8],
    request: Request,
    deadline: Deadline,
    services: &HostServices,
) -> Result<ManagedResponse, RuntimeFailure> {
    let cancelled = Arc::new(AtomicBool::new(false));
    let mut operation = Box::pin(transport.request(
        scope.clone(),
        endpoint.to_owned(),
        credential.to_vec(),
        request,
        services,
        Arc::clone(&cancelled),
    ));
    let mut timer = services.time().expect("validated time").wait_until(deadline);
    let result = poll_fn(|context| {
        if let Poll::Ready(result) = operation.as_mut().poll(context) {
            return Poll::Ready(Some(result));
        }
        if timer.as_mut().poll(context).is_ready() {
            return Poll::Ready(None);
        }
        Poll::Pending
    })
    .await;
    match result {
        Some(result) => result,
        None => {
            cancelled.store(true, Ordering::SeqCst);
            let _ = operation.await;
            Err(failure(
                "swallowtail.anthropic.managed.setup_deadline",
                "Anthropic Managed Agents setup exceeded the operation deadline",
            ))
        }
    }
}

struct ManagedSecret(Vec<u8>);

impl Drop for ManagedSecret {
    fn drop(&mut self) {
        self.0.fill(0);
    }
}
