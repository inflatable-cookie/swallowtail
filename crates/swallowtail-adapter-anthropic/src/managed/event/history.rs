pub(crate) fn parse_history(input: &[u8]) -> Result<Vec<ManagedEvent>, RuntimeFailure> {
    let page = parse_history_page(input)?;
    if page.next_page.is_some() {
        return Err(protocol_failure("event history pagination"));
    }
    Ok(page.events)
}

pub(crate) struct ManagedHistoryPage {
    pub events: Vec<ManagedEvent>,
    pub next_page: Option<String>,
}

pub(crate) fn parse_history_page(input: &[u8]) -> Result<ManagedHistoryPage, RuntimeFailure> {
    if input.len() > MAX_STREAM_BYTES {
        return Err(protocol_failure("event history bound"));
    }
    let value: Value =
        serde_json::from_slice(input).map_err(|_| protocol_failure("event history JSON"))?;
    let next_page = match value.get("next_page") {
        None | Some(Value::Null) => None,
        Some(Value::String(page)) if !page.trim().is_empty() && page.len() <= 1_024 => {
            Some(page.clone())
        }
        _ => return Err(protocol_failure("event history pagination")),
    };
    let events = value
        .get("data")
        .and_then(Value::as_array)
        .ok_or_else(|| protocol_failure("event history data"))?
        .iter()
        .cloned()
        .map(parse_value)
        .collect::<Result<Vec<_>, _>>()?;
    Ok(ManagedHistoryPage { events, next_page })
}
