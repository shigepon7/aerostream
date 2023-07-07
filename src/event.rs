//! Event to be received via EventStream
use std::{
  collections::HashMap,
  fmt::Display,
  io::{Cursor, Seek},
};

use anyhow::Result;
use chrono::{DateTime, Utc};
use libipld::{cbor::DagCborCodec, prelude::Codec, Cid, DagCbor, Ipld, Link};

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

/// Operations on the Repository
#[derive(Debug, Clone, DagCbor)]
#[allow(non_snake_case)]
pub struct RepoOp {
  pub action: String,
  pub path: String,
  pub cid: Option<Link<Cid>>,
}

#[derive(Debug, Clone)]
pub struct FeedPost {
  pub text: String,
  pub created_at: DateTime<Utc>,
  pub langs: Option<Vec<String>>,
}

impl FeedPost {
  pub fn from(ipld: Ipld) -> Option<Self> {
    let text = match ipld.get("text").ok()? {
      Ipld::String(s) => s.clone(),
      _ => return None,
    };
    let created_at = match ipld.get("createdAt").ok()? {
      Ipld::String(s) => s.clone(),
      _ => return None,
    }
    .parse()
    .ok()?;
    let langs = ipld.get("langs").ok().and_then(|ipld| match ipld {
      Ipld::List(l) => Some(
        l.iter()
          .filter_map(|i| match i {
            Ipld::String(s) => Some(s.clone()),
            _ => None,
          })
          .collect(),
      ),
      _ => None,
    });
    Some(Self {
      text,
      created_at,
      langs,
    })
  }
}

#[derive(Debug, Clone, DagCbor)]
#[allow(non_snake_case)]
struct CommitInner {
  pub seq: i64,
  pub rebase: bool,
  pub tooBig: bool,
  pub repo: String,
  pub commit: Link<Cid>,
  pub prev: Option<Link<Cid>>,
  pub blocks: Ipld,
  pub ops: Vec<RepoOp>,
  pub blobs: Vec<Link<Cid>>,
  pub time: String,
}

/// Event "#commit"
#[derive(Debug, Clone)]
pub struct Commit {
  pub seq: i64,
  pub rebase: bool,
  pub too_big: bool,
  pub repo: String,
  pub commit: Link<Cid>,
  pub prev: Option<Link<Cid>>,
  pub blocks: Blocks,
  pub ops: Vec<RepoOp>,
  pub blobs: Vec<Blob>,
  pub time: DateTime<Utc>,
}

impl From<CommitInner> for Commit {
  fn from(value: CommitInner) -> Self {
    let blocks = if let Ipld::Bytes(blocks) = &value.blocks {
      Blocks::from(blocks.as_slice())
    } else {
      Blocks::default()
    };
    let blobs = (&value).into();
    Self {
      seq: value.seq,
      rebase: value.rebase,
      too_big: value.tooBig,
      repo: value.repo,
      commit: value.commit,
      prev: value.prev,
      blocks,
      ops: value.ops,
      blobs,
      time: value.time.parse().unwrap_or_default(),
    }
  }
}

impl Commit {
  /// Returns the posts included in an operation to the repository
  pub fn get_post(&self) -> Vec<(String, FeedPost)> {
    self
      .ops
      .iter()
      .filter(|op| op.action == "create" && op.path.starts_with("app.bsky.feed.post"))
      .filter_map(|op| op.cid.map(|c| (op.path.clone(), c)))
      .filter_map(|(path, cid)| {
        self
          .blocks
          .get(&cid)
          .and_then(|ipld| FeedPost::from(ipld).map(|fp| (path, fp)))
      })
      .collect()
  }

  /// Returns the text of all posts included in an operation to the repository
  pub fn get_post_text(&self) -> Vec<String> {
    log::info!("{:?}", self.get_post());
    self.get_post().into_iter().map(|(_, fp)| fp.text).collect()
  }

  /// Returns the cid of first post included in an operation to the repository
  pub fn get_post_path(&self) -> Option<String> {
    self.get_post().first().map(|(path, _)| path.clone())
  }
}

/// Blob attached to Event
#[derive(Debug, Clone)]
pub struct Blob {
  pub did: String,
  pub cid: String,
}

impl Display for Blob {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_fmt(format_args!(
      "https://bsky.social/xrpc/com.atproto.sync.getBlob?did={}&cid={}",
      self.did, self.cid
    ))
  }
}

impl Blob {
  /// Create a Blob instance from DID and CID
  pub fn new<T1: ToString, T2: ToString>(did: T1, cid: T2) -> Self {
    Self {
      did: did.to_string(),
      cid: cid.to_string(),
    }
  }
}

impl From<&CommitInner> for Vec<Blob> {
  fn from(value: &CommitInner) -> Self {
    value
      .blobs
      .iter()
      .map(|b| Blob::new(value.repo.clone(), b.to_string()))
      .collect()
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

#[derive(Debug, Clone, DagCbor)]
struct HandleInner {
  seq: i64,
  did: String,
  handle: String,
  time: String,
}

/// Event "#handle"
#[derive(Debug, Clone)]
pub struct Handle {
  pub seq: i64,
  pub did: String,
  pub handle: String,
  pub time: DateTime<Utc>,
}

impl From<HandleInner> for Handle {
  fn from(value: HandleInner) -> Self {
    Self {
      seq: value.seq,
      did: value.did,
      handle: value.handle,
      time: value.time.parse().unwrap_or_default(),
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

/// Event "#migrate"
#[derive(Debug, Clone)]
pub struct Migrate {
  pub seq: i64,
  pub did: String,
  pub migrate_to: Option<String>,
  pub time: DateTime<Utc>,
}

impl From<MigrateInner> for Migrate {
  fn from(value: MigrateInner) -> Self {
    Self {
      seq: value.seq,
      did: value.did,
      migrate_to: value.migrateTo,
      time: value.time.parse().unwrap_or_default(),
    }
  }
}

#[derive(Debug, Clone, DagCbor)]
struct TombstoneInner {
  seq: i64,
  did: String,
  time: String,
}

/// Event "#tombstone"
#[derive(Debug, Clone)]
pub struct Tombstone {
  pub seq: i64,
  pub did: String,
  pub time: DateTime<Utc>,
}

impl From<TombstoneInner> for Tombstone {
  fn from(value: TombstoneInner) -> Self {
    Self {
      seq: value.seq,
      did: value.did,
      time: value.time.parse().unwrap_or_default(),
    }
  }
}

/// Event "#info"
#[derive(Debug, Clone, DagCbor)]
pub struct Info {
  pub name: String,
  pub message: Option<String>,
}

/// Payload of Event
#[derive(Debug, Clone, Default)]
pub enum Payload {
  Commit(Commit),
  Handle(Handle),
  Migrate(Migrate),
  Tombstone(Tombstone),
  Info(Info),
  #[default]
  Null,
}

impl Payload {
  /// Get sequence number
  pub fn get_seq(&self) -> Option<i64> {
    match self {
      Self::Commit(c) => Some(c.seq),
      Self::Handle(h) => Some(h.seq),
      Self::Migrate(m) => Some(m.seq),
      Self::Tombstone(t) => Some(t.seq),
      Self::Info(_) => None,
      Self::Null => None,
    }
  }

  /// Get event time
  pub fn get_time(&self) -> Option<DateTime<Utc>> {
    match self {
      Self::Commit(c) => Some(c.time),
      Self::Handle(h) => Some(h.time),
      Self::Migrate(m) => Some(m.time),
      Self::Tombstone(t) => Some(t.time),
      Self::Info(_) => None,
      Self::Null => None,
    }
  }
}

/// Event
#[derive(Debug, Clone, Default)]
pub struct Event {
  pub header: Header,
  pub payload: Payload,
}

fn parse_commit(payload: &[u8]) -> Payload {
  match DagCborCodec
    .decode::<CommitInner>(payload)
    .map(|c| Payload::Commit(Commit::from(c)))
  {
    Ok(p) => p,
    Err(e) => {
      log::warn!("Event::Commit decode error {}", e);
      Payload::default()
    }
  }
}

fn parse_handle(payload: &[u8]) -> Payload {
  match DagCborCodec
    .decode::<HandleInner>(payload)
    .map(|h| Payload::Handle(Handle::from(h)))
  {
    Ok(p) => p,
    Err(e) => {
      log::warn!("Event::Handle decode error {}", e);
      Payload::default()
    }
  }
}

fn parse_migrate(payload: &[u8]) -> Payload {
  match DagCborCodec
    .decode::<MigrateInner>(payload)
    .map(|m| Payload::Migrate(Migrate::from(m)))
  {
    Ok(p) => p,
    Err(e) => {
      log::warn!("Event::Migrate decode error {}", e);
      Payload::default()
    }
  }
}

fn parse_tombstone(payload: &[u8]) -> Payload {
  match DagCborCodec
    .decode::<TombstoneInner>(payload)
    .map(|t| Payload::Tombstone(Tombstone::from(t)))
  {
    Ok(p) => p,
    Err(e) => {
      log::warn!("Event::Tombstone decode error {}", e);
      Payload::default()
    }
  }
}

fn parse_info(payload: &[u8]) -> Payload {
  match DagCborCodec
    .decode::<Info>(payload)
    .map(|i| Payload::Info(i))
  {
    Ok(p) => p,
    Err(e) => {
      log::warn!("Event::Info decode error {}", e);
      Payload::default()
    }
  }
}

impl From<&[u8]> for Event {
  fn from(data: &[u8]) -> Self {
    let mut ret = Self::default();
    let header = DagCborCodec.decode::<Header>(data).unwrap_or_default();
    ret.header = header.clone();
    if header.op < 0 {
      return ret;
    }
    let payload = data
      .get(header.len().unwrap_or_default()..)
      .unwrap_or_default();
    ret.payload = match header.t.as_str() {
      "#commit" => parse_commit(payload),
      "#handle" => parse_handle(payload),
      "#migrate" => parse_migrate(payload),
      "#tombstone" => parse_tombstone(payload),
      "#info" => parse_info(payload),
      _ => {
        log::warn!("unimplemented {:?}", header);
        return ret;
      }
    };
    ret
  }
}

impl Event {
  /// Returns Payload if Event is Commit
  pub fn as_commit(&self) -> Option<&Commit> {
    if let Payload::Commit(c) = &self.payload {
      Some(c)
    } else {
      None
    }
  }

  /// Returns Payload if Event is Handle
  pub fn as_handle(&self) -> Option<&Handle> {
    if let Payload::Handle(h) = &self.payload {
      Some(h)
    } else {
      None
    }
  }

  /// Returns Payload if Event is Migrate
  pub fn as_migrate(&self) -> Option<&Migrate> {
    if let Payload::Migrate(m) = &self.payload {
      Some(m)
    } else {
      None
    }
  }

  /// Returns Payload if Event is Tombstone
  pub fn as_tombstone(&self) -> Option<&Tombstone> {
    if let Payload::Tombstone(t) = &self.payload {
      Some(t)
    } else {
      None
    }
  }

  /// Returns Payload if Event is Info
  pub fn as_info(&self) -> Option<&Info> {
    if let Payload::Info(i) = &self.payload {
      Some(i)
    } else {
      None
    }
  }

  /// Get sequence number
  pub fn get_seq(&self) -> Option<i64> {
    self.payload.get_seq()
  }

  /// Get Event Time
  pub fn get_time(&self) -> Option<DateTime<Utc>> {
    self.payload.get_time()
  }

  /// Returns whether or not Event is empty
  pub fn is_empty(&self) -> bool {
    matches!(self.payload, Payload::Null)
  }
}
