impl OpenCodePreparedIntegration {
    fn prepare_catalogue_inner(
        &self,
        input: OpenCodeCatalogueProfileInput,
    ) -> Result<OpenCodePreparedCatalogue, PreparationFailure> {
        let capabilities =
            CapabilityProfile::new([CapabilityRequirement::new(Capability::ModelCatalog, [])]);
        let instance = instance_with_capabilities(self, capabilities.clone());
        let requirements = requirements(
            self,
            DriverRole::ModelCatalog,
            capabilities.iter().map(|(capability, constraints)| {
                CapabilityRequirement::new(capability, constraints.iter().cloned())
            }),
            false,
            false,
        );
        let plan = build_plan(self, &instance, None, &requirements)?;
        let (request_id, deadline) = input.into_parts();
        let request = match deadline {
            Some(deadline) => ModelCatalogRequest::new(request_id).with_deadline(deadline),
            None => ModelCatalogRequest::new(request_id),
        };
        Ok(OpenCodePreparedCatalogue {
            evidence: OpenCodePreparedEvidence::from_prepared(self, plan)?,
            request,
        })
    }

    fn prepare_session_inner(
        &self,
        input: OpenCodeSessionProfileInput,
    ) -> Result<OpenCodePreparedSession, PreparationFailure> {
        let (request_id, model, working_resource, deadline, image_attachments, provider_callbacks) =
            input.into_parts();
        let capabilities = crate::prepared::all_capabilities();
        let activity_profile = crate::activity::profile::activity_profile(self)?;
        let session_capabilities = crate::activity::profile::with_activity(
            callback_resource_access(
                CapabilityProfile::new(
                    capabilities
                        .iter()
                        .filter(|(capability, _)| {
                            !matches!(
                                *capability,
                                Capability::ModelCatalog | Capability::ProviderSessionDelete
                            ) && (image_attachments || *capability != Capability::Attachments)
                        })
                        .map(|(capability, constraints)| {
                            CapabilityRequirement::new(capability, constraints.iter().cloned())
                        }),
                ),
                provider_callbacks,
            ),
            &activity_profile,
        );
        let instance = instance_with_capabilities(self, session_capabilities.clone());
        let (route_id, route_revision, provider_id, model_id, _) = model.into_parts();
        let route = ModelRoute::new(
            route_id,
            route_revision,
            self.instance().id().clone(),
            model_id,
            session_capabilities.clone(),
        )
        .with_provider_id(provider_id);
        let requirements = requirements(
            self,
            DriverRole::InteractiveSession,
            session_capabilities
                .iter()
                .map(|(capability, constraints)| {
                    CapabilityRequirement::new(capability, constraints.iter().cloned())
                }),
            image_attachments,
            provider_callbacks,
        );
        let plan = build_plan(self, &instance, Some(&route), &requirements)?;
        let request = OpenSessionRequest::from_plan(&plan, request_id, working_resource, deadline)?;
        Ok(OpenCodePreparedSession {
            evidence: OpenCodePreparedEvidence::from_prepared_with_activity(
                self,
                plan,
                activity_profile,
            )?,
            request,
            management_instance: lifecycle_management_instance(self),
        })
    }

    fn prepare_run_inner(
        &self,
        input: OpenCodeRunProfileInput,
    ) -> Result<OpenCodePreparedRun, PreparationFailure> {
        let OpenCodeRunProfileParts {
            request_id,
            model,
            content,
            working_resource,
            reasoning,
            structured_output,
            deadline,
            attachments,
            provider_callbacks,
        } = input.into_parts();
        validate_attachments(&attachments)?;
        let image_attachments = !attachments.is_empty();
        let (route_id, route_revision, provider_id, model_id, catalogue_entry) = model.into_parts();
        validate_generation_controls(
            &provider_id,
            &model_id,
            catalogue_entry.as_ref(),
            reasoning.as_ref(),
            structured_output.as_ref(),
        )?;
        let activity_profile = crate::activity::profile::activity_profile(self)?;
        let capabilities = crate::activity::profile::with_activity(
            callback_resource_access(
                run_capabilities(
                    reasoning.as_ref(),
                    structured_output.as_ref(),
                    image_attachments,
                ),
                provider_callbacks,
            ),
            &activity_profile,
        );
        let instance = instance_with_capabilities(self, capabilities.clone());
        let route = ModelRoute::new(
            route_id,
            route_revision,
            self.instance().id().clone(),
            model_id,
            capabilities.clone(),
        )
        .with_provider_id(provider_id);
        let requirements = run_requirements(
            self,
            capabilities.iter().map(|(capability, constraints)| {
                CapabilityRequirement::new(capability, constraints.iter().cloned())
            }),
            image_attachments,
            provider_callbacks,
        );
        let plan = build_plan(self, &instance, Some(&route), &requirements)?;
        let mut policy = OperationPolicy::offline()
            .with_provider_retention(ProviderRetentionPolicy::TemporaryAllowed)
            .with_harness_isolation(swallowtail_core::HarnessIsolation::AmbientHost)
            .with_harness_configuration_posture(
                swallowtail_core::HarnessConfigurationPosture::Ambient,
            );
        if let Some(reasoning) = reasoning {
            policy = policy.with_reasoning_mode(reasoning);
        }
        let mut request = StructuredRunRequest::new(request_id, content, policy)
            .with_working_resource(working_resource)
            .with_attachments(attachments);
        if let Some(output) = structured_output {
            request = request.with_structured_output(output);
        }
        if let Some(deadline) = deadline {
            request = request.with_deadline(deadline);
        }
        Ok(OpenCodePreparedRun {
            evidence: OpenCodePreparedEvidence::from_prepared_with_activity(
                self,
                plan,
                activity_profile,
            )?,
            request,
        })
    }

    fn prepare_delete_session_inner(
        &self,
        input: OpenCodeSessionManagementInput,
    ) -> Result<OpenCodePreparedDelete, PreparationFailure> {
        let (request_id, binding, deadline, allow_unverified_newer) = input.into_parts();
        if !self.server().is_qualified() && !allow_unverified_newer {
            return Err(failure(
                "swallowtail.opencode.preparation.lifecycle_unverified_newer",
                "Newer unverified OpenCode deletion requires explicit acceptance",
            ));
        }
        let action = ProviderSessionManagementAction::Delete(
            ProviderSessionDeletionStrength::ProviderDataDeleted,
        );
        let capability = CapabilityRequirement::new(Capability::ProviderSessionDelete, []);
        let instance =
            instance_with_capabilities(self, CapabilityProfile::new([capability.clone()]));
        let requirements = management_requirements(self, [capability]);
        let preflight = build_plan(self, &instance, None, &requirements)?;
        let agreement = ProviderSessionManagementAgreement::new(
            binding,
            action,
            ProviderSessionInitialStateRequirement::UnarchivedOrArchived,
            ProviderSessionAffectedScope::ProviderDefinedDescendants,
            ProviderSessionActivityEvidence::CallerAssertedInactive,
            ProviderSessionCancellationPosture::BeforeDispatchOnly,
            deadline,
        );
        let plan = ProviderSessionManagementPlan::new(preflight, agreement).map_err(|_| {
            failure(
                "swallowtail.opencode.preparation.lifecycle_binding_mismatch",
                "OpenCode session-management binding does not match this prepared integration",
            )
        })?;
        let request = DeleteProviderSessionRequest::from_plan(request_id, &plan).map_err(|_| {
            failure(
                "swallowtail.opencode.preparation.lifecycle_request_invalid",
                "OpenCode delete request could not be prepared",
            )
        })?;
        Ok(OpenCodePreparedDelete {
            evidence: PreparedProviderSessionManagementEvidence::from_plan(plan)?,
            request,
        })
    }
}
