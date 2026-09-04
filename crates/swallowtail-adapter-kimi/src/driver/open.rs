use swallowtail_runtime::EffectiveReasoningSetup;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum KimiAcknowledgement {
    Absent,
    Effective(String),
    Rejected(String),
    RequestedNotDispatched,
}

pub(crate) struct KimiOpenObservation {
    pub(crate) behavior: crate::selection::KimiAcpBehavior,
    pub(crate) reasoning: KimiAcknowledgement,
    pub(crate) plan: KimiAcknowledgement,
}

pub(crate) struct KimiOpenRejection {
    failure: swallowtail_runtime::RuntimeFailure,
    pub(crate) reasoning: KimiAcknowledgement,
    pub(crate) plan: KimiAcknowledgement,
    pub(crate) behavior: Option<crate::selection::KimiAcpBehavior>,
}

impl KimiOpenRejection {
    pub(crate) const fn runtime(failure: swallowtail_runtime::RuntimeFailure) -> Self {
        Self {
            failure,
            reasoning: KimiAcknowledgement::Absent,
            plan: KimiAcknowledgement::Absent,
            behavior: None,
        }
    }

    pub(crate) const fn observed(
        failure: RuntimeFailure,
        reasoning: KimiAcknowledgement,
        plan: KimiAcknowledgement,
        behavior: crate::selection::KimiAcpBehavior,
    ) -> Self {
        Self {
            failure,
            reasoning,
            plan,
            behavior: Some(behavior),
        }
    }

    pub(crate) const fn failure(&self) -> &swallowtail_runtime::RuntimeFailure {
        &self.failure
    }

    pub(crate) fn into_failure(self) -> swallowtail_runtime::RuntimeFailure {
        self.failure
    }
}

impl From<swallowtail_runtime::RuntimeFailure> for KimiOpenRejection {
    fn from(failure: swallowtail_runtime::RuntimeFailure) -> Self {
        Self::runtime(failure)
    }
}

pub(crate) struct KimiReasoningConfirmation {
    pub(crate) effective: EffectiveReasoningSetup,
    pub(crate) provider_value: String,
}

pub(crate) struct KimiConfirmationRejection {
    pub(crate) failure: swallowtail_runtime::RuntimeFailure,
    pub(crate) provider_value: Option<String>,
}

impl From<swallowtail_runtime::RuntimeFailure> for KimiConfirmationRejection {
    fn from(failure: swallowtail_runtime::RuntimeFailure) -> Self {
        Self {
            failure,
            provider_value: None,
        }
    }
}

pub(crate) type KimiOpenLifecycleResult = Result<
    (Box<dyn swallowtail_runtime::InteractiveSessionHandle>, KimiOpenObservation),
    KimiOpenRejection,
>;
