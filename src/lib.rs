//! # Aerostream
//!
//! Aerostream is Bluesky client using EventStream.
//!
//! It can be used as a library or as a command line tool.
//!
//! ## To use as a command line tool.
//!
//! ```shell
//! cargo install aerostream -F terminal
//! aerostream
//! ```
//!
//! ### Notes
//!
//! - Only CUI, No need to log in.
//! - Instead, you can't post, repost and like.
//! - Configuration file must be edited in a text editor, and there is no configuration screen in the application.
//!
//! ### Edit filters.yaml
//!
//! ```yaml
//! filters:
//!   - name: <Column Name>
//!     subscribes:
//!       dids:
//!         - <DID to identify the repository to subscribe to>
//!       handles:
//!         - <Handle to identify the repository to subscribe to>
//!     keywords:
//!       includes:
//!         - <Keywords to include in Column even if you are not subscribed>
//!       excludes:
//!         - <Keywords to exclude in Column even if you are subscribed>
//!     langs:
//!       includes:
//!         - <Languages to include in Column even if you are not subscribed>
//!       excludes:
//!         - <Languages to exclude in Column even if you are subscribed>
//! ```
//!
//! ### Operation
//!
//! - q or Ctrl+c : quit this application
//! - F5 or Ctrl+r : redraw screen
//! - s : subscribe to the repository of the focused post in "Favorites" filter
//! - u : unsubscribe to the repository of the focused post in "Favorites" filter
//! - LEFT or RIGHT : change the filter in focus
//! - UP or DOWN : change the post in focus
//! - ESC : take the focus off the post
//!
//! ## To use as a library
//!
//! ```rust
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
//!   let mut client = Client::default();
//!   client.set_timeout(5);
//!   client.connect_ws()?;
//!   for (filter, event) in client.next_event_filtered_all()?.iter() {
//!     let Some(commit) = event.as_commit() else {
//!       continue;
//!     };
//!     let posts = commit.get_post_text();
//!     if posts.is_empty() {
//!       continue;
//!     }
//!     let text = posts.join(" ").replace("\n", " ");
//!     let time = commit.time.with_timezone(&Local).format("%m/%d %H:%M");
//!     let handle = match client.get_repo(&commit.repo) {
//!       Ok(r) => r.handle.clone(),
//!       _ => String::from("UNKNOWN"),
//!     };
//!     let blobs = commit
//!       .blobs
//!       .iter()
//!       .map(|b| b.to_string())
//!       .collect::<Vec<_>>();
//!     print!("{} : {} : {} : {}", filter, time, handle, text);
//!     if !commit.blobs.is_empty() {
//!       println!(" : {}", blobs.join(","));
//!     } else {
//!       println!("");
//!     }
//!     stdout().flush().ok();
//!   }
//!   Ok(())
//! }
//! ```

/// Atproto API from lexicions
pub mod api;
pub mod client;
pub mod event;
pub mod filter;

pub use client::{Client, Repo};
pub use event::{Blocks, Event, Header};
pub use filter::{Filter, Filters, Keywords, Subscribes};
