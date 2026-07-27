use super::trace::{FixtureEvent, record};
use crate::{ProviderSessionManagementFixture, poll_immediate};
use std::sync::{Arc, Mutex};
use swallowtail_core::{ProviderRequestRef, ProviderSessionManagementAction};
use swallowtail_runtime::{
    ArchiveProviderSessionRequest, BoxFuture, CancellationControl, DeleteProviderSessionRequest,
    DriverRegistration, HostServices, ProviderSessionManagementDriver,
    ProviderSessionManagementOutcome, ProviderSessionManagementPlan, RestoreProviderSessionRequest,
    RuntimeFailure, ScopeId, validate_provider_session_management_request,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum FixtureBehavior {
    Apply,
    AlreadyAbsent,
    CancelAfterDispatch,
    DeadlineAfterDispatch,
}

struct FixtureDriver {
    behavior: FixtureBehavior,
    events: Arc<Mutex<Vec<FixtureEvent>>>,
}

impl FixtureDriver {
    fn execute(
        &self,
        plan: ProviderSessionManagementPlan,
        agreement: &swallowtail_runtime::ProviderSessionManagementAgreement,
        cancellation: Arc<swallowtail_runtime::ImmediateCancellation>,
        services: HostServices,
    ) -> BoxFuture<'static, Result<ProviderSessionManagementOutcome, RuntimeFailure>> {
        let agreement = agreement.clone();
        let behavior = self.behavior;
        let events = Arc::clone(&self.events);
        Box::pin(async move {
            validate_provider_session_management_request(&plan, &agreement, &services)?;
            if cancellation.is_requested() {
                return Ok(ProviderSessionManagementOutcome::new(
                    agreement.binding().clone(),
                    swallowtail_core::ProviderSessionManagementEffect::failed_before_effect(
                        agreement.action(),
                    ),
                ));
            }
            if let Some(deadline) = agreement.deadline()
                && services.time().expect("fixture plan requires time").now() >= deadline.instant()
            {
                return Ok(ProviderSessionManagementOutcome::new(
                    agreement.binding().clone(),
                    swallowtail_core::ProviderSessionManagementEffect::failed_before_effect(
                        agreement.action(),
                    ),
                ));
            }

            let scope =
                ScopeId::new("fixture.session-management-scope").expect("fixture scope is valid");
            let task = services
                .task()
                .expect("fixture plan requires task service")
                .spawn(scope.clone(), Box::pin(async {}))?;
            record(&events, FixtureEvent::TaskStarted);
            let credential = services
                .credential()
                .expect("fixture plan requires credential service")
                .acquire(
                    scope.clone(),
                    plan.preflight()
                        .credential_reference()
                        .expect("fixture credential is bound")
                        .clone(),
                    plan.preflight().endpoint_audience().clone(),
                )
                .await?;
            record(&events, FixtureEvent::CredentialAcquired);
            let resource = services
                .working_resource()
                .expect("fixture plan requires working-resource service")
                .resolve(
                    scope,
                    agreement
                        .binding()
                        .working_resource()
                        .expect("fixture resource is bound")
                        .clone(),
                    swallowtail_runtime::ResourceAccess::Read,
                    swallowtail_runtime::ResourceRepresentation::Filesystem,
                )
                .await?;
            record(&events, FixtureEvent::ResourceAcquired);
            record(&events, FixtureEvent::Dispatched);

            let effect = match behavior {
                FixtureBehavior::Apply => {
                    swallowtail_core::ProviderSessionManagementEffect::applied(
                        agreement.action(),
                        agreement.affected_scope(),
                    )
                }
                FixtureBehavior::AlreadyAbsent => {
                    let strength = agreement
                        .action()
                        .deletion_strength()
                        .expect("already-absent behavior is delete-only");
                    swallowtail_core::ProviderSessionManagementEffect::target_already_absent(
                        strength,
                        agreement.affected_scope(),
                    )
                }
                FixtureBehavior::CancelAfterDispatch => {
                    cancellation.request().await?;
                    swallowtail_core::ProviderSessionManagementEffect::unconfirmed_after_effect(
                        agreement.action(),
                    )
                }
                FixtureBehavior::DeadlineAfterDispatch => {
                    services
                        .time()
                        .expect("fixture plan requires time")
                        .wait_until(
                            agreement
                                .deadline()
                                .expect("deadline behavior needs deadline"),
                        )
                        .await;
                    swallowtail_core::ProviderSessionManagementEffect::unconfirmed_after_effect(
                        agreement.action(),
                    )
                }
            };

            task.join().await?;
            record(&events, FixtureEvent::TaskJoined);
            services
                .working_resource()
                .expect("fixture plan requires working-resource service")
                .release(resource)
                .await;
            record(&events, FixtureEvent::ResourceReleased);
            services
                .credential()
                .expect("fixture plan requires credential service")
                .release(credential)
                .await;
            record(&events, FixtureEvent::CredentialReleased);

            Ok(
                ProviderSessionManagementOutcome::new(agreement.binding().clone(), effect)
                    .with_provider_request_ref(
                        ProviderRequestRef::new("fixture.private.provider-request")
                            .expect("fixture provider request is valid"),
                    ),
            )
        })
    }
}

impl ProviderSessionManagementDriver for FixtureDriver {
    fn archive_session(
        &self,
        plan: ProviderSessionManagementPlan,
        request: ArchiveProviderSessionRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<ProviderSessionManagementOutcome, RuntimeFailure>> {
        self.execute(
            plan,
            request.agreement(),
            Arc::clone(request.cancellation()),
            services,
        )
    }

    fn restore_session(
        &self,
        plan: ProviderSessionManagementPlan,
        request: RestoreProviderSessionRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<ProviderSessionManagementOutcome, RuntimeFailure>> {
        self.execute(
            plan,
            request.agreement(),
            Arc::clone(request.cancellation()),
            services,
        )
    }

    fn delete_session(
        &self,
        plan: ProviderSessionManagementPlan,
        request: DeleteProviderSessionRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<ProviderSessionManagementOutcome, RuntimeFailure>> {
        self.execute(
            plan,
            request.agreement(),
            Arc::clone(request.cancellation()),
            services,
        )
    }
}

pub(super) fn execute(
    fixture: &ProviderSessionManagementFixture,
    plan: ProviderSessionManagementPlan,
    services: HostServices,
    behavior: FixtureBehavior,
    cancel_before: bool,
) -> (ProviderSessionManagementOutcome, Vec<FixtureEvent>) {
    let events = Arc::new(Mutex::new(Vec::new()));
    let driver = Arc::new(FixtureDriver {
        behavior,
        events: Arc::clone(&events),
    });
    let registration = DriverRegistration::new(fixture.driver().clone())
        .with_provider_session_management(driver)
        .expect("fixture descriptor declares management");
    let role = registration
        .provider_session_management()
        .expect("fixture management role is registered");
    let request_id =
        swallowtail_runtime::RequestId::new("fixture-management").expect("request id is valid");
    let outcome = match plan.agreement().action() {
        ProviderSessionManagementAction::Archive => {
            let request =
                ArchiveProviderSessionRequest::from_plan(request_id, &plan).expect("request valid");
            if cancel_before {
                poll_immediate(request.cancellation().request()).expect("cancellation succeeds");
            }
            poll_immediate(role.archive_session(plan, request, services))
        }
        ProviderSessionManagementAction::Restore => {
            let request =
                RestoreProviderSessionRequest::from_plan(request_id, &plan).expect("request valid");
            if cancel_before {
                poll_immediate(request.cancellation().request()).expect("cancellation succeeds");
            }
            poll_immediate(role.restore_session(plan, request, services))
        }
        ProviderSessionManagementAction::Delete(_) => {
            let request =
                DeleteProviderSessionRequest::from_plan(request_id, &plan).expect("request valid");
            if cancel_before {
                poll_immediate(request.cancellation().request()).expect("cancellation succeeds");
            }
            poll_immediate(role.delete_session(plan, request, services))
        }
    }
    .expect("fixture management operation succeeds");
    let recorded = events
        .lock()
        .expect("fixture event lock is not poisoned")
        .clone();
    (outcome, recorded)
}
