macro_rules! catalogue_driver_items {
    () => {
            fn list_models(
                &self,
                plan: PreflightPlan,
                request: ModelCatalogRequest,
                services: HostServices,
            ) -> BoxFuture<'_, Result<Vec<ModelCatalogEntry>, RuntimeFailure>>;
    };
}
