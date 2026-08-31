use super::*;
use crate::driver::config::ReasoningConfirmation;
use crate::driver::validation::validate_initialize;
use crate::driver::{ClaudeAgentOpenRejection, ClaudeAgentReasoningAcknowledgement};
use crate::selection::ClaudeAgentPlanSelection;
use swallowtail_core::{HarnessMode, ReasoningMode};

impl ClaudeAgentAcpDriver {
    pub(in crate::driver) async fn start_session(
        &self,
        plan: &PreflightPlan,
        request: &OpenSessionRequest,
        services: &HostServices,
        selected: ClaudeAgentPlanSelection,
        reasoning: Option<ReasoningMode>,
    ) -> Result<ClaudeAgentSessionHandle, RuntimeFailure> {
        self.start_session_with_acknowledgement(plan, request, services, selected, reasoning)
            .await
            .map(|(session, _)| session)
            .map_err(ClaudeAgentOpenRejection::into_failure)
    }

    pub(in crate::driver) async fn start_session_with_acknowledgement(
        &self,
        plan: &PreflightPlan,
        request: &OpenSessionRequest,
        services: &HostServices,
        selected: ClaudeAgentPlanSelection,
        reasoning: Option<ReasoningMode>,
    ) -> Result<
        (
            ClaudeAgentSessionHandle,
            ClaudeAgentReasoningAcknowledgement,
        ),
        ClaudeAgentOpenRejection,
    > {
        let working_resource = request
            .working_resource()
            .expect("validated working resource")
            .clone();
        let resource_access = request
            .access_policy()
            .resource_access()
            .expect("validated working-resource access");
        let mut pending = self
            .start_attachment(
                plan,
                request.request_id(),
                working_resource,
                request.access_policy(),
                services,
            )
            .await?;
        let opened: Result<_, ClaudeAgentOpenRejection> = async {
            let mut acknowledgement = ClaudeAgentReasoningAcknowledgement::NotRequested;
            let initialized = pending
                .connection
                .initialize()
                .await
                .map_err(ClaudeAgentOpenRejection::from)?;
            let lifecycle = validate_initialize(&initialized, &selected)?;
            let owned_session_cleanup =
                plan.requirements().operation_shape() == OperationShape::StructuredRun
                    && crate::driver::validation::run_owns_session_cleanup(plan)?;
            if owned_session_cleanup
                && (!lifecycle.close || !lifecycle.delete || !selected.is_qualified())
            {
                return Err(failure(
                    "swallowtail.claude_agent.acp.owned_cleanup_unavailable",
                    "Claude Agent did not negotiate the qualified close and delete lifecycle required by this run",
                )
                .into());
            }
            let model = plan.model_id().expect("validated model").as_str();
            let response = pending
                .connection
                .new_session(pending.cwd.clone(), model)
                .await?;
            let provider_id = crate::driver::config::parse_session_id(&response)?;
            pending.connection.set_session_id(provider_id.clone())?;
            if selected.behavior().supports_config_options() {
                crate::driver::config::validate_model_option(&response)?;
                let configured = pending
                    .connection
                    .set_config_option(&provider_id, "model", model)
                    .await?;
                crate::driver::config::confirm_model(&configured, model)?;
                if let Some(reasoning) = reasoning.as_ref() {
                    crate::driver::config::validate_reasoning_option(&configured, reasoning)?;
                    let confirmed = pending
                        .connection
                        .set_config_option(&provider_id, "effort", reasoning.as_str())
                        .await?;
                    acknowledgement = match crate::driver::config::confirm_reasoning(
                        &confirmed,
                        reasoning,
                    )? {
                        ReasoningConfirmation::Effective(value) => {
                            ClaudeAgentReasoningAcknowledgement::Effective(value)
                        }
                        ReasoningConfirmation::Rejected(value) => {
                            return Err(ClaudeAgentOpenRejection::rejected(
                                failure(
                                    "swallowtail.claude_agent.acp.reasoning_mismatch",
                                    "Claude Agent reasoning confirmation does not match the requested mode",
                                ),
                                value,
                            ));
                        }
                    };
                }
                if request.options().harness_mode() == Some(HarnessMode::Plan) {
                    crate::driver::config::validate_plan_mode_option(&configured)?;
                    let confirmed = pending
                        .connection
                        .set_config_option(&provider_id, "mode", "plan")
                        .await?;
                    crate::driver::config::confirm_plan_mode(&confirmed)?;
                }
            } else {
                crate::driver::config::validate_legacy_model(&response, model)?;
            }
            if resource_access == ResourceAccess::ReadWrite {
                crate::driver::config::validate_write_mode(&response)?;
                pending
                    .connection
                    .set_session_mode(&provider_id, "acceptEdits")
                    .await?;
            }
            let provider_ref = SessionRef::new(&provider_id).map_err(|_| malformed())?;
            let binding = SessionResumeBinding::new(
                provider_ref.clone(),
                plan.instance_id().clone(),
                plan.execution_host_id().clone(),
                plan.model_route_id().expect("validated route").clone(),
                plan.model_id().expect("validated model").clone(),
                request
                    .working_resource()
                    .expect("validated resource")
                    .clone(),
                request.access_policy().clone(),
            );
            let handle = pending.take_handle(
                SessionHandleInput {
                    request_id: request.request_id().clone(),
                    provider_ref,
                    binding,
                    provider_requests: request.access_policy().provider_requests().clone(),
                    execution_host_id: plan.execution_host_id().clone(),
                    native_close: lifecycle.close && selected.is_qualified(),
                    native_delete: lifecycle.delete && selected.is_qualified(),
                },
                services,
            )?;
            Ok::<_, ClaudeAgentOpenRejection>((handle, acknowledgement))
        }
        .await;
        match opened {
            Ok(opened) => Ok(opened),
            Err(error) => {
                let _ = pending.abort(services).await;
                Err(error)
            }
        }
    }
}
