use crate::rpc::{RpcConnection, failure};
use crate::session::CodexSessionHandle;
use crate::session_access::CodexSessionAccess;
use crate::session_input::CodexSessionInput;
use serde_json::Value;
use std::sync::Arc;
use swallowtail_core::{PreflightPlan, SessionRef};
use swallowtail_runtime::{
    InteractiveSessionHandle, JoinedTask, RequestId, RuntimeFailure, RuntimeSessionId,
    SessionResumeBinding,
};

pub(crate) struct PendingSessionOpen {
    request_id: RequestId,
    connection: Arc<RpcConnection>,
    task: Box<dyn JoinedTask>,
    session_input: CodexSessionInput,
    deadline_planned: bool,
    access: CodexSessionAccess,
}

impl PendingSessionOpen {
    pub(crate) fn new(
        request_id: RequestId,
        connection: Arc<RpcConnection>,
        task: Box<dyn JoinedTask>,
        session_input: CodexSessionInput,
        deadline_planned: bool,
        access: CodexSessionAccess,
    ) -> Self {
        Self {
            request_id,
            connection,
            task,
            session_input,
            deadline_planned,
            access,
        }
    }

    pub(crate) async fn finish(
        self,
        plan: &PreflightPlan,
        response: Result<Value, RuntimeFailure>,
        expected_provider_id: Option<&str>,
    ) -> Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure> {
        let response = match response {
            Ok(response) => response,
            Err(error) => {
                self.abort().await;
                return Err(error);
            }
        };
        let provider_id = response
            .get("thread")
            .and_then(|thread| thread.get("id"))
            .and_then(Value::as_str)
            .map(str::to_owned);
        let provider_id = match provider_id {
            Some(provider_id) => provider_id,
            None => {
                self.abort().await;
                return Err(malformed_response());
            }
        };
        if expected_provider_id.is_some_and(|expected| expected != provider_id) {
            self.abort().await;
            return Err(failure(
                "swallowtail.codex.app_server.resume_provider_mismatch",
                "Codex app-server resumed a different provider session",
            ));
        }
        let provider_ref = match SessionRef::new(&provider_id) {
            Ok(provider_ref) => provider_ref,
            Err(_) => {
                self.abort().await;
                return Err(malformed_response());
            }
        };
        let runtime_id =
            RuntimeSessionId::new(format!("codex-app-server:{}", self.request_id.as_str()))
                .expect("request id produces a valid session id");
        let resume_binding = resume_binding(plan, provider_ref)?;
        let runtime = self.session_input.into_runtime(
            self.deadline_planned,
            self.access.turn_sandbox_policy(),
            self.access.provider_requests(),
        );
        Ok(Box::new(CodexSessionHandle::new(
            self.request_id,
            runtime_id,
            resume_binding,
            self.connection,
            self.task,
            runtime,
            self.access,
        )))
    }

    async fn abort(self) {
        let _ = self.connection.cancel_session().await;
        let _ = self.task.join().await;
        let _ = self.access.release().await;
    }
}

fn resume_binding(
    plan: &PreflightPlan,
    provider_ref: SessionRef,
) -> Result<SessionResumeBinding, RuntimeFailure> {
    let route = plan.model_route_id().cloned().ok_or_else(|| {
        failure(
            "swallowtail.codex.app_server.model_route_missing",
            "Codex app-server session requires a preflight-bound model route",
        )
    })?;
    let model = plan.model_id().cloned().ok_or_else(|| {
        failure(
            "swallowtail.codex.app_server.model_missing",
            "Codex app-server session requires a preflight-bound model",
        )
    })?;
    Ok(SessionResumeBinding::new(
        provider_ref,
        plan.instance_id().clone(),
        plan.execution_host_id().clone(),
        route,
        model,
    ))
}

fn malformed_response() -> RuntimeFailure {
    failure(
        "swallowtail.codex.app_server.malformed_response",
        "Codex app-server returned a malformed response",
    )
}
