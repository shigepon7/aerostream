//! Event to be received via EventStream
use std::{
  collections::HashMap,
  io::{Cursor, Seek},
  str::FromStr,
};

use anyhow::{anyhow, bail, Result};
use chrono::{DateTime, Utc};
use libipld::{cbor::DagCborCodec, json::DagJsonCodec, prelude::Codec, Cid, DagCbor, Ipld, Link};

use crate::api::{
  AppBskyFeedPost, ComAtprotoSyncSubscribereposCommit, ComAtprotoSyncSubscribereposHandle,
  ComAtprotoSyncSubscribereposInfo, ComAtprotoSyncSubscribereposMainMessage,
  ComAtprotoSyncSubscribereposMigrate, ComAtprotoSyncSubscribereposRepoop,
  ComAtprotoSyncSubscribereposTombstone,
};

/// Event Header
#[derive(Debug, Clone, DagCbor, Default)]
pub struct Header {
  pub op: i64,
  pub t: String,
}

impl Header {
  /// Returns the length in encoded format
  pub fn len(&self) -> Result<usize> {
    Ok(DagCborCodec.encode(self)?.len())
  }
}

impl ComAtprotoSyncSubscribereposCommit {
  /// Returns the posts included in an operation to the repository
  pub fn get_post(&self) -> Vec<(ComAtprotoSyncSubscribereposRepoop, AppBskyFeedPost)> {
    let ret = self
      .ops
      .iter()
      .filter(|op| op.path.starts_with("app.bsky.feed.post"))
      .filter_map(|op| Cid::from_str(&op.cid).ok().map(|c| (op, c)))
      .filter_map(|(op, cid)| {
        Blocks::from(self.blocks.as_slice())
          .get(&cid)
          .and_then(|ipld| {
            DagJsonCodec
              .encode(&ipld)
              .ok()
              .and_then(|j| serde_json::from_slice(&j).ok())
              .map(|p| (op.clone(), p))
          })
      })
      .collect();
    ret
  }

  /// Returns the text of all posts included in an operation to the repository
  pub fn get_post_text(&self) -> Vec<String> {
    self
      .get_post()
      .into_iter()
      .filter_map(|(op, fp)| (op.action == "create").then(|| fp.text))
      .collect()
  }

  /// Returns the cid of first post included in an operation to the repository
  pub fn get_post_path(&self) -> Option<String> {
    self.get_post().first().map(|(op, _)| op.path.clone())
  }
}

/// Serialized Block Information
#[derive(Debug, Clone)]
pub struct Blocks {
  pub header: Ipld,
  pub data: HashMap<Cid, Ipld>,
}

impl Default for Blocks {
  fn default() -> Self {
    Self {
      header: Ipld::Null,
      data: HashMap::new(),
    }
  }
}

fn get_block(data: &[u8]) -> Result<(Vec<u8>, usize)> {
  let mut buf = Cursor::new(data);
  let variant = leb128::read::unsigned(&mut buf)?;
  let start = buf.stream_position()? as usize;
  let end = start + (variant as usize);
  Ok((data.get(start..end).unwrap_or_default().to_vec(), end))
}

fn get_cid(data: &[u8]) -> Result<(Cid, usize)> {
  let mut buf = Cursor::new(data);
  let cid = Cid::read_bytes(&mut buf)?;
  Ok((cid, buf.stream_position()? as usize))
}

impl From<&[u8]> for Blocks {
  fn from(data: &[u8]) -> Self {
    let mut ret = HashMap::new();
    let (header, len) = match get_block(data) {
      Ok(b) => b,
      _ => return Self::default(),
    };
    let header = match DagCborCodec.decode::<Ipld>(header.as_slice()) {
      Ok(h) => h,
      _ => return Self::default(),
    };
    let mut data = data.get(len..).unwrap_or_default();
    while !data.is_empty() {
      let Ok((block, len)) = get_block(data) else {
        break;
      };
      data = data.get(len..).unwrap_or_default();
      let Ok((cid, len)) = get_cid(block.as_slice()) else {
        break;
      };
      let block = block.get(len..).unwrap_or_default();
      let Ok(data) = DagCborCodec.decode(block) else {
        break;
      };
      ret.insert(cid, data);
    }
    Self { header, data: ret }
  }
}

impl Blocks {
  /// Returns data with the specified CID from Blocks
  pub fn get(&self, cid: &Cid) -> Option<Ipld> {
    self.data.get(cid).cloned()
  }
}

/// Event
#[derive(Debug, Clone, Default)]
pub struct Event {
  pub header: Header,
  pub payload: ComAtprotoSyncSubscribereposMainMessage,
}

/// Op
#[derive(Debug, Clone, DagCbor)]
#[allow(non_snake_case)]
pub struct RepoOp {
  pub action: String,
  pub path: String,
  pub cid: Option<Link<Cid>>,
}

#[derive(Debug, Clone, DagCbor)]
#[allow(non_snake_case)]
struct CommitInner {
  pub seq: i64,
  pub rebase: bool,
  pub tooBig: bool,
  pub repo: String,
  pub commit: Link<Cid>,
  pub rev: String,
  pub since: String,
  pub blocks: Ipld,
  pub ops: Vec<RepoOp>,
  pub blobs: Vec<Link<Cid>>,
  pub time: String,
  pub prev: Option<Link<Cid>>,
}

impl From<CommitInner> for ComAtprotoSyncSubscribereposCommit {
  fn from(value: CommitInner) -> Self {
    Self {
      seq: value.seq,
      rebase: value.rebase,
      too_big: value.tooBig,
      repo: value.repo.clone(),
      commit: value.commit.to_string(),
      rev: value.rev.clone(),
      since: value.since.clone(),
      blocks: match &value.blocks {
        Ipld::Bytes(b) => b.clone(),
        _ => Vec::new(),
      },
      ops: value
        .ops
        .iter()
        .map(|op| ComAtprotoSyncSubscribereposRepoop {
          action: op.action.clone(),
          path: op.path.clone(),
          cid: op.cid.map(|c| c.to_string()).unwrap_or_default(),
          ..Default::default()
        })
        .collect(),
      blobs: value
        .blobs
        .iter()
        .map(|b| {
          format!(
            "https://bsky.social/xrpc/com.atproto.sync.getBlob?did={}&cid={}",
            value.repo, b
          )
        })
        .collect(),
      time: value.time.parse().unwrap_or_default(),
      prev: value.prev.map(|p| p.to_string()),
      ..Default::default()
    }
  }
}

#[derive(Debug, Clone, DagCbor)]
struct HandleInner {
  seq: i64,
  did: String,
  handle: String,
  time: String,
}

impl From<HandleInner> for ComAtprotoSyncSubscribereposHandle {
  fn from(value: HandleInner) -> Self {
    Self {
      seq: value.seq,
      did: value.did,
      handle: value.handle,
      time: value.time.parse().unwrap_or_default(),
      ..Default::default()
    }
  }
}

#[derive(Debug, Clone, DagCbor)]
#[allow(non_snake_case)]
struct MigrateInner {
  seq: i64,
  did: String,
  migrateTo: Option<String>,
  time: String,
}

impl From<MigrateInner> for ComAtprotoSyncSubscribereposMigrate {
  fn from(value: MigrateInner) -> Self {
    Self {
      seq: value.seq,
      did: value.did,
      migrate_to: value.migrateTo.unwrap_or_default(),
      time: value.time.parse().unwrap_or_default(),
      ..Default::default()
    }
  }
}

#[derive(Debug, Clone, DagCbor)]
struct TombstoneInner {
  seq: i64,
  did: String,
  time: String,
}

impl From<TombstoneInner> for ComAtprotoSyncSubscribereposTombstone {
  fn from(value: TombstoneInner) -> Self {
    Self {
      seq: value.seq,
      did: value.did,
      time: value.time.parse().unwrap_or_default(),
      ..Default::default()
    }
  }
}

#[derive(Debug, Clone, DagCbor)]
struct InfoInner {
  pub name: String,
  pub message: Option<String>,
}

impl From<InfoInner> for ComAtprotoSyncSubscribereposInfo {
  fn from(value: InfoInner) -> Self {
    Self {
      name: value.name,
      message: value.message,
      ..Default::default()
    }
  }
}

impl TryFrom<&[u8]> for Event {
  type Error = anyhow::Error;
  fn try_from(value: &[u8]) -> std::result::Result<Self, Self::Error> {
    let mut ret = Self::default();
    let header = DagCborCodec.decode::<Header>(value)?;
    ret.header = header.clone();
    if header.op < 0 {
      bail!("header is negative");
    }
    let payload = value
      .get(header.len().unwrap_or_default()..)
      .ok_or_else(|| anyhow!("payload is short"))?;
    ret.payload = match header.t.as_str() {
      "#commit" => {
        ComAtprotoSyncSubscribereposMainMessage::ComAtprotoSyncSubscribereposCommit(Box::new(
          ComAtprotoSyncSubscribereposCommit::from(DagCborCodec.decode::<CommitInner>(&payload)?),
        ))
      }
      "#handle" => {
        ComAtprotoSyncSubscribereposMainMessage::ComAtprotoSyncSubscribereposHandle(Box::new(
          ComAtprotoSyncSubscribereposHandle::from(DagCborCodec.decode::<HandleInner>(&payload)?),
        ))
      }
      "#migrate" => {
        ComAtprotoSyncSubscribereposMainMessage::ComAtprotoSyncSubscribereposMigrate(Box::new(
          ComAtprotoSyncSubscribereposMigrate::from(DagCborCodec.decode::<MigrateInner>(&payload)?),
        ))
      }
      "#tombstone" => {
        ComAtprotoSyncSubscribereposMainMessage::ComAtprotoSyncSubscribereposTombstone(Box::new(
          ComAtprotoSyncSubscribereposTombstone::from(
            DagCborCodec.decode::<TombstoneInner>(&payload)?,
          ),
        ))
      }
      "#info" => {
        ComAtprotoSyncSubscribereposMainMessage::ComAtprotoSyncSubscribereposInfo(Box::new(
          ComAtprotoSyncSubscribereposInfo::from(DagCborCodec.decode::<InfoInner>(&payload)?),
        ))
      }
      t => bail!("unknown event type {}", t),
    };
    Ok(ret)
  }
}

impl Event {
  /// Returns Payload if Event is Commit
  pub fn as_commit(&self) -> Option<&ComAtprotoSyncSubscribereposCommit> {
    if let ComAtprotoSyncSubscribereposMainMessage::ComAtprotoSyncSubscribereposCommit(c) =
      &self.payload
    {
      Some(c)
    } else {
      None
    }
  }

  /// Returns Payload if Event is Handle
  pub fn as_handle(&self) -> Option<&ComAtprotoSyncSubscribereposHandle> {
    if let ComAtprotoSyncSubscribereposMainMessage::ComAtprotoSyncSubscribereposHandle(h) =
      &self.payload
    {
      Some(h)
    } else {
      None
    }
  }

  /// Returns Payload if Event is Migrate
  pub fn as_migrate(&self) -> Option<&ComAtprotoSyncSubscribereposMigrate> {
    if let ComAtprotoSyncSubscribereposMainMessage::ComAtprotoSyncSubscribereposMigrate(m) =
      &self.payload
    {
      Some(m)
    } else {
      None
    }
  }

  /// Returns Payload if Event is Tombstone
  pub fn as_tombstone(&self) -> Option<&ComAtprotoSyncSubscribereposTombstone> {
    if let ComAtprotoSyncSubscribereposMainMessage::ComAtprotoSyncSubscribereposTombstone(t) =
      &self.payload
    {
      Some(t)
    } else {
      None
    }
  }

  /// Returns Payload if Event is Info
  pub fn as_info(&self) -> Option<&ComAtprotoSyncSubscribereposInfo> {
    if let ComAtprotoSyncSubscribereposMainMessage::ComAtprotoSyncSubscribereposInfo(i) =
      &self.payload
    {
      Some(i)
    } else {
      None
    }
  }

  /// Get sequence number
  pub fn get_seq(&self) -> Option<i64> {
    match &self.payload {
      ComAtprotoSyncSubscribereposMainMessage::ComAtprotoSyncSubscribereposCommit(c) => Some(c.seq),
      ComAtprotoSyncSubscribereposMainMessage::ComAtprotoSyncSubscribereposIdentity(id) => {
        Some(id.seq)
      }
      ComAtprotoSyncSubscribereposMainMessage::ComAtprotoSyncSubscribereposHandle(h) => Some(h.seq),
      ComAtprotoSyncSubscribereposMainMessage::ComAtprotoSyncSubscribereposMigrate(m) => {
        Some(m.seq)
      }
      ComAtprotoSyncSubscribereposMainMessage::ComAtprotoSyncSubscribereposTombstone(t) => {
        Some(t.seq)
      }
      ComAtprotoSyncSubscribereposMainMessage::ComAtprotoSyncSubscribereposInfo(_) => None,
      ComAtprotoSyncSubscribereposMainMessage::Other => None,
    }
  }

  /// Get Event Time
  pub fn get_time(&self) -> Option<DateTime<Utc>> {
    match &self.payload {
      ComAtprotoSyncSubscribereposMainMessage::ComAtprotoSyncSubscribereposCommit(c) => {
        Some(c.time)
      }
      ComAtprotoSyncSubscribereposMainMessage::ComAtprotoSyncSubscribereposIdentity(id) => {
        Some(id.time)
      }
      ComAtprotoSyncSubscribereposMainMessage::ComAtprotoSyncSubscribereposHandle(h) => {
        Some(h.time)
      }
      ComAtprotoSyncSubscribereposMainMessage::ComAtprotoSyncSubscribereposMigrate(m) => {
        Some(m.time)
      }
      ComAtprotoSyncSubscribereposMainMessage::ComAtprotoSyncSubscribereposTombstone(t) => {
        Some(t.time)
      }
      ComAtprotoSyncSubscribereposMainMessage::ComAtprotoSyncSubscribereposInfo(_) => None,
      ComAtprotoSyncSubscribereposMainMessage::Other => None,
    }
  }
}
