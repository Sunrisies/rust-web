use actix_utils::future::{ok, ready, Ready};
use actix_web::{dev::Payload, error::QueryPayloadError, Error, FromRequest, HttpRequest};
use serde::de::DeserializeOwned;
use std::{collections::HashMap, fmt, ops, sync::Arc};

use crate::AppError;

#[derive(Clone, Default)]
pub struct QueryConfig {
    #[allow(clippy::type_complexity)]
    err_handler: Option<Arc<dyn Fn(QueryPayloadError, &HttpRequest) -> Error + Send + Sync>>,
}

impl QueryConfig {
    /// Set custom error handler
    pub fn error_handler<F>(mut self, f: F) -> Self
    where
        F: Fn(QueryPayloadError, &HttpRequest) -> Error + Send + Sync + 'static,
    {
        self.err_handler = Some(Arc::new(f));
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]

pub struct Query<T>(pub T);

impl<T> Query<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T: DeserializeOwned + std::fmt::Debug> Query<T> {
    pub fn from_query(query_str: &str) -> Result<Self, QueryPayloadError> {
        serde_urlencoded::from_str::<T>(query_str)
            .map(Self)
            .map_err(QueryPayloadError::Deserialize)
    }
}

impl<T> ops::Deref for Query<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> ops::DerefMut for Query<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T: fmt::Display> fmt::Display for Query<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T: DeserializeOwned + std::fmt::Debug> FromRequest for Query<T> {
    type Error = Error;
    type Future = Ready<Result<Self, Error>>;

    #[inline]
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let _error_handler = req
            .app_data::<QueryConfig>()
            .and_then(|c| c.err_handler.clone());

        // 如果检测当前参数是空的话可以直接过去
        let query = req.query_string();
        log::info!("Query: {:?}", query);
        let _params: HashMap<_, _> = url::form_urlencoded::parse(query.as_bytes())
            .into_owned()
            .collect();

        serde_urlencoded::from_str::<T>(req.query_string())
            .map(|val| ok(Query(val)))
            .unwrap_or_else(move |_| {
                log::debug!(
                    "Failed during Query extractor deserialization. \
                     Request path: {:?}",
                    req.path()
                );

                let new_err =
                    AppError::Unauthorized("当前参数错误或者当前参数不能为空".to_string());
                ready(Err(new_err.into()))
            })
    }
}
