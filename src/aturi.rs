//! at:// Uri
use std::fmt::Display;

use indexmap::IndexMap;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

const ATP_URI_REGEX: &str = r#"^(at://)?((?:did:[a-zA-Z0-9:%-]+)|(?:[a-zA-Z0-9][a-zA-Z0-9.:-]*))(/[^?#\s]*)?(\?[^#\s]+)?(#[^\s]+)?$"#;

/// at:// Uri
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AtUri {
  pub hash: Option<String>,
  pub host: Option<String>,
  pub pathname: Option<String>,
  pub search_params: Option<IndexMap<String, String>>,
}

impl Display for AtUri {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let host = self.host.as_ref().map(|h| h.as_str()).unwrap_or_default();
    let path = self
      .pathname
      .as_ref()
      .map(|p| match p.starts_with("/") {
        true => p.clone(),
        false => format!("/{}", p),
      })
      .unwrap_or(String::from("/"));
    let qs = self
      .search_params
      .as_ref()
      .map(|sp| {
        format!(
          "?{}",
          sp.iter()
            .map(|(k, v)| format!("{}={}", urlencoding::encode(k), urlencoding::encode(v)))
            .collect::<Vec<_>>()
            .join("&")
        )
      })
      .unwrap_or_default();
    let hash = self
      .hash
      .as_ref()
      .map(|h| match h.starts_with("#") {
        true => h.clone(),
        false => format!("#{}", h),
      })
      .unwrap_or_default();
    f.write_fmt(format_args!("at://{host}{path}{qs}{hash}"))
  }
}

impl AtUri {
  /// Create new at:// Uri
  pub fn new(hash: &str, host: &str, pathname: &str, search_params: &str) -> Self {
    Self {
      hash: (!hash.is_empty()).then(|| hash.to_string()),
      host: (!host.is_empty()).then(|| host.to_string()),
      pathname: (!pathname.is_empty()).then(|| pathname.to_string()),
      search_params: (!search_params.is_empty()).then(|| {
        url::form_urlencoded::parse(search_params.as_bytes())
          .into_iter()
          .map(|(k, v)| (k.to_string(), v.to_string()))
          .collect::<IndexMap<String, String>>()
      }),
    }
  }

  /// Parse at:// Uri
  pub fn from_uri<T: ToString>(uri: T) -> Option<Self> {
    let re = Regex::new(ATP_URI_REGEX).ok()?;
    let uri = uri.to_string();
    let caps = re.captures(&uri)?;
    Some(Self::new(
      caps.get(5).map(|c| c.as_str()).unwrap_or_default(),
      caps.get(2).map(|c| c.as_str()).unwrap_or_default(),
      caps.get(3).map(|c| c.as_str()).unwrap_or_default(),
      caps
        .get(4)
        .and_then(|c| c.as_str().get(1..))
        .unwrap_or_default(),
    ))
  }

  /// Returns at:// Uri first path
  pub fn collection(&self) -> Option<&str> {
    self.pathname.as_ref().and_then(|p| p.split("/").nth(1))
  }

  /// Returns at:// Uri second path
  pub fn rkey(&self) -> Option<&str> {
    self.pathname.as_ref().and_then(|p| p.split("/").nth(2))
  }
}
