//! Aerostream
//! ===
//!
//! Aerostream is Bluesky client using EventStream.
//!
//! ```
//! use std::{
//!   io::{stdout, Write},
//!   time::Duration,
//! };
//!
//! use aerostream::Client;
//! use anyhow::Result;
//! use chrono::Local;
//!
//! fn main() -> Result<()> {
//!   env_logger::init();
//!   let mut client = Client::default();
//!   client.set_timeout(5);
//!   client.connect_ws()?;
//!   loop {
//!     for (filter, event) in client.next_event_filtered_all()?.iter() {
//!       let Some(commit) = event.as_commit() else {
//!         continue;
//!       };
//!       let posts = commit.get_post_text();
//!       if posts.is_empty() {
//!         continue;
//!       }
//!       let text = posts.join(" ").replace("\n", " ");
//!       let time = commit.time.with_timezone(&Local).format("%m/%d %H:%M");
//!       let handle = match client.get_repo(&commit.repo) {
//!         Ok(r) => r.handle.clone(),
//!         _ => String::from("UNKNOWN"),
//!       };
//!       let blobs = commit
//!         .blobs
//!         .iter()
//!         .map(|b| b.to_string())
//!         .collect::<Vec<_>>();
//!       print!("{} : {} : {} : {}", filter, time, handle, text);
//!       if !commit.blobs.is_empty() {
//!         println!(" : {}", blobs.join(","));
//!       } else {
//!         println!("");
//!       }
//!       stdout().flush().ok();
//!     }
//!     std::thread::sleep(Duration::from_millis(10));
//!   }
//! }
//! ```

pub mod client;
pub mod event;
pub mod filter;

pub use client::{Client, Repo};
pub use event::{
  Blob, Blocks, Commit, Event, Handle, Header, Info, Migrate, Payload, RepoOp, Tombstone,
};
pub use filter::{Filter, Filters, Keywords, Subscribes};
