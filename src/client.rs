//! HTTP and WebSocket clients to connect to Bluesky
use std::collections::HashMap;
use std::fs::File;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{spawn, JoinHandle};
use std::time::Duration;

use anyhow::{bail, Result};
use chrono::{DateTime, Datelike, Utc};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tungstenite::Message;
use ureq::{Agent, AgentBuilder, Proxy};

use crate::{Event, Filters};

/// Client to use Bluesky server
pub struct Client {
  agent: Agent,
  host: String,
  repo_store: HashMap<String, Repo>,
  handle_store: HashMap<String, String>,
  thread: Option<JoinHandle<()>>,
  rx: HashMap<String, Receiver<Event>>,
  tx: Option<Sender<String>>,
  last_received: Arc<Mutex<DateTime<Utc>>>,
  filters: Arc<Mutex<Filters>>,
  timeout: chrono::Duration,
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
      thread: None,
      rx: HashMap::new(),
      tx: None,
      last_received: Arc::new(Mutex::new(DateTime::default())),
      filters: Arc::new(Mutex::new(Filters::default())),
      timeout: chrono::Duration::seconds(60),
    };
    let mut filters = File::open("filters.yaml")
      .ok()
      .and_then(|y| serde_yaml::from_reader::<_, Filters>(y).ok())
      .unwrap_or_default();
    filters.init(&mut client);
    client.filters = Arc::new(Mutex::new(filters));
    client.save_filters();
    client
  }
}

fn receiver_thread(
  rx: Receiver<String>,
  host: String,
  last_received: Arc<Mutex<DateTime<Utc>>>,
  mut tx_map: HashMap<String, Sender<Event>>,
  filters: Arc<Mutex<Filters>>,
) {
  let mut last_seq = None;
  let mut is_terminating = false;
  loop {
    if let Ok(msg) = rx.try_recv() {
      if !msg.is_empty() {
        log::warn!("old thread terminate");
        break;
      }
    }
    let url = match last_seq {
      Some(seq) => format!(
        "wss://{}/xrpc/com.atproto.sync.subscribeRepos?cursor={}",
        host, seq
      ),
      None => format!("wss://{}/xrpc/com.atproto.sync.subscribeRepos", host),
    };
    log::info!("{}", url);
    let (mut ws, _) = tungstenite::connect(&url).unwrap();
    log::info!("websocket connected");
    loop {
      if let Ok(msg) = rx.try_recv() {
        if !msg.is_empty() {
          log::warn!("old thread terminate");
          is_terminating = true;
          break;
        }
      }
      log::debug!("WAIT WEBSOCKET MESSAGE");
      match ws.read_message() {
        Ok(Message::Binary(b)) => {
          log::debug!("RECEIVED BINARY MESSAGE");
          let event = Event::from(b.as_slice());
          if let Some(seq) = event.get_seq() {
            last_seq = Some(seq);
          }
          if let Some(time) = event.get_time() {
            if let Ok(mut write) = last_received.lock() {
              *write = time;
            }
          }
          if let Some(tx) = tx_map.get_mut("") {
            tx.send(event).unwrap();
          } else {
            if let Ok(filters) = filters.lock() {
              for filter in filters.get_filters().iter() {
                if filter.is_match(&event) {
                  if let Some(tx) = tx_map.get_mut(&filter.name) {
                    tx.send(event.clone()).unwrap();
                  }
                }
              }
            }
          }
        }
        Ok(m) => {
          log::debug!("RECEIVED OTHER MESSAGE {}", m);
        }
        Err(e) => {
          log::debug!("WEBSOCKET RECEIVE ERROR {}", e);
          break;
        }
      }
    }
    if is_terminating {
      break;
    }
  }
}

impl Client {
  fn save_filters(&self) {
    if let Ok(file) = File::create("filters.yaml") {
      if let Ok(filters) = self.filters.lock() {
        if let Err(e) = serde_yaml::to_writer(file, &*filters) {
          log::warn!("Filter save error {}", e);
        }
      }
    }
  }

  /// Set Host
  pub fn set_host<T: ToString>(&mut self, host: T) {
    self.host = host.to_string();
  }

  /// Set timeout for waiting to receive WebSocket events
  pub fn set_timeout(&mut self, seconds: i64) {
    self.timeout = chrono::Duration::seconds(seconds);
  }

  /// Connect to WebSocket
  pub fn connect_ws(&mut self) -> Result<()> {
    let host = self.host.clone();
    let mut tx_map = HashMap::new();
    let mut rx_map = HashMap::new();
    if let Ok(filters) = self.filters.lock() {
      let filters = filters.get_filters();
      if filters.is_empty() {
        let (tx, rx) = channel();
        tx_map.insert(String::from(""), tx);
        rx_map.insert(String::from(""), rx);
      } else {
        for filter in filters.iter().map(|f| f.name.clone()) {
          let (tx, rx) = channel();
          tx_map.insert(filter.clone(), tx);
          rx_map.insert(filter.clone(), rx);
        }
      }
    }
    let filters = Arc::clone(&self.filters);
    self.last_received = Arc::new(Mutex::new(DateTime::default()));
    let last_received = Arc::clone(&self.last_received);
    let (tx, rx) = channel();
    self.tx = Some(tx);
    self.rx = rx_map;
    self.thread = Some(spawn(move || {
      receiver_thread(rx, host, last_received, tx_map, filters);
    }));
    Ok(())
  }

  fn check_and_restart_websocket(&mut self) -> Result<()> {
    if match self.last_received.lock() {
      Ok(read) => {
        let last_received = *read;
        last_received.year() > 2000 && (Utc::now() - last_received) > self.timeout
      }
      _ => false,
    } {
      log::warn!(
        "Received nothing from WebSocket for {} seconds.",
        self.timeout.num_seconds()
      );
      std::thread::sleep(Duration::from_secs(1));
      if let Some(tx) = &self.tx {
        tx.send(String::from("terminate")).unwrap();
      }
      self.connect_ws()?;
    }
    Ok(())
  }

  /// Receive event from WebSocket, apply filter, and return event with name of matching filter
  pub fn next_event_filtered_all(&mut self) -> Result<Vec<(String, Event)>> {
    let mut ret = Vec::new();
    for (name, rx) in self.rx.iter() {
      while let Ok(event) = rx.try_recv() {
        ret.push((name.clone(), event));
      }
    }
    self.check_and_restart_websocket()?;
    Ok(ret)
  }

  /// Returns events from WebSocket that match the specified filter
  pub fn next_event_filtered<T: ToString>(&mut self, name: T) -> Result<Event> {
    if let Some(rx) = self.rx.get(&name.to_string()) {
      let event = match rx.try_recv() {
        Ok(event) => event,
        _ => Event::default(),
      };
      self.check_and_restart_websocket()?;
      Ok(event)
    } else {
      bail!("no such name filter");
    }
  }

  /// Receive next event from WebSocket
  pub fn next_event(&mut self) -> Result<Event> {
    self.next_event_filtered("")
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
    log::debug!("HTTP REQUEST {} {}", method, xrpc);
    let ret = if let Some(b) = body {
      request.send_json(b)?.into_json::<D>()?
    } else {
      request.call()?.into_json::<D>()?
    };
    log::debug!("HTTP RESPONSE {} {}", method, xrpc);
    Ok(ret)
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

  /// Add a repository to subscribe to the Filter given by name
  pub fn subscribe_repo<T1: ToString, T2: ToString>(&mut self, name: T1, did: T2) -> Result<()> {
    if let Ok(mut filters) = self.filters.lock() {
      filters.subscribe_repo(name, did)?;
    }
    self.save_filters();
    Ok(())
  }

  /// Remove a repository to subscribe to the Filter given by name
  pub fn unsubscribe_repo<T1: ToString, T2: ToString>(&mut self, name: T1, did: T2) -> Result<()> {
    if let Ok(mut filters) = self.filters.lock() {
      filters.unsubscribe_repo(name, did)?;
    }
    self.save_filters();
    Ok(())
  }

  /// Add a handle to subscribe to the Filter given by name
  pub fn subscribe_handle<T1: ToString, T2: ToString>(
    &mut self,
    name: T1,
    handle: T2,
  ) -> Result<()> {
    let mut filters = match self.filters.lock() {
      Ok(mut filters) => {
        filters.subscribe_handle(name, handle)?;
        filters.clone()
      }
      _ => Filters::default(),
    };
    filters.init(self);
    self.filters = Arc::new(Mutex::new(filters));
    self.save_filters();
    Ok(())
  }

  /// Remove a handle to subscribe to the Filter given by name
  pub fn unsubscribe_handle<T1: ToString, T2: ToString>(
    &mut self,
    name: T1,
    handle: T2,
  ) -> Result<()> {
    let name = name.to_string();
    let handle = handle.to_string();
    if let Ok(mut filters) = self.filters.lock() {
      filters.unsubscribe_handle(&name, &handle)?;
    }
    let did = self.get_handle(handle)?;
    self.unsubscribe_repo(name, did)?;
    Ok(())
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
