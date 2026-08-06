#![deny(missing_docs)]

use swallowtail_core::{
    Capability, CapabilityConstraint, ExecutionLayer, OperationRequirements, OperationShape,
    PreflightPlan, ReasoningMode, SafeDiagnostic,
};

use crate::{RuntimeFailure, SessionOptions};

const MAXIMUM_MODEL_OPTIONS: usize = 256;
const MAXIMUM_MODEL_OPTION_TEXT_BYTES: usize = 256;

/// One opaque model selector advertised by an already-open provider session.
///
/// This is negotiated session evidence, not a standalone catalogue entry.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NegotiatedSessionModelOption {
    value: String,
    display_name: Option<String>,
}

impl NegotiatedSessionModelOption {
    /// Creates one bounded provider-native option and optional display name.
    pub fn new(
        value: impl Into<String>,
        display_name: Option<String>,
    ) -> Result<Self, RuntimeFailure> {
        let value = bounded_model_option_text(value.into())?;
        let display_name = display_name.map(bounded_model_option_text).transpose()?;
        Ok(Self {
            value,
            display_name,
        })
    }

    #[must_use]
    /// Returns the opaque provider-native selector value.
    pub fn value(&self) -> &str {
        &self.value
    }

    #[must_use]
    /// Returns provider display text, when supplied.
    pub fn display_name(&self) -> Option<&str> {
        self.display_name.as_deref()
    }
}

/// Bounded model-selector snapshot negotiated while opening or attaching a
/// provider session.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NegotiatedSessionModelOptions {
    current_value: String,
    options: Vec<NegotiatedSessionModelOption>,
}

impl NegotiatedSessionModelOptions {
    /// Creates a bounded unique snapshot containing its current value.
    pub fn new(
        current_value: impl Into<String>,
        options: impl IntoIterator<Item = NegotiatedSessionModelOption>,
    ) -> Result<Self, RuntimeFailure> {
        let current_value = bounded_model_option_text(current_value.into())?;
        let options = options.into_iter().collect::<Vec<_>>();
        if options.is_empty()
            || options.len() > MAXIMUM_MODEL_OPTIONS
            || !options.iter().any(|option| option.value() == current_value)
        {
            return Err(model_option_failure());
        }
        let mut values = std::collections::BTreeSet::new();
        if options
            .iter()
            .any(|option| !values.insert(option.value().to_owned()))
        {
            return Err(model_option_failure());
        }
        Ok(Self {
            current_value,
            options,
        })
    }

    #[must_use]
    /// Returns the provider-native value effective for the open session.
    pub fn current_value(&self) -> &str {
        &self.current_value
    }

    /// Iterates the bounded advertised options in provider order.
    pub fn options(&self) -> impl ExactSizeIterator<Item = &NegotiatedSessionModelOption> {
        self.options.iter()
    }
}

/// Session lifecycle on which a portable option may be negotiated.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SessionLifecycleOperation {
    /// Create a new provider session.
    Open,
    /// Load a retained provider session with replay.
    Load,
    /// Resume a retained provider session without replay.
    Resume,
}

/// Pending exact reasoning selection awaiting provider confirmation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NegotiatedReasoningSetup {
    operation: SessionLifecycleOperation,
    requested: ReasoningMode,
}

impl NegotiatedReasoningSetup {
    #[must_use]
    /// Returns the lifecycle operation being configured.
    pub const fn operation(&self) -> SessionLifecycleOperation {
        self.operation
    }

    #[must_use]
    /// Returns the exact portable reasoning mode requested by the consumer.
    pub const fn requested(&self) -> &ReasoningMode {
        &self.requested
    }

    /// Confirms the provider-effective mode, rejecting any mismatch.
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

/// Confirmed equality between requested and provider-effective reasoning modes.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EffectiveReasoningSetup {
    requested: ReasoningMode,
    effective: ReasoningMode,
}

impl EffectiveReasoningSetup {
    #[must_use]
    /// Returns the consumer-requested reasoning mode.
    pub const fn requested(&self) -> &ReasoningMode {
        &self.requested
    }

    #[must_use]
    /// Returns the provider-confirmed effective reasoning mode.
    pub const fn effective(&self) -> &ReasoningMode {
        &self.effective
    }
}

/// Prepares negotiated reasoning only when one exact preflight constraint and
/// session option agree for the selected lifecycle.
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

fn bounded_model_option_text(value: String) -> Result<String, RuntimeFailure> {
    if value.is_empty()
        || value.len() > MAXIMUM_MODEL_OPTION_TEXT_BYTES
        || value.trim() != value
        || value.chars().any(char::is_control)
    {
        Err(model_option_failure())
    } else {
        Ok(value)
    }
}

fn model_option_failure() -> RuntimeFailure {
    failure(
        "swallowtail.negotiated_model_options.invalid",
        "Harness returned invalid bounded negotiated model options",
    )
}

#[cfg(test)]
mod tests;
