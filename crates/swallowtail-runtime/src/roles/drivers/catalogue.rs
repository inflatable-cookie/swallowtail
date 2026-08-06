macro_rules! catalogue_driver_items {
    () => {
            /// Lists the models admitted by the supplied preflight plan.
            fn list_models(
                &self,
                plan: PreflightPlan,
                request: ModelCatalogRequest,
                services: HostServices,
            ) -> BoxFuture<'_, Result<Vec<ModelCatalogEntry>, RuntimeFailure>>;
    };
}
