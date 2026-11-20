use crate::process_close::AppServerTask;
use crate::rpc::{RpcConnection, failure};
use crate::session::CodexSessionHandle;
use crate::session_access::CodexSessionAccess;
use crate::session_input::CodexSessionInput;
use crate::session_replay::{project_thread_history, validate_thread_history_bounds};
use serde_json::Value;
use std::sync::Arc;
use swallowtail_core::{PreflightPlan, SessionRef};
use swallowtail_runtime::{
    InteractiveSessionHandle, LoadedSession, RequestId, RuntimeFailure, RuntimeSessionId,
    SessionResumeBinding,
};

pub(crate) struct PendingSessionOpen {
    request_id: RequestId,
    connection: Arc<RpcConnection>,
    task: AppServerTask,
    session_input: CodexSessionInput,
    deadline_planned: bool,
    access: CodexSessionAccess,
}

impl PendingSessionOpen {
    pub(crate) fn new(
        request_id: RequestId,
        connection: Arc<RpcConnection>,
        task: AppServerTask,
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
        if let Err(error) = validate_thread_history_bounds(&response) {
            self.abort().await;
            return Err(error);
        }
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
        let resume_binding = match resume_binding(
            plan,
            provider_ref,
            self.access.working_resource().clone(),
            self.access.policy().clone(),
        ) {
            Ok(binding) => binding,
            Err(error) => {
                // The started provider session is released before the failure
                // reaches the caller, so no process or workspace lease survives.
                self.abort().await;
                return Err(error);
            }
        };
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

    pub(crate) async fn finish_loaded(
        self,
        plan: &PreflightPlan,
        response: Result<Value, RuntimeFailure>,
        expected_provider_id: &str,
    ) -> Result<LoadedSession, RuntimeFailure> {
        let response = match response {
            Ok(response) => response,
            Err(error) => {
                self.abort().await;
                return Err(error);
            }
        };
        let session = match SessionRef::new(expected_provider_id) {
            Ok(session) => session,
            Err(_) => {
                self.abort().await;
                return Err(malformed_response());
            }
        };
        let replay = match project_thread_history(&response, &session) {
            Ok(replay) => replay,
            Err(error) => {
                self.abort().await;
                return Err(error);
            }
        };
        let handle = self
            .finish(plan, Ok(response), Some(expected_provider_id))
            .await?;
        Ok(LoadedSession::new(replay, handle))
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
    working_resource: swallowtail_runtime::WorkingResourceRef,
    access_policy: swallowtail_core::SessionAccessPolicy,
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
        working_resource,
        access_policy,
    ))
}

fn malformed_response() -> RuntimeFailure {
    failure(
        "swallowtail.codex.app_server.malformed_response",
        "Codex app-server returned a malformed response",
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::preflight_fixture::{
        app_server_session_plan_without_model_route, bounded_workspace_plan,
    };
    use crate::session_access::codex_bounded_workspace_access_policy;
    use futures_executor::block_on;
    use swallowtail_runtime::{
        ExecutableRef, ProcessRequest, ScopeId, SessionOptions, WorkingResourceRef,
    };
    use swallowtail_testkit::{RecordedHostCall, RecordingHostServices, RecordingOutcome};

    #[test]
    fn resume_binding_failure_aborts_and_releases_the_started_session() {
        block_on(async {
            let plan = bounded_workspace_plan();
            let recording = RecordingHostServices::for_host(
                plan.execution_host_id().clone(),
                RecordingOutcome::Succeed,
            );
            let services = recording.services();
            let scope = ScopeId::new("codex-open-binding-failure").unwrap();
            let resource = WorkingResourceRef::new("workspace.main").unwrap();
            let access = CodexSessionAccess::prepare(
                &plan,
                &codex_bounded_workspace_access_policy(),
                &resource,
                scope.clone(),
                services,
            )
            .await
            .expect("writable access acquires a resource lease");
            let process = services
                .process()
                .unwrap()
                .start(
                    scope.clone(),
                    ProcessRequest::new(ExecutableRef::from_instance_target(
                        plan.instance_target_ref(),
                    )),
                )
                .await
                .unwrap();
            let connection = RpcConnection::new(Arc::from(process), services.clone());
            let task =
                AppServerTask::spawn(services.task().unwrap().clone(), scope, Box::pin(async {}))
                    .unwrap();
            let input = CodexSessionInput::for_open(&plan, &SessionOptions::default(), services)
                .expect("session input prepares");

            // The public open rejects a missing model sooner. Exercise the
            // failure defensively once a process and writable lease are owned.
            let plan_without_route = app_server_session_plan_without_model_route([]);
            let pending = PendingSessionOpen::new(
                RequestId::new("open-binding-failure").unwrap(),
                connection,
                task,
                input,
                false,
                access,
            );
            let result = pending
                .finish(
                    &plan_without_route,
                    Ok(serde_json::json!({"thread": {"id": "thread-started"}})),
                    None,
                )
                .await;

            let Err(error) = result else {
                panic!("a started session without a resume binding must fail");
            };
            assert_eq!(
                error.diagnostic().code(),
                "swallowtail.codex.app_server.model_route_missing"
            );
            assert_eq!(
                recording.calls(),
                [
                    RecordedHostCall::WorkingResourceResolve,
                    RecordedHostCall::ProcessStart,
                    RecordedHostCall::TaskSpawn,
                    RecordedHostCall::ProcessForceStop,
                    RecordedHostCall::TaskJoin,
                    RecordedHostCall::WorkingResourceRelease,
                ]
            );
        });
    }
}
