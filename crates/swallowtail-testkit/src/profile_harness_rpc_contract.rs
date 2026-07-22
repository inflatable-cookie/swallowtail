use crate::profile_fixture::restrictive_rpc_policy;
use crate::{
    CallbackExchangeFixture, ConformanceAssertion, ConformanceReport, ProfilePreflightFixture,
    SyntheticProfile, successful_callback_response,
};
use std::collections::VecDeque;
use swallowtail_core::{
    FilesystemBoundary, HarnessBackgroundAction, HarnessConfigurationSource, HarnessIsolation,
    HarnessMessageClass, InterfaceSupportStatus, PreflightDimension, ResourceAccess,
};
use swallowtail_runtime::{
    CallbackAbandonment, CallbackId, CallbackRequest, CallbackWaitState, Deadline, EventDelivery,
    HarnessCommandAcknowledgement, HarnessCommandId, HarnessCommandResponse, HarnessUiDialog,
    HarnessUiDialogKind, HarnessUiDisplay, HarnessUiDisplayKind, MonotonicInstant,
    OperationContent, RuntimeEvent, RuntimeEventKind, RuntimeTurnId,
};

pub(crate) fn run() -> ConformanceReport {
    let mut report = ConformanceReport::new(SyntheticProfile::LongLivedRpcHarness);
    assert_preflight_contract();
    assert_scheduling_contract();
    assert_ui_relay_contract();

    report.record(ConformanceAssertion::InterfaceVersionQualified);
    report.record(ConformanceAssertion::HarnessPolicyExact);
    report.record(ConformanceAssertion::HarnessScheduling);
    report.record(ConformanceAssertion::CommandAcknowledgement);
    report.record(ConformanceAssertion::HarnessUiRelay);
    report
}

fn assert_preflight_contract() {
    let fixture = ProfilePreflightFixture::harness_rpc_contract();
    let plan = fixture
        .preflight()
        .expect("qualified RPC preflight succeeds");
    assert_eq!(plan.interface_versions().len(), 1);
    assert_eq!(plan.harness_rpc_policy(), Some(&restrictive_rpc_policy()));
    assert!(plan.provider_id().is_some());
    assert!(plan.model_route_id().is_some());
    assert!(plan.model_id().is_some());
    let access = plan
        .requirements()
        .session_access_policy()
        .expect("interactive RPC access policy is bound");
    assert_eq!(access.resource_access(), Some(ResourceAccess::Read));
    assert_eq!(
        access.harness_isolation(),
        Some(HarnessIsolation::AmbientHost)
    );
    assert_eq!(access.filesystem_boundary(), None::<FilesystemBoundary>);
    let policy = plan.harness_rpc_policy().unwrap();
    assert_eq!(policy.scheduling().maximum_active_operations().get(), 1);
    assert_eq!(policy.scheduling().maximum_completed_prompts().get(), 2);
    assert_eq!(policy.scheduling().maximum_pending_steering().get(), 1);
    assert_eq!(policy.scheduling().maximum_pending_follow_up().get(), 1);
    for source in [
        HarnessConfigurationSource::Extensions,
        HarnessConfigurationSource::Skills,
        HarnessConfigurationSource::PromptTemplates,
        HarnessConfigurationSource::ContextFiles,
    ] {
        assert!(!policy.permits_configuration_source(source));
    }
    for action in [
        HarnessBackgroundAction::UpdateCheck,
        HarnessBackgroundAction::Telemetry,
        HarnessBackgroundAction::PackageMutation,
        HarnessBackgroundAction::AutomaticRetry,
    ] {
        assert!(!policy.permits_background_action(action));
    }

    let mut unknown_version = ProfilePreflightFixture::harness_rpc_contract();
    unknown_version.require_harness_rpc_version("1.0.1");
    let failure = unknown_version
        .preflight()
        .expect_err("unqualified interface version fails");
    assert_eq!(failure.dimension(), PreflightDimension::InterfaceVersion);

    for version in ["0.8.0", "0.8.4", "0.8.9"] {
        assert_window_match(
            version,
            InterfaceSupportStatus::Deprecated,
            "fixture.rpc-v1",
        );
    }
    for version in ["0.9.0", "0.9.6", "1.0.0"] {
        assert_window_match(
            version,
            InterfaceSupportStatus::Maintained,
            "fixture.rpc-v2",
        );
    }

    for version in ["0.9.5", "1.0.1", "0.7.9"] {
        let mut unsupported = ProfilePreflightFixture::harness_rpc_contract();
        unsupported.use_harness_rpc_compatibility_window(version);
        let failure = unsupported
            .preflight()
            .expect_err("excluded or out-of-window version fails");
        assert_eq!(failure.dimension(), PreflightDimension::InterfaceVersion);
    }

    let mut mismatched_policy = ProfilePreflightFixture::harness_rpc_contract();
    let one = std::num::NonZeroU32::new(1).unwrap();
    mismatched_policy.require_harness_rpc_policy(swallowtail_core::HarnessRpcPolicy::restrictive(
        swallowtail_core::HarnessSchedulingBounds::new(one, one, one, one),
    ));
    let failure = mismatched_policy
        .preflight()
        .expect_err("mismatched RPC policy fails");
    assert_eq!(failure.dimension(), PreflightDimension::HarnessRpcPolicy);
}

fn assert_window_match(
    version: &str,
    support_status: InterfaceSupportStatus,
    behavior_revision: &str,
) {
    let mut fixture = ProfilePreflightFixture::harness_rpc_contract();
    fixture.use_harness_rpc_compatibility_window(version);
    let plan = fixture
        .preflight()
        .expect("qualified compatibility-window point succeeds");
    let binding = plan.interface_versions().next().unwrap();
    let classification = plan.classify_interface_version(binding).unwrap();
    assert_eq!(classification.support_status(), support_status);
    assert_eq!(
        classification.behavior_revision().as_str(),
        behavior_revision
    );
}

fn assert_scheduling_contract() {
    let mut scheduler = Scheduler::default();
    assert_eq!(scheduler.submit(HarnessMessageClass::Prompt), Ack::Accepted);
    assert_eq!(scheduler.submit(HarnessMessageClass::Prompt), Ack::Rejected);
    assert_eq!(
        scheduler.submit(HarnessMessageClass::FollowUp),
        Ack::Accepted
    );
    assert_eq!(
        scheduler.submit(HarnessMessageClass::FollowUp),
        Ack::Rejected
    );
    assert_eq!(
        scheduler.submit(HarnessMessageClass::Steering),
        Ack::Accepted
    );
    assert_eq!(
        scheduler.submit(HarnessMessageClass::Steering),
        Ack::Rejected
    );
    assert_eq!(
        scheduler.finish_active(),
        vec![HarnessMessageClass::Steering, HarnessMessageClass::FollowUp]
    );

    let response = HarnessCommandResponse::new(
        HarnessCommandId::new("fixture-command").unwrap(),
        HarnessCommandAcknowledgement::Accepted,
    );
    assert_eq!(
        response.acknowledgement(),
        HarnessCommandAcknowledgement::Accepted
    );
}

fn assert_ui_relay_contract() {
    let dialog = HarnessUiDialog::new(
        HarnessUiDialogKind::Select,
        OperationContent::new("private title").unwrap(),
        Some(OperationContent::new("private prompt").unwrap()),
        [OperationContent::new("private option").unwrap()],
        4,
        128,
    )
    .unwrap();
    let request = CallbackRequest::harness_ui_dialog(
        CallbackId::new("fixture-ui-callback").unwrap(),
        RuntimeTurnId::new("fixture-ui-turn").unwrap(),
        4,
        Some(Deadline::at(MonotonicInstant::from_ticks(40))),
        dialog,
    );
    let mut exchange = CallbackExchangeFixture::new(request);
    exchange
        .abandon(CallbackAbandonment::TimedOut)
        .expect("dialog times out while waiting");
    assert_eq!(
        exchange.state(),
        CallbackWaitState::Abandoned(CallbackAbandonment::TimedOut)
    );
    let late = successful_callback_response(exchange.request());
    assert!(exchange.respond(late).is_err());

    let display = HarnessUiDisplay::new(
        HarnessUiDisplayKind::Status,
        OperationContent::new("private status").unwrap(),
        64,
    )
    .unwrap();
    let event = RuntimeEvent::new(5, RuntimeEventKind::HarnessUiDisplay(display));
    assert_eq!(event.delivery(), EventDelivery::Semantic);
    assert!(!format!("{event:?}").contains("private status"));
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Ack {
    Accepted,
    Rejected,
}

#[derive(Default)]
struct Scheduler {
    active: bool,
    steering: VecDeque<HarnessMessageClass>,
    follow_up: VecDeque<HarnessMessageClass>,
}

impl Scheduler {
    fn submit(&mut self, class: HarnessMessageClass) -> Ack {
        match class {
            HarnessMessageClass::Prompt if !self.active => {
                self.active = true;
                Ack::Accepted
            }
            HarnessMessageClass::Prompt => Ack::Rejected,
            HarnessMessageClass::Steering if self.active && self.steering.is_empty() => {
                self.steering.push_back(class);
                Ack::Accepted
            }
            HarnessMessageClass::FollowUp if self.active && self.follow_up.is_empty() => {
                self.follow_up.push_back(class);
                Ack::Accepted
            }
            HarnessMessageClass::Steering | HarnessMessageClass::FollowUp => Ack::Rejected,
        }
    }

    fn finish_active(&mut self) -> Vec<HarnessMessageClass> {
        self.active = false;
        self.steering
            .drain(..)
            .chain(self.follow_up.drain(..))
            .collect()
    }
}
