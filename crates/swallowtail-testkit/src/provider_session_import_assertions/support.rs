fn fixtures() -> [ProviderSessionImportFixture; 2] {
    [
        ProviderSessionImportFixture::local(),
        ProviderSessionImportFixture::remote_authoritative(),
    ]
}

fn standard_bounds() -> swallowtail_core::ProviderSessionCatalogueBounds {
    provider_session_catalogue_bounds(2, 4, 64, 128, 128)
}

