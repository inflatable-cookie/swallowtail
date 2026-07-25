use crate::error::{RemoteAcpError, capacity_error, protocol_error};
use cookie_store::{CookieStore, RawCookie};
use reqwest::header::HeaderMap;
use std::num::{NonZeroU32, NonZeroU64};
use url::Url;

pub(crate) struct BoundedCookieStore {
    store: CookieStore,
    maximum_count: usize,
    maximum_bytes: usize,
}

impl BoundedCookieStore {
    pub(crate) fn new(
        maximum_count: NonZeroU32,
        maximum_bytes: NonZeroU64,
    ) -> Result<Self, RemoteAcpError> {
        Ok(Self {
            store: CookieStore::default(),
            maximum_count: usize::try_from(maximum_count.get()).map_err(|_| capacity_error())?,
            maximum_bytes: usize::try_from(maximum_bytes.get()).map_err(|_| capacity_error())?,
        })
    }

    pub(crate) fn store_response(
        &mut self,
        headers: &HeaderMap,
        endpoint: &Url,
    ) -> Result<(), RemoteAcpError> {
        let mut candidate = self.store.clone();
        for value in headers.get_all(reqwest::header::SET_COOKIE) {
            let raw = value.to_str().map_err(|_| protocol_error())?;
            if raw.len() > self.maximum_bytes {
                return Err(capacity_error());
            }
            let cookie = RawCookie::parse(raw.to_owned())
                .map_err(|_| protocol_error())?
                .into_owned();
            candidate
                .insert_raw(&cookie, endpoint)
                .map_err(|_| protocol_error())?;
        }
        let count = candidate.iter_unexpired().count();
        let bytes = candidate
            .iter_unexpired()
            .map(|cookie| cookie.name().len().saturating_add(cookie.value().len()))
            .sum::<usize>();
        if count > self.maximum_count || bytes > self.maximum_bytes {
            return Err(capacity_error());
        }
        self.store = candidate;
        Ok(())
    }

    pub(crate) fn request_header(&self, endpoint: &Url) -> Result<Option<String>, RemoteAcpError> {
        let header = self
            .store
            .get_request_values(endpoint)
            .map(|(name, value)| format!("{name}={value}"))
            .collect::<Vec<_>>()
            .join("; ");
        if header.len() > self.maximum_bytes {
            return Err(capacity_error());
        }
        Ok((!header.is_empty()).then_some(header))
    }
}

#[cfg(test)]
mod tests {
    use super::BoundedCookieStore;
    use reqwest::header::{HeaderMap, HeaderValue, SET_COOKIE};
    use std::num::{NonZeroU32, NonZeroU64};
    use url::Url;

    #[test]
    fn cookies_are_scoped_bounded_and_never_debugged() {
        let endpoint = Url::parse("https://agent.example.test/acp").unwrap();
        let mut store =
            BoundedCookieStore::new(NonZeroU32::new(2).unwrap(), NonZeroU64::new(128).unwrap())
                .unwrap();
        let mut headers = HeaderMap::new();
        headers.append(
            SET_COOKIE,
            HeaderValue::from_static("affinity=private; Secure; HttpOnly; Path=/"),
        );
        store.store_response(&headers, &endpoint).unwrap();
        assert_eq!(
            store.request_header(&endpoint).unwrap().as_deref(),
            Some("affinity=private")
        );
        let other = Url::parse("https://other.example.test/acp").unwrap();
        assert!(store.request_header(&other).unwrap().is_none());
    }
}
