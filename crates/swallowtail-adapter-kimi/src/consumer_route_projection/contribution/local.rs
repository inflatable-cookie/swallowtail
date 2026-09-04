use super::super::builder::{Projection, Route};
use crate::{
    KimiLocalServerPreparedArchive, KimiLocalServerPreparedBindingImport,
    KimiLocalServerPreparedCatalogue, KimiLocalServerPreparedReconciliation,
    KimiLocalServerPreparedRestore, KimiLocalServerPreparedRun, KimiLocalServerPreparedSession,
};
use swallowtail_runtime::{
    ConsumerRouteOmissionSemantics, ConsumerRouteProjectionContribution,
    ConsumerRouteProjectionFailure, ConsumerRouteProjectionSourceId,
};

impl KimiLocalServerPreparedCatalogue {
    /// Emits exact prepared local-server model-catalogue truth.
    pub fn consumer_route_projection_contribution(
        &self,
        source_id: ConsumerRouteProjectionSourceId,
    ) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
        Projection::prepared(self.plan(), Route::Local, source_id)
            .capabilities()
            .build()
    }
}

impl KimiLocalServerPreparedRun {
    /// Emits exact prepared local-server structured-run truth.
    pub fn consumer_route_projection_contribution(
        &self,
        source_id: ConsumerRouteProjectionSourceId,
    ) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
        let configuration = self.configuration();
        let mut projection = Projection::prepared(self.plan(), Route::Local, source_id)
            .capabilities()
            .named_feature("feature.provider-managed-recovery")
            .portable_feature(swallowtail_runtime::ConsumerRouteFeatureId::PersistentSessionPosture)
            .model_selection()
            .reasoning_control(
                self.request()
                    .policy()
                    .reasoning_mode()
                    .map(|value| value.as_str()),
                false,
            )
            .named_control(
                "control.managed-recovery",
                "accepted",
                ConsumerRouteOmissionSemantics::Required,
            )
            .named_control(
                "control.permission-mode",
                permission(configuration.permission_mode()),
                ConsumerRouteOmissionSemantics::Required,
            )
            .named_control(
                "control.provider-profile",
                configuration.profile().unwrap_or("provider default"),
                ConsumerRouteOmissionSemantics::PreservesRouteBehavior,
            )
            .named_control(
                "control.disabled-tools",
                "bounded disabled-tool set",
                ConsumerRouteOmissionSemantics::SuppliesNothing,
            );
        if configuration.maximum_reattachments() != 0 {
            projection = projection.named_control(
                "control.stream-reattachment",
                "at most one reattachment",
                ConsumerRouteOmissionSemantics::SuppliesNothing,
            );
        }
        projection.build()
    }
}

impl KimiLocalServerPreparedSession {
    /// Emits exact prepared local-server interactive-session truth.
    pub fn consumer_route_projection_contribution(
        &self,
        source_id: ConsumerRouteProjectionSourceId,
    ) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
        let configuration = self.configuration();
        let mut projection = Projection::prepared(self.plan(), Route::Local, source_id)
            .capabilities()
            .portable_feature(swallowtail_runtime::ConsumerRouteFeatureId::PersistentSessionPosture)
            .model_selection()
            .reasoning_control(
                self.request()
                    .options()
                    .reasoning_mode()
                    .map(|value| value.as_str()),
                false,
            )
            .named_control(
                "control.permission-mode",
                permission(configuration.permission_mode()),
                ConsumerRouteOmissionSemantics::Required,
            )
            .named_control(
                "control.provider-profile",
                configuration.profile().unwrap_or("provider default"),
                ConsumerRouteOmissionSemantics::PreservesRouteBehavior,
            )
            .named_control(
                "control.disabled-tools",
                "bounded disabled-tool set",
                ConsumerRouteOmissionSemantics::SuppliesNothing,
            );
        if configuration.permission_mode() == crate::KimiLocalServerPermissionMode::Manual {
            projection = projection
                .named_feature("feature.permission-exchange")
                .named_feature("feature.question-exchange");
        }
        if configuration.active_turn_detachment() {
            projection = projection.named_control(
                "control.active-turn-detachment",
                "enabled",
                ConsumerRouteOmissionSemantics::SuppliesNothing,
            );
        }
        projection.build()
    }
}

macro_rules! simple_projection {
    ($type:ty) => {
        impl $type {
            /// Emits exact prepared local-server operation truth.
            pub fn consumer_route_projection_contribution(
                &self,
                source_id: ConsumerRouteProjectionSourceId,
            ) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
                Projection::prepared(self.plan().preflight(), Route::Local, source_id)
                    .capabilities()
                    .build()
            }
        }
    };
}

simple_projection!(KimiLocalServerPreparedArchive);
simple_projection!(KimiLocalServerPreparedRestore);
simple_projection!(KimiLocalServerPreparedReconciliation);

impl KimiLocalServerPreparedBindingImport {
    /// Emits only the exact prepared binding-import facade truth.
    pub fn consumer_route_projection_contribution(
        &self,
        source_id: ConsumerRouteProjectionSourceId,
    ) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
        Projection::prepared(self.projection_plan(), Route::Local, source_id)
            .capabilities()
            .build()
    }
}

fn permission(value: crate::KimiLocalServerPermissionMode) -> &'static str {
    match value {
        crate::KimiLocalServerPermissionMode::Manual => "manual",
        crate::KimiLocalServerPermissionMode::Auto => "auto",
        crate::KimiLocalServerPermissionMode::Yolo => "yolo",
    }
}
