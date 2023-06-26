//! HTTP and WebSocket clients to connect to Bluesky
use std::{collections::HashMap, fs::File, net::TcpStream};

use anyhow::{bail, Result};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tungstenite::{stream::MaybeTlsStream, Message, WebSocket};
use ureq::{Agent, AgentBuilder, Proxy};

use crate::{Event, Filters};

/// Client to use Bluesky server
pub struct Client {
  agent: Agent,
  host: String,
  repo_store: HashMap<String, Repo>,
  handle_store: HashMap<String, String>,
  ws: Option<WebSocket<MaybeTlsStream<TcpStream>>>,
  filters: Filters,
}

impl Default for Client {
  fn default() -> Self {
    let mut agent = AgentBuilder::new();
    if let Some(proxy) = std::env::var("HTTPS_PROXY")
      .ok()
      .or_else(|| std::env::var("https_proxy").ok())
    {
      if let Ok(p) = Proxy::new(proxy) {
        agent = agent.proxy(p);
      }
    }
    let mut client = Self {
      agent: agent.build(),
      host: String::from("bsky.social"),
      repo_store: HashMap::new(),
      handle_store: HashMap::new(),
      ws: None,
      filters: Filters::default(),
    };
    let mut filters = File::open("filters.yaml")
      .ok()
      .and_then(|y| serde_yaml::from_reader::<_, Filters>(y).ok())
      .unwrap_or_default();
    filters.init(&mut client);
    if let Ok(file) = File::create("filters.yaml") {
      if let Err(e) = serde_yaml::to_writer(file, &filters) {
        log::warn!("Filter save error {}", e);
      }
    }
    client.filters = filters;
    client
  }
}

impl Client {
  /// Connect to WebSocket
  pub fn connect_ws(&mut self) -> Result<()> {
    self.ws = Some(
      tungstenite::connect(&format!(
        "wss://{}/xrpc/com.atproto.sync.subscribeRepos",
        self.host
      ))?
      .0,
    );
    Ok(())
  }

  /// Receive next event from WebSocket
  pub fn next_event(&mut self) -> Result<Event> {
    let Some(ws) = self.ws.as_mut() else {
      bail!("WebSocket is not connected");
    };
    if let Message::Binary(b) = ws.read_message()? {
      Ok(Event::from(b.as_slice()))
    } else {
      Ok(Event::default())
    }
  }

  /// Receive event from WebSocket, apply filter, and return event with name of matching filter
  pub fn next_event_filtered_all(&mut self) -> Result<Vec<(String, Event)>> {
    let filters = self.filters.get_filters();
    let mut ret = Vec::new();
    loop {
      let event = self.next_event()?;
      for filter in filters.iter() {
        if filter.is_match(&event) {
          ret.push((filter.name.clone(), event.clone()));
        }
      }
      if !ret.is_empty() {
        return Ok(ret);
      }
    }
  }

  /// Returns events from WebSocket that match the specified filter
  pub fn next_event_filtered<T: ToString>(&mut self, name: T) -> Result<Event> {
    let name = name.to_string();
    loop {
      if let Some((_, event)) = self
        .next_event_filtered_all()?
        .into_iter()
        .find(|(n, _)| *n == name)
      {
        return Ok(event);
      }
    }
  }

  fn request<S: Serialize, D: DeserializeOwned>(
    &self,
    method: &str,
    xrpc: &str,
    query: &[(&str, String)],
    body: Option<S>,
  ) -> Result<D> {
    let mut request = self
      .agent
      .request(method, &format!("https://{}/xrpc/{}", self.host, xrpc));
    if !query.is_empty() {
      request = request.query_pairs(query.iter().map(|(k, v)| (*k, v.as_str())));
    }
    if let Some(b) = body {
      Ok(request.send_json(b)?.into_json::<D>()?)
    } else {
      Ok(request.call()?.into_json::<D>()?)
    }
  }

  /// Call com.atproto.repo.describeRepo to return repository information
  pub fn describe_repo<T: ToString>(&self, did: T) -> Result<Repo> {
    Ok(self.request(
      "GET",
      "com.atproto.repo.describeRepo",
      &[("repo", did.to_string())],
      None::<&str>,
    )?)
  }

  /// Return repository information via cache
  pub fn get_repo<T: ToString>(&mut self, did: T) -> Result<Repo> {
    let did = did.to_string();
    match self.repo_store.get(&did) {
      Some(r) => Ok(r.clone()),
      None => {
        let repo = self.describe_repo(&did)?;
        self.repo_store.insert(did, repo.clone());
        Ok(repo)
      }
    }
  }

  /// Call com.atproto.identity.resolveHandle to return DID information
  pub fn resolve_handle<T: ToString>(&self, handle: T) -> Result<String> {
    let result = self.request::<&str, HashMap<String, String>>(
      "GET",
      "com.atproto.identity.resolveHandle",
      &[("handle", handle.to_string())],
      None,
    )?;
    Ok(match result.get("did") {
      Some(d) => d.clone(),
      None => bail!("No such handle"),
    })
  }

  /// Return DID information via cache
  pub fn get_handle<T: ToString>(&mut self, handle: T) -> Result<String> {
    let handle = handle.to_string();
    match self.handle_store.get(&handle) {
      Some(h) => Ok(h.clone()),
      None => {
        let did = self.resolve_handle(&handle)?;
        self.handle_store.insert(handle, did.clone());
        Ok(did)
      }
    }
  }
}

/// Repository Information
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Repo {
  pub handle: String,
  pub did: String,
  pub collections: Vec<String>,
  pub handle_is_correct: bool,
}
