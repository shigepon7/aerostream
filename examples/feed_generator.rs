use std::path::{Path, PathBuf};

use aerostream::{
  api::{AppBskyFeedGetfeedskeleton, ComAtprotoSyncSubscribereposCommit},
  Algorithm, AtUri, Client, Cursor, FeedGenerator, FeedPost, FeedPosts, Subscription,
};
use anyhow::Result;

#[derive(Clone)]
struct KeyWord {
  posts: FeedPosts,
  name: String,
  keyword: String,
  storage: String,
}

impl KeyWord {
  fn new<T1: ToString, T2: ToString, T3: ToString>(name: T1, keyword: T2, storage: T3) -> Self {
    let storage = storage.to_string();
    let posts = FeedPosts::from(
      std::fs::File::open(Self::feed_filename(&storage))
        .ok()
        .and_then(|feed| serde_json::from_reader::<_, Vec<FeedPost>>(feed).ok())
        .unwrap_or_default(),
    );
    let name = name.to_string();
    let keyword = keyword.to_string();
    Self {
      posts,
      name,
      keyword,
      storage,
    }
  }

  fn feed_filename<T: ToString>(storage: T) -> PathBuf {
    Path::new(&storage.to_string()).join("feed.json")
  }
}

impl Algorithm for KeyWord {
  fn get_name(&self) -> &str {
    self.name.as_str()
  }

  fn handler(
    &self,
    limit: Option<usize>,
    cursor: Option<String>,
    _access_did: Option<String>,
    _jwt: Option<String>,
  ) -> AppBskyFeedGetfeedskeleton {
    let limit = limit.unwrap_or(50);
    let cursor = cursor.and_then(|c| Cursor::parse(&c));
    self.posts.get_old_posts(limit, cursor)
  }
}

impl Subscription for KeyWord {
  fn handler(&mut self, records: Vec<ComAtprotoSyncSubscribereposCommit>) {
    let mut new_posts: Vec<FeedPost> = Vec::new();
    let mut deleted_posts: Vec<String> = Vec::new();
    for record in records.iter() {
      for (op, post) in record.get_post().into_iter() {
        let uri = AtUri::new("", &record.repo, &op.path, "").to_string();
        match op.action.as_str() {
          "create" => {
            if post.text.contains(&self.keyword) {
              new_posts.push(FeedPost::new(&uri, &op.cid, &record.repo, &post));
            }
          }
          "delete" => deleted_posts.push(uri),
          _ => (),
        }
      }
    }
    if !deleted_posts.is_empty() {
      log::info!("delete {} posts", deleted_posts.len());
      self.posts.delete_posts(&deleted_posts);
    }
    if !new_posts.is_empty() {
      log::info!("add {} posts", new_posts.len());
      self.posts.append_posts(&new_posts);
    }
    if !new_posts.is_empty() || !deleted_posts.is_empty() {
      let posts = self.posts.get_all_posts();
      if let Ok(feed) = std::fs::File::create(Self::feed_filename(&self.storage)) {
        serde_json::to_writer(feed, &posts).ok();
      }
    }
  }
}

fn main() -> Result<()> {
  env_logger::init();
  let handle = std::env::var("FEEDGENERATOR_PUBLISHER_HANDLE")?;
  let password = std::env::var("FEEDGENERATOR_PUBLISHER_PASSWORD")?;
  let host = std::env::var("FEEDGENERATOR_HOST")?;
  let threads = std::env::var("FEEDGENERATOR_THREADS")?.parse()?;
  let storage = std::env::var("FEEDGENERATOR_STORAGE_PATH")?;
  let feed_handle = std::env::var("FEEDGENERATOR_FEED_HANDLE")?;
  let feed_password = std::env::var("FEEDGENERATOR_FEED_PASSWORD")?;
  let mut client = Client::default();
  client.login(&feed_handle, &feed_password).unwrap();
  let taste = KeyWord::new("taste", "美味", storage);
  let mut server = FeedGenerator::new(handle, password, host, threads);
  server.add_algorithm(Box::new(taste.clone()));
  server.set_subscription(Box::new(taste));
  server.start()?;
  Ok(())
}
