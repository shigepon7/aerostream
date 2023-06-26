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

use serde::{Deserialize, Serialize};

use crate::{Client, Commit, Event, Handle, Payload};

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
}

/// Filter by Keyword
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Keywords {
  pub includes: Option<Vec<String>>,
  pub excludes: Option<Vec<String>>,
}

impl Keywords {
  /// Returns whether the specified string is included in the Event received from all repositories
  pub fn includes(&self, commit: &Commit) -> bool {
    let Some(includes) = &self.includes else {
      return false;
    };
    commit
      .get_post_text()
      .iter()
      .any(|p| includes.iter().any(|i| p.contains(i)))
  }

  /// Returns whether the specified string is included in the Event received from the subscribed repository.
  pub fn excludes(&self, commit: &Commit) -> bool {
    let Some(excludes) = &self.excludes else {
      return false;
    };
    commit
      .get_post_text()
      .iter()
      .any(|p| excludes.iter().any(|i| p.contains(i)))
  }
}

/// Filter
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Filter {
  pub name: String,
  pub subscribes: Option<Subscribes>,
  pub keywords: Option<Keywords>,
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

  fn is_follows_match(&self, commit: &Commit) -> bool {
    match &self.subscribes {
      Some(f) => f.is_match(&commit.repo),
      None => false,
    }
  }

  fn is_handle_match(&self, handle: &Handle) -> bool {
    match &self.subscribes {
      Some(f) => f.is_match(&handle.did),
      None => false,
    }
  }

  fn is_keywords_includes(&self, commit: &Commit) -> bool {
    match &self.keywords {
      Some(k) => k.includes(commit),
      None => false,
    }
  }

  fn is_keywords_excludes(&self, commit: &Commit) -> bool {
    match &self.keywords {
      Some(k) => k.excludes(commit),
      None => false,
    }
  }

  /// Returns whether or not the filter is matched
  pub fn is_match(&self, event: &Event) -> bool {
    match &event.payload {
      Payload::Commit(c) => match self.is_follows_match(c) {
        true => !self.is_keywords_excludes(c),
        false => self.is_keywords_includes(c),
      },
      Payload::Handle(h) => self.is_handle_match(h),
      _ => true,
    }
  }
}

/// Set of filters
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Filters {
  pub filters: Vec<Filter>,
}

impl Filters {
  /// Initialize all included filters
  pub fn init(&mut self, client: &mut Client) {
    for filter in self.filters.iter_mut() {
      filter.init(client);
    }
  }

  /// Returns all included filters
  pub fn get_filters(&self) -> Vec<Filter> {
    self.filters.clone()
  }
}
