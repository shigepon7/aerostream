Aerostream
===

Aerostream is Bluesky client using EventStream.

```rust
use std::io::{stdout, Write};

use aerostream::Client;
use anyhow::Result;
use chrono::Local;

fn main() -> Result<()> {
  let mut client = Client::default();
  client.connect_ws()?;
  loop {
    let events = client.next_event_filtered_all()?;
    for (filter, event) in events.iter() {
      let Some(commit) = event.as_commit() else {
        continue;
      };
      let posts = commit.get_post_text();
      if posts.is_empty() {
        continue;
      }
      let text = posts.join(" ").replace("\n", " ");
      let time = commit.time.with_timezone(&Local).format("%m/%d %H:%M");
      let handle = match client.get_repo(&commit.repo) {
        Ok(r) => r.handle.clone(),
        _ => String::from("UNKNOWN"),
      };
      let blobs = commit
        .blobs
        .iter()
        .map(|b| b.to_string())
        .collect::<Vec<_>>();
      print!("{} : {} : {} : {}", filter, time, handle, text);
      if !commit.blobs.is_empty() {
        println!("{}", blobs.join(","));
      } else {
        println!("");
      }
    }
    stdout().flush().ok();
  }
}
```
