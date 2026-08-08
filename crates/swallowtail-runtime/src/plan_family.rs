#![deny(missing_docs)]

//! One shared agreement/plan/request validation core for the runtime plan
//! family.
//!
//! Every role in the family (provider-session management, reconciliation,
//! import, catalogue, run reconciliation, recovered-resource cleanup)
//! validates its immutable plan through ordered [`PlanRule`] tables, and its
//! requests and host-service readiness through the shared validators in this
//! module. Per-role differences are visible as rule tables, not as divergent
//! hand-written validators.

use crate::{HostServices, RuntimeFailure};
use swallowtail_core::{PreflightPlan, SafeDiagnostic};

/// One ordered plan-validation rule for a role agreement.
///
/// A rule passes when `check` returns `true`; the first failing rule yields
/// its exact code and message, preserving per-role failure truth.
pub(crate) struct PlanRule<A> {
    code: &'static str,
    message: &'static str,
    check: fn(&PreflightPlan, &A) -> bool,
}

impl<A> PlanRule<A> {
    /// Builds one rule with its exact failure code and message.
    pub(crate) const fn new(
        code: &'static str,
        message: &'static str,
        check: fn(&PreflightPlan, &A) -> bool,
    ) -> Self {
        Self {
            code,
            message,
            check,
        }
    }
}

/// Runs a role's validation rules in order against the plan and agreement.
///
/// Returns the first failing rule's exact diagnostic, so each role keeps its
/// original failure ordering and codes.
pub(crate) fn check_plan_rules<A>(
    preflight: &PreflightPlan,
    agreement: &A,
    rules: &[PlanRule<A>],
) -> Result<(), RuntimeFailure> {
    for rule in rules {
        if !(rule.check)(preflight, agreement) {
            return Err(failure(rule.code, rule.message));
        }
    }
    Ok(())
}

/// Verifies that a request's agreement still matches its immutable plan.
pub(crate) fn validate_agreement_matches_plan<A: PartialEq>(
    plan_agreement: &A,
    request_agreement: &A,
    code: &'static str,
    message: &'static str,
) -> Result<(), RuntimeFailure> {
    if plan_agreement == request_agreement {
        Ok(())
    } else {
        Err(failure(code, message))
    }
}

/// Verifies the execution host and every required host service are available.
pub(crate) fn validate_execution_services(
    preflight: &PreflightPlan,
    services: &HostServices,
    code: &'static str,
    message: &'static str,
) -> Result<(), RuntimeFailure> {
    services.require_execution_host(preflight.execution_host_id())?;
    let available = services.available_kinds();
    if preflight
        .requirements()
        .host_services()
        .any(|required| !available.contains(&required))
    {
        services.emit_failure_debug(
            crate::DebugObservationKind::Lifecycle,
            "prepared.plan",
            "plan.validate_host_services",
            code,
            message,
        );
        return Err(failure(code, message));
    }
    Ok(())
}

/// Builds one safe runtime failure from a static code and message.
pub(crate) fn failure(code: &'static str, message: &'static str) -> RuntimeFailure {
    RuntimeFailure::new(SafeDiagnostic::new(code, message))
}

/// Generates one role's plan, prepared evidence, and typed request structs.
///
/// Each section is optional, so every role uses exactly the shared skeleton
/// pieces that match its shape:
///
/// - `plan` emits the plan and prepared-evidence structs
/// - `requests` emits the typed request structs
///
/// The generated items are public and keep every role's exact public shape;
/// only the shared skeleton is centralized here. `validate_plan` is resolved
/// at the invocation site, so each role's rule table stays in its own module.
macro_rules! plan_family {
    (
        $( plan: $plan:tt )?
        $( requests: $requests:tt )?
    ) => {
        $(
            plan_family!(@plan $plan);
        )?
        $(
            plan_family!(@requests $requests);
        )?
    };
    (@plan {
        plan_type: $plan_type:ident,
        prepared_type: $prepared_type:ident,
        agreement: $agreement:ty,
        plan_doc: $plan_doc:literal,
        prepared_doc: $prepared_doc:literal,
        agreement_doc: $agreement_doc:literal,
    }) => {
        #[doc = $plan_doc]
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub struct $plan_type {
            preflight: swallowtail_core::PreflightPlan,
            agreement: $agreement,
        }

        impl $plan_type {
            /// Validates a preflight plan against the immutable agreement.
            pub fn new(
                preflight: swallowtail_core::PreflightPlan,
                agreement: $agreement,
            ) -> Result<Self, $crate::RuntimeFailure> {
                validate_plan(&preflight, &agreement)?;
                Ok(Self {
                    preflight,
                    agreement,
                })
            }

            #[must_use]
            /// Returns the immutable preflight plan.
            pub const fn preflight(&self) -> &swallowtail_core::PreflightPlan {
                &self.preflight
            }

            #[doc = $agreement_doc]
            #[must_use]
            pub const fn agreement(&self) -> &$agreement {
                &self.agreement
            }
        }

    };
    (@prepared {
        plan_type: $plan_type:ident,
        prepared_type: $prepared_type:ident,
        agreement: $agreement:ty,
        prepared_doc: $prepared_doc:literal,
        agreement_doc: $agreement_doc:literal,
    }) => {
        #[doc = $prepared_doc]
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub struct $prepared_type {
            operation: $crate::PreparedOperationEvidence,
            plan: $plan_type,
        }

        impl $prepared_type {
            /// Binds prepared access evidence to a validated plan.
            pub fn from_plan(
                plan: $plan_type,
                access: $crate::PreparedAccessEvidence,
            ) -> Result<Self, $crate::PreparationFailure> {
                let operation = $crate::PreparedOperationEvidence::from_plan(
                    plan.preflight().clone(),
                    access,
                )?;
                Ok(Self { operation, plan })
            }

            #[must_use]
            /// Returns the shared prepared-operation evidence.
            pub const fn operation(&self) -> &$crate::PreparedOperationEvidence {
                &self.operation
            }

            #[must_use]
            /// Returns the validated plan.
            pub const fn plan(&self) -> &$plan_type {
                &self.plan
            }
        }
    };
    (@requests {
        plan_type: $plan_type:ident,
        agreement: $agreement:ty,
        agreement_doc: $agreement_doc:literal,
        scope: $scope:expr,
        ns: $ns:literal,
        requests: [
            $(
                $request:ident = $request_doc:literal {
                    new_doc: $new_doc:literal,
                    new_arg: $new_arg:ident : $new_arg_ty:ty,
                    agreement_expr: $agreement_expr:expr,
                    from_plan_doc: $from_plan_doc:literal,
                    from_plan_arg: $from_plan_arg:ident,
                    request_id_doc: $request_id_doc:literal,
                    extra: $extra:expr,
                    extra_code: $extra_code:literal,
                    extra_message: $extra_message:literal,
                }
            )*
        ]
    }) => {
        $(
            #[doc = $request_doc]
            #[derive(Clone, Debug)]
            pub struct $request {
                request_id: $crate::RequestId,
                agreement: $agreement,
                cancellation: std::sync::Arc<$crate::ImmediateCancellation>,
            }

            impl $request {
                #[doc = $new_doc]
                pub fn new(
                    request_id: $crate::RequestId,
                    $new_arg: $new_arg_ty,
                    cancellation: std::sync::Arc<$crate::ImmediateCancellation>,
                ) -> Result<Self, $crate::RuntimeFailure> {
                    let agreement = $agreement_expr;
                    if !($extra) {
                        return Err($crate::plan_family::failure(
                            $extra_code,
                            $extra_message,
                        ));
                    }
                    if cancellation.scope()
                        != $scope
                    {
                        return Err($crate::plan_family::failure(
                            concat!($ns, ".cancellation_scope_mismatch"),
                            "Request has the wrong cancellation scope",
                        ));
                    }
                    Ok(Self {
                        request_id,
                        agreement,
                        cancellation,
                    })
                }

                #[doc = $from_plan_doc]
                pub fn from_plan(
                    request_id: $crate::RequestId,
                    plan: &$plan_type,
                ) -> Result<Self, $crate::RuntimeFailure> {
                    plan_family!(@from_plan $from_plan_arg {
                        request_id: request_id,
                        plan: plan,
                        scope: $scope
                    })
                }

                #[doc = $request_id_doc]
                #[must_use]
                pub const fn request_id(&self) -> &$crate::RequestId {
                    &self.request_id
                }

                #[doc = $agreement_doc]
                #[must_use]
                pub const fn agreement(&self) -> &$agreement {
                    &self.agreement
                }

                #[must_use]
                /// Returns the role-scoped cancellation control.
                pub const fn cancellation(&self) -> &std::sync::Arc<$crate::ImmediateCancellation> {
                    &self.cancellation
                }
            }
        )*
    };
    (@from_plan pass_plan { request_id: $request_id:ident, plan: $plan:ident, scope: $scope:expr }) => {
        Self::new(
            $request_id,
            $plan,
            std::sync::Arc::new($crate::ImmediateCancellation::new($scope)),
        )
    };
    (@from_plan plan_agreement { request_id: $request_id:ident, plan: $plan:ident, scope: $scope:expr }) => {
        Self::new(
            $request_id,
            $plan.agreement().clone(),
            std::sync::Arc::new($crate::ImmediateCancellation::new($scope)),
        )
    };
}

pub(crate) use plan_family;
