//! HTTP and WebSocket clients to connect to Bluesky
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{sleep, spawn, JoinHandle};
use std::time::Duration;

use anyhow::{anyhow, bail, Result};
use chrono::{DateTime, Datelike, Utc};
use serde::Deserialize;
use tungstenite::Message;

use crate::api::*;
use crate::{Event, Filters};

/// Client to use Bluesky server
pub struct Client {
  pub client: crate::api::Client,
  repo: Option<String>,
  repo_store: Arc<Mutex<HashMap<String, ComAtprotoRepoDescriberepo>>>,
  handle_store: Arc<Mutex<HashMap<String, String>>>,
  thread: Option<JoinHandle<()>>,
  rx: Arc<Mutex<HashMap<String, Receiver<Event>>>>,
  last_received: Arc<Mutex<DateTime<Utc>>>,
  filters: Arc<Mutex<Filters>>,
  timeout: chrono::Duration,
}

impl Clone for Client {
  fn clone(&self) -> Self {
    Self {
      client: crate::api::Client::new(self.client.get_host(), self.client.get_proxy()),
      repo: None,
      repo_store: Arc::clone(&self.repo_store),
      handle_store: Arc::clone(&self.handle_store),
      thread: None,
      rx: Arc::clone(&self.rx),
      last_received: Arc::new(Mutex::new(DateTime::default())),
      filters: Arc::clone(&self.filters),
      timeout: self.timeout.clone(),
    }
  }
}

impl Default for Client {
  fn default() -> Self {
    let proxy = std::env::var("HTTPS_PROXY")
      .ok()
      .or_else(|| std::env::var("https_proxy").ok());
    let mut client = Self {
      client: crate::api::Client::new("bsky.social", proxy),
      repo: None,
      repo_store: Arc::new(Mutex::new(HashMap::new())),
      handle_store: Arc::new(Mutex::new(HashMap::new())),
      thread: None,
      rx: Arc::new(Mutex::new(HashMap::new())),
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
  host: String,
  last_received: Arc<Mutex<DateTime<Utc>>>,
  mut tx_map: HashMap<String, Sender<Event>>,
  filters: Arc<Mutex<Filters>>,
) {
  let mut last_seq = None;
  let mut is_terminating = false;
  loop {
    let client = crate::api::Client::new(&host, None::<&str>);
    let mut ws = match client.com_atproto_sync_subscriberepos(last_seq) {
      Ok(res) => res,
      Err(e) => {
        log::warn!("WebSocket connect error : {}", e);
        break;
      }
    };
    log::info!("websocket connected");
    loop {
      log::debug!("WAIT WEBSOCKET MESSAGE");
      match ws.read() {
        Ok(Message::Binary(b)) => {
          log::debug!("RECEIVED BINARY MESSAGE");
          let event = match Event::try_from(b.as_slice()) {
            Ok(e) => e,
            Err(e) => {
              log::debug!("{}", e);
              continue;
            }
          };
          if let Some(seq) = event.get_seq() {
            last_seq = Some(seq);
          }
          if let Some(time) = event.get_time() {
            if let Ok(mut write) = last_received.lock() {
              *write = time;
            }
          }
          if let Some(tx) = tx_map.get_mut("") {
            if tx.send(event).is_err() {
              log::warn!("Already new receiver thread is created");
              is_terminating = true;
              break;
            }
          } else {
            if let Ok(filters) = filters.lock() {
              for filter in filters.get_filters().iter() {
                if filter.is_match(&event) {
                  if let Some(tx) = tx_map.get_mut(&filter.name) {
                    if tx.send(event.clone()).is_err() {
                      log::warn!("Already new receiver thread is created");
                      is_terminating = true;
                      break;
                    }
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
    self.client = crate::api::Client::new(host, self.client.get_proxy());
  }

  /// Set timeout for waiting to receive WebSocket events
  pub fn set_timeout(&mut self, seconds: i64) {
    self.timeout = chrono::Duration::seconds(seconds);
  }

  /// Login to Bluesky server
  pub fn login<T1: ToString, T2: ToString>(&mut self, id: T1, pw: T2) -> Result<()> {
    let id = id.to_string();
    let session = self
      .client
      .com_atproto_server_createsession(&id, &pw.to_string())?;
    self.client.set_jwt(Some(session.access_jwt));
    self.repo = self
      .client
      .com_atproto_identity_resolvehandle(&id)
      .ok()
      .map(|h| h.did);
    Ok(())
  }

  /// Post text to Bluesky server
  pub fn post<T: ToString>(&self, text: T) -> Result<()> {
    let repo = self.repo.as_ref().ok_or_else(|| anyhow!("no login"))?;
    let post = AppBskyFeedPost {
      text: text.to_string(),
      created_at: Utc::now(),
      ..Default::default()
    };
    let record = Record::AppBskyFeedPost(post);
    self.client.com_atproto_repo_createrecord(
      repo,
      "app.bsky.feed.post",
      &record,
      None,
      None,
      None,
    )?;
    Ok(())
  }

  /// Post image to Bluesky server
  pub fn post_image<T1: ToString, P: AsRef<Path>, T2: ToString>(
    &self,
    text: T1,
    file: P,
    content_type: T2,
  ) -> Result<()> {
    let repo = self.repo.as_ref().ok_or_else(|| anyhow!("no login"))?;
    let file = std::fs::read(file)?;
    let blob = self
      .client
      .com_atproto_repo_uploadblob(&file, &content_type.to_string())?;
    let image = AppBskyEmbedImagesImage {
      alt: text.to_string(),
      image: blob.blob.clone(),
    };
    let images = AppBskyEmbedImages {
      images: vec![image],
    };
    let embed = Some(AppBskyFeedPostMainEmbed::AppBskyEmbedImages(Box::new(
      images,
    )));
    let post = AppBskyFeedPost {
      text: text.to_string(),
      created_at: Utc::now(),
      embed,
      ..Default::default()
    };
    let record = Record::AppBskyFeedPost(post);
    self.client.com_atproto_repo_createrecord(
      repo,
      "app.bsky.feed.post",
      &record,
      None,
      None,
      None,
    )?;
    Ok(())
  }

  /// Connect to WebSocket
  pub fn connect_ws(&mut self) -> Result<()> {
    let host = self.client.get_host();
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
    self.rx = Arc::new(Mutex::new(rx_map));
    self.thread = Some(spawn(move || {
      receiver_thread(host, last_received, tx_map, filters);
    }));
    Ok(())
  }

  fn check_and_restart_websocket(&mut self) -> Result<bool> {
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
      self.connect_ws()?;
      return Ok(true);
    }
    Ok(false)
  }

  /// Receive event from WebSocket, apply filter, and return event with name of matching filter
  pub fn next_event_filtered_all(&mut self) -> Result<Vec<(String, Event)>> {
    let mut ret = Vec::new();
    match self.rx.lock() {
      Ok(rx_map) => {
        for (name, rx) in rx_map.iter() {
          while let Ok(event) = rx.try_recv() {
            ret.push((name.clone(), event));
          }
        }
      }
      Err(e) => bail!("{}", e),
    }
    if !self.check_and_restart_websocket()? && ret.is_empty() {
      sleep(Duration::from_millis(10));
    }
    Ok(ret)
  }

  /// Returns events from WebSocket that match the specified filter
  pub fn next_event_filtered<T: ToString>(&mut self, name: T) -> Result<Event> {
    let mut event = Event::default();
    match self.rx.lock() {
      Ok(rx_map) => {
        if let Some(rx) = rx_map.get(&name.to_string()) {
          if let Ok(e) = rx.try_recv() {
            event = e;
          };
        } else {
          bail!("no such name filter");
        }
      }
      Err(e) => bail!("{}", e),
    }
    if !self.check_and_restart_websocket()? {
      sleep(Duration::from_millis(10));
    }
    Ok(event)
  }

  /// Receive next event from WebSocket
  pub fn next_event(&mut self) -> Result<Event> {
    self.next_event_filtered("")
  }

  /// Return repository information via cache
  pub fn get_repo<T: ToString>(&mut self, did: T) -> Result<ComAtprotoRepoDescriberepo> {
    let did = did.to_string();
    if let Ok(mut repo_store) = self.repo_store.lock() {
      match repo_store.get(&did) {
        Some(r) => {
          return Ok(r.clone());
        }
        None => {
          let repo = self.client.com_atproto_repo_describerepo(&did)?;
          repo_store.insert(did, repo.clone());
          return Ok(repo);
        }
      }
    }
    self.get_repo(did)
  }

  /// Return DID information via cache
  pub fn get_handle<T: ToString>(&mut self, handle: T) -> Result<String> {
    let handle = handle.to_string();
    if let Ok(mut handle_store) = self.handle_store.lock() {
      match handle_store.get(&handle) {
        Some(h) => {
          return Ok(h.clone());
        }
        None => {
          let did = self.client.com_atproto_identity_resolvehandle(&handle)?;
          handle_store.insert(handle, did.did.clone());
          return Ok(did.did.clone());
        }
      }
    }
    Ok(
      self
        .client
        .com_atproto_identity_resolvehandle(&handle)?
        .did
        .clone(),
    )
  }

  /// Get Filter names
  pub fn get_filter_names(&self) -> Vec<String> {
    match self.filters.lock() {
      Ok(filters) => filters.get_filters().into_iter().map(|f| f.name).collect(),
      Err(_) => vec![String::from("")],
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

  /// Add a Timeline Filter of user given by name
  pub fn add_timeline<T: ToString>(&mut self, handle: T) -> Result<()> {
    let filters = match self.filters.lock() {
      Ok(filters) => Arc::new(Mutex::new(filters.add_timeline(&self.client, handle)?)),
      _ => self.filters.clone(),
    };
    self.filters = filters;
    self.save_filters();
    Ok(())
  }

  /// Remove a Timeline Filter of user given by name
  pub fn remove_timeline<T: ToString>(&mut self, handle: T) {
    match self.filters.lock() {
      Ok(mut filters) => {
        filters.remove_timeline(handle);
      }
      _ => (),
    }
    self.save_filters();
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
