use swallowtail_core::{
    Capability, CapabilityConstraint, ExecutionLayer, OperationRequirements, OperationShape,
    PreflightPlan, ReasoningMode, SafeDiagnostic,
};

use crate::{RuntimeFailure, SessionOptions};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SessionLifecycleOperation {
    Open,
    Load,
    Resume,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NegotiatedReasoningSetup {
    operation: SessionLifecycleOperation,
    requested: ReasoningMode,
}

impl NegotiatedReasoningSetup {
    #[must_use]
    pub const fn operation(&self) -> SessionLifecycleOperation {
        self.operation
    }

    #[must_use]
    pub const fn requested(&self) -> &ReasoningMode {
        &self.requested
    }

    pub fn confirm(
        self,
        effective: ReasoningMode,
    ) -> Result<EffectiveReasoningSetup, RuntimeFailure> {
        if self.requested != effective {
            return Err(failure(
                "swallowtail.negotiated_reasoning.effective_mismatch",
                "Harness reasoning confirmation does not match the requested mode",
            ));
        }
        Ok(EffectiveReasoningSetup {
            requested: self.requested,
            effective,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EffectiveReasoningSetup {
    requested: ReasoningMode,
    effective: ReasoningMode,
}

impl EffectiveReasoningSetup {
    #[must_use]
    pub const fn requested(&self) -> &ReasoningMode {
        &self.requested
    }

    #[must_use]
    pub const fn effective(&self) -> &ReasoningMode {
        &self.effective
    }
}

pub fn prepare_negotiated_reasoning_setup(
    plan: &PreflightPlan,
    operation: SessionLifecycleOperation,
    options: &SessionOptions,
) -> Result<Option<NegotiatedReasoningSetup>, RuntimeFailure> {
    prepare_from_requirements(plan.requirements(), operation, options)
}

fn prepare_from_requirements(
    requirements: &OperationRequirements,
    operation: SessionLifecycleOperation,
    options: &SessionOptions,
) -> Result<Option<NegotiatedReasoningSetup>, RuntimeFailure> {
    if requirements.execution_layer() != ExecutionLayer::HarnessInteraction
        || requirements.operation_shape() != OperationShape::InteractiveSession
    {
        return Err(failure(
            "swallowtail.negotiated_reasoning.operation_rejected",
            "Negotiated reasoning requires an interactive harness operation",
        ));
    }

    let reasoning_requirements = requirements
        .capabilities()
        .filter(|requirement| requirement.capability() == Capability::ReasoningSelection)
        .collect::<Vec<_>>();
    let requested = options.reasoning_mode();

    if requested.is_none() && reasoning_requirements.is_empty() {
        return Ok(None);
    }
    let [requirement] = reasoning_requirements.as_slice() else {
        return Err(plan_mismatch());
    };
    let constraints = requirement.constraints().collect::<Vec<_>>();
    let [CapabilityConstraint::ReasoningMode(planned)] = constraints.as_slice() else {
        return Err(plan_mismatch());
    };
    let Some(requested) = requested else {
        return Err(plan_mismatch());
    };
    if requested != planned {
        return Err(plan_mismatch());
    }
    if operation != SessionLifecycleOperation::Open {
        return Err(failure(
            "swallowtail.negotiated_reasoning.lifecycle_rejected",
            "Reasoning setup is not supported for this session lifecycle",
        ));
    }

    Ok(Some(NegotiatedReasoningSetup {
        operation,
        requested: requested.clone(),
    }))
}

fn plan_mismatch() -> RuntimeFailure {
    failure(
        "swallowtail.negotiated_reasoning.plan_mismatch",
        "Requested reasoning does not match one exact preflight capability constraint",
    )
}

fn failure(code: &'static str, message: &'static str) -> RuntimeFailure {
    RuntimeFailure::new(SafeDiagnostic::new(code, message))
}

#[cfg(test)]
mod tests;
