#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Request {
    pub method: Method,
    pub path: String,
    pub query: Vec<(String, String)>,
    pub body: Option<Vec<u8>>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Method {
    Delete,
    Get,
    Post,
}

impl Request {
    pub(crate) fn delete(path: impl Into<String>) -> Self {
        Self {
            method: Method::Delete,
            path: path.into(),
            query: Vec::new(),
            body: None,
        }
    }

    pub(crate) fn get(path: impl Into<String>) -> Self {
        Self {
            method: Method::Get,
            path: path.into(),
            query: Vec::new(),
            body: None,
        }
    }

    pub(crate) fn post(path: impl Into<String>, body: Option<Value>) -> Self {
        Self {
            method: Method::Post,
            path: path.into(),
            query: Vec::new(),
            body: body.map(|value| serde_json::to_vec(&value).expect("JSON value serializes")),
        }
    }

    pub(crate) fn with_directory(mut self, directory: &str) -> Self {
        self.query
            .push(("directory".to_owned(), directory.to_owned()));
        self
    }

    pub(crate) fn with_query(mut self, key: &str, value: impl Into<String>) -> Self {
        self.query.push((key.to_owned(), value.into()));
        self
    }
}

#[derive(Debug)]
pub(crate) struct Response {
    pub status: u32,
    pub body: Vec<u8>,
    pub next_cursor: Option<String>,
}

#[derive(Deserialize)]
struct ProviderList {
    all: Vec<Provider>,
    #[serde(rename = "default")]
    defaults: BTreeMap<String, String>,
}

#[derive(Deserialize)]
struct Provider {
    id: String,
    models: BTreeMap<String, ProviderModel>,
}

#[derive(Deserialize)]
struct ProviderModel {
    id: String,
    name: Option<String>,
    limit: Option<ModelLimit>,
    capabilities: ModelCapabilities,
    #[serde(default)]
    variants: BTreeMap<String, Value>,
}

#[derive(Deserialize)]
struct ModelCapabilities {
    reasoning: bool,
    toolcall: bool,
}

#[derive(Clone, Copy, Deserialize)]
struct ModelLimit {
    input: Option<u64>,
    output: Option<u64>,
}

pub(crate) fn parse_catalog(response: &Response) -> Result<Vec<ModelCatalogEntry>, RuntimeFailure> {
    require_success(response, "provider catalogue request")?;
    let providers: ProviderList = parse_json(&response.body, "provider catalogue response")?;
    let mut entries = Vec::new();
    for provider in providers.all {
        let provider_id = ProviderId::new(provider.id.clone()).map_err(|_| {
            failure(
                "swallowtail.opencode.catalog_invalid",
                "OpenCode returned an invalid provider identity",
            )
        })?;
        for (key, model) in provider.models {
            if key != model.id {
                return Err(failure(
                    "swallowtail.opencode.catalog_invalid",
                    "OpenCode returned inconsistent model identities",
                ));
            }
            let model_id = ModelId::new(model.id.clone()).map_err(|_| {
                failure(
                    "swallowtail.opencode.catalog_invalid",
                    "OpenCode returned an invalid model identity",
                )
            })?;
            let mut metadata = match model.name {
                Some(name) => ModelMetadata::with_display_name(name).map_err(|_| {
                    failure(
                        "swallowtail.opencode.catalog_invalid",
                        "OpenCode returned invalid model metadata",
                    )
                })?,
                None => ModelMetadata::default(),
            };
            metadata =
                metadata.with_default(providers.defaults.get(&provider.id) == Some(&model.id));
            if let Some(limit) = model.limit {
                metadata =
                    metadata.with_token_limits(ModelTokenLimits::new(limit.input, limit.output));
            }
            let modes = model
                .variants
                .keys()
                .map(|mode| {
                    ReasoningMode::new(mode.clone()).map_err(|_| {
                        failure(
                            "swallowtail.opencode.catalog_invalid",
                            "OpenCode returned an invalid reasoning variant",
                        )
                    })
                })
                .collect::<Result<Vec<_>, _>>()?;
            if !model.capabilities.reasoning && !modes.is_empty() {
                return Err(failure(
                    "swallowtail.opencode.catalog_invalid",
                    "OpenCode returned inconsistent reasoning capability evidence",
                ));
            }
            if model.capabilities.reasoning {
                metadata = metadata.with_reasoning(ReasoningMetadata::new(modes, None));
            }
            metadata = metadata.with_catalog_observations(
                ModelCatalogObservations::new(
                    IntegrationFamilyId::new("opencode")
                        .expect("static OpenCode integration family is valid"),
                )
                .with_reasoning_supported(model.capabilities.reasoning)
                .with_tool_calling_supported(model.capabilities.toolcall),
            );
            entries.push(
                ModelCatalogEntry::new(model_id, metadata).with_provider_id(provider_id.clone()),
            );
        }
    }
    Ok(entries)
}

