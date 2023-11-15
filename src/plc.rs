use std::collections::HashMap;

use anyhow::Result;
use chrono::{DateTime, Utc};
use libipld::Cid;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use ureq::{Agent, Proxy};

use crate::api::DidDoc;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlcOperation {
  pub rotation_keys: Vec<String>,
  pub verification_methods: HashMap<String, String>,
  pub also_known_as: Vec<String>,
  pub services: HashMap<String, HashMap<String, String>>,
  pub prev: Option<String>,
  pub sig: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlcTombstone {
  pub prev: String,
  pub sig: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Create {
  pub signing_key: String,
  pub recovery_key: String,
  pub handle: String,
  pub service: String,
  pub prev: Option<String>,
  pub sig: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "$type", rename_all = "camelCase")]
pub enum PlcOp {
  PlcOperation(PlcOperation),
  PlcTombstone(PlcTombstone),
  Create(Create),
}

impl Default for PlcOp {
  fn default() -> Self {
    Self::PlcOperation(PlcOperation::default())
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LogEntry {
  pub did: String,
  pub operation: PlcOp,
  pub cid: Cid,
  pub nullified: bool,
  pub created_at: DateTime<Utc>,
}

pub struct Plc {
  host: String,
  agent: Agent,
}

impl Default for Plc {
  fn default() -> Self {
    match std::env::var("HTTPS_PROXY")
      .ok()
      .or_else(|| std::env::var("https_proxy").ok())
    {
      Some(proxy) => Self {
        host: String::from("plc.directory"),
        agent: ureq::builder().proxy(Proxy::new(proxy).unwrap()).build(),
      },
      None => Self {
        host: String::from("plc.directory"),
        agent: ureq::agent(),
      },
    }
  }
}

impl Plc {
  pub fn resolve_did(&self, did: &str) -> Result<DidDoc> {
    Ok(
      self
        .agent
        .get(&format!("https://{}/{}", self.host, did))
        .call()?
        .into_json()?,
    )
  }

  pub fn create_plc_op(&self, did: &str, op: &PlcOp) -> Result<Value> {
    Ok(
      self
        .agent
        .post(&format!("https://{}/{}", self.host, did))
        .send_json(op)?
        .into_json()?,
    )
  }

  pub fn get_plc_op_log(&self, did: &str) -> Result<Vec<PlcOp>> {
    Ok(
      self
        .agent
        .get(&format!("https://{}/{}/log", self.host, did))
        .call()?
        .into_json()?,
    )
  }

  pub fn get_plc_audit_log(&self, did: &str) -> Result<Vec<LogEntry>> {
    Ok(
      self
        .agent
        .get(&format!("https://{}/{}/log/audit", self.host, did))
        .call()?
        .into_json()?,
    )
  }

  pub fn get_last_op(&self, did: &str) -> Result<LogEntry> {
    Ok(
      self
        .agent
        .get(&format!("https://{}/{}/log/last", self.host, did))
        .call()?
        .into_json()?,
    )
  }

  pub fn get_plc_data(&self, did: &str) -> Result<Value> {
    Ok(
      self
        .agent
        .get(&format!("https://{}/{}/data", self.host, did))
        .call()?
        .into_json()?,
    )
  }

  pub fn export(&self, count: Option<i64>, after: DateTime<Utc>) -> Result<Vec<LogEntry>> {
    let mut count = count.unwrap_or(10);
    if count > 1000 {
      count = 1000;
    }
    let res = self
      .agent
      .get(&format!("https://{}/export", self.host))
      .query_pairs([
        ("count", count.to_string().as_str()),
        ("after", after.to_string().as_str()),
      ])
      .call()?
      .into_string()?;
    Ok(
      res
        .lines()
        .filter_map(|l| serde_json::from_str(l).ok())
        .collect(),
    )
  }
}
