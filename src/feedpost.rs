//! Utility definitions to consturct feed generator
use std::{
  cmp::Ordering,
  fmt::Display,
  str::FromStr,
  sync::{Arc, RwLock},
};

use chrono::{DateTime, Utc};
use libipld::Cid;
use serde::{Deserialize, Serialize};

use crate::api::{AppBskyFeedDefsSkeletonfeedpost, AppBskyFeedGetfeedskeleton, AppBskyFeedPost};

/// Structure of PDS posts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedPost {
  pub uri: String,
  pub cid: Cid,
  pub repo: String,
  pub indexed_at: DateTime<Utc>,
  pub post: AppBskyFeedPost,
}

impl FeedPost {
  /// Create a record of PDS post
  pub fn new(uri: &str, cid: &str, repo: &str, post: &AppBskyFeedPost) -> Self {
    Self {
      uri: uri.to_string(),
      cid: Cid::from_str(cid).unwrap_or_default(),
      repo: repo.to_string(),
      indexed_at: Utc::now(),
      post: post.clone(),
    }
  }

  /// Compare if the post is older than the cursor
  pub fn is_old(&self, cursor: &Cursor) -> bool {
    match self.indexed_at.cmp(&cursor.indexed_at) {
      Ordering::Less => true,
      Ordering::Equal => self.cid < cursor.cid,
      Ordering::Greater => false,
    }
  }

  /// Convert to cursor
  pub fn to_cursor(&self) -> Cursor {
    Cursor {
      indexed_at: self.indexed_at,
      cid: self.cid,
    }
  }

  pub fn to_response(&self) -> AppBskyFeedDefsSkeletonfeedpost {
    AppBskyFeedDefsSkeletonfeedpost {
      post: self.uri.clone(),
      ..Default::default()
    }
  }
}

/// List of PDS posts
pub struct FeedPosts {
  pub posts: Arc<RwLock<Vec<FeedPost>>>,
}

impl Default for FeedPosts {
  fn default() -> Self {
    Self {
      posts: Arc::new(RwLock::new(Vec::new())),
    }
  }
}

impl Clone for FeedPosts {
  fn clone(&self) -> Self {
    Self {
      posts: Arc::clone(&self.posts),
    }
  }
}

impl From<Vec<FeedPost>> for FeedPosts {
  fn from(value: Vec<FeedPost>) -> Self {
    Self {
      posts: Arc::new(RwLock::new(value)),
    }
  }
}

impl FeedPosts {
  /// Append PDS posts to this list
  pub fn append_posts(&mut self, new_posts: &[FeedPost]) {
    let mut new_posts = new_posts.to_vec();
    let mut posts = self.posts.write().unwrap();
    posts.append(&mut new_posts);
  }

  /// Delete PDS posts from this list
  pub fn delete_posts(&self, uris: &[String]) {
    let mut posts = self.posts.write().unwrap();
    *posts = posts
      .iter()
      .filter(|p| !uris.contains(&p.uri))
      .cloned()
      .collect::<Vec<_>>();
  }

  /// Get all PDS posts of this list
  pub fn get_all_posts(&self) -> Vec<FeedPost> {
    let mut posts = self.posts.read().unwrap().clone();
    posts.sort_by(|a, b| match b.indexed_at.cmp(&a.indexed_at) {
      Ordering::Equal => b.cid.cmp(&a.cid),
      o => o,
    });
    posts
  }

  /// Get a fixed number of PDS posts
  pub fn get_old_posts(&self, limit: usize, cursor: Option<Cursor>) -> AppBskyFeedGetfeedskeleton {
    let posts = self.get_all_posts();
    let posts = match cursor {
      Some(c) => posts
        .into_iter()
        .filter(|p| p.is_old(&c))
        .collect::<Vec<_>>(),
      None => posts,
    };
    let feed = posts.iter().take(limit).cloned().collect::<Vec<_>>();
    AppBskyFeedGetfeedskeleton {
      feed: feed.iter().map(|p| p.to_response()).collect::<Vec<_>>(),
      cursor: (posts.last().map(|p| &p.uri) != feed.last().map(|p| &p.uri))
        .then(|| feed)
        .and_then(|f| f.last().map(|p| p.to_cursor().to_string())),
    }
  }
}

/// Cursor
pub struct Cursor {
  pub indexed_at: DateTime<Utc>,
  pub cid: Cid,
}

impl Display for Cursor {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_fmt(format_args!(
      "{}::{}",
      self.indexed_at.timestamp_millis(),
      self.cid.to_string()
    ))
  }
}

impl Cursor {
  /// Parse http request parameter as a cursor
  pub fn parse(cursor: &str) -> Option<Self> {
    let mut sp = cursor.split("::");
    let ts = sp.next()?.parse::<i64>().ok()?;
    let indexed_at = DateTime::from_timestamp(ts / 1000, (ts % 1000) as u32)?;
    let cid = Cid::from_str(sp.next()?).ok()?;
    Some(Self { indexed_at, cid })
  }

  /// Convert PDS post to cursor
  pub fn from_feedpost(post: &FeedPost) -> Option<Self> {
    Some(Self {
      indexed_at: post.indexed_at,
      cid: post.cid,
    })
  }
}
