//! Filter to return only matched events from EventStream
//!
//! Read the file "filters.yaml" in the current directory and filter only the necessary events.
//!
//! ```yaml
//! filters:
//!   - name: bluesky team
//!     subscribes:
//!       dids:
//!         - did:plc:yk4dd2qkboz2yv6tpubpc6co
//!         - did:plc:oky5czdrnfjpqslsw2a5iclo
//!         - did:plc:q3wucypudqw4oju3ezdcv7uy
//!         - did:plc:44ybard66vv44zksje25o7dz
//!         - did:plc:tpg43qhh4lw4ksiffs4nbda3
//!         - did:plc:vjug55kidv6sye7ykr5faxxn
//!         - did:plc:l3rouwludahu3ui3bt66mfvj
//!         - did:plc:ragtjsm2j2vknwkz3zp4oxrd
//!         - did:plc:vpkhqolt662uhesyj6nxm7ys
//!         - did:plc:fgsn4gf2dlgnybo4nbej5b2s
//!         - did:plc:6fktaamhhxdqb2ypum33kbkj
//!       handles:
//!         - jay.bsky.team
//!         - pfrazee.com
//!         - why.bsky.team
//!         - iamrosewang.bsky.social
//!         - dholms.xyz
//!         - divy.zone
//!         - jakegold.us
//!         - bnewbold.net
//!         - ansh.bsky.team
//!         - emily.bsky.team
//!         - jack.bsky.social
//!     keywords:
//!       includes:
//!         - bluesky
//!       excludes:
//!         - twitter
//! ```
use std::collections::HashSet;

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::{api::*, Client, Event};

/// Filter by repository (DID or handle) to which you are subscribed
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Subscribes {
  pub dids: Option<Vec<String>>,
  pub handles: Option<Vec<String>>,
}

impl Subscribes {
  /// Returns whether or not the filter is matched
  pub fn is_match(&self, repo: &str) -> bool {
    if let Some(dids) = &self.dids {
      if dids.iter().any(|d| repo == *d) {
        return true;
      }
    }
    false
  }

  /// Add a repository to subscribe
  pub fn subscribe_repo<T: ToString>(&mut self, did: T) -> Result<()> {
    match self.dids.as_mut() {
      Some(dids) => {
        dids.push(did.to_string());
      }
      None => {
        self.dids = Some(vec![did.to_string()]);
      }
    }
    Ok(())
  }

  /// Remove a repository to subscribe
  pub fn unsubscribe_repo<T: ToString>(&mut self, did: T) -> Result<()> {
    let did = did.to_string();
    match self.dids.as_ref() {
      Some(dids) => {
        self.dids = Some(dids.into_iter().filter(|d| **d != did).cloned().collect());
      }
      None => bail!("no such did"),
    }
    Ok(())
  }

  /// Add a handle to subscribe
  pub fn subscribe_handle<T: ToString>(&mut self, handle: T) -> Result<()> {
    match self.handles.as_mut() {
      Some(handles) => {
        let handle = handle.to_string();
        if handles.iter().all(|h| *h != handle) {
          handles.push(handle.to_string());
        }
      }
      None => {
        self.handles = Some(vec![handle.to_string()]);
      }
    }
    Ok(())
  }

  /// Remove a handle to subscribe
  pub fn unsubscribe_handle<T: ToString>(&mut self, handle: T) -> Result<()> {
    let handle = handle.to_string();
    match self.handles.as_ref() {
      Some(handles) => {
        self.handles = Some(
          handles
            .into_iter()
            .filter(|h| **h != handle)
            .cloned()
            .collect(),
        );
      }
      None => bail!("no such handle"),
    }
    Ok(())
  }
}

/// Filter by Keyword
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Keywords {
  pub includes: Option<Vec<String>>,
  pub excludes: Option<Vec<String>>,
}

impl Keywords {
  /// Returns whether the specified string is included in the Event
  pub fn includes(&self, posts: &[AppBskyFeedPost]) -> bool {
    let Some(includes) = &self.includes else {
      return false;
    };
    posts
      .iter()
      .any(|p| includes.iter().any(|i| p.text.contains(i)))
  }

  /// Returns whether the specified string is included in the Event
  pub fn excludes(&self, posts: &[AppBskyFeedPost]) -> bool {
    let Some(excludes) = &self.excludes else {
      return false;
    };
    posts
      .iter()
      .any(|p| excludes.iter().any(|i| p.text.contains(i)))
  }
}

/// Filter by Language
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Langs {
  pub includes: Option<Vec<String>>,
  pub excludes: Option<Vec<String>>,
}

impl Langs {
  /// Returns whether the specified language is included in the Event
  pub fn includes(&self, posts: &[AppBskyFeedPost]) -> bool {
    let Some(includes) = &self.includes else {
      return false;
    };
    posts.iter().any(|p| {
      includes
        .iter()
        .any(|i| p.langs.as_ref().map(|l| l.contains(i)).unwrap_or(false))
    })
  }

  /// Returns whether the specified string is excluded in the Event
  pub fn excludes(&self, posts: &[AppBskyFeedPost]) -> bool {
    let Some(excludes) = &self.excludes else {
      return false;
    };
    posts.iter().any(|p| {
      excludes
        .iter()
        .any(|i| p.langs.as_ref().map(|l| l.contains(i)).unwrap_or(false))
    })
  }
}

/// Filter
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Filter {
  pub name: String,
  pub subscribes: Option<Subscribes>,
  pub keywords: Option<Keywords>,
  pub langs: Option<Langs>,
}

impl Default for Filter {
  fn default() -> Self {
    Self {
      name: String::new(),
      subscribes: Some(Subscribes {
        dids: Some(Vec::new()),
        handles: Some(Vec::new()),
      }),
      keywords: Some(Keywords {
        includes: Some(Vec::new()),
        excludes: Some(Vec::new()),
      }),
      langs: Some(Langs {
        includes: Some(Vec::new()),
        excludes: Some(Vec::new()),
      }),
    }
  }
}

impl Filter {
  /// Convert the Handle in the filter to a DID
  pub fn init(&mut self, client: &mut Client) {
    let Some(follows) = self.subscribes.as_mut() else {
      return;
    };
    let Some(handles) = &follows.handles else {
      return;
    };
    let converted = handles
      .iter()
      .filter_map(|h| client.get_handle(h).ok())
      .collect::<HashSet<_>>();
    let dids = follows
      .dids
      .clone()
      .map(|d| d.into_iter().collect::<HashSet<_>>())
      .unwrap_or_default();
    let dids = dids.union(&converted).cloned().collect::<Vec<_>>();
    if dids.is_empty() {
      follows.dids = None;
    } else {
      follows.dids = Some(dids);
    }
    log::debug!("{:?}", self);
  }

  fn is_follows_match(&self, commit: &ComAtprotoSyncSubscribereposCommit) -> bool {
    match &self.subscribes {
      Some(f) => f.is_match(&commit.repo),
      None => false,
    }
  }

  fn is_handle_match(&self, handle: &ComAtprotoSyncSubscribereposHandle) -> bool {
    match &self.subscribes {
      Some(f) => f.is_match(&handle.did),
      None => false,
    }
  }

  /// Returns whether or not the filter is matched
  pub fn is_match(&self, event: &Event) -> bool {
    if self.subscribes.is_none() && self.keywords.is_none() && self.langs.is_none() {
      return true;
    }
    match &event.payload {
      ComAtprotoSyncSubscribereposMainMessage::ComAtprotoSyncSubscribereposCommit(c) => {
        let posts = c
          .get_post()
          .into_iter()
          .map(|(_, fp)| fp)
          .collect::<Vec<_>>();
        match self.is_follows_match(c) {
          true => {
            !self
              .keywords
              .as_ref()
              .map(|k| k.excludes(&posts))
              .unwrap_or(false)
              && !self
                .langs
                .as_ref()
                .map(|l| l.excludes(&posts))
                .unwrap_or(false)
          }
          false => {
            self
              .keywords
              .as_ref()
              .map(|k| k.includes(&posts))
              .unwrap_or(false)
              || self
                .langs
                .as_ref()
                .map(|l| l.includes(&posts))
                .unwrap_or(false)
          }
        }
      }
      ComAtprotoSyncSubscribereposMainMessage::ComAtprotoSyncSubscribereposHandle(h) => {
        self.is_handle_match(h)
      }
      _ => true,
    }
  }

  /// Add a repository to subscribe to the Filter
  pub fn subscribe_repo<T: ToString>(&mut self, did: T) -> Result<()> {
    if self.subscribes.is_none() {
      self.subscribes = Some(Subscribes::default());
    }
    match self.subscribes.as_mut() {
      Some(s) => s.subscribe_repo(did),
      None => bail!("cannot modify filter"),
    }
  }

  /// Remove a repository to subscribe to the Filter
  pub fn unsubscribe_repo<T: ToString>(&mut self, did: T) -> Result<()> {
    if self.subscribes.is_none() {
      bail!("no such did");
    }
    match self.subscribes.as_mut() {
      Some(s) => s.unsubscribe_repo(did),
      None => bail!("cannot modify filter"),
    }
  }

  /// Add a handle to subscribe to the Filter
  pub fn subscribe_handle<T: ToString>(&mut self, handle: T) -> Result<()> {
    if self.subscribes.is_none() {
      self.subscribes = Some(Subscribes::default());
    }
    match self.subscribes.as_mut() {
      Some(s) => s.subscribe_handle(handle),
      None => bail!("cannot modify filter"),
    }
  }

  /// Remove a handle to subscribe to the Filter
  pub fn unsubscribe_handle<T: ToString>(&mut self, handle: T) -> Result<()> {
    if self.subscribes.is_none() {
      bail!("no such handle");
    }
    match self.subscribes.as_mut() {
      Some(s) => s.unsubscribe_handle(handle),
      None => bail!("cannot modify filter"),
    }
  }
}

/// Set of filters
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Filters {
  pub filters: Vec<Filter>,
}

impl Default for Filters {
  fn default() -> Self {
    Self {
      filters: vec![
        Filter {
          name: String::from("All"),
          subscribes: None,
          keywords: None,
          langs: None,
        },
        Filter {
          name: String::from("Favorites"),
          ..Default::default()
        },
      ],
    }
  }
}

impl Filters {
  /// Initialize all included filters
  pub fn init(&mut self, client: &mut Client) {
    for filter in self.filters.iter_mut() {
      filter.init(client);
    }
  }

  /// Add user's timeline as a filter
  pub fn add_timeline<T: ToString>(&self, client: &crate::api::Client, handle: T) -> Result<Self> {
    let follows = client.app_bsky_graph_getfollows(&handle.to_string(), None, None)?;
    let mut filters = self
      .filters
      .clone()
      .into_iter()
      .filter(|f| f.name != handle.to_string())
      .collect::<Vec<_>>();
    filters.push(Filter {
      name: handle.to_string(),
      subscribes: Some(Subscribes {
        dids: Some(
          follows
            .follows
            .iter()
            .map(|f| f.did.clone())
            .collect::<Vec<_>>(),
        ),
        handles: None,
      }),
      ..Default::default()
    });
    Ok(Self { filters })
  }

  /// Remove user's timeline as a filter
  pub fn remove_timeline<T: ToString>(&mut self, handle: T) {
    self.filters = self
      .filters
      .clone()
      .into_iter()
      .filter(|f| f.name != handle.to_string())
      .collect::<Vec<_>>();
  }

  /// Returns all included filters
  pub fn get_filters(&self) -> Vec<Filter> {
    self.filters.clone()
  }

  /// Add a repository to subscribe to the Filter given by name
  pub fn subscribe_repo<T1: ToString, T2: ToString>(&mut self, name: T1, did: T2) -> Result<()> {
    let Some(filter) = self.filters.iter_mut().find(|f| f.name == name.to_string()) else {
      bail!("no such named filter");
    };
    filter.subscribe_repo(did)
  }

  /// Remove a repository to subscribe to the Filter given by name
  pub fn unsubscribe_repo<T1: ToString, T2: ToString>(&mut self, name: T1, did: T2) -> Result<()> {
    let Some(filter) = self.filters.iter_mut().find(|f| f.name == name.to_string()) else {
      bail!("no such named filter");
    };
    filter.unsubscribe_repo(did)
  }

  /// Add a handle to subscribe to the Filter given by name
  pub fn subscribe_handle<T1: ToString, T2: ToString>(
    &mut self,
    name: T1,
    handle: T2,
  ) -> Result<()> {
    let Some(filter) = self.filters.iter_mut().find(|f| f.name == name.to_string()) else {
      bail!("no such named filter");
    };
    filter.subscribe_handle(handle)
  }

  /// Remove a handle to subscribe to the Filter given by name
  pub fn unsubscribe_handle<T1: ToString, T2: ToString>(
    &mut self,
    name: T1,
    did: T2,
  ) -> Result<()> {
    let Some(filter) = self.filters.iter_mut().find(|f| f.name == name.to_string()) else {
      bail!("no such named filter");
    };
    filter.unsubscribe_handle(did)
  }
}
