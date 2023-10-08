//! Feed generator
use std::{collections::HashMap, str::FromStr, sync::Arc, thread::spawn};

use anyhow::{bail, Result};
use libipld::multibase::Base;
use serde::Serialize;
use tiny_http::{Header, Response, Server};
use url::Url;

use crate::{
  api::{
    AppBskyFeedDescribefeedgenerator, AppBskyFeedDescribefeedgeneratorFeed,
    AppBskyFeedGetfeedskeleton, ComAtprotoSyncSubscribereposCommit,
  },
  AtUri, Client,
};

/// Custom Feed
pub trait Algorithm: Sync + Send {
  fn get_name(&self) -> &str;
  fn handler(
    &self,
    limit: Option<usize>,
    cursor: Option<String>,
    access_did: Option<String>,
    jwt: Option<String>,
  ) -> AppBskyFeedGetfeedskeleton;

  fn to_feed(&self, publisher: &str) -> AppBskyFeedDescribefeedgeneratorFeed {
    AppBskyFeedDescribefeedgeneratorFeed {
      uri: AtUri::new(
        "",
        publisher,
        &format!("/app.bsky.feed.generator/{}", self.get_name()),
        "",
      )
      .to_string(),
      ..Default::default()
    }
  }
}

/// Collect PDS commits
pub trait Subscription {
  fn handler(&mut self, records: Vec<ComAtprotoSyncSubscribereposCommit>);
}

#[derive(Serialize)]
struct Service {
  id: String,
  #[serde(rename = "type")]
  service_type: String,
  #[serde(rename = "serviceEndpoint")]
  service_endpoint: String,
}

impl Service {
  fn new<T: ToString>(hostname: T) -> Self {
    Self {
      id: String::from("#bsky_fg"),
      service_type: String::from("BskyFeedGenerator"),
      service_endpoint: format!("https://{}", hostname.to_string()),
    }
  }
}

#[derive(Serialize)]
struct DidDocument {
  #[serde(rename = "@context")]
  context: Vec<String>,
  id: String,
  service: Vec<Service>,
}

impl DidDocument {
  fn new<T: ToString>(hostname: T) -> Self {
    Self {
      context: vec![String::from("https://www.w3.org/ns/did/v1")],
      id: format!("did:web:{}", hostname.to_string()),
      service: vec![Service::new(hostname)],
    }
  }
}

fn worker(server: Arc<Server>, context: Arc<Context>) {
  loop {
    let request = match server.recv() {
      Ok(r) => r,
      Err(e) => {
        log::warn!("http receive error {}", e);
        break;
      }
    };
    let url = match Url::parse("http://localhost") {
      Ok(u) => u,
      Err(e) => {
        log::warn!("url parse error {}", e);
        request
          .respond(Response::from_string(format!(r#"{{"error": "{}"}}"#, e)).with_status_code(500))
          .ok();
        break;
      }
    };
    let url = match url.join(request.url()) {
      Ok(u) => u,
      Err(e) => {
        log::warn!("url join error {}", e);
        request
          .respond(Response::from_string(format!(r#"{{"error": "{}"}}"#, e)).with_status_code(400))
          .ok();
        break;
      }
    };
    let mut paths = url.path().split("/");
    let queries = url
      .query_pairs()
      .map(|(k, v)| (k.into_owned(), v.into_owned()))
      .collect::<HashMap<_, _>>();
    let header = match Header::from_str("Content-Type: application/json") {
      Ok(h) => h,
      Err(_) => {
        log::warn!("response header error");
        request
          .respond(Response::from_string("NG").with_status_code(400))
          .ok();
        break;
      }
    };
    let mut response = Response::from_string("NG").with_status_code(404);
    match paths.nth(1) {
      Some(".well-known") => match paths.next() {
        Some("did.json") => {
          response = Response::from_string(
            match serde_json::to_string(&DidDocument::new(&context.hostname)) {
              Ok(s) => s,
              Err(e) => {
                log::warn!("json format error {}", e);
                request
                  .respond(
                    Response::from_string(format!(r#"{{"error": "{}"}}"#, e)).with_status_code(500),
                  )
                  .ok();
                break;
              }
            },
          )
          .with_header(header);
        }
        _ => (),
      },
      Some("xrpc") => match paths.next() {
        Some("app.bsky.feed.describeFeedGenerator") => {
          let rsp = AppBskyFeedDescribefeedgenerator {
            did: context.get_service_did(),
            feeds: context
              .algorithms
              .as_ref()
              .map(|algos| {
                algos
                  .iter()
                  .map(|a| a.to_feed(&context.publisher))
                  .collect::<Vec<_>>()
              })
              .unwrap_or_default(),
            links: None,
          };
          response = match serde_json::to_string(&rsp) {
            Ok(r) => Response::from_string(r).with_header(header),
            Err(e) => {
              log::warn!("json format error {}", e);
              request
                .respond(
                  Response::from_string(format!(r#"{{"error": "{}"}}"#, e)).with_status_code(500),
                )
                .ok();
              break;
            }
          };
        }
        Some("app.bsky.feed.getFeedSkeleton") => {
          let feed = queries
            .get("feed")
            .map(|f| f.clone())
            .unwrap_or_else(|| String::new());
          let mut algo = None;
          if let Some(feed) = AtUri::from_uri(feed) {
            if let Some(host) = &feed.host {
              if *host == context.publisher {
                if let Some(collection) = feed.collection() {
                  if collection == "app.bsky.feed.generator" {
                    if let Some(rkey) = feed.rkey() {
                      if let Some(algos) = &context.algorithms {
                        algo = algos.iter().find(|a| a.get_name() == rkey);
                      }
                    }
                  }
                }
              }
            };
          }
          if let Some(a) = &algo {
            let token = request.headers().iter().find_map(|header| {
              (header.field.as_str() == "authorization").then(|| header.value.to_string())
            });
            let message = token
              .as_ref()
              .and_then(|token| token.split(".").nth(1).map(|t| t.to_string()));
            let body = message.and_then(|message| Base::Base64Url.decode(message).ok());
            let json =
              body.and_then(|body| serde_json::from_slice::<serde_json::Value>(&body).ok());
            let did = json.and_then(|json| {
              json
                .get("iss")
                .map(|value| value.to_string().replace('"', ""))
            });
            let rsp = a.handler(
              queries.get("limit").and_then(|l| l.parse::<usize>().ok()),
              queries.get("cursor").cloned(),
              did,
              token,
            );
            response = match serde_json::to_string(&rsp) {
              Ok(r) => {
                log::info!("{}", r);
                Response::from_string(r).with_header(header)
              }
              Err(e) => {
                log::warn!("json format error {}", e);
                request
                  .respond(
                    Response::from_string(format!(r#"{{"error": "{}"}}"#, e)).with_status_code(500),
                  )
                  .ok();
                break;
              }
            };
          }
        }
        _ => (),
      },
      _ => (),
    }
    if let Err(e) = request.respond(response) {
      log::warn!("response send error {}", e);
      break;
    }
  }
}

struct Context {
  publisher: String,
  hostname: String,
  algorithms: Option<Vec<Box<dyn Algorithm>>>,
}

impl Context {
  fn get_service_did(&self) -> String {
    format!("did:web:{}", self.hostname)
  }
}

/// Feed generator
pub struct FeedGenerator {
  threads: usize,
  hostname: String,
  id: String,
  pw: String,
  algorithms: Option<Vec<Box<dyn Algorithm>>>,
  subscirption: Option<Box<dyn Subscription>>,
}

impl FeedGenerator {
  /// Create new feed generator
  pub fn new<T1: ToString, T2: ToString, T3: ToString>(
    id: T1,
    pw: T2,
    hostname: T3,
    threads: usize,
  ) -> Self {
    Self {
      threads,
      hostname: hostname.to_string(),
      id: id.to_string(),
      pw: pw.to_string(),
      algorithms: None,
      subscirption: None,
    }
  }

  /// Remove custom feed from feed generator
  pub fn remove_algorithm(&mut self, algorithm: &dyn Algorithm) {
    let current = self.algorithms.take().unwrap_or_default();
    let mut algorithms = Vec::new();
    for a in current.into_iter() {
      if a.get_name() != algorithm.get_name() {
        algorithms.push(a);
      }
    }
    self.algorithms = (!algorithms.is_empty()).then(|| algorithms);
  }

  /// Add custom feed from feed generator
  pub fn add_algorithm(&mut self, algorithm: Box<dyn Algorithm>) {
    self.remove_algorithm(algorithm.as_ref());
    match self.algorithms.as_mut() {
      Some(algos) => {
        algos.push(algorithm);
      }
      None => {
        self.algorithms = Some(vec![algorithm]);
      }
    }
  }

  /// Add collector for PDS commits
  pub fn set_subscription(&mut self, subscription: Box<dyn Subscription>) {
    self.subscirption = Some(subscription);
  }

  /// Start feed generator
  pub fn start(&mut self) -> Result<()> {
    let mut client = Client::default();
    client.login(&self.id, &self.pw)?;
    client.connect_ws()?;
    let publisher = match self.id.starts_with("did:plc:") {
      true => self.id.clone(),
      false => client.get_handle(&self.id)?,
    };
    let context = Arc::new(Context {
      publisher,
      hostname: self.hostname.clone(),
      algorithms: self.algorithms.take(),
    });
    std::fs::remove_file("filters.yaml").ok();
    let server = Arc::new(match Server::http("0.0.0.0:8000") {
      Ok(s) => s,
      Err(e) => bail!("{}", e),
    });
    let mut guards = Vec::new();
    for _ in 0..self.threads {
      let server = Arc::clone(&server);
      let context = Arc::clone(&context);
      guards.push(spawn(move || worker(server, context)));
    }
    loop {
      for (no, guard) in guards.iter_mut().enumerate() {
        if guard.is_finished() {
          log::warn!("restart thread {}", no);
          let server = Arc::clone(&server);
          let context = Arc::clone(&context);
          *guard = spawn(move || worker(server, context));
        }
      }
      let commits = client
        .next_event_filtered_all()?
        .into_iter()
        .filter_map(|(_, e)| e.as_commit().cloned())
        .collect::<Vec<_>>();
      if let Some(s) = &mut self.subscirption {
        s.handler(commits);
      }
    }
  }
}
