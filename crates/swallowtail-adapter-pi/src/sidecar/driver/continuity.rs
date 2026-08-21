//! Load and resume attachment for bound provider sessions.
//!
//! Both operations spawn a fresh sidecar at the exact leased working
//! directory, switch to the bound provider session through the public SDK
//! runtime, and verify the effective cwd and session reference before
//! readiness. Load additionally completes the bounded typed replay phase
//! before the state re-check; resume emits no replay by construction.

use super::validation::{AttachmentSurface, validate_attachment};
use super::{PiSdkSidecarDriver, session_binding, startup};
use crate::failure::failure;
use crate::sidecar::replay::MAXIMUM_REPLAY_ITEMS;
use crate::sidecar::wire::PiSdkSidecarCommand;
use serde_json::{Value, json};
use std::future::{Future, poll_fn};
use std::task::Poll;
use swallowtail_core::PreflightPlan;
use swallowtail_runtime::{
    Deadline, HostServices, InteractiveSessionHandle, RuntimeFailure, SessionReplayItem,
};

/// The operation-neutral pieces of one load or resume request.
pub(super) struct AttachmentRequest {
    pub(super) request_id: swallowtail_runtime::RequestId,
    pub(super) binding: swallowtail_runtime::SessionResumeBinding,
    pub(super) working_resource: swallowtail_runtime::WorkingResourceRef,
    pub(super) deadline: Option<Deadline>,
    pub(super) plan_agreement: swallowtail_runtime::SessionPlanAgreement,
    pub(super) options_empty: bool,
}

pub(super) struct AttachedSession {
    pub(super) handle: Box<dyn InteractiveSessionHandle>,
    pub(super) replay: Vec<SessionReplayItem>,
}

impl PiSdkSidecarDriver {
    pub(super) async fn attach(
        &self,
        plan: PreflightPlan,
        request: AttachmentRequest,
        services: HostServices,
        replay: bool,
    ) -> Result<AttachedSession, RuntimeFailure> {
        validate_attachment(
            &plan,
            &services,
            &self.credential,
            &AttachmentSurface {
                plan_agreement: &request.plan_agreement,
                access_policy: request.plan_agreement.access_policy(),
                provider_state_policy: request.plan_agreement.provider_state_policy(),
                working_resource: &request.working_resource,
                options_empty: request.options_empty,
                binding: &request.binding,
                replay,
            },
        )?;
        if request.deadline.is_some_and(|deadline| {
            services
                .time()
                .expect("validated sidecar time service")
                .now()
                >= deadline.instant()
        }) {
            return Err(failure(
                "swallowtail.pi.sdk-sidecar.attach_deadline_elapsed",
                "Pi SDK sidecar attachment deadline elapsed before startup",
            ));
        }
        let kind = if replay { "load" } else { "resume" };
        let pending = self
            .spawn_session(
                &plan,
                request.request_id.clone(),
                request.working_resource.clone(),
                request.plan_agreement.access_policy(),
                kind,
                services,
            )
            .await?;
        match self
            .attach_phases(
                &plan,
                &pending.connection,
                &pending.leased_cwd,
                &pending.services,
                &request,
                replay,
            )
            .await
        {
            Ok((binding, replay_items)) => Ok(AttachedSession {
                handle: Box::new(pending.into_handle(&plan, binding)),
                replay: replay_items,
            }),
            Err(error) => {
                pending.abort().await;
                Err(error)
            }
        }
    }

    async fn attach_phases(
        &self,
        plan: &PreflightPlan,
        connection: &std::sync::Arc<crate::sidecar::connection::SidecarConnection>,
        leased_cwd: &str,
        services: &HostServices,
        request: &AttachmentRequest,
        replay: bool,
    ) -> Result<
        (
            swallowtail_runtime::SessionResumeBinding,
            Vec<SessionReplayItem>,
        ),
        RuntimeFailure,
    > {
        // Bootstrap builds only in-memory state at the leased cwd; the fresh
        // session it creates is discarded by the switch below.
        startup::bootstrap(connection, plan, leased_cwd).await?;
        let bound_ref = request.binding.provider_session_ref().as_provider_value();
        let switch = connection.command(
            "switch-1".to_owned(),
            PiSdkSidecarCommand::SessionSwitch,
            json!({"sessionRef": bound_ref, "expectedCwd": leased_cwd}),
        );
        let switch = raced(switch, request.deadline, services).await??;
        if !switch.success {
            return Err(failure(
                "swallowtail.pi.sdk-sidecar.switch_rejected",
                "Pi SDK sidecar rejected the bound provider session switch",
            ));
        }
        let data = switch.data.as_ref();
        if data
            .and_then(|data| data.get("effectiveCwd"))
            .and_then(Value::as_str)
            != Some(leased_cwd)
        {
            return Err(failure(
                "swallowtail.pi.sdk-sidecar.switch_cwd_mismatch",
                "Pi SDK sidecar effective cwd did not match the host-leased working resource",
            ));
        }
        if data
            .and_then(|data| data.get("sessionRef"))
            .and_then(Value::as_str)
            != Some(bound_ref)
        {
            return Err(failure(
                "swallowtail.pi.sdk-sidecar.session_substituted",
                "Pi SDK sidecar attached a different provider session than the bound one",
            ));
        }
        let replay_items = if replay {
            connection.arm_replay(request.binding.provider_session_ref().clone())?;
            let command = connection.command(
                "replay-1".to_owned(),
                PiSdkSidecarCommand::SessionReplay,
                json!({"maxItems": MAXIMUM_REPLAY_ITEMS}),
            );
            let response = match raced(command, request.deadline, services).await {
                Ok(Ok(response)) => response,
                Ok(Err(error)) | Err(error) => {
                    let _ = connection.take_replay();
                    return Err(error);
                }
            };
            let collector = connection.take_replay();
            if !response.success {
                return Err(failure(
                    "swallowtail.pi.sdk-sidecar.replay_rejected",
                    "Pi SDK sidecar rejected the bounded session replay",
                ));
            }
            let reported = response
                .data
                .as_ref()
                .and_then(|data| data.get("items"))
                .and_then(Value::as_u64);
            let complete = response
                .data
                .as_ref()
                .and_then(|data| data.get("complete"))
                .and_then(Value::as_bool)
                == Some(true);
            match (collector, reported) {
                (Some(collector), Some(reported)) => collector.finish(reported, complete)?,
                _ => {
                    return Err(failure(
                        "swallowtail.pi.sdk-sidecar.replay_incomplete",
                        "Pi SDK sidecar replay response did not match the transported replay",
                    ));
                }
            }
        } else {
            Vec::new()
        };
        startup::check_state(connection, plan, leased_cwd, Some(bound_ref)).await?;
        let binding = session_binding(
            plan,
            bound_ref,
            &request.working_resource,
            request.plan_agreement.access_policy(),
        )?;
        Ok((binding, replay_items))
    }
}

async fn raced<F>(
    command: F,
    deadline: Option<Deadline>,
    services: &HostServices,
) -> Result<F::Output, RuntimeFailure>
where
    F: Future,
{
    let Some(deadline) = deadline else {
        return Ok(command.await);
    };
    let mut wait = services
        .time()
        .cloned()
        .expect("validated sidecar time service")
        .wait_until(deadline);
    let mut command = Box::pin(command);
    poll_fn(|context| {
        if let Poll::Ready(result) = command.as_mut().poll(context) {
            return Poll::Ready(Ok(result));
        }
        if wait.as_mut().poll(context).is_ready() {
            return Poll::Ready(Err(failure(
                "swallowtail.pi.sdk-sidecar.attach_timed_out",
                "Pi SDK sidecar attachment timed out before readiness",
            )));
        }
        Poll::Pending
    })
    .await
}
