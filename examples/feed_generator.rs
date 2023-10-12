use std::{
  path::{Path, PathBuf},
  time::Duration,
};

use aerostream::{
  api::{
    AppBskyFeedGenerator, AppBskyFeedGetfeedskeleton, ComAtprotoSyncSubscribereposCommit, Record,
  },
  Algorithm, AtUri, Client, Cursor, FeedGenerator, FeedPost, FeedPosts, Subscription,
};
use anyhow::Result;
use chrono::Utc;

#[derive(Clone)]
struct KeyWord {
  posts: FeedPosts,
  name: String,
  keyword: String,
  storage: String,
  publisher: String,
}

impl KeyWord {
  fn new<T1: ToString, T2: ToString, T3: ToString, T4: ToString>(
    name: T1,
    keyword: T2,
    storage: T3,
    publisher: T4,
  ) -> Self {
    let storage = storage.to_string();
    let posts = FeedPosts::from(
      std::fs::File::open(Self::feed_filename(&storage))
        .ok()
        .and_then(|feed| serde_json::from_reader::<_, Vec<FeedPost>>(feed).ok())
        .unwrap_or_default(),
    );
    Self {
      posts,
      name: name.to_string(),
      keyword: keyword.to_string(),
      storage,
      publisher: publisher.to_string(),
    }
  }

  fn feed_filename<T: ToString>(storage: T) -> PathBuf {
    Path::new(&storage.to_string()).join("feed.json")
  }
}

impl Algorithm for KeyWord {
  fn get_name(&self) -> String {
    self.name.clone()
  }

  fn get_publisher(&self) -> String {
    self.publisher.clone()
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
  let mut client = Client::default();
  client.login(&handle, &password).unwrap();
  let publisher = match handle.starts_with("did:plc:") {
    true => handle.clone(),
    false => client.get_handle(&handle)?,
  };
  let keyword = "美味";
  let taste = KeyWord::new("taste", keyword, storage, publisher);
  client
    .client
    .com_atproto_repo_putrecord(
      &handle,
      "app.bsky.feed.generator",
      "taste",
      &Record::AppBskyFeedGenerator(AppBskyFeedGenerator {
        did: String::from("did:web:{host}"),
        display_name: keyword.to_string(),
        created_at: Utc::now(),
        ..Default::default()
      }),
      None,
      None,
      None,
    )
    .unwrap();
  let mut server = FeedGenerator::new(host, threads);
  server.add_algorithm(Box::new(taste.clone()));
  server.set_subscription(Box::new(taste));
  server.start()?;
  loop {
    std::thread::sleep(Duration::from_secs(1));
  }
}
