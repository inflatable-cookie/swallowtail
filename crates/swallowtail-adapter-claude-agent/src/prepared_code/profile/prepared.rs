use super::super::ClaudeCodePreparedIntegration;
use crate::ClaudeCodeMaximumTurns;
use swallowtail_core::{ObservableActivityProfile, PreflightPlan};
use swallowtail_runtime::{
    BoxFuture, HostServices, PreparationFailure, PreparedOperationEvidence, RunHandle,
    RuntimeFailure, StructuredRunDriver, StructuredRunRequest,
};

#[derive(Clone, Debug, Eq, PartialEq)]
/// Portable evidence for a prepared native Claude Code run.
pub struct ClaudeCodePreparedEvidence {
    observation: swallowtail_core::InstalledExecutableObservation,
    environment: swallowtail_runtime::EnvironmentRef,
    operation: PreparedOperationEvidence,
    maximum_turns: Option<ClaudeCodeMaximumTurns>,
    watchers: bool,
}

impl ClaudeCodePreparedEvidence {
    fn from_prepared(
        prepared: &ClaudeCodePreparedIntegration,
        plan: PreflightPlan,
        activity_profile: swallowtail_core::ObservableActivityProfile,
        maximum_turns: Option<ClaudeCodeMaximumTurns>,
        watchers: bool,
    ) -> Result<Self, PreparationFailure> {
        Ok(Self {
            observation: prepared.observation().clone(),
            environment: prepared.environment().clone(),
            operation: PreparedOperationEvidence::from_plan_with_activity_profile(
                plan,
                prepared.access_evidence().clone(),
                activity_profile,
            )?,
            maximum_turns,
            watchers,
        })
    }

    /// Returns the prepared maximum agentic-turn bound when one was selected.
    #[must_use]
    pub const fn maximum_turns(&self) -> Option<ClaudeCodeMaximumTurns> {
        self.maximum_turns
    }

    /// Reports whether this prepared run opted into the watcher candidate.
    #[must_use]
    pub const fn watchers(&self) -> bool {
        self.watchers
    }

    /// Returns the qualified installed-executable observation.
    #[must_use]
    pub const fn observation(&self) -> &swallowtail_core::InstalledExecutableObservation {
        &self.observation
    }

    /// Returns the prepared access evidence.
    #[must_use]
    pub const fn access(&self) -> &swallowtail_runtime::PreparedAccessEvidence {
        self.operation.access()
    }

    /// Returns the complete prepared-operation evidence.
    #[must_use]
    pub const fn operation(&self) -> &PreparedOperationEvidence {
        &self.operation
    }

    /// Returns the admitted observable-activity profile.
    #[must_use]
    pub const fn observable_activity(&self) -> &swallowtail_core::ObservableActivityProfile {
        self.operation.observable_activity()
    }

    /// Returns the validated preflight plan.
    #[must_use]
    pub const fn plan(&self) -> &PreflightPlan {
        self.operation.plan()
    }

    fn low_level_driver(&self) -> crate::ClaudeCodeHeadlessDriver {
        crate::ClaudeCodeHeadlessDriver::new(self.environment.clone())
    }

    /// Creates the driver that actually dispatches this prepared run.
    ///
    /// Deliberately private. A maximum-turn bound and the `(plan, request)`
    /// pair it was prepared against are only ever brought together here, which
    /// is why no comparison between them is needed: they cannot disagree.
    fn bound_driver(&self) -> crate::ClaudeCodeHeadlessDriver {
        let driver = self.low_level_driver();
        let driver = match self.maximum_turns {
            Some(maximum_turns) => driver.with_maximum_turns(maximum_turns),
            None => driver,
        };
        if self.watchers {
            driver.with_watchers()
        } else {
            driver
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Prepared one-shot native Claude Code structured run.
pub struct ClaudeCodePreparedRun {
    evidence: ClaudeCodePreparedEvidence,
    request: StructuredRunRequest,
}

impl ClaudeCodePreparedRun {
    /// Returns portable evidence for the prepared run.
    #[must_use]
    pub const fn evidence(&self) -> &ClaudeCodePreparedEvidence {
        &self.evidence
    }

    /// Returns the validated preflight plan.
    #[must_use]
    pub const fn plan(&self) -> &PreflightPlan {
        self.evidence.plan()
    }

    /// Returns the bound structured-run request.
    #[must_use]
    pub const fn request(&self) -> &StructuredRunRequest {
        &self.request
    }

    /// Returns the prepared maximum agentic-turn bound when one was selected.
    #[must_use]
    pub const fn maximum_turns(&self) -> Option<ClaudeCodeMaximumTurns> {
        self.evidence.maximum_turns()
    }

    /// Reports whether this prepared run opted into the watcher candidate.
    #[must_use]
    pub const fn watchers(&self) -> bool {
        self.evidence.watchers()
    }

    /// Creates the low-level native headless driver for this run.
    ///
    /// The returned driver never carries a maximum-turn bound, even when
    /// [`Self::maximum_turns`] is `Some`. A bound is execution state that only
    /// means anything alongside the exact `(plan, request)` pair it was
    /// prepared with, and nothing in that pair records it, so an extracted
    /// driver could be handed another run's plan and silently dispatch the
    /// wrong bound. [`Self::start_run`] is therefore the only path that
    /// dispatches one.
    ///
    /// Everything else about the returned driver is unchanged, so this remains
    /// the low-level seam for callers who drive the route themselves.
    #[must_use]
    pub fn low_level_driver(&self) -> crate::ClaudeCodeHeadlessDriver {
        self.evidence.low_level_driver()
    }

    /// Starts the prepared run with caller-supplied host services.
    ///
    /// This is the only surface that dispatches a prepared maximum-turn bound,
    /// and it always pairs that bound with this run's own plan and request.
    pub fn start_run(
        &self,
        services: HostServices,
    ) -> BoxFuture<'static, Result<Box<dyn RunHandle>, RuntimeFailure>> {
        let driver = self.evidence.bound_driver();
        let plan = self.plan().clone();
        let request = self.request.clone();
        Box::pin(async move { driver.start_run(plan, request, services).await })
    }

    /// Splits the prepared run into evidence, plan, and request.
    #[must_use]
    pub fn into_parts(
        self,
    ) -> (
        ClaudeCodePreparedEvidence,
        PreflightPlan,
        StructuredRunRequest,
    ) {
        let plan = self.evidence.plan().clone();
        (self.evidence, plan, self.request)
    }
}

pub(super) fn new_prepared_run(
    prepared: &ClaudeCodePreparedIntegration,
    plan: PreflightPlan,
    activity: ObservableActivityProfile,
    maximum_turns: Option<ClaudeCodeMaximumTurns>,
    watchers: bool,
    request: StructuredRunRequest,
) -> Result<ClaudeCodePreparedRun, PreparationFailure> {
    Ok(ClaudeCodePreparedRun {
        evidence: ClaudeCodePreparedEvidence::from_prepared(
            prepared,
            plan,
            activity,
            maximum_turns,
            watchers,
        )?,
        request,
    })
}
