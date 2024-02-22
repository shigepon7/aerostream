use std::{
  collections::HashMap,
  fmt::{Debug, Display},
  io::{Cursor, Seek},
  net::TcpStream,
  str::FromStr,
};

use anyhow::Result;
use chrono::{DateTime, Utc};
use libipld::{cbor::DagCborCodec, prelude::Codec, Cid, Ipld};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use serde_with::skip_serializing_none;
use tungstenite::{stream::MaybeTlsStream, WebSocket};
use ureq::{Agent, AgentBuilder, Proxy};
use url::Url;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyGraphDefsListpurpose(String);

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoAdminDefsSubjectreviewstate(String);

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoModerationDefsReasontype(String);

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyActorDefsPreferences(Vec<AppBskyActorDefsPreferencesItem>);

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyActorDefsProfileviewbasic {
  pub did: String,
  pub handle: String,
  #[serde(rename = "displayName")]
  pub display_name: Option<String>,
  pub avatar: Option<String>,
  pub viewer: Option<AppBskyActorDefsViewerstate>,
  pub labels: Option<Vec<ComAtprotoLabelDefsLabel>>,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyActorDefsProfileview {
  pub did: String,
  pub handle: String,
  #[serde(rename = "displayName")]
  pub display_name: Option<String>,
  pub description: Option<String>,
  pub avatar: Option<String>,
  #[serde(rename = "indexedAt")]
  pub indexed_at: Option<DateTime<Utc>>,
  pub viewer: Option<AppBskyActorDefsViewerstate>,
  pub labels: Option<Vec<ComAtprotoLabelDefsLabel>>,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyActorDefsProfileviewdetailed {
  pub did: String,
  pub handle: String,
  #[serde(rename = "displayName")]
  pub display_name: Option<String>,
  pub description: Option<String>,
  pub avatar: Option<String>,
  pub banner: Option<String>,
  #[serde(rename = "followersCount")]
  pub followers_count: Option<i64>,
  #[serde(rename = "followsCount")]
  pub follows_count: Option<i64>,
  #[serde(rename = "postsCount")]
  pub posts_count: Option<i64>,
  #[serde(rename = "indexedAt")]
  pub indexed_at: Option<DateTime<Utc>>,
  pub viewer: Option<AppBskyActorDefsViewerstate>,
  pub labels: Option<Vec<ComAtprotoLabelDefsLabel>>,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

/// Metadata about the requesting account&#39;s relationship with the subject account. Only has meaningful content for authed requests.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyActorDefsViewerstate {
  pub muted: Option<bool>,
  #[serde(rename = "mutedByList")]
  pub muted_by_list: Option<AppBskyGraphDefsListviewbasic>,
  #[serde(rename = "blockedBy")]
  pub blocked_by: Option<bool>,
  pub blocking: Option<String>,
  #[serde(rename = "blockingByList")]
  pub blocking_by_list: Option<AppBskyGraphDefsListviewbasic>,
  pub following: Option<String>,
  #[serde(rename = "followedBy")]
  pub followed_by: Option<String>,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyActorDefsAdultcontentpref {
  pub enabled: bool,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyActorDefsContentlabelpref {
  pub label: String,
  pub visibility: String,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyActorDefsSavedfeedspref {
  pub pinned: Vec<String>,
  pub saved: Vec<String>,
  #[serde(rename = "timelineIndex")]
  pub timeline_index: Option<i64>,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyActorDefsPersonaldetailspref {
  #[serde(rename = "birthDate")]
  pub birth_date: Option<DateTime<Utc>>,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyActorDefsFeedviewpref {
  pub feed: String,
  #[serde(rename = "hideReplies")]
  pub hide_replies: Option<bool>,
  #[serde(rename = "hideRepliesByUnfollowed")]
  pub hide_replies_by_unfollowed: Option<bool>,
  #[serde(rename = "hideRepliesByLikeCount")]
  pub hide_replies_by_like_count: Option<i64>,
  #[serde(rename = "hideReposts")]
  pub hide_reposts: Option<bool>,
  #[serde(rename = "hideQuotePosts")]
  pub hide_quote_posts: Option<bool>,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyActorDefsThreadviewpref {
  pub sort: Option<String>,
  #[serde(rename = "prioritizeFollowedUsers")]
  pub prioritize_followed_users: Option<bool>,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyActorDefsInterestspref {
  pub tags: Vec<String>,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

/// A representation of some externally linked content (eg, a URL and &#39;card&#39;), embedded in a Bluesky record (eg, a post).
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyEmbedExternal {
  pub external: AppBskyEmbedExternalExternal,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyEmbedExternalExternal {
  pub uri: String,
  pub title: String,
  pub description: String,
  pub thumb: Option<Blob>,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyEmbedExternalView {
  pub external: AppBskyEmbedExternalViewexternal,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyEmbedExternalViewexternal {
  pub uri: String,
  pub title: String,
  pub description: String,
  pub thumb: Option<String>,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyEmbedImages {
  pub images: Vec<AppBskyEmbedImagesImage>,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyEmbedImagesImage {
  pub image: Blob,
  pub alt: String,
  #[serde(rename = "aspectRatio")]
  pub aspect_ratio: Option<AppBskyEmbedImagesAspectratio>,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

/// width:height represents an aspect ratio. It may be approximate, and may not correspond to absolute dimensions in any given unit.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyEmbedImagesAspectratio {
  pub width: i64,
  pub height: i64,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyEmbedImagesView {
  pub images: Vec<AppBskyEmbedImagesViewimage>,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyEmbedImagesViewimage {
  pub thumb: String,
  pub fullsize: String,
  pub alt: String,
  #[serde(rename = "aspectRatio")]
  pub aspect_ratio: Option<AppBskyEmbedImagesAspectratio>,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyEmbedRecord {
  pub record: ComAtprotoRepoStrongref,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyEmbedRecordView {
  pub record: AppBskyEmbedRecordViewRecord,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyEmbedRecordViewrecord {
  pub uri: String,
  pub cid: CidString,
  pub author: AppBskyActorDefsProfileviewbasic,
  pub value: Record,
  #[serde(rename = "indexedAt")]
  pub indexed_at: DateTime<Utc>,
  pub labels: Option<Vec<ComAtprotoLabelDefsLabel>>,
  pub embeds: Option<Vec<AppBskyEmbedRecordViewrecordEmbedsItem>>,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyEmbedRecordViewnotfound {
  pub uri: String,
  #[serde(rename = "notFound")]
  pub not_found: bool,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyEmbedRecordViewblocked {
  pub uri: String,
  pub blocked: bool,
  pub author: AppBskyFeedDefsBlockedauthor,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyEmbedRecordwithmedia {
  pub record: AppBskyEmbedRecord,
  pub media: AppBskyEmbedRecordwithmediaMainMedia,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyEmbedRecordwithmediaView {
  pub record: AppBskyEmbedRecordView,
  pub media: AppBskyEmbedRecordwithmediaViewMedia,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyFeedDefsPostview {
  pub uri: String,
  pub cid: CidString,
  pub author: AppBskyActorDefsProfileviewbasic,
  pub record: Record,
  #[serde(rename = "indexedAt")]
  pub indexed_at: DateTime<Utc>,
  pub embed: Option<AppBskyFeedDefsPostviewEmbed>,
  #[serde(rename = "replyCount")]
  pub reply_count: Option<i64>,
  #[serde(rename = "repostCount")]
  pub repost_count: Option<i64>,
  #[serde(rename = "likeCount")]
  pub like_count: Option<i64>,
  pub viewer: Option<AppBskyFeedDefsViewerstate>,
  pub labels: Option<Vec<ComAtprotoLabelDefsLabel>>,
  pub threadgate: Option<AppBskyFeedDefsThreadgateview>,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

/// Metadata about the requesting account&#39;s relationship with the subject content. Only has meaningful content for authed requests.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyFeedDefsViewerstate {
  pub repost: Option<String>,
  pub like: Option<String>,
  #[serde(rename = "replyDisabled")]
  pub reply_disabled: Option<bool>,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyFeedDefsFeedviewpost {
  pub post: AppBskyFeedDefsPostview,
  pub reply: Option<AppBskyFeedDefsReplyref>,
  pub reason: Option<AppBskyFeedDefsFeedviewpostReason>,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyFeedDefsReplyref {
  pub root: AppBskyFeedDefsReplyrefRoot,
  pub parent: AppBskyFeedDefsReplyrefParent,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyFeedDefsReasonrepost {
  pub by: AppBskyActorDefsProfileviewbasic,
  #[serde(rename = "indexedAt")]
  pub indexed_at: DateTime<Utc>,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyFeedDefsThreadviewpost {
  pub post: AppBskyFeedDefsPostview,
  pub parent: Option<AppBskyFeedDefsThreadviewpostParent>,
  pub replies: Option<Vec<AppBskyFeedDefsThreadviewpostRepliesItem>>,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyFeedDefsNotfoundpost {
  pub uri: String,
  #[serde(rename = "notFound")]
  pub not_found: bool,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyFeedDefsBlockedpost {
  pub uri: String,
  pub blocked: bool,
  pub author: AppBskyFeedDefsBlockedauthor,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyFeedDefsBlockedauthor {
  pub did: String,
  pub viewer: Option<AppBskyActorDefsViewerstate>,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyFeedDefsGeneratorview {
  pub uri: String,
  pub cid: CidString,
  pub did: String,
  pub creator: AppBskyActorDefsProfileview,
  #[serde(rename = "displayName")]
  pub display_name: String,
  #[serde(rename = "indexedAt")]
  pub indexed_at: DateTime<Utc>,
  pub description: Option<String>,
  #[serde(rename = "descriptionFacets")]
  pub description_facets: Option<Vec<AppBskyRichtextFacet>>,
  pub avatar: Option<String>,
  #[serde(rename = "likeCount")]
  pub like_count: Option<i64>,
  pub viewer: Option<AppBskyFeedDefsGeneratorviewerstate>,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyFeedDefsGeneratorviewerstate {
  pub like: Option<String>,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyFeedDefsSkeletonfeedpost {
  pub post: String,
  pub reason: Option<AppBskyFeedDefsSkeletonfeedpostReason>,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyFeedDefsSkeletonreasonrepost {
  pub repost: String,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyFeedDefsThreadgateview {
  pub uri: Option<String>,
  pub cid: Option<CidString>,
  pub record: Option<Record>,
  pub lists: Option<Vec<AppBskyGraphDefsListviewbasic>>,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyFeedDescribefeedgeneratorFeed {
  pub uri: String,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyFeedDescribefeedgeneratorLinks {
  #[serde(rename = "privacyPolicy")]
  pub privacy_policy: Option<String>,
  #[serde(rename = "termsOfService")]
  pub terms_of_service: Option<String>,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyFeedGetlikesLike {
  #[serde(rename = "indexedAt")]
  pub indexed_at: DateTime<Utc>,
  #[serde(rename = "createdAt")]
  pub created_at: DateTime<Utc>,
  pub actor: AppBskyActorDefsProfileview,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyFeedPostReplyref {
  pub root: ComAtprotoRepoStrongref,
  pub parent: ComAtprotoRepoStrongref,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

/// Deprecated: use facets instead.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyFeedPostEntity {
  pub index: AppBskyFeedPostTextslice,
  #[serde(rename = "type")]
  pub value_type: String,
  pub value: String,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

/// Deprecated. Use app.bsky.richtext instead -- A text segment. Start is inclusive, end is exclusive. Indices are for utf16-encoded strings.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyFeedPostTextslice {
  pub start: i64,
  pub end: i64,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

/// Allow replies from actors mentioned in your post.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyFeedThreadgateMentionrule {
  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

/// Allow replies from actors you follow.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyFeedThreadgateFollowingrule {
  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

/// Allow replies from actors on a list.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyFeedThreadgateListrule {
  pub list: String,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyGraphDefsListviewbasic {
  pub uri: String,
  pub cid: CidString,
  pub name: String,
  pub purpose: AppBskyGraphDefsListpurpose,
  pub avatar: Option<String>,
  pub viewer: Option<AppBskyGraphDefsListviewerstate>,
  #[serde(rename = "indexedAt")]
  pub indexed_at: Option<DateTime<Utc>>,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyGraphDefsListview {
  pub uri: String,
  pub cid: CidString,
  pub creator: AppBskyActorDefsProfileview,
  pub name: String,
  pub purpose: AppBskyGraphDefsListpurpose,
  #[serde(rename = "indexedAt")]
  pub indexed_at: DateTime<Utc>,
  pub description: Option<String>,
  #[serde(rename = "descriptionFacets")]
  pub description_facets: Option<Vec<AppBskyRichtextFacet>>,
  pub avatar: Option<String>,
  pub viewer: Option<AppBskyGraphDefsListviewerstate>,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyGraphDefsListitemview {
  pub uri: String,
  pub subject: AppBskyActorDefsProfileview,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyGraphDefsListviewerstate {
  pub muted: Option<bool>,
  pub blocked: Option<String>,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

/// indicates that a handle or DID could not be resolved
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyGraphDefsNotfoundactor {
  pub actor: String,
  #[serde(rename = "notFound")]
  pub not_found: bool,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

/// lists the bi-directional graph relationships between one actor (not indicated in the object), and the target actors (the DID included in the object)
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyGraphDefsRelationship {
  pub did: String,
  pub following: Option<String>,
  #[serde(rename = "followedBy")]
  pub followed_by: Option<String>,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyNotificationListnotificationsNotification {
  pub uri: String,
  pub cid: CidString,
  pub author: AppBskyActorDefsProfileview,
  pub reason: String,
  pub record: Record,
  #[serde(rename = "isRead")]
  pub is_read: bool,
  #[serde(rename = "indexedAt")]
  pub indexed_at: DateTime<Utc>,
  #[serde(rename = "reasonSubject")]
  pub reason_subject: Option<String>,
  pub labels: Option<Vec<ComAtprotoLabelDefsLabel>>,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

/// Annotation of a sub-string within rich text.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyRichtextFacet {
  pub index: AppBskyRichtextFacetByteslice,
  pub features: Vec<AppBskyRichtextFacetMainFeaturesItem>,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

/// Facet feature for mention of another account. The text is usually a handle, including a &#39;@&#39; prefix, but the facet reference is a DID.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyRichtextFacetMention {
  pub did: String,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

/// Facet feature for a URL. The text URL may have been simplified or truncated, but the facet reference should be a complete URL.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyRichtextFacetLink {
  pub uri: String,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

/// Facet feature for a hashtag. The text usually includes a &#39;#&#39; prefix, but the facet reference should not (except in the case of &#39;double hash tags&#39;).
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyRichtextFacetTag {
  pub tag: String,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

/// Specifies the sub-string range a facet feature applies to. Start index is inclusive, end index is exclusive. Indices are zero-indexed, counting bytes of the UTF-8 encoded text. NOTE: some languages, like Javascript, use UTF-16 or Unicode codepoints for string slice indexing; in these languages, convert to byte arrays before working with facets.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyRichtextFacetByteslice {
  #[serde(rename = "byteStart")]
  pub byte_start: i64,
  #[serde(rename = "byteEnd")]
  pub byte_end: i64,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyUnspeccedDefsSkeletonsearchpost {
  pub uri: String,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyUnspeccedDefsSkeletonsearchactor {
  pub did: String,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyUnspeccedGettaggedsuggestionsSuggestion {
  pub tag: String,
  #[serde(rename = "subjectType")]
  pub subject_type: String,
  pub subject: String,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoAdminDefsStatusattr {
  pub applied: bool,
  #[serde(rename = "r#ref")]
  pub r_ref: Option<String>,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoAdminDefsModeventview {
  pub id: i64,
  pub event: ComAtprotoAdminDefsModeventviewEvent,
  pub subject: ComAtprotoAdminDefsModeventviewSubject,
  #[serde(rename = "subjectBlobCids")]
  pub subject_blob_cids: Vec<String>,
  #[serde(rename = "createdBy")]
  pub created_by: String,
  #[serde(rename = "createdAt")]
  pub created_at: DateTime<Utc>,
  #[serde(rename = "creatorHandle")]
  pub creator_handle: Option<String>,
  #[serde(rename = "subjectHandle")]
  pub subject_handle: Option<String>,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoAdminDefsModeventviewdetail {
  pub id: i64,
  pub event: ComAtprotoAdminDefsModeventviewdetailEvent,
  pub subject: ComAtprotoAdminDefsModeventviewdetailSubject,
  #[serde(rename = "subjectBlobs")]
  pub subject_blobs: Vec<ComAtprotoAdminDefsBlobview>,
  #[serde(rename = "createdBy")]
  pub created_by: String,
  #[serde(rename = "createdAt")]
  pub created_at: DateTime<Utc>,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoAdminDefsReportview {
  pub id: i64,
  #[serde(rename = "reasonType")]
  pub reason_type: ComAtprotoModerationDefsReasontype,
  pub subject: ComAtprotoAdminDefsReportviewSubject,
  #[serde(rename = "reportedBy")]
  pub reported_by: String,
  #[serde(rename = "createdAt")]
  pub created_at: DateTime<Utc>,
  #[serde(rename = "resolvedByActionIds")]
  pub resolved_by_action_ids: Vec<i64>,
  pub comment: Option<String>,
  #[serde(rename = "subjectRepoHandle")]
  pub subject_repo_handle: Option<String>,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoAdminDefsSubjectstatusview {
  pub id: i64,
  pub subject: ComAtprotoAdminDefsSubjectstatusviewSubject,
  #[serde(rename = "updatedAt")]
  pub updated_at: DateTime<Utc>,
  #[serde(rename = "createdAt")]
  pub created_at: DateTime<Utc>,
  #[serde(rename = "reviewState")]
  pub review_state: ComAtprotoAdminDefsSubjectreviewstate,
  #[serde(rename = "subjectBlobCids")]
  pub subject_blob_cids: Option<Vec<CidString>>,
  #[serde(rename = "subjectRepoHandle")]
  pub subject_repo_handle: Option<String>,
  pub comment: Option<String>,
  #[serde(rename = "muteUntil")]
  pub mute_until: Option<DateTime<Utc>>,
  #[serde(rename = "lastReviewedBy")]
  pub last_reviewed_by: Option<String>,
  #[serde(rename = "lastReviewedAt")]
  pub last_reviewed_at: Option<DateTime<Utc>>,
  #[serde(rename = "lastReportedAt")]
  pub last_reported_at: Option<DateTime<Utc>>,
  #[serde(rename = "lastAppealedAt")]
  pub last_appealed_at: Option<DateTime<Utc>>,
  pub takendown: Option<bool>,
  pub appealed: Option<bool>,
  #[serde(rename = "suspendUntil")]
  pub suspend_until: Option<DateTime<Utc>>,
  pub tags: Option<Vec<String>>,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoAdminDefsReportviewdetail {
  pub id: i64,
  #[serde(rename = "reasonType")]
  pub reason_type: ComAtprotoModerationDefsReasontype,
  pub subject: ComAtprotoAdminDefsReportviewdetailSubject,
  #[serde(rename = "reportedBy")]
  pub reported_by: String,
  #[serde(rename = "createdAt")]
  pub created_at: DateTime<Utc>,
  #[serde(rename = "resolvedByActions")]
  pub resolved_by_actions: Vec<ComAtprotoAdminDefsModeventview>,
  pub comment: Option<String>,
  #[serde(rename = "subjectStatus")]
  pub subject_status: Option<ComAtprotoAdminDefsSubjectstatusview>,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoAdminDefsRepoview {
  pub did: String,
  pub handle: String,
  #[serde(rename = "relatedRecords")]
  pub related_records: Vec<Record>,
  #[serde(rename = "indexedAt")]
  pub indexed_at: DateTime<Utc>,
  pub moderation: ComAtprotoAdminDefsModeration,
  pub email: Option<String>,
  #[serde(rename = "invitedBy")]
  pub invited_by: Option<ComAtprotoServerDefsInvitecode>,
  #[serde(rename = "invitesDisabled")]
  pub invites_disabled: Option<bool>,
  #[serde(rename = "inviteNote")]
  pub invite_note: Option<String>,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoAdminDefsRepoviewdetail {
  pub did: String,
  pub handle: String,
  #[serde(rename = "relatedRecords")]
  pub related_records: Vec<Record>,
  #[serde(rename = "indexedAt")]
  pub indexed_at: DateTime<Utc>,
  pub moderation: ComAtprotoAdminDefsModerationdetail,
  pub email: Option<String>,
  pub labels: Option<Vec<ComAtprotoLabelDefsLabel>>,
  #[serde(rename = "invitedBy")]
  pub invited_by: Option<ComAtprotoServerDefsInvitecode>,
  pub invites: Option<Vec<ComAtprotoServerDefsInvitecode>>,
  #[serde(rename = "invitesDisabled")]
  pub invites_disabled: Option<bool>,
  #[serde(rename = "inviteNote")]
  pub invite_note: Option<String>,
  #[serde(rename = "emailConfirmedAt")]
  pub email_confirmed_at: Option<DateTime<Utc>>,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoAdminDefsAccountview {
  pub did: String,
  pub handle: String,
  #[serde(rename = "indexedAt")]
  pub indexed_at: DateTime<Utc>,
  pub email: Option<String>,
  #[serde(rename = "relatedRecords")]
  pub related_records: Option<Vec<Record>>,
  #[serde(rename = "invitedBy")]
  pub invited_by: Option<ComAtprotoServerDefsInvitecode>,
  pub invites: Option<Vec<ComAtprotoServerDefsInvitecode>>,
  #[serde(rename = "invitesDisabled")]
  pub invites_disabled: Option<bool>,
  #[serde(rename = "emailConfirmedAt")]
  pub email_confirmed_at: Option<DateTime<Utc>>,
  #[serde(rename = "inviteNote")]
  pub invite_note: Option<String>,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoAdminDefsRepoviewnotfound {
  pub did: String,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoAdminDefsReporef {
  pub did: String,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoAdminDefsRepoblobref {
  pub did: String,
  pub cid: CidString,
  #[serde(rename = "recordUri")]
  pub record_uri: Option<String>,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoAdminDefsRecordview {
  pub uri: String,
  pub cid: CidString,
  pub value: Record,
  #[serde(rename = "blobCids")]
  pub blob_cids: Vec<CidString>,
  #[serde(rename = "indexedAt")]
  pub indexed_at: DateTime<Utc>,
  pub moderation: ComAtprotoAdminDefsModeration,
  pub repo: ComAtprotoAdminDefsRepoview,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoAdminDefsRecordviewdetail {
  pub uri: String,
  pub cid: CidString,
  pub value: Record,
  pub blobs: Vec<ComAtprotoAdminDefsBlobview>,
  #[serde(rename = "indexedAt")]
  pub indexed_at: DateTime<Utc>,
  pub moderation: ComAtprotoAdminDefsModerationdetail,
  pub repo: ComAtprotoAdminDefsRepoview,
  pub labels: Option<Vec<ComAtprotoLabelDefsLabel>>,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoAdminDefsRecordviewnotfound {
  pub uri: String,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoAdminDefsModeration {
  #[serde(rename = "subjectStatus")]
  pub subject_status: Option<ComAtprotoAdminDefsSubjectstatusview>,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoAdminDefsModerationdetail {
  #[serde(rename = "subjectStatus")]
  pub subject_status: Option<ComAtprotoAdminDefsSubjectstatusview>,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoAdminDefsBlobview {
  pub cid: CidString,
  #[serde(rename = "mimeType")]
  pub mime_type: String,
  pub size: i64,
  #[serde(rename = "createdAt")]
  pub created_at: DateTime<Utc>,
  pub details: Option<ComAtprotoAdminDefsBlobviewDetails>,
  pub moderation: Option<ComAtprotoAdminDefsModeration>,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoAdminDefsImagedetails {
  pub width: i64,
  pub height: i64,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoAdminDefsVideodetails {
  pub width: i64,
  pub height: i64,
  pub length: i64,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

/// Take down a subject permanently or temporarily
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoAdminDefsModeventtakedown {
  pub comment: Option<String>,
  #[serde(rename = "durationInHours")]
  pub duration_in_hours: Option<i64>,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

/// Revert take down action on a subject
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoAdminDefsModeventreversetakedown {
  pub comment: Option<String>,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

/// Resolve appeal on a subject
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoAdminDefsModeventresolveappeal {
  pub comment: Option<String>,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

/// Add a comment to a subject
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoAdminDefsModeventcomment {
  pub comment: String,
  pub sticky: Option<bool>,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

/// Report a subject
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoAdminDefsModeventreport {
  #[serde(rename = "reportType")]
  pub report_type: ComAtprotoModerationDefsReasontype,
  pub comment: Option<String>,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

/// Apply/Negate labels on a subject
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoAdminDefsModeventlabel {
  #[serde(rename = "createLabelVals")]
  pub create_label_vals: Vec<String>,
  #[serde(rename = "negateLabelVals")]
  pub negate_label_vals: Vec<String>,
  pub comment: Option<String>,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoAdminDefsModeventacknowledge {
  pub comment: Option<String>,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoAdminDefsModeventescalate {
  pub comment: Option<String>,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

/// Mute incoming reports on a subject
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoAdminDefsModeventmute {
  #[serde(rename = "durationInHours")]
  pub duration_in_hours: i64,
  pub comment: Option<String>,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

/// Unmute action on a subject
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoAdminDefsModeventunmute {
  pub comment: Option<String>,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

/// Keep a log of outgoing email to a user
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoAdminDefsModeventemail {
  #[serde(rename = "subjectLine")]
  pub subject_line: String,
  pub comment: Option<String>,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

/// Add/Remove a tag on a subject
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoAdminDefsModeventtag {
  pub add: Vec<String>,
  pub remove: Vec<String>,
  pub comment: Option<String>,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoAdminDefsCommunicationtemplateview {
  pub id: String,
  pub name: String,
  #[serde(rename = "contentMarkdown")]
  pub content_markdown: String,
  pub disabled: bool,
  #[serde(rename = "lastUpdatedBy")]
  pub last_updated_by: String,
  #[serde(rename = "createdAt")]
  pub created_at: DateTime<Utc>,
  #[serde(rename = "updatedAt")]
  pub updated_at: DateTime<Utc>,
  pub subject: Option<String>,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

/// Metadata tag on an atproto resource (eg, repo or record).
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoLabelDefsLabel {
  pub src: String,
  pub uri: String,
  pub val: String,
  pub cts: DateTime<Utc>,
  pub cid: Option<CidString>,
  pub neg: Option<bool>,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

/// Metadata tags on an atproto record, published by the author within the record.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoLabelDefsSelflabels {
  pub values: Vec<ComAtprotoLabelDefsSelflabel>,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

/// Metadata tag on an atproto record, published by the author within the record. Note that schemas should use #selfLabels, not #selfLabel.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoLabelDefsSelflabel {
  pub val: String,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoLabelSubscribelabelsLabels {
  pub seq: i64,
  pub labels: Vec<ComAtprotoLabelDefsLabel>,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoLabelSubscribelabelsInfo {
  pub name: String,
  pub message: Option<String>,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

/// Operation which creates a new record.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoRepoApplywritesCreate {
  pub collection: String,
  pub value: Record,
  pub rkey: Option<String>,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

/// Operation which updates an existing record.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoRepoApplywritesUpdate {
  pub collection: String,
  pub rkey: String,
  pub value: Record,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

/// Operation which deletes an existing record.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoRepoApplywritesDelete {
  pub collection: String,
  pub rkey: String,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoRepoListmissingblobsRecordblob {
  pub cid: CidString,
  #[serde(rename = "recordUri")]
  pub record_uri: String,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoRepoListrecordsRecord {
  pub uri: String,
  pub cid: CidString,
  pub value: Record,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoRepoStrongref {
  pub uri: String,
  pub cid: CidString,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoServerCreateapppasswordApppassword {
  pub name: String,
  pub password: String,
  #[serde(rename = "createdAt")]
  pub created_at: DateTime<Utc>,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoServerCreateinvitecodesAccountcodes {
  pub account: String,
  pub codes: Vec<String>,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoServerDefsInvitecode {
  pub code: String,
  pub available: i64,
  pub disabled: bool,
  #[serde(rename = "forAccount")]
  pub for_account: String,
  #[serde(rename = "createdBy")]
  pub created_by: String,
  #[serde(rename = "createdAt")]
  pub created_at: DateTime<Utc>,
  pub uses: Vec<ComAtprotoServerDefsInvitecodeuse>,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoServerDefsInvitecodeuse {
  #[serde(rename = "usedBy")]
  pub used_by: String,
  #[serde(rename = "usedAt")]
  pub used_at: DateTime<Utc>,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoServerDescribeserverLinks {
  #[serde(rename = "privacyPolicy")]
  pub privacy_policy: Option<String>,
  #[serde(rename = "termsOfService")]
  pub terms_of_service: Option<String>,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoServerListapppasswordsApppassword {
  pub name: String,
  #[serde(rename = "createdAt")]
  pub created_at: DateTime<Utc>,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoSyncListreposRepo {
  pub did: String,
  pub head: CidString,
  pub rev: String,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

/// Represents an update of repository state. Note that empty commits are allowed, which include no repo data changes, but an update to rev and signature.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoSyncSubscribereposCommit {
  pub seq: i64,
  pub rebase: bool,
  #[serde(rename = "tooBig")]
  pub too_big: bool,
  pub repo: String,
  pub commit: String,
  pub rev: String,
  pub since: String,
  pub blocks: Vec<u8>,
  pub ops: Vec<ComAtprotoSyncSubscribereposRepoop>,
  pub blobs: Vec<String>,
  pub time: DateTime<Utc>,
  pub prev: Option<String>,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

/// Represents a change to an account&#39;s identity. Could be an updated handle, signing key, or pds hosting endpoint. Serves as a prod to all downstream services to refresh their identity cache.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoSyncSubscribereposIdentity {
  pub seq: i64,
  pub did: String,
  pub time: DateTime<Utc>,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

/// Represents an update of the account&#39;s handle, or transition to/from invalid state. NOTE: Will be deprecated in favor of #identity.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoSyncSubscribereposHandle {
  pub seq: i64,
  pub did: String,
  pub handle: String,
  pub time: DateTime<Utc>,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

/// Represents an account moving from one PDS instance to another. NOTE: not implemented; account migration uses #identity instead
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoSyncSubscribereposMigrate {
  pub seq: i64,
  pub did: String,
  #[serde(rename = "migrateTo")]
  pub migrate_to: String,
  pub time: DateTime<Utc>,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

/// Indicates that an account has been deleted. NOTE: may be deprecated in favor of #identity or a future #account event
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoSyncSubscribereposTombstone {
  pub seq: i64,
  pub did: String,
  pub time: DateTime<Utc>,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoSyncSubscribereposInfo {
  pub name: String,
  pub message: Option<String>,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

/// A repo operation, ie a mutation of a single record.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoSyncSubscribereposRepoop {
  pub action: String,
  pub path: String,
  pub cid: String,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

/// A declaration of a Bluesky account profile.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyActorProfile {
  #[serde(rename = "displayName")]
  pub display_name: Option<String>,
  pub description: Option<String>,
  pub avatar: Option<Blob>,
  pub banner: Option<Blob>,
  pub labels: Option<AppBskyActorProfileMainLabels>,
}

/// Record declaring of the existence of a feed generator, and containing metadata about it. The record can exist in any repository.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyFeedGenerator {
  pub did: String,
  #[serde(rename = "displayName")]
  pub display_name: String,
  #[serde(rename = "createdAt")]
  pub created_at: DateTime<Utc>,
  pub description: Option<String>,
  #[serde(rename = "descriptionFacets")]
  pub description_facets: Option<Vec<AppBskyRichtextFacet>>,
  pub avatar: Option<Blob>,
  pub labels: Option<AppBskyFeedGeneratorMainLabels>,
}

/// Record declaring a &#39;like&#39; of a piece of subject content.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyFeedLike {
  pub subject: ComAtprotoRepoStrongref,
  #[serde(rename = "createdAt")]
  pub created_at: DateTime<Utc>,
}

/// Record containing a Bluesky post.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyFeedPost {
  pub text: String,
  #[serde(rename = "createdAt")]
  pub created_at: DateTime<Utc>,
  pub entities: Option<Vec<AppBskyFeedPostEntity>>,
  pub facets: Option<Vec<AppBskyRichtextFacet>>,
  pub reply: Option<AppBskyFeedPostReplyref>,
  pub embed: Option<AppBskyFeedPostMainEmbed>,
  pub langs: Option<Vec<String>>,
  pub labels: Option<AppBskyFeedPostMainLabels>,
  pub tags: Option<Vec<String>>,
}

/// Record representing a &#39;repost&#39; of an existing Bluesky post.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyFeedRepost {
  pub subject: ComAtprotoRepoStrongref,
  #[serde(rename = "createdAt")]
  pub created_at: DateTime<Utc>,
}

/// Record defining interaction gating rules for a thread (aka, reply controls). The record key (rkey) of the threadgate record must match the record key of the thread&#39;s root post, and that record must be in the same repository..
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyFeedThreadgate {
  pub post: String,
  #[serde(rename = "createdAt")]
  pub created_at: DateTime<Utc>,
  pub allow: Option<Vec<AppBskyFeedThreadgateMainAllowItem>>,
}

/// Record declaring a &#39;block&#39; relationship against another account. NOTE: blocks are public in Bluesky; see blog posts for details.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyGraphBlock {
  pub subject: String,
  #[serde(rename = "createdAt")]
  pub created_at: DateTime<Utc>,
}

/// Record declaring a social &#39;follow&#39; relationship of another account. Duplicate follows will be ignored by the AppView.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyGraphFollow {
  pub subject: String,
  #[serde(rename = "createdAt")]
  pub created_at: DateTime<Utc>,
}

/// Record representing a list of accounts (actors). Scope includes both moderation-oriented lists and curration-oriented lists.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyGraphList {
  pub purpose: AppBskyGraphDefsListpurpose,
  pub name: String,
  #[serde(rename = "createdAt")]
  pub created_at: DateTime<Utc>,
  pub description: Option<String>,
  #[serde(rename = "descriptionFacets")]
  pub description_facets: Option<Vec<AppBskyRichtextFacet>>,
  pub avatar: Option<Blob>,
  pub labels: Option<AppBskyGraphListMainLabels>,
}

/// Record representing a block relationship against an entire an entire list of accounts (actors).
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyGraphListblock {
  pub subject: String,
  #[serde(rename = "createdAt")]
  pub created_at: DateTime<Utc>,
}

/// Record representing an account&#39;s inclusion on a specific list. The AppView will ignore duplicate listitem records.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyGraphListitem {
  pub subject: String,
  pub list: String,
  #[serde(rename = "createdAt")]
  pub created_at: DateTime<Utc>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "$type")]
pub enum Record {
  #[serde(rename = "app.bsky.actor.profile")]
  AppBskyActorProfile(AppBskyActorProfile),

  #[serde(rename = "app.bsky.feed.generator")]
  AppBskyFeedGenerator(AppBskyFeedGenerator),

  #[serde(rename = "app.bsky.feed.like")]
  AppBskyFeedLike(AppBskyFeedLike),

  #[serde(rename = "app.bsky.feed.post")]
  AppBskyFeedPost(AppBskyFeedPost),

  #[serde(rename = "app.bsky.feed.repost")]
  AppBskyFeedRepost(AppBskyFeedRepost),

  #[serde(rename = "app.bsky.feed.threadgate")]
  AppBskyFeedThreadgate(AppBskyFeedThreadgate),

  #[serde(rename = "app.bsky.graph.block")]
  AppBskyGraphBlock(AppBskyGraphBlock),

  #[serde(rename = "app.bsky.graph.follow")]
  AppBskyGraphFollow(AppBskyGraphFollow),

  #[serde(rename = "app.bsky.graph.list")]
  AppBskyGraphList(AppBskyGraphList),

  #[serde(rename = "app.bsky.graph.listblock")]
  AppBskyGraphListblock(AppBskyGraphListblock),

  #[serde(rename = "app.bsky.graph.listitem")]
  AppBskyGraphListitem(AppBskyGraphListitem),

  #[serde(other)]
  Other,
}

impl Default for Record {
  fn default() -> Self {
    Self::AppBskyFeedPost(AppBskyFeedPost::default())
  }
}

pub fn ipld_to_string(ipld: &Ipld) -> String {
  match ipld {
    Ipld::Bool(b) => b.to_string(),
    Ipld::Bytes(b) => format!(
      "[{}]",
      b.iter()
        .map(|v| v.to_string())
        .collect::<Vec<_>>()
        .join(",")
    ),
    Ipld::Float(f) => f.to_string(),
    Ipld::Integer(i) => i.to_string(),
    Ipld::Link(l) => format!("{{\"$link\": \"{}\"}}", l.to_string()),
    Ipld::List(l) => format!(
      "[{}]",
      l.iter()
        .map(|v| ipld_to_string(v))
        .collect::<Vec<_>>()
        .join(",")
    ),
    Ipld::Map(m) => format!(
      "{{{}}}",
      m.iter()
        .map(|(k, v)| format!("\"{}\":{}", k, ipld_to_string(v)))
        .collect::<Vec<_>>()
        .join(",")
    ),
    Ipld::Null => String::from("null"),
    Ipld::String(s) => format!(
      "\"{}\"",
      s.replace(r"\", r"\\")
        .replace('"', "\\\"")
        .replace("\n", r"\n")
    ),
  }
}

impl TryFrom<&Ipld> for Record {
  type Error = anyhow::Error;
  fn try_from(value: &Ipld) -> std::result::Result<Self, Self::Error> {
    Ok(serde_json::from_str(&ipld_to_string(value))?)
  }
}

impl Record {
  pub fn as_app_bsky_actor_profile(&self) -> Option<&AppBskyActorProfile> {
    match self {
      Self::AppBskyActorProfile(v) => Some(v),
      _ => None,
    }
  }

  pub fn as_app_bsky_feed_generator(&self) -> Option<&AppBskyFeedGenerator> {
    match self {
      Self::AppBskyFeedGenerator(v) => Some(v),
      _ => None,
    }
  }

  pub fn as_app_bsky_feed_like(&self) -> Option<&AppBskyFeedLike> {
    match self {
      Self::AppBskyFeedLike(v) => Some(v),
      _ => None,
    }
  }

  pub fn as_app_bsky_feed_post(&self) -> Option<&AppBskyFeedPost> {
    match self {
      Self::AppBskyFeedPost(v) => Some(v),
      _ => None,
    }
  }

  pub fn as_app_bsky_feed_repost(&self) -> Option<&AppBskyFeedRepost> {
    match self {
      Self::AppBskyFeedRepost(v) => Some(v),
      _ => None,
    }
  }

  pub fn as_app_bsky_feed_threadgate(&self) -> Option<&AppBskyFeedThreadgate> {
    match self {
      Self::AppBskyFeedThreadgate(v) => Some(v),
      _ => None,
    }
  }

  pub fn as_app_bsky_graph_block(&self) -> Option<&AppBskyGraphBlock> {
    match self {
      Self::AppBskyGraphBlock(v) => Some(v),
      _ => None,
    }
  }

  pub fn as_app_bsky_graph_follow(&self) -> Option<&AppBskyGraphFollow> {
    match self {
      Self::AppBskyGraphFollow(v) => Some(v),
      _ => None,
    }
  }

  pub fn as_app_bsky_graph_list(&self) -> Option<&AppBskyGraphList> {
    match self {
      Self::AppBskyGraphList(v) => Some(v),
      _ => None,
    }
  }

  pub fn as_app_bsky_graph_listblock(&self) -> Option<&AppBskyGraphListblock> {
    match self {
      Self::AppBskyGraphListblock(v) => Some(v),
      _ => None,
    }
  }

  pub fn as_app_bsky_graph_listitem(&self) -> Option<&AppBskyGraphListitem> {
    match self {
      Self::AppBskyGraphListitem(v) => Some(v),
      _ => None,
    }
  }

  pub fn get_created_at(&self) -> Option<DateTime<Utc>> {
    match self {
      Self::AppBskyActorProfile(_) => None,
      Self::AppBskyFeedGenerator(v) => Some(v.created_at),
      Self::AppBskyFeedLike(v) => Some(v.created_at),
      Self::AppBskyFeedPost(v) => Some(v.created_at),
      Self::AppBskyFeedRepost(v) => Some(v.created_at),
      Self::AppBskyFeedThreadgate(v) => Some(v.created_at),
      Self::AppBskyGraphBlock(v) => Some(v.created_at),
      Self::AppBskyGraphFollow(v) => Some(v.created_at),
      Self::AppBskyGraphList(v) => Some(v.created_at),
      Self::AppBskyGraphListblock(v) => Some(v.created_at),
      Self::AppBskyGraphListitem(v) => Some(v.created_at),
      _ => None,
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NodeEntry {
  pub p: i64,
  pub k: Vec<u8>,
  pub v: Link,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Node {
  pub l: Option<Link>,
  pub e: Vec<NodeEntry>,
}

impl TryFrom<&Ipld> for Node {
  type Error = anyhow::Error;
  fn try_from(value: &Ipld) -> std::result::Result<Self, Self::Error> {
    Ok(serde_json::from_str(&ipld_to_string(value))?)
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Commit {
  pub did: String,
  pub version: i64,
  pub prev: Option<Link>,
  pub data: Link,
  pub sig: Vec<u8>,
}

impl TryFrom<&Ipld> for Commit {
  type Error = anyhow::Error;
  fn try_from(value: &Ipld) -> std::result::Result<Self, Self::Error> {
    Ok(serde_json::from_str(&ipld_to_string(value))?)
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CidString(String);

impl CidString {
  pub fn to_cid(&self) -> Result<Cid> {
    Ok(Cid::from_str(&self.0)?)
  }
}

impl Display for CidString {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(&self.0)
  }
}

#[derive(Debug, Clone)]
pub enum Block {
  Commit(Commit),
  Node(Node),
  Record(Record),
}

impl Block {
  pub fn as_commit(&self) -> Option<&Commit> {
    match self {
      Self::Commit(c) => Some(c),
      _ => None,
    }
  }

  pub fn as_node(&self) -> Option<&Node> {
    match self {
      Self::Node(n) => Some(n),
      _ => None,
    }
  }

  pub fn as_record(&self) -> Option<&Record> {
    match self {
      Self::Record(r) => Some(r),
      _ => None,
    }
  }
}

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
        log::warn!("cannot decode CAR block");
        break;
      };
      data = data.get(len..).unwrap_or_default();
      let Ok((cid, len)) = get_cid(block.as_slice()) else {
        log::warn!("cannot decode CAR cid");
        continue;
      };
      let block = block.get(len..).unwrap_or_default();
      let Ok(data) = DagCborCodec.decode::<Ipld>(block) else {
        log::warn!("cannot decode CAR ipld");
        continue;
      };
      ret.insert(cid, data);
    }
    Self { header, data: ret }
  }
}

impl Blocks {
  pub fn iter(&self) -> std::collections::hash_map::Iter<Cid, Ipld> {
    self.data.iter()
  }

  pub fn get(&self, cid: &Cid) -> Option<Ipld> {
    self.data.get(cid).cloned()
  }

  pub fn get_blocks(&self) -> HashMap<Cid, Block> {
    self
      .data
      .iter()
      .filter_map(|(cid, i)| match Commit::try_from(i) {
        Ok(c) => Some((*cid, Block::Commit(c))),
        Err(_) => match Node::try_from(i) {
          Ok(n) => Some((*cid, Block::Node(n))),
          Err(_) => match Record::try_from(i) {
            Ok(r) => Some((*cid, Block::Record(r))),
            Err(_) => {
              log::warn!("unknown IPLD {}", ipld_to_string(i));
              None
            }
          },
        },
      })
      .collect()
  }
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AtProtoPds {
  #[serde(rename = "type")]
  pub pds_type: String,
  pub service_endpoint: String,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "id")]
pub enum AtprotoService {
  #[serde(rename = "#atproto_pds")]
  AtprotoPds(AtProtoPds),
  #[serde(other)]
  Other,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DidDoc {
  #[serde(rename = "@context")]
  pub context: Vec<String>,
  pub id: String,
  pub also_known_as: Vec<String>,
  pub verification_method: Vec<HashMap<String, String>>,
  pub service: Vec<AtprotoService>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Link {
  #[serde(rename = "$link")]
  pub link: String,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Blob {
  #[serde(rename = "$type")]
  pub blob_type: Option<String>,
  #[serde(rename = "ref")]
  pub blob_ref: Option<Link>,
  #[serde(rename = "mimeType")]
  pub mime_type: String,
  pub size: Option<i64>,
  pub cid: Option<String>,
}

impl Default for Blob {
  fn default() -> Self {
    Self {
      blob_type: Some(String::from("blob")),
      blob_ref: Some(Link::default()),
      mime_type: String::new(),
      size: Some(0),
      cid: None,
    }
  }
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyActorGetpreferences {
  pub preferences: AppBskyActorDefsPreferences,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyActorGetprofiles {
  pub profiles: Vec<AppBskyActorDefsProfileviewdetailed>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyActorGetsuggestions {
  pub actors: Vec<AppBskyActorDefsProfileview>,
  pub cursor: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyActorSearchactors {
  pub actors: Vec<AppBskyActorDefsProfileview>,
  pub cursor: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyActorSearchactorstypeahead {
  pub actors: Vec<AppBskyActorDefsProfileviewbasic>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyFeedDescribefeedgenerator {
  pub did: String,
  pub feeds: Vec<AppBskyFeedDescribefeedgeneratorFeed>,
  pub links: Option<AppBskyFeedDescribefeedgeneratorLinks>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyFeedGetactorfeeds {
  pub feeds: Vec<AppBskyFeedDefsGeneratorview>,
  pub cursor: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyFeedGetactorlikes {
  pub feed: Vec<AppBskyFeedDefsFeedviewpost>,
  pub cursor: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyFeedGetauthorfeed {
  pub feed: Vec<AppBskyFeedDefsFeedviewpost>,
  pub cursor: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyFeedGetfeed {
  pub feed: Vec<AppBskyFeedDefsFeedviewpost>,
  pub cursor: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyFeedGetfeedgenerator {
  pub view: AppBskyFeedDefsGeneratorview,
  #[serde(rename = "isOnline")]
  pub is_online: bool,
  #[serde(rename = "isValid")]
  pub is_valid: bool,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyFeedGetfeedgenerators {
  pub feeds: Vec<AppBskyFeedDefsGeneratorview>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyFeedGetfeedskeleton {
  pub feed: Vec<AppBskyFeedDefsSkeletonfeedpost>,
  pub cursor: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyFeedGetlikes {
  pub uri: String,
  pub likes: Vec<AppBskyFeedGetlikesLike>,
  pub cid: Option<CidString>,
  pub cursor: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyFeedGetlistfeed {
  pub feed: Vec<AppBskyFeedDefsFeedviewpost>,
  pub cursor: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyFeedGetpostthread {
  pub thread: AppBskyFeedGetpostthreadMainOutputThread,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyFeedGetposts {
  pub posts: Vec<AppBskyFeedDefsPostview>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyFeedGetrepostedby {
  pub uri: String,
  #[serde(rename = "repostedBy")]
  pub reposted_by: Vec<AppBskyActorDefsProfileview>,
  pub cid: Option<CidString>,
  pub cursor: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyFeedGetsuggestedfeeds {
  pub feeds: Vec<AppBskyFeedDefsGeneratorview>,
  pub cursor: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyFeedGettimeline {
  pub feed: Vec<AppBskyFeedDefsFeedviewpost>,
  pub cursor: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyFeedSearchposts {
  pub posts: Vec<AppBskyFeedDefsPostview>,
  pub cursor: Option<String>,
  #[serde(rename = "hitsTotal")]
  pub hits_total: Option<i64>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyGraphGetblocks {
  pub blocks: Vec<AppBskyActorDefsProfileview>,
  pub cursor: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyGraphGetfollowers {
  pub subject: AppBskyActorDefsProfileview,
  pub followers: Vec<AppBskyActorDefsProfileview>,
  pub cursor: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyGraphGetfollows {
  pub subject: AppBskyActorDefsProfileview,
  pub follows: Vec<AppBskyActorDefsProfileview>,
  pub cursor: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyGraphGetlist {
  pub list: AppBskyGraphDefsListview,
  pub items: Vec<AppBskyGraphDefsListitemview>,
  pub cursor: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyGraphGetlistblocks {
  pub lists: Vec<AppBskyGraphDefsListview>,
  pub cursor: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyGraphGetlistmutes {
  pub lists: Vec<AppBskyGraphDefsListview>,
  pub cursor: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyGraphGetlists {
  pub lists: Vec<AppBskyGraphDefsListview>,
  pub cursor: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyGraphGetmutes {
  pub mutes: Vec<AppBskyActorDefsProfileview>,
  pub cursor: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyGraphGetrelationships {
  pub relationships: Vec<AppBskyGraphGetrelationshipsMainOutputRelationshipsItem>,
  pub actor: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyGraphGetsuggestedfollowsbyactor {
  pub suggestions: Vec<AppBskyActorDefsProfileview>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyNotificationGetunreadcount {
  pub count: i64,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyNotificationListnotifications {
  pub notifications: Vec<AppBskyNotificationListnotificationsNotification>,
  pub cursor: Option<String>,
  #[serde(rename = "seenAt")]
  pub seen_at: Option<DateTime<Utc>>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyUnspeccedGetpopularfeedgenerators {
  pub feeds: Vec<AppBskyFeedDefsGeneratorview>,
  pub cursor: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyUnspeccedGettaggedsuggestions {
  pub suggestions: Vec<AppBskyUnspeccedGettaggedsuggestionsSuggestion>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyUnspeccedSearchactorsskeleton {
  pub actors: Vec<AppBskyUnspeccedDefsSkeletonsearchactor>,
  pub cursor: Option<String>,
  #[serde(rename = "hitsTotal")]
  pub hits_total: Option<i64>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyUnspeccedSearchpostsskeleton {
  pub posts: Vec<AppBskyUnspeccedDefsSkeletonsearchpost>,
  pub cursor: Option<String>,
  #[serde(rename = "hitsTotal")]
  pub hits_total: Option<i64>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoAdminGetaccountinfos {
  pub infos: Vec<ComAtprotoAdminDefsAccountview>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoAdminGetinvitecodes {
  pub codes: Vec<ComAtprotoServerDefsInvitecode>,
  pub cursor: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoAdminGetsubjectstatus {
  pub subject: ComAtprotoAdminGetsubjectstatusMainOutputSubject,
  pub takedown: Option<ComAtprotoAdminDefsStatusattr>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoAdminListcommunicationtemplates {
  #[serde(rename = "communicationTemplates")]
  pub communication_templates: Vec<ComAtprotoAdminDefsCommunicationtemplateview>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoAdminQuerymoderationevents {
  pub events: Vec<ComAtprotoAdminDefsModeventview>,
  pub cursor: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoAdminQuerymoderationstatuses {
  #[serde(rename = "subjectStatuses")]
  pub subject_statuses: Vec<ComAtprotoAdminDefsSubjectstatusview>,
  pub cursor: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoAdminSearchrepos {
  pub repos: Vec<ComAtprotoAdminDefsRepoview>,
  pub cursor: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoIdentityGetrecommendeddidcredentials {
  #[serde(rename = "rotationKeys")]
  pub rotation_keys: Option<Vec<String>>,
  #[serde(rename = "alsoKnownAs")]
  pub also_known_as: Option<Vec<String>>,
  #[serde(rename = "verificationMethods")]
  pub verification_methods: Option<Record>,
  pub services: Option<Record>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoIdentityResolvehandle {
  pub did: String,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoLabelQuerylabels {
  pub labels: Vec<ComAtprotoLabelDefsLabel>,
  pub cursor: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoRepoDescriberepo {
  pub handle: String,
  pub did: String,
  #[serde(rename = "didDoc")]
  pub did_doc: DidDoc,
  pub collections: Vec<String>,
  #[serde(rename = "handleIsCorrect")]
  pub handle_is_correct: bool,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoRepoGetrecord {
  pub uri: String,
  pub value: Record,
  pub cid: Option<CidString>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoRepoListmissingblobs {
  pub blobs: Vec<ComAtprotoRepoListmissingblobsRecordblob>,
  pub cursor: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoRepoListrecords {
  pub records: Vec<ComAtprotoRepoListrecordsRecord>,
  pub cursor: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoServerCheckaccountstatus {
  pub activated: bool,
  #[serde(rename = "validDid")]
  pub valid_did: bool,
  #[serde(rename = "repoCommit")]
  pub repo_commit: CidString,
  #[serde(rename = "repoRev")]
  pub repo_rev: String,
  #[serde(rename = "repoBlocks")]
  pub repo_blocks: i64,
  #[serde(rename = "indexedRecords")]
  pub indexed_records: i64,
  #[serde(rename = "privateStateValues")]
  pub private_state_values: i64,
  #[serde(rename = "expectedBlobs")]
  pub expected_blobs: i64,
  #[serde(rename = "importedBlobs")]
  pub imported_blobs: i64,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoServerDescribeserver {
  #[serde(rename = "availableUserDomains")]
  pub available_user_domains: Vec<String>,
  pub did: String,
  #[serde(rename = "inviteCodeRequired")]
  pub invite_code_required: Option<bool>,
  #[serde(rename = "phoneVerificationRequired")]
  pub phone_verification_required: Option<bool>,
  pub links: Option<ComAtprotoServerDescribeserverLinks>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoServerGetaccountinvitecodes {
  pub codes: Vec<ComAtprotoServerDefsInvitecode>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoServerGetserviceauth {
  pub token: String,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoServerGetsession {
  pub handle: String,
  pub did: String,
  pub email: Option<String>,
  #[serde(rename = "emailConfirmed")]
  pub email_confirmed: Option<bool>,
  #[serde(rename = "didDoc")]
  pub did_doc: Option<DidDoc>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoServerListapppasswords {
  pub passwords: Vec<ComAtprotoServerListapppasswordsApppassword>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoSyncGethead {
  pub root: CidString,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoSyncGetlatestcommit {
  pub cid: CidString,
  pub rev: String,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoSyncListblobs {
  pub cids: Vec<CidString>,
  pub cursor: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoSyncListrepos {
  pub repos: Vec<ComAtprotoSyncListreposRepo>,
  pub cursor: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoTempChecksignupqueue {
  pub activated: bool,
  #[serde(rename = "placeInQueue")]
  pub place_in_queue: Option<i64>,
  #[serde(rename = "estimatedTimeMs")]
  pub estimated_time_ms: Option<i64>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoTempFetchlabels {
  pub labels: Vec<ComAtprotoLabelDefsLabel>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoAdminSendemail {
  pub sent: bool,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoAdminUpdatesubjectstatus {
  pub subject: ComAtprotoAdminUpdatesubjectstatusMainOutputSubject,
  pub takedown: Option<ComAtprotoAdminDefsStatusattr>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoIdentitySignplcoperation {
  pub operation: Record,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoModerationCreatereport {
  pub id: i64,
  #[serde(rename = "reasonType")]
  pub reason_type: ComAtprotoModerationDefsReasontype,
  pub subject: ComAtprotoModerationCreatereportMainOutputSubject,
  #[serde(rename = "reportedBy")]
  pub reported_by: String,
  #[serde(rename = "createdAt")]
  pub created_at: DateTime<Utc>,
  pub reason: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoRepoCreaterecord {
  pub uri: String,
  pub cid: CidString,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoRepoPutrecord {
  pub uri: String,
  pub cid: CidString,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoRepoUploadblob {
  pub blob: Blob,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoServerCreateaccount {
  #[serde(rename = "accessJwt")]
  pub access_jwt: String,
  #[serde(rename = "refreshJwt")]
  pub refresh_jwt: String,
  pub handle: String,
  pub did: String,
  #[serde(rename = "didDoc")]
  pub did_doc: Option<DidDoc>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoServerCreateinvitecode {
  pub code: String,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoServerCreateinvitecodes {
  pub codes: Vec<ComAtprotoServerCreateinvitecodesAccountcodes>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoServerCreatesession {
  #[serde(rename = "accessJwt")]
  pub access_jwt: String,
  #[serde(rename = "refreshJwt")]
  pub refresh_jwt: String,
  pub handle: String,
  pub did: String,
  #[serde(rename = "didDoc")]
  pub did_doc: Option<DidDoc>,
  pub email: Option<String>,
  #[serde(rename = "emailConfirmed")]
  pub email_confirmed: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoServerRefreshsession {
  #[serde(rename = "accessJwt")]
  pub access_jwt: String,
  #[serde(rename = "refreshJwt")]
  pub refresh_jwt: String,
  pub handle: String,
  pub did: String,
  #[serde(rename = "didDoc")]
  pub did_doc: Option<DidDoc>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoServerRequestemailupdate {
  #[serde(rename = "tokenRequired")]
  pub token_required: bool,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoServerReservesigningkey {
  #[serde(rename = "signingKey")]
  pub signing_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "$type")]
pub enum AppBskyActorDefsPreferencesItem {
  #[serde(rename = "app.bsky.actor.defs#adultContentPref")]
  AppBskyActorDefsAdultcontentpref(Box<AppBskyActorDefsAdultcontentpref>),
  #[serde(rename = "app.bsky.actor.defs#contentLabelPref")]
  AppBskyActorDefsContentlabelpref(Box<AppBskyActorDefsContentlabelpref>),
  #[serde(rename = "app.bsky.actor.defs#savedFeedsPref")]
  AppBskyActorDefsSavedfeedspref(Box<AppBskyActorDefsSavedfeedspref>),
  #[serde(rename = "app.bsky.actor.defs#personalDetailsPref")]
  AppBskyActorDefsPersonaldetailspref(Box<AppBskyActorDefsPersonaldetailspref>),
  #[serde(rename = "app.bsky.actor.defs#feedViewPref")]
  AppBskyActorDefsFeedviewpref(Box<AppBskyActorDefsFeedviewpref>),
  #[serde(rename = "app.bsky.actor.defs#threadViewPref")]
  AppBskyActorDefsThreadviewpref(Box<AppBskyActorDefsThreadviewpref>),
  #[serde(rename = "app.bsky.actor.defs#interestsPref")]
  AppBskyActorDefsInterestspref(Box<AppBskyActorDefsInterestspref>),

  #[serde(other)]
  Other,
}

impl Default for AppBskyActorDefsPreferencesItem {
  fn default() -> Self {
    Self::AppBskyActorDefsAdultcontentpref(Box::new(AppBskyActorDefsAdultcontentpref::default()))
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "$type")]
pub enum AppBskyActorProfileMainLabels {
  #[serde(rename = "com.atproto.label.defs#selfLabels")]
  ComAtprotoLabelDefsSelflabels(Box<ComAtprotoLabelDefsSelflabels>),

  #[serde(other)]
  Other,
}

impl Default for AppBskyActorProfileMainLabels {
  fn default() -> Self {
    Self::ComAtprotoLabelDefsSelflabels(Box::new(ComAtprotoLabelDefsSelflabels::default()))
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "$type")]
pub enum AppBskyEmbedRecordViewRecord {
  #[serde(rename = "app.bsky.embed.record#viewRecord")]
  AppBskyEmbedRecordViewrecord(Box<AppBskyEmbedRecordViewrecord>),
  #[serde(rename = "app.bsky.embed.record#viewNotFound")]
  AppBskyEmbedRecordViewnotfound(Box<AppBskyEmbedRecordViewnotfound>),
  #[serde(rename = "app.bsky.embed.record#viewBlocked")]
  AppBskyEmbedRecordViewblocked(Box<AppBskyEmbedRecordViewblocked>),
  #[serde(rename = "app.bsky.feed.defs#generatorView")]
  AppBskyFeedDefsGeneratorview(Box<AppBskyFeedDefsGeneratorview>),
  #[serde(rename = "app.bsky.graph.defs#listView")]
  AppBskyGraphDefsListview(Box<AppBskyGraphDefsListview>),

  #[serde(other)]
  Other,
}

impl Default for AppBskyEmbedRecordViewRecord {
  fn default() -> Self {
    Self::AppBskyEmbedRecordViewrecord(Box::new(AppBskyEmbedRecordViewrecord::default()))
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "$type")]
pub enum AppBskyEmbedRecordViewrecordEmbedsItem {
  #[serde(rename = "app.bsky.embed.images#view")]
  AppBskyEmbedImagesView(Box<AppBskyEmbedImagesView>),
  #[serde(rename = "app.bsky.embed.external#view")]
  AppBskyEmbedExternalView(Box<AppBskyEmbedExternalView>),
  #[serde(rename = "app.bsky.embed.record#view")]
  AppBskyEmbedRecordView(Box<AppBskyEmbedRecordView>),
  #[serde(rename = "app.bsky.embed.recordWithMedia#view")]
  AppBskyEmbedRecordwithmediaView(Box<AppBskyEmbedRecordwithmediaView>),

  #[serde(other)]
  Other,
}

impl Default for AppBskyEmbedRecordViewrecordEmbedsItem {
  fn default() -> Self {
    Self::AppBskyEmbedImagesView(Box::new(AppBskyEmbedImagesView::default()))
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "$type")]
pub enum AppBskyEmbedRecordwithmediaMainMedia {
  #[serde(rename = "app.bsky.embed.images")]
  AppBskyEmbedImages(Box<AppBskyEmbedImages>),
  #[serde(rename = "app.bsky.embed.external")]
  AppBskyEmbedExternal(Box<AppBskyEmbedExternal>),

  #[serde(other)]
  Other,
}

impl Default for AppBskyEmbedRecordwithmediaMainMedia {
  fn default() -> Self {
    Self::AppBskyEmbedImages(Box::new(AppBskyEmbedImages::default()))
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "$type")]
pub enum AppBskyEmbedRecordwithmediaViewMedia {
  #[serde(rename = "app.bsky.embed.images#view")]
  AppBskyEmbedImagesView(Box<AppBskyEmbedImagesView>),
  #[serde(rename = "app.bsky.embed.external#view")]
  AppBskyEmbedExternalView(Box<AppBskyEmbedExternalView>),

  #[serde(other)]
  Other,
}

impl Default for AppBskyEmbedRecordwithmediaViewMedia {
  fn default() -> Self {
    Self::AppBskyEmbedImagesView(Box::new(AppBskyEmbedImagesView::default()))
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "$type")]
pub enum AppBskyFeedDefsPostviewEmbed {
  #[serde(rename = "app.bsky.embed.images#view")]
  AppBskyEmbedImagesView(Box<AppBskyEmbedImagesView>),
  #[serde(rename = "app.bsky.embed.external#view")]
  AppBskyEmbedExternalView(Box<AppBskyEmbedExternalView>),
  #[serde(rename = "app.bsky.embed.record#view")]
  AppBskyEmbedRecordView(Box<AppBskyEmbedRecordView>),
  #[serde(rename = "app.bsky.embed.recordWithMedia#view")]
  AppBskyEmbedRecordwithmediaView(Box<AppBskyEmbedRecordwithmediaView>),

  #[serde(other)]
  Other,
}

impl Default for AppBskyFeedDefsPostviewEmbed {
  fn default() -> Self {
    Self::AppBskyEmbedImagesView(Box::new(AppBskyEmbedImagesView::default()))
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "$type")]
pub enum AppBskyFeedDefsFeedviewpostReason {
  #[serde(rename = "app.bsky.feed.defs#reasonRepost")]
  AppBskyFeedDefsReasonrepost(Box<AppBskyFeedDefsReasonrepost>),

  #[serde(other)]
  Other,
}

impl Default for AppBskyFeedDefsFeedviewpostReason {
  fn default() -> Self {
    Self::AppBskyFeedDefsReasonrepost(Box::new(AppBskyFeedDefsReasonrepost::default()))
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "$type")]
pub enum AppBskyFeedDefsReplyrefRoot {
  #[serde(rename = "app.bsky.feed.defs#postView")]
  AppBskyFeedDefsPostview(Box<AppBskyFeedDefsPostview>),
  #[serde(rename = "app.bsky.feed.defs#notFoundPost")]
  AppBskyFeedDefsNotfoundpost(Box<AppBskyFeedDefsNotfoundpost>),
  #[serde(rename = "app.bsky.feed.defs#blockedPost")]
  AppBskyFeedDefsBlockedpost(Box<AppBskyFeedDefsBlockedpost>),

  #[serde(other)]
  Other,
}

impl Default for AppBskyFeedDefsReplyrefRoot {
  fn default() -> Self {
    Self::AppBskyFeedDefsPostview(Box::new(AppBskyFeedDefsPostview::default()))
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "$type")]
pub enum AppBskyFeedDefsReplyrefParent {
  #[serde(rename = "app.bsky.feed.defs#postView")]
  AppBskyFeedDefsPostview(Box<AppBskyFeedDefsPostview>),
  #[serde(rename = "app.bsky.feed.defs#notFoundPost")]
  AppBskyFeedDefsNotfoundpost(Box<AppBskyFeedDefsNotfoundpost>),
  #[serde(rename = "app.bsky.feed.defs#blockedPost")]
  AppBskyFeedDefsBlockedpost(Box<AppBskyFeedDefsBlockedpost>),

  #[serde(other)]
  Other,
}

impl Default for AppBskyFeedDefsReplyrefParent {
  fn default() -> Self {
    Self::AppBskyFeedDefsPostview(Box::new(AppBskyFeedDefsPostview::default()))
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "$type")]
pub enum AppBskyFeedDefsThreadviewpostParent {
  #[serde(rename = "app.bsky.feed.defs#threadViewPost")]
  AppBskyFeedDefsThreadviewpost(Box<AppBskyFeedDefsThreadviewpost>),
  #[serde(rename = "app.bsky.feed.defs#notFoundPost")]
  AppBskyFeedDefsNotfoundpost(Box<AppBskyFeedDefsNotfoundpost>),
  #[serde(rename = "app.bsky.feed.defs#blockedPost")]
  AppBskyFeedDefsBlockedpost(Box<AppBskyFeedDefsBlockedpost>),

  #[serde(other)]
  Other,
}

impl Default for AppBskyFeedDefsThreadviewpostParent {
  fn default() -> Self {
    Self::AppBskyFeedDefsThreadviewpost(Box::new(AppBskyFeedDefsThreadviewpost::default()))
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "$type")]
pub enum AppBskyFeedDefsThreadviewpostRepliesItem {
  #[serde(rename = "app.bsky.feed.defs#threadViewPost")]
  AppBskyFeedDefsThreadviewpost(Box<AppBskyFeedDefsThreadviewpost>),
  #[serde(rename = "app.bsky.feed.defs#notFoundPost")]
  AppBskyFeedDefsNotfoundpost(Box<AppBskyFeedDefsNotfoundpost>),
  #[serde(rename = "app.bsky.feed.defs#blockedPost")]
  AppBskyFeedDefsBlockedpost(Box<AppBskyFeedDefsBlockedpost>),

  #[serde(other)]
  Other,
}

impl Default for AppBskyFeedDefsThreadviewpostRepliesItem {
  fn default() -> Self {
    Self::AppBskyFeedDefsThreadviewpost(Box::new(AppBskyFeedDefsThreadviewpost::default()))
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "$type")]
pub enum AppBskyFeedDefsSkeletonfeedpostReason {
  #[serde(rename = "app.bsky.feed.defs#skeletonReasonRepost")]
  AppBskyFeedDefsSkeletonreasonrepost(Box<AppBskyFeedDefsSkeletonreasonrepost>),

  #[serde(other)]
  Other,
}

impl Default for AppBskyFeedDefsSkeletonfeedpostReason {
  fn default() -> Self {
    Self::AppBskyFeedDefsSkeletonreasonrepost(Box::new(
      AppBskyFeedDefsSkeletonreasonrepost::default(),
    ))
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "$type")]
pub enum AppBskyFeedGeneratorMainLabels {
  #[serde(rename = "com.atproto.label.defs#selfLabels")]
  ComAtprotoLabelDefsSelflabels(Box<ComAtprotoLabelDefsSelflabels>),

  #[serde(other)]
  Other,
}

impl Default for AppBskyFeedGeneratorMainLabels {
  fn default() -> Self {
    Self::ComAtprotoLabelDefsSelflabels(Box::new(ComAtprotoLabelDefsSelflabels::default()))
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "$type")]
pub enum AppBskyFeedGetpostthreadMainOutputThread {
  #[serde(rename = "app.bsky.feed.defs#threadViewPost")]
  AppBskyFeedDefsThreadviewpost(Box<AppBskyFeedDefsThreadviewpost>),
  #[serde(rename = "app.bsky.feed.defs#notFoundPost")]
  AppBskyFeedDefsNotfoundpost(Box<AppBskyFeedDefsNotfoundpost>),
  #[serde(rename = "app.bsky.feed.defs#blockedPost")]
  AppBskyFeedDefsBlockedpost(Box<AppBskyFeedDefsBlockedpost>),

  #[serde(other)]
  Other,
}

impl Default for AppBskyFeedGetpostthreadMainOutputThread {
  fn default() -> Self {
    Self::AppBskyFeedDefsThreadviewpost(Box::new(AppBskyFeedDefsThreadviewpost::default()))
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "$type")]
pub enum AppBskyFeedPostMainEmbed {
  #[serde(rename = "app.bsky.embed.images")]
  AppBskyEmbedImages(Box<AppBskyEmbedImages>),
  #[serde(rename = "app.bsky.embed.external")]
  AppBskyEmbedExternal(Box<AppBskyEmbedExternal>),
  #[serde(rename = "app.bsky.embed.record")]
  AppBskyEmbedRecord(Box<AppBskyEmbedRecord>),
  #[serde(rename = "app.bsky.embed.recordWithMedia")]
  AppBskyEmbedRecordwithmedia(Box<AppBskyEmbedRecordwithmedia>),

  #[serde(other)]
  Other,
}

impl Default for AppBskyFeedPostMainEmbed {
  fn default() -> Self {
    Self::AppBskyEmbedImages(Box::new(AppBskyEmbedImages::default()))
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "$type")]
pub enum AppBskyFeedPostMainLabels {
  #[serde(rename = "com.atproto.label.defs#selfLabels")]
  ComAtprotoLabelDefsSelflabels(Box<ComAtprotoLabelDefsSelflabels>),

  #[serde(other)]
  Other,
}

impl Default for AppBskyFeedPostMainLabels {
  fn default() -> Self {
    Self::ComAtprotoLabelDefsSelflabels(Box::new(ComAtprotoLabelDefsSelflabels::default()))
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "$type")]
pub enum AppBskyFeedThreadgateMainAllowItem {
  #[serde(rename = "app.bsky.feed.threadgate#mentionRule")]
  AppBskyFeedThreadgateMentionrule(Box<AppBskyFeedThreadgateMentionrule>),
  #[serde(rename = "app.bsky.feed.threadgate#followingRule")]
  AppBskyFeedThreadgateFollowingrule(Box<AppBskyFeedThreadgateFollowingrule>),
  #[serde(rename = "app.bsky.feed.threadgate#listRule")]
  AppBskyFeedThreadgateListrule(Box<AppBskyFeedThreadgateListrule>),

  #[serde(other)]
  Other,
}

impl Default for AppBskyFeedThreadgateMainAllowItem {
  fn default() -> Self {
    Self::AppBskyFeedThreadgateMentionrule(Box::new(AppBskyFeedThreadgateMentionrule::default()))
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "$type")]
pub enum AppBskyGraphGetrelationshipsMainOutputRelationshipsItem {
  #[serde(rename = "app.bsky.graph.defs#relationship")]
  AppBskyGraphDefsRelationship(Box<AppBskyGraphDefsRelationship>),
  #[serde(rename = "app.bsky.graph.defs#notFoundActor")]
  AppBskyGraphDefsNotfoundactor(Box<AppBskyGraphDefsNotfoundactor>),

  #[serde(other)]
  Other,
}

impl Default for AppBskyGraphGetrelationshipsMainOutputRelationshipsItem {
  fn default() -> Self {
    Self::AppBskyGraphDefsRelationship(Box::new(AppBskyGraphDefsRelationship::default()))
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "$type")]
pub enum AppBskyGraphListMainLabels {
  #[serde(rename = "com.atproto.label.defs#selfLabels")]
  ComAtprotoLabelDefsSelflabels(Box<ComAtprotoLabelDefsSelflabels>),

  #[serde(other)]
  Other,
}

impl Default for AppBskyGraphListMainLabels {
  fn default() -> Self {
    Self::ComAtprotoLabelDefsSelflabels(Box::new(ComAtprotoLabelDefsSelflabels::default()))
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "$type")]
pub enum AppBskyRichtextFacetMainFeaturesItem {
  #[serde(rename = "app.bsky.richtext.facet#mention")]
  AppBskyRichtextFacetMention(Box<AppBskyRichtextFacetMention>),
  #[serde(rename = "app.bsky.richtext.facet#link")]
  AppBskyRichtextFacetLink(Box<AppBskyRichtextFacetLink>),
  #[serde(rename = "app.bsky.richtext.facet#tag")]
  AppBskyRichtextFacetTag(Box<AppBskyRichtextFacetTag>),

  #[serde(other)]
  Other,
}

impl Default for AppBskyRichtextFacetMainFeaturesItem {
  fn default() -> Self {
    Self::AppBskyRichtextFacetMention(Box::new(AppBskyRichtextFacetMention::default()))
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "$type")]
pub enum ComAtprotoAdminDefsModeventviewEvent {
  #[serde(rename = "com.atproto.admin.defs#modEventTakedown")]
  ComAtprotoAdminDefsModeventtakedown(Box<ComAtprotoAdminDefsModeventtakedown>),
  #[serde(rename = "com.atproto.admin.defs#modEventReverseTakedown")]
  ComAtprotoAdminDefsModeventreversetakedown(Box<ComAtprotoAdminDefsModeventreversetakedown>),
  #[serde(rename = "com.atproto.admin.defs#modEventComment")]
  ComAtprotoAdminDefsModeventcomment(Box<ComAtprotoAdminDefsModeventcomment>),
  #[serde(rename = "com.atproto.admin.defs#modEventReport")]
  ComAtprotoAdminDefsModeventreport(Box<ComAtprotoAdminDefsModeventreport>),
  #[serde(rename = "com.atproto.admin.defs#modEventLabel")]
  ComAtprotoAdminDefsModeventlabel(Box<ComAtprotoAdminDefsModeventlabel>),
  #[serde(rename = "com.atproto.admin.defs#modEventAcknowledge")]
  ComAtprotoAdminDefsModeventacknowledge(Box<ComAtprotoAdminDefsModeventacknowledge>),
  #[serde(rename = "com.atproto.admin.defs#modEventEscalate")]
  ComAtprotoAdminDefsModeventescalate(Box<ComAtprotoAdminDefsModeventescalate>),
  #[serde(rename = "com.atproto.admin.defs#modEventMute")]
  ComAtprotoAdminDefsModeventmute(Box<ComAtprotoAdminDefsModeventmute>),
  #[serde(rename = "com.atproto.admin.defs#modEventEmail")]
  ComAtprotoAdminDefsModeventemail(Box<ComAtprotoAdminDefsModeventemail>),
  #[serde(rename = "com.atproto.admin.defs#modEventResolveAppeal")]
  ComAtprotoAdminDefsModeventresolveappeal(Box<ComAtprotoAdminDefsModeventresolveappeal>),

  #[serde(other)]
  Other,
}

impl Default for ComAtprotoAdminDefsModeventviewEvent {
  fn default() -> Self {
    Self::ComAtprotoAdminDefsModeventtakedown(Box::new(
      ComAtprotoAdminDefsModeventtakedown::default(),
    ))
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "$type")]
pub enum ComAtprotoAdminDefsModeventviewSubject {
  #[serde(rename = "com.atproto.admin.defs#repoRef")]
  ComAtprotoAdminDefsReporef(Box<ComAtprotoAdminDefsReporef>),
  #[serde(rename = "com.atproto.repo.strongRef")]
  ComAtprotoRepoStrongref(Box<ComAtprotoRepoStrongref>),

  #[serde(other)]
  Other,
}

impl Default for ComAtprotoAdminDefsModeventviewSubject {
  fn default() -> Self {
    Self::ComAtprotoAdminDefsReporef(Box::new(ComAtprotoAdminDefsReporef::default()))
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "$type")]
pub enum ComAtprotoAdminDefsModeventviewdetailEvent {
  #[serde(rename = "com.atproto.admin.defs#modEventTakedown")]
  ComAtprotoAdminDefsModeventtakedown(Box<ComAtprotoAdminDefsModeventtakedown>),
  #[serde(rename = "com.atproto.admin.defs#modEventReverseTakedown")]
  ComAtprotoAdminDefsModeventreversetakedown(Box<ComAtprotoAdminDefsModeventreversetakedown>),
  #[serde(rename = "com.atproto.admin.defs#modEventComment")]
  ComAtprotoAdminDefsModeventcomment(Box<ComAtprotoAdminDefsModeventcomment>),
  #[serde(rename = "com.atproto.admin.defs#modEventReport")]
  ComAtprotoAdminDefsModeventreport(Box<ComAtprotoAdminDefsModeventreport>),
  #[serde(rename = "com.atproto.admin.defs#modEventLabel")]
  ComAtprotoAdminDefsModeventlabel(Box<ComAtprotoAdminDefsModeventlabel>),
  #[serde(rename = "com.atproto.admin.defs#modEventAcknowledge")]
  ComAtprotoAdminDefsModeventacknowledge(Box<ComAtprotoAdminDefsModeventacknowledge>),
  #[serde(rename = "com.atproto.admin.defs#modEventEscalate")]
  ComAtprotoAdminDefsModeventescalate(Box<ComAtprotoAdminDefsModeventescalate>),
  #[serde(rename = "com.atproto.admin.defs#modEventMute")]
  ComAtprotoAdminDefsModeventmute(Box<ComAtprotoAdminDefsModeventmute>),
  #[serde(rename = "com.atproto.admin.defs#modEventEmail")]
  ComAtprotoAdminDefsModeventemail(Box<ComAtprotoAdminDefsModeventemail>),
  #[serde(rename = "com.atproto.admin.defs#modEventResolveAppeal")]
  ComAtprotoAdminDefsModeventresolveappeal(Box<ComAtprotoAdminDefsModeventresolveappeal>),

  #[serde(other)]
  Other,
}

impl Default for ComAtprotoAdminDefsModeventviewdetailEvent {
  fn default() -> Self {
    Self::ComAtprotoAdminDefsModeventtakedown(Box::new(
      ComAtprotoAdminDefsModeventtakedown::default(),
    ))
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "$type")]
pub enum ComAtprotoAdminDefsModeventviewdetailSubject {
  #[serde(rename = "com.atproto.admin.defs#repoView")]
  ComAtprotoAdminDefsRepoview(Box<ComAtprotoAdminDefsRepoview>),
  #[serde(rename = "com.atproto.admin.defs#repoViewNotFound")]
  ComAtprotoAdminDefsRepoviewnotfound(Box<ComAtprotoAdminDefsRepoviewnotfound>),
  #[serde(rename = "com.atproto.admin.defs#recordView")]
  ComAtprotoAdminDefsRecordview(Box<ComAtprotoAdminDefsRecordview>),
  #[serde(rename = "com.atproto.admin.defs#recordViewNotFound")]
  ComAtprotoAdminDefsRecordviewnotfound(Box<ComAtprotoAdminDefsRecordviewnotfound>),

  #[serde(other)]
  Other,
}

impl Default for ComAtprotoAdminDefsModeventviewdetailSubject {
  fn default() -> Self {
    Self::ComAtprotoAdminDefsRepoview(Box::new(ComAtprotoAdminDefsRepoview::default()))
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "$type")]
pub enum ComAtprotoAdminDefsReportviewSubject {
  #[serde(rename = "com.atproto.admin.defs#repoRef")]
  ComAtprotoAdminDefsReporef(Box<ComAtprotoAdminDefsReporef>),
  #[serde(rename = "com.atproto.repo.strongRef")]
  ComAtprotoRepoStrongref(Box<ComAtprotoRepoStrongref>),

  #[serde(other)]
  Other,
}

impl Default for ComAtprotoAdminDefsReportviewSubject {
  fn default() -> Self {
    Self::ComAtprotoAdminDefsReporef(Box::new(ComAtprotoAdminDefsReporef::default()))
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "$type")]
pub enum ComAtprotoAdminDefsSubjectstatusviewSubject {
  #[serde(rename = "com.atproto.admin.defs#repoRef")]
  ComAtprotoAdminDefsReporef(Box<ComAtprotoAdminDefsReporef>),
  #[serde(rename = "com.atproto.repo.strongRef")]
  ComAtprotoRepoStrongref(Box<ComAtprotoRepoStrongref>),

  #[serde(other)]
  Other,
}

impl Default for ComAtprotoAdminDefsSubjectstatusviewSubject {
  fn default() -> Self {
    Self::ComAtprotoAdminDefsReporef(Box::new(ComAtprotoAdminDefsReporef::default()))
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "$type")]
pub enum ComAtprotoAdminDefsReportviewdetailSubject {
  #[serde(rename = "com.atproto.admin.defs#repoView")]
  ComAtprotoAdminDefsRepoview(Box<ComAtprotoAdminDefsRepoview>),
  #[serde(rename = "com.atproto.admin.defs#repoViewNotFound")]
  ComAtprotoAdminDefsRepoviewnotfound(Box<ComAtprotoAdminDefsRepoviewnotfound>),
  #[serde(rename = "com.atproto.admin.defs#recordView")]
  ComAtprotoAdminDefsRecordview(Box<ComAtprotoAdminDefsRecordview>),
  #[serde(rename = "com.atproto.admin.defs#recordViewNotFound")]
  ComAtprotoAdminDefsRecordviewnotfound(Box<ComAtprotoAdminDefsRecordviewnotfound>),

  #[serde(other)]
  Other,
}

impl Default for ComAtprotoAdminDefsReportviewdetailSubject {
  fn default() -> Self {
    Self::ComAtprotoAdminDefsRepoview(Box::new(ComAtprotoAdminDefsRepoview::default()))
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "$type")]
pub enum ComAtprotoAdminDefsBlobviewDetails {
  #[serde(rename = "com.atproto.admin.defs#imageDetails")]
  ComAtprotoAdminDefsImagedetails(Box<ComAtprotoAdminDefsImagedetails>),
  #[serde(rename = "com.atproto.admin.defs#videoDetails")]
  ComAtprotoAdminDefsVideodetails(Box<ComAtprotoAdminDefsVideodetails>),

  #[serde(other)]
  Other,
}

impl Default for ComAtprotoAdminDefsBlobviewDetails {
  fn default() -> Self {
    Self::ComAtprotoAdminDefsImagedetails(Box::new(ComAtprotoAdminDefsImagedetails::default()))
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "$type")]
pub enum ComAtprotoAdminEmitmoderationeventMainInputEvent {
  #[serde(rename = "com.atproto.admin.defs#modEventTakedown")]
  ComAtprotoAdminDefsModeventtakedown(Box<ComAtprotoAdminDefsModeventtakedown>),
  #[serde(rename = "com.atproto.admin.defs#modEventAcknowledge")]
  ComAtprotoAdminDefsModeventacknowledge(Box<ComAtprotoAdminDefsModeventacknowledge>),
  #[serde(rename = "com.atproto.admin.defs#modEventEscalate")]
  ComAtprotoAdminDefsModeventescalate(Box<ComAtprotoAdminDefsModeventescalate>),
  #[serde(rename = "com.atproto.admin.defs#modEventComment")]
  ComAtprotoAdminDefsModeventcomment(Box<ComAtprotoAdminDefsModeventcomment>),
  #[serde(rename = "com.atproto.admin.defs#modEventLabel")]
  ComAtprotoAdminDefsModeventlabel(Box<ComAtprotoAdminDefsModeventlabel>),
  #[serde(rename = "com.atproto.admin.defs#modEventReport")]
  ComAtprotoAdminDefsModeventreport(Box<ComAtprotoAdminDefsModeventreport>),
  #[serde(rename = "com.atproto.admin.defs#modEventMute")]
  ComAtprotoAdminDefsModeventmute(Box<ComAtprotoAdminDefsModeventmute>),
  #[serde(rename = "com.atproto.admin.defs#modEventReverseTakedown")]
  ComAtprotoAdminDefsModeventreversetakedown(Box<ComAtprotoAdminDefsModeventreversetakedown>),
  #[serde(rename = "com.atproto.admin.defs#modEventUnmute")]
  ComAtprotoAdminDefsModeventunmute(Box<ComAtprotoAdminDefsModeventunmute>),
  #[serde(rename = "com.atproto.admin.defs#modEventEmail")]
  ComAtprotoAdminDefsModeventemail(Box<ComAtprotoAdminDefsModeventemail>),
  #[serde(rename = "com.atproto.admin.defs#modEventTag")]
  ComAtprotoAdminDefsModeventtag(Box<ComAtprotoAdminDefsModeventtag>),

  #[serde(other)]
  Other,
}

impl Default for ComAtprotoAdminEmitmoderationeventMainInputEvent {
  fn default() -> Self {
    Self::ComAtprotoAdminDefsModeventtakedown(Box::new(
      ComAtprotoAdminDefsModeventtakedown::default(),
    ))
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "$type")]
pub enum ComAtprotoAdminEmitmoderationeventMainInputSubject {
  #[serde(rename = "com.atproto.admin.defs#repoRef")]
  ComAtprotoAdminDefsReporef(Box<ComAtprotoAdminDefsReporef>),
  #[serde(rename = "com.atproto.repo.strongRef")]
  ComAtprotoRepoStrongref(Box<ComAtprotoRepoStrongref>),

  #[serde(other)]
  Other,
}

impl Default for ComAtprotoAdminEmitmoderationeventMainInputSubject {
  fn default() -> Self {
    Self::ComAtprotoAdminDefsReporef(Box::new(ComAtprotoAdminDefsReporef::default()))
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "$type")]
pub enum ComAtprotoAdminGetsubjectstatusMainOutputSubject {
  #[serde(rename = "com.atproto.admin.defs#repoRef")]
  ComAtprotoAdminDefsReporef(Box<ComAtprotoAdminDefsReporef>),
  #[serde(rename = "com.atproto.repo.strongRef")]
  ComAtprotoRepoStrongref(Box<ComAtprotoRepoStrongref>),
  #[serde(rename = "com.atproto.admin.defs#repoBlobRef")]
  ComAtprotoAdminDefsRepoblobref(Box<ComAtprotoAdminDefsRepoblobref>),

  #[serde(other)]
  Other,
}

impl Default for ComAtprotoAdminGetsubjectstatusMainOutputSubject {
  fn default() -> Self {
    Self::ComAtprotoAdminDefsReporef(Box::new(ComAtprotoAdminDefsReporef::default()))
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "$type")]
pub enum ComAtprotoAdminUpdatesubjectstatusMainInputSubject {
  #[serde(rename = "com.atproto.admin.defs#repoRef")]
  ComAtprotoAdminDefsReporef(Box<ComAtprotoAdminDefsReporef>),
  #[serde(rename = "com.atproto.repo.strongRef")]
  ComAtprotoRepoStrongref(Box<ComAtprotoRepoStrongref>),
  #[serde(rename = "com.atproto.admin.defs#repoBlobRef")]
  ComAtprotoAdminDefsRepoblobref(Box<ComAtprotoAdminDefsRepoblobref>),

  #[serde(other)]
  Other,
}

impl Default for ComAtprotoAdminUpdatesubjectstatusMainInputSubject {
  fn default() -> Self {
    Self::ComAtprotoAdminDefsReporef(Box::new(ComAtprotoAdminDefsReporef::default()))
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "$type")]
pub enum ComAtprotoAdminUpdatesubjectstatusMainOutputSubject {
  #[serde(rename = "com.atproto.admin.defs#repoRef")]
  ComAtprotoAdminDefsReporef(Box<ComAtprotoAdminDefsReporef>),
  #[serde(rename = "com.atproto.repo.strongRef")]
  ComAtprotoRepoStrongref(Box<ComAtprotoRepoStrongref>),
  #[serde(rename = "com.atproto.admin.defs#repoBlobRef")]
  ComAtprotoAdminDefsRepoblobref(Box<ComAtprotoAdminDefsRepoblobref>),

  #[serde(other)]
  Other,
}

impl Default for ComAtprotoAdminUpdatesubjectstatusMainOutputSubject {
  fn default() -> Self {
    Self::ComAtprotoAdminDefsReporef(Box::new(ComAtprotoAdminDefsReporef::default()))
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "$type")]
pub enum ComAtprotoLabelSubscribelabelsMainMessage {
  #[serde(rename = "com.atproto.label.subscribeLabels#labels")]
  ComAtprotoLabelSubscribelabelsLabels(Box<ComAtprotoLabelSubscribelabelsLabels>),
  #[serde(rename = "com.atproto.label.subscribeLabels#info")]
  ComAtprotoLabelSubscribelabelsInfo(Box<ComAtprotoLabelSubscribelabelsInfo>),

  #[serde(other)]
  Other,
}

impl Default for ComAtprotoLabelSubscribelabelsMainMessage {
  fn default() -> Self {
    Self::ComAtprotoLabelSubscribelabelsLabels(Box::new(
      ComAtprotoLabelSubscribelabelsLabels::default(),
    ))
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "$type")]
pub enum ComAtprotoModerationCreatereportMainInputSubject {
  #[serde(rename = "com.atproto.admin.defs#repoRef")]
  ComAtprotoAdminDefsReporef(Box<ComAtprotoAdminDefsReporef>),
  #[serde(rename = "com.atproto.repo.strongRef")]
  ComAtprotoRepoStrongref(Box<ComAtprotoRepoStrongref>),

  #[serde(other)]
  Other,
}

impl Default for ComAtprotoModerationCreatereportMainInputSubject {
  fn default() -> Self {
    Self::ComAtprotoAdminDefsReporef(Box::new(ComAtprotoAdminDefsReporef::default()))
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "$type")]
pub enum ComAtprotoModerationCreatereportMainOutputSubject {
  #[serde(rename = "com.atproto.admin.defs#repoRef")]
  ComAtprotoAdminDefsReporef(Box<ComAtprotoAdminDefsReporef>),
  #[serde(rename = "com.atproto.repo.strongRef")]
  ComAtprotoRepoStrongref(Box<ComAtprotoRepoStrongref>),

  #[serde(other)]
  Other,
}

impl Default for ComAtprotoModerationCreatereportMainOutputSubject {
  fn default() -> Self {
    Self::ComAtprotoAdminDefsReporef(Box::new(ComAtprotoAdminDefsReporef::default()))
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "$type")]
pub enum ComAtprotoRepoApplywritesMainInputWritesItem {
  #[serde(rename = "com.atproto.repo.applyWrites#create")]
  ComAtprotoRepoApplywritesCreate(Box<ComAtprotoRepoApplywritesCreate>),
  #[serde(rename = "com.atproto.repo.applyWrites#update")]
  ComAtprotoRepoApplywritesUpdate(Box<ComAtprotoRepoApplywritesUpdate>),
  #[serde(rename = "com.atproto.repo.applyWrites#delete")]
  ComAtprotoRepoApplywritesDelete(Box<ComAtprotoRepoApplywritesDelete>),

  #[serde(other)]
  Other,
}

impl Default for ComAtprotoRepoApplywritesMainInputWritesItem {
  fn default() -> Self {
    Self::ComAtprotoRepoApplywritesCreate(Box::new(ComAtprotoRepoApplywritesCreate::default()))
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "$type")]
pub enum ComAtprotoSyncSubscribereposMainMessage {
  #[serde(rename = "com.atproto.sync.subscribeRepos#commit")]
  ComAtprotoSyncSubscribereposCommit(Box<ComAtprotoSyncSubscribereposCommit>),
  #[serde(rename = "com.atproto.sync.subscribeRepos#identity")]
  ComAtprotoSyncSubscribereposIdentity(Box<ComAtprotoSyncSubscribereposIdentity>),
  #[serde(rename = "com.atproto.sync.subscribeRepos#handle")]
  ComAtprotoSyncSubscribereposHandle(Box<ComAtprotoSyncSubscribereposHandle>),
  #[serde(rename = "com.atproto.sync.subscribeRepos#migrate")]
  ComAtprotoSyncSubscribereposMigrate(Box<ComAtprotoSyncSubscribereposMigrate>),
  #[serde(rename = "com.atproto.sync.subscribeRepos#tombstone")]
  ComAtprotoSyncSubscribereposTombstone(Box<ComAtprotoSyncSubscribereposTombstone>),
  #[serde(rename = "com.atproto.sync.subscribeRepos#info")]
  ComAtprotoSyncSubscribereposInfo(Box<ComAtprotoSyncSubscribereposInfo>),

  #[serde(other)]
  Other,
}

impl Default for ComAtprotoSyncSubscribereposMainMessage {
  fn default() -> Self {
    Self::ComAtprotoSyncSubscribereposCommit(
      Box::new(ComAtprotoSyncSubscribereposCommit::default()),
    )
  }
}

pub struct Client {
  host: String,
  bgs_host: String,
  proxy: Option<String>,
  jwt: Option<String>,
  agent: Agent,
}

impl Client {
  pub fn new<T1: ToString, T2: ToString, T3: ToString>(
    host: T1,
    bgs_host: T2,
    proxy: Option<T3>,
  ) -> Self {
    Self {
      host: host.to_string(),
      bgs_host: bgs_host.to_string(),
      proxy: proxy.as_ref().map(|p| p.to_string()),
      jwt: None,
      agent: match proxy {
        Some(p) => match Proxy::new(p.to_string()) {
          Ok(pr) => AgentBuilder::new().proxy(pr).build(),
          _ => Agent::new(),
        },
        _ => Agent::new(),
      },
    }
  }

  pub fn set_jwt(&mut self, jwt: Option<String>) {
    self.jwt = jwt;
  }

  pub fn get_jwt(&self) -> Option<String> {
    self.jwt.clone()
  }

  pub fn get_host(&self) -> String {
    self.host.clone()
  }

  pub fn get_bgs_host(&self) -> String {
    self.bgs_host.clone()
  }

  pub fn get_proxy(&self) -> Option<String> {
    self.proxy.clone()
  }

  /// Get private preferences attached to the current account. Expected use is synchronization between multiple devices, and import/export during account migration. Requires auth.

  pub fn app_bsky_actor_getpreferences(&self) -> Result<AppBskyActorGetpreferences> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/app.bsky.actor.getPreferences",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    Ok(req.call()?.into_json()?)
  }

  /// Get detailed profile view of an actor. Does not require auth, but contains relevant metadata with auth.

  pub fn app_bsky_actor_getprofile(
    &self,
    actor: &str,
  ) -> Result<AppBskyActorDefsProfileviewdetailed> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/app.bsky.actor.getProfile",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut _q = Vec::new();

    _q.push(("actor", actor));

    Ok(req.query_pairs(_q).call()?.into_json()?)
  }

  /// Get detailed profile views of multiple actors.

  pub fn app_bsky_actor_getprofiles(&self, actors: &[&str]) -> Result<AppBskyActorGetprofiles> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/app.bsky.actor.getProfiles",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut _q = Vec::new();

    let mut actors_value = actors.iter().map(|i| ("actors", *i)).collect::<Vec<_>>();

    _q.append(&mut actors_value);

    Ok(req.query_pairs(_q).call()?.into_json()?)
  }

  /// Get a list of suggested actors. Expected use is discovery of accounts to follow during new account onboarding.

  pub fn app_bsky_actor_getsuggestions(
    &self,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<AppBskyActorGetsuggestions> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/app.bsky.actor.getSuggestions",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut _q = Vec::new();

    let limit_value = serde_json::to_string(&limit)?;

    if limit.is_some() {
      _q.push(("limit", limit_value.as_str()));
    };

    if cursor.is_some() {
      _q.push(("cursor", cursor.unwrap_or_default()));
    }

    Ok(req.query_pairs(_q).call()?.into_json()?)
  }

  /// Find actors (profiles) matching search criteria. Does not require auth.

  pub fn app_bsky_actor_searchactors(
    &self,
    term: Option<&str>,
    q: Option<&str>,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<AppBskyActorSearchactors> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/app.bsky.actor.searchActors",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut _q = Vec::new();

    if term.is_some() {
      _q.push(("term", term.unwrap_or_default()));
    };

    if q.is_some() {
      _q.push(("q", q.unwrap_or_default()));
    }

    let limit_value = serde_json::to_string(&limit)?;

    if limit.is_some() {
      _q.push(("limit", limit_value.as_str()));
    };

    if cursor.is_some() {
      _q.push(("cursor", cursor.unwrap_or_default()));
    }

    Ok(req.query_pairs(_q).call()?.into_json()?)
  }

  /// Find actor suggestions for a prefix search term. Expected use is for auto-completion during text field entry. Does not require auth.

  pub fn app_bsky_actor_searchactorstypeahead(
    &self,
    term: Option<&str>,
    q: Option<&str>,
    limit: Option<i64>,
  ) -> Result<AppBskyActorSearchactorstypeahead> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/app.bsky.actor.searchActorsTypeahead",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut _q = Vec::new();

    if term.is_some() {
      _q.push(("term", term.unwrap_or_default()));
    };

    if q.is_some() {
      _q.push(("q", q.unwrap_or_default()));
    }

    let limit_value = serde_json::to_string(&limit)?;

    if limit.is_some() {
      _q.push(("limit", limit_value.as_str()));
    }

    Ok(req.query_pairs(_q).call()?.into_json()?)
  }

  /// Get information about a feed generator, including policies and offered feed URIs. Does not require auth; implemented by Feed Generator services (not App View).

  pub fn app_bsky_feed_describefeedgenerator(&self) -> Result<AppBskyFeedDescribefeedgenerator> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/app.bsky.feed.describeFeedGenerator",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    Ok(req.call()?.into_json()?)
  }

  /// Get a list of feeds (feed generator records) created by the actor (in the actor&#39;s repo).

  pub fn app_bsky_feed_getactorfeeds(
    &self,
    actor: &str,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<AppBskyFeedGetactorfeeds> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/app.bsky.feed.getActorFeeds",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut _q = Vec::new();

    _q.push(("actor", actor));

    let limit_value = serde_json::to_string(&limit)?;

    if limit.is_some() {
      _q.push(("limit", limit_value.as_str()));
    };

    if cursor.is_some() {
      _q.push(("cursor", cursor.unwrap_or_default()));
    }

    Ok(req.query_pairs(_q).call()?.into_json()?)
  }

  /// Get a list of posts liked by an actor. Does not require auth.

  pub fn app_bsky_feed_getactorlikes(
    &self,
    actor: &str,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<AppBskyFeedGetactorlikes> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/app.bsky.feed.getActorLikes",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut _q = Vec::new();

    _q.push(("actor", actor));

    let limit_value = serde_json::to_string(&limit)?;

    if limit.is_some() {
      _q.push(("limit", limit_value.as_str()));
    };

    if cursor.is_some() {
      _q.push(("cursor", cursor.unwrap_or_default()));
    }

    Ok(req.query_pairs(_q).call()?.into_json()?)
  }

  /// Get a view of an actor&#39;s &#39;author feed&#39; (post and reposts by the author). Does not require auth.

  pub fn app_bsky_feed_getauthorfeed(
    &self,
    actor: &str,
    limit: Option<i64>,
    cursor: Option<&str>,
    filter: Option<&str>,
  ) -> Result<AppBskyFeedGetauthorfeed> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/app.bsky.feed.getAuthorFeed",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut _q = Vec::new();

    _q.push(("actor", actor));

    let limit_value = serde_json::to_string(&limit)?;

    if limit.is_some() {
      _q.push(("limit", limit_value.as_str()));
    };

    if cursor.is_some() {
      _q.push(("cursor", cursor.unwrap_or_default()));
    };

    if filter.is_some() {
      _q.push(("filter", filter.unwrap_or_default()));
    }

    Ok(req.query_pairs(_q).call()?.into_json()?)
  }

  /// Get a hydrated feed from an actor&#39;s selected feed generator. Implemented by App View.

  pub fn app_bsky_feed_getfeed(
    &self,
    feed: &str,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<AppBskyFeedGetfeed> {
    let mut req = self
      .agent
      .get(&format!("https://{}/xrpc/app.bsky.feed.getFeed", self.host));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut _q = Vec::new();

    _q.push(("feed", feed));

    let limit_value = serde_json::to_string(&limit)?;

    if limit.is_some() {
      _q.push(("limit", limit_value.as_str()));
    };

    if cursor.is_some() {
      _q.push(("cursor", cursor.unwrap_or_default()));
    }

    Ok(req.query_pairs(_q).call()?.into_json()?)
  }

  /// Get information about a feed generator. Implemented by AppView.

  pub fn app_bsky_feed_getfeedgenerator(&self, feed: &str) -> Result<AppBskyFeedGetfeedgenerator> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/app.bsky.feed.getFeedGenerator",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut _q = Vec::new();

    _q.push(("feed", feed));

    Ok(req.query_pairs(_q).call()?.into_json()?)
  }

  /// Get information about a list of feed generators.

  pub fn app_bsky_feed_getfeedgenerators(
    &self,
    feeds: &[&str],
  ) -> Result<AppBskyFeedGetfeedgenerators> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/app.bsky.feed.getFeedGenerators",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut _q = Vec::new();

    let mut feeds_value = feeds.iter().map(|i| ("feeds", *i)).collect::<Vec<_>>();

    _q.append(&mut feeds_value);

    Ok(req.query_pairs(_q).call()?.into_json()?)
  }

  /// Get a skeleton of a feed provided by a feed generator. Auth is optional, depending on provider requirements, and provides the DID of the requester. Implemented by Feed Generator Service.

  pub fn app_bsky_feed_getfeedskeleton(
    &self,
    feed: &str,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<AppBskyFeedGetfeedskeleton> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/app.bsky.feed.getFeedSkeleton",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut _q = Vec::new();

    _q.push(("feed", feed));

    let limit_value = serde_json::to_string(&limit)?;

    if limit.is_some() {
      _q.push(("limit", limit_value.as_str()));
    };

    if cursor.is_some() {
      _q.push(("cursor", cursor.unwrap_or_default()));
    }

    Ok(req.query_pairs(_q).call()?.into_json()?)
  }

  /// Get like records which reference a subject (by AT-URI and CID).

  pub fn app_bsky_feed_getlikes(
    &self,
    uri: &str,
    cid: Option<&CidString>,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<AppBskyFeedGetlikes> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/app.bsky.feed.getLikes",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut _q = Vec::new();

    _q.push(("uri", uri));

    let cid_value = serde_json::to_string(&cid)?;

    if cid.is_some() {
      _q.push(("cid", cid_value.as_str()));
    }

    let limit_value = serde_json::to_string(&limit)?;

    if limit.is_some() {
      _q.push(("limit", limit_value.as_str()));
    };

    if cursor.is_some() {
      _q.push(("cursor", cursor.unwrap_or_default()));
    }

    Ok(req.query_pairs(_q).call()?.into_json()?)
  }

  /// Get a feed of recent posts from a list (posts and reposts from any actors on the list). Does not require auth.

  pub fn app_bsky_feed_getlistfeed(
    &self,
    list: &str,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<AppBskyFeedGetlistfeed> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/app.bsky.feed.getListFeed",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut _q = Vec::new();

    _q.push(("list", list));

    let limit_value = serde_json::to_string(&limit)?;

    if limit.is_some() {
      _q.push(("limit", limit_value.as_str()));
    };

    if cursor.is_some() {
      _q.push(("cursor", cursor.unwrap_or_default()));
    }

    Ok(req.query_pairs(_q).call()?.into_json()?)
  }

  /// Get posts in a thread. Does not require auth, but additional metadata and filtering will be applied for authed requests.

  pub fn app_bsky_feed_getpostthread(
    &self,
    uri: &str,
    depth: Option<i64>,
    parent_height: Option<i64>,
  ) -> Result<AppBskyFeedGetpostthread> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/app.bsky.feed.getPostThread",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut _q = Vec::new();

    _q.push(("uri", uri));

    let depth_value = serde_json::to_string(&depth)?;

    if depth.is_some() {
      _q.push(("depth", depth_value.as_str()));
    }

    let parent_height_value = serde_json::to_string(&parent_height)?;

    if parent_height.is_some() {
      _q.push(("parent_height", parent_height_value.as_str()));
    }

    Ok(req.query_pairs(_q).call()?.into_json()?)
  }

  /// Gets post views for a specified list of posts (by AT-URI). This is sometimes referred to as &#39;hydrating&#39; a &#39;feed skeleton&#39;.

  pub fn app_bsky_feed_getposts(&self, uris: &[&str]) -> Result<AppBskyFeedGetposts> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/app.bsky.feed.getPosts",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut _q = Vec::new();

    let mut uris_value = uris.iter().map(|i| ("uris", *i)).collect::<Vec<_>>();

    _q.append(&mut uris_value);

    Ok(req.query_pairs(_q).call()?.into_json()?)
  }

  /// Get a list of reposts for a given post.

  pub fn app_bsky_feed_getrepostedby(
    &self,
    uri: &str,
    cid: Option<&CidString>,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<AppBskyFeedGetrepostedby> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/app.bsky.feed.getRepostedBy",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut _q = Vec::new();

    _q.push(("uri", uri));

    let cid_value = serde_json::to_string(&cid)?;

    if cid.is_some() {
      _q.push(("cid", cid_value.as_str()));
    }

    let limit_value = serde_json::to_string(&limit)?;

    if limit.is_some() {
      _q.push(("limit", limit_value.as_str()));
    };

    if cursor.is_some() {
      _q.push(("cursor", cursor.unwrap_or_default()));
    }

    Ok(req.query_pairs(_q).call()?.into_json()?)
  }

  /// Get a list of suggested feeds (feed generators) for the requesting account.

  pub fn app_bsky_feed_getsuggestedfeeds(
    &self,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<AppBskyFeedGetsuggestedfeeds> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/app.bsky.feed.getSuggestedFeeds",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut _q = Vec::new();

    let limit_value = serde_json::to_string(&limit)?;

    if limit.is_some() {
      _q.push(("limit", limit_value.as_str()));
    };

    if cursor.is_some() {
      _q.push(("cursor", cursor.unwrap_or_default()));
    }

    Ok(req.query_pairs(_q).call()?.into_json()?)
  }

  /// Get a view of the requesting account&#39;s home timeline. This is expected to be some form of reverse-chronological feed.

  pub fn app_bsky_feed_gettimeline(
    &self,
    algorithm: Option<&str>,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<AppBskyFeedGettimeline> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/app.bsky.feed.getTimeline",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut _q = Vec::new();

    if algorithm.is_some() {
      _q.push(("algorithm", algorithm.unwrap_or_default()));
    }

    let limit_value = serde_json::to_string(&limit)?;

    if limit.is_some() {
      _q.push(("limit", limit_value.as_str()));
    };

    if cursor.is_some() {
      _q.push(("cursor", cursor.unwrap_or_default()));
    }

    Ok(req.query_pairs(_q).call()?.into_json()?)
  }

  /// Find posts matching search criteria, returning views of those posts.

  pub fn app_bsky_feed_searchposts(
    &self,
    q: &str,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<AppBskyFeedSearchposts> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/app.bsky.feed.searchPosts",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut _q = Vec::new();

    _q.push(("q", q));

    let limit_value = serde_json::to_string(&limit)?;

    if limit.is_some() {
      _q.push(("limit", limit_value.as_str()));
    };

    if cursor.is_some() {
      _q.push(("cursor", cursor.unwrap_or_default()));
    }

    Ok(req.query_pairs(_q).call()?.into_json()?)
  }

  /// Enumerates which accounts the requesting account is currently blocking. Requires auth.

  pub fn app_bsky_graph_getblocks(
    &self,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<AppBskyGraphGetblocks> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/app.bsky.graph.getBlocks",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut _q = Vec::new();

    let limit_value = serde_json::to_string(&limit)?;

    if limit.is_some() {
      _q.push(("limit", limit_value.as_str()));
    };

    if cursor.is_some() {
      _q.push(("cursor", cursor.unwrap_or_default()));
    }

    Ok(req.query_pairs(_q).call()?.into_json()?)
  }

  /// Enumerates accounts which follow a specified account (actor).

  pub fn app_bsky_graph_getfollowers(
    &self,
    actor: &str,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<AppBskyGraphGetfollowers> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/app.bsky.graph.getFollowers",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut _q = Vec::new();

    _q.push(("actor", actor));

    let limit_value = serde_json::to_string(&limit)?;

    if limit.is_some() {
      _q.push(("limit", limit_value.as_str()));
    };

    if cursor.is_some() {
      _q.push(("cursor", cursor.unwrap_or_default()));
    }

    Ok(req.query_pairs(_q).call()?.into_json()?)
  }

  /// Enumerates accounts which a specified account (actor) follows.

  pub fn app_bsky_graph_getfollows(
    &self,
    actor: &str,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<AppBskyGraphGetfollows> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/app.bsky.graph.getFollows",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut _q = Vec::new();

    _q.push(("actor", actor));

    let limit_value = serde_json::to_string(&limit)?;

    if limit.is_some() {
      _q.push(("limit", limit_value.as_str()));
    };

    if cursor.is_some() {
      _q.push(("cursor", cursor.unwrap_or_default()));
    }

    Ok(req.query_pairs(_q).call()?.into_json()?)
  }

  /// Gets a &#39;view&#39; (with additional context) of a specified list.

  pub fn app_bsky_graph_getlist(
    &self,
    list: &str,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<AppBskyGraphGetlist> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/app.bsky.graph.getList",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut _q = Vec::new();

    _q.push(("list", list));

    let limit_value = serde_json::to_string(&limit)?;

    if limit.is_some() {
      _q.push(("limit", limit_value.as_str()));
    };

    if cursor.is_some() {
      _q.push(("cursor", cursor.unwrap_or_default()));
    }

    Ok(req.query_pairs(_q).call()?.into_json()?)
  }

  /// Get mod lists that the requesting account (actor) is blocking. Requires auth.

  pub fn app_bsky_graph_getlistblocks(
    &self,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<AppBskyGraphGetlistblocks> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/app.bsky.graph.getListBlocks",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut _q = Vec::new();

    let limit_value = serde_json::to_string(&limit)?;

    if limit.is_some() {
      _q.push(("limit", limit_value.as_str()));
    };

    if cursor.is_some() {
      _q.push(("cursor", cursor.unwrap_or_default()));
    }

    Ok(req.query_pairs(_q).call()?.into_json()?)
  }

  /// Enumerates mod lists that the requesting account (actor) currently has muted. Requires auth.

  pub fn app_bsky_graph_getlistmutes(
    &self,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<AppBskyGraphGetlistmutes> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/app.bsky.graph.getListMutes",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut _q = Vec::new();

    let limit_value = serde_json::to_string(&limit)?;

    if limit.is_some() {
      _q.push(("limit", limit_value.as_str()));
    };

    if cursor.is_some() {
      _q.push(("cursor", cursor.unwrap_or_default()));
    }

    Ok(req.query_pairs(_q).call()?.into_json()?)
  }

  /// Enumerates the lists created by a specified account (actor).

  pub fn app_bsky_graph_getlists(
    &self,
    actor: &str,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<AppBskyGraphGetlists> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/app.bsky.graph.getLists",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut _q = Vec::new();

    _q.push(("actor", actor));

    let limit_value = serde_json::to_string(&limit)?;

    if limit.is_some() {
      _q.push(("limit", limit_value.as_str()));
    };

    if cursor.is_some() {
      _q.push(("cursor", cursor.unwrap_or_default()));
    }

    Ok(req.query_pairs(_q).call()?.into_json()?)
  }

  /// Enumerates accounts that the requesting account (actor) currently has muted. Requires auth.

  pub fn app_bsky_graph_getmutes(
    &self,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<AppBskyGraphGetmutes> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/app.bsky.graph.getMutes",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut _q = Vec::new();

    let limit_value = serde_json::to_string(&limit)?;

    if limit.is_some() {
      _q.push(("limit", limit_value.as_str()));
    };

    if cursor.is_some() {
      _q.push(("cursor", cursor.unwrap_or_default()));
    }

    Ok(req.query_pairs(_q).call()?.into_json()?)
  }

  /// Enumerates public relationships between one account, and a list of other accounts. Does not require auth.

  pub fn app_bsky_graph_getrelationships(
    &self,
    actor: &str,
    others: Option<&[&str]>,
  ) -> Result<AppBskyGraphGetrelationships> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/app.bsky.graph.getRelationships",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut _q = Vec::new();

    _q.push(("actor", actor));

    let others_value = serde_json::to_string(&others)?;

    if others.is_some() {
      _q.push(("others", others_value.as_str()));
    }

    Ok(req.query_pairs(_q).call()?.into_json()?)
  }

  /// Enumerates follows similar to a given account (actor). Expected use is to recommend additional accounts immediately after following one account.

  pub fn app_bsky_graph_getsuggestedfollowsbyactor(
    &self,
    actor: &str,
  ) -> Result<AppBskyGraphGetsuggestedfollowsbyactor> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/app.bsky.graph.getSuggestedFollowsByActor",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut _q = Vec::new();

    _q.push(("actor", actor));

    Ok(req.query_pairs(_q).call()?.into_json()?)
  }

  /// Count the number of unread notifications for the requesting account. Requires auth.

  pub fn app_bsky_notification_getunreadcount(
    &self,
    seen_at: Option<&DateTime<Utc>>,
  ) -> Result<AppBskyNotificationGetunreadcount> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/app.bsky.notification.getUnreadCount",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut _q = Vec::new();

    let seen_at_value = serde_json::to_string(&seen_at)?;

    if seen_at.is_some() {
      _q.push(("seen_at", seen_at_value.as_str()));
    }

    Ok(req.query_pairs(_q).call()?.into_json()?)
  }

  /// Enumerate notifications for the requesting account. Requires auth.

  pub fn app_bsky_notification_listnotifications(
    &self,
    limit: Option<i64>,
    cursor: Option<&str>,
    seen_at: Option<&DateTime<Utc>>,
  ) -> Result<AppBskyNotificationListnotifications> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/app.bsky.notification.listNotifications",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut _q = Vec::new();

    let limit_value = serde_json::to_string(&limit)?;

    if limit.is_some() {
      _q.push(("limit", limit_value.as_str()));
    };

    if cursor.is_some() {
      _q.push(("cursor", cursor.unwrap_or_default()));
    }

    let seen_at_value = serde_json::to_string(&seen_at)?;

    if seen_at.is_some() {
      _q.push(("seen_at", seen_at_value.as_str()));
    }

    Ok(req.query_pairs(_q).call()?.into_json()?)
  }

  /// An unspecced view of globally popular feed generators.

  pub fn app_bsky_unspecced_getpopularfeedgenerators(
    &self,
    limit: Option<i64>,
    cursor: Option<&str>,
    query: Option<&str>,
  ) -> Result<AppBskyUnspeccedGetpopularfeedgenerators> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/app.bsky.unspecced.getPopularFeedGenerators",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut _q = Vec::new();

    let limit_value = serde_json::to_string(&limit)?;

    if limit.is_some() {
      _q.push(("limit", limit_value.as_str()));
    };

    if cursor.is_some() {
      _q.push(("cursor", cursor.unwrap_or_default()));
    };

    if query.is_some() {
      _q.push(("query", query.unwrap_or_default()));
    }

    Ok(req.query_pairs(_q).call()?.into_json()?)
  }

  /// Get a list of suggestions (feeds and users) tagged with categories

  pub fn app_bsky_unspecced_gettaggedsuggestions(
    &self,
  ) -> Result<AppBskyUnspeccedGettaggedsuggestions> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/app.bsky.unspecced.getTaggedSuggestions",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    Ok(req.call()?.into_json()?)
  }

  /// Backend Actors (profile) search, returns only skeleton.

  pub fn app_bsky_unspecced_searchactorsskeleton(
    &self,
    q: &str,
    typeahead: Option<bool>,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<AppBskyUnspeccedSearchactorsskeleton> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/app.bsky.unspecced.searchActorsSkeleton",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut _q = Vec::new();

    _q.push(("q", q));

    let typeahead_value = serde_json::to_string(&typeahead)?;

    if typeahead.is_some() {
      _q.push(("typeahead", typeahead_value.as_str()));
    }

    let limit_value = serde_json::to_string(&limit)?;

    if limit.is_some() {
      _q.push(("limit", limit_value.as_str()));
    };

    if cursor.is_some() {
      _q.push(("cursor", cursor.unwrap_or_default()));
    }

    Ok(req.query_pairs(_q).call()?.into_json()?)
  }

  /// Backend Posts search, returns only skeleton

  pub fn app_bsky_unspecced_searchpostsskeleton(
    &self,
    q: &str,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<AppBskyUnspeccedSearchpostsskeleton> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/app.bsky.unspecced.searchPostsSkeleton",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut _q = Vec::new();

    _q.push(("q", q));

    let limit_value = serde_json::to_string(&limit)?;

    if limit.is_some() {
      _q.push(("limit", limit_value.as_str()));
    };

    if cursor.is_some() {
      _q.push(("cursor", cursor.unwrap_or_default()));
    }

    Ok(req.query_pairs(_q).call()?.into_json()?)
  }

  /// Get details about an account.

  pub fn com_atproto_admin_getaccountinfo(
    &self,
    did: &str,
  ) -> Result<ComAtprotoAdminDefsAccountview> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/com.atproto.admin.getAccountInfo",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut _q = Vec::new();

    _q.push(("did", did));

    Ok(req.query_pairs(_q).call()?.into_json()?)
  }

  /// Get details about some accounts.

  pub fn com_atproto_admin_getaccountinfos(
    &self,
    dids: &[&str],
  ) -> Result<ComAtprotoAdminGetaccountinfos> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/com.atproto.admin.getAccountInfos",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut _q = Vec::new();

    let mut dids_value = dids.iter().map(|i| ("dids", *i)).collect::<Vec<_>>();

    _q.append(&mut dids_value);

    Ok(req.query_pairs(_q).call()?.into_json()?)
  }

  /// Get an admin view of invite codes.

  pub fn com_atproto_admin_getinvitecodes(
    &self,
    sort: Option<&str>,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<ComAtprotoAdminGetinvitecodes> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/com.atproto.admin.getInviteCodes",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut _q = Vec::new();

    if sort.is_some() {
      _q.push(("sort", sort.unwrap_or_default()));
    }

    let limit_value = serde_json::to_string(&limit)?;

    if limit.is_some() {
      _q.push(("limit", limit_value.as_str()));
    };

    if cursor.is_some() {
      _q.push(("cursor", cursor.unwrap_or_default()));
    }

    Ok(req.query_pairs(_q).call()?.into_json()?)
  }

  /// Get details about a moderation event.

  pub fn com_atproto_admin_getmoderationevent(
    &self,
    id: i64,
  ) -> Result<ComAtprotoAdminDefsModeventviewdetail> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/com.atproto.admin.getModerationEvent",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut _q = Vec::new();

    let id_value = serde_json::to_string(&id)?;

    _q.push(("id", id_value.as_str()));

    Ok(req.query_pairs(_q).call()?.into_json()?)
  }

  /// Get details about a record.

  pub fn com_atproto_admin_getrecord(
    &self,
    uri: &str,
    cid: Option<&CidString>,
  ) -> Result<ComAtprotoAdminDefsRecordviewdetail> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/com.atproto.admin.getRecord",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut _q = Vec::new();

    _q.push(("uri", uri));

    let cid_value = serde_json::to_string(&cid)?;

    if cid.is_some() {
      _q.push(("cid", cid_value.as_str()));
    }

    Ok(req.query_pairs(_q).call()?.into_json()?)
  }

  /// Get details about a repository.

  pub fn com_atproto_admin_getrepo(&self, did: &str) -> Result<ComAtprotoAdminDefsRepoviewdetail> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/com.atproto.admin.getRepo",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut _q = Vec::new();

    _q.push(("did", did));

    Ok(req.query_pairs(_q).call()?.into_json()?)
  }

  /// Get the service-specific admin status of a subject (account, record, or blob).

  pub fn com_atproto_admin_getsubjectstatus(
    &self,
    did: Option<&str>,
    uri: Option<&str>,
    blob: Option<&CidString>,
  ) -> Result<ComAtprotoAdminGetsubjectstatus> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/com.atproto.admin.getSubjectStatus",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut _q = Vec::new();

    if did.is_some() {
      _q.push(("did", did.unwrap_or_default()));
    };

    if uri.is_some() {
      _q.push(("uri", uri.unwrap_or_default()));
    }

    let blob_value = serde_json::to_string(&blob)?;

    if blob.is_some() {
      _q.push(("blob", blob_value.as_str()));
    }

    Ok(req.query_pairs(_q).call()?.into_json()?)
  }

  /// Get list of all communication templates.

  pub fn com_atproto_admin_listcommunicationtemplates(
    &self,
  ) -> Result<ComAtprotoAdminListcommunicationtemplates> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/com.atproto.admin.listCommunicationTemplates",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    Ok(req.call()?.into_json()?)
  }

  /// List moderation events related to a subject.

  pub fn com_atproto_admin_querymoderationevents(
    &self,
    types: Option<&[&str]>,
    created_by: Option<&str>,
    sort_direction: Option<&str>,
    created_after: Option<&DateTime<Utc>>,
    created_before: Option<&DateTime<Utc>>,
    subject: Option<&str>,
    include_all_user_records: Option<bool>,
    limit: Option<i64>,
    has_comment: Option<bool>,
    comment: Option<&str>,
    added_labels: Option<&[&str]>,
    removed_labels: Option<&[&str]>,
    added_tags: Option<&[&str]>,
    removed_tags: Option<&[&str]>,
    report_types: Option<&[&str]>,
    cursor: Option<&str>,
  ) -> Result<ComAtprotoAdminQuerymoderationevents> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/com.atproto.admin.queryModerationEvents",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut _q = Vec::new();

    let types_value = serde_json::to_string(&types)?;

    if types.is_some() {
      _q.push(("types", types_value.as_str()));
    };

    if created_by.is_some() {
      _q.push(("created_by", created_by.unwrap_or_default()));
    };

    if sort_direction.is_some() {
      _q.push(("sort_direction", sort_direction.unwrap_or_default()));
    }

    let created_after_value = serde_json::to_string(&created_after)?;

    if created_after.is_some() {
      _q.push(("created_after", created_after_value.as_str()));
    }

    let created_before_value = serde_json::to_string(&created_before)?;

    if created_before.is_some() {
      _q.push(("created_before", created_before_value.as_str()));
    };

    if subject.is_some() {
      _q.push(("subject", subject.unwrap_or_default()));
    }

    let include_all_user_records_value = serde_json::to_string(&include_all_user_records)?;

    if include_all_user_records.is_some() {
      _q.push((
        "include_all_user_records",
        include_all_user_records_value.as_str(),
      ));
    }

    let limit_value = serde_json::to_string(&limit)?;

    if limit.is_some() {
      _q.push(("limit", limit_value.as_str()));
    }

    let has_comment_value = serde_json::to_string(&has_comment)?;

    if has_comment.is_some() {
      _q.push(("has_comment", has_comment_value.as_str()));
    };

    if comment.is_some() {
      _q.push(("comment", comment.unwrap_or_default()));
    }

    let added_labels_value = serde_json::to_string(&added_labels)?;

    if added_labels.is_some() {
      _q.push(("added_labels", added_labels_value.as_str()));
    }

    let removed_labels_value = serde_json::to_string(&removed_labels)?;

    if removed_labels.is_some() {
      _q.push(("removed_labels", removed_labels_value.as_str()));
    }

    let added_tags_value = serde_json::to_string(&added_tags)?;

    if added_tags.is_some() {
      _q.push(("added_tags", added_tags_value.as_str()));
    }

    let removed_tags_value = serde_json::to_string(&removed_tags)?;

    if removed_tags.is_some() {
      _q.push(("removed_tags", removed_tags_value.as_str()));
    }

    let report_types_value = serde_json::to_string(&report_types)?;

    if report_types.is_some() {
      _q.push(("report_types", report_types_value.as_str()));
    };

    if cursor.is_some() {
      _q.push(("cursor", cursor.unwrap_or_default()));
    }

    Ok(req.query_pairs(_q).call()?.into_json()?)
  }

  /// View moderation statuses of subjects (record or repo).

  pub fn com_atproto_admin_querymoderationstatuses(
    &self,
    subject: Option<&str>,
    comment: Option<&str>,
    reported_after: Option<&DateTime<Utc>>,
    reported_before: Option<&DateTime<Utc>>,
    reviewed_after: Option<&DateTime<Utc>>,
    reviewed_before: Option<&DateTime<Utc>>,
    include_muted: Option<bool>,
    review_state: Option<&str>,
    ignore_subjects: Option<&[&str]>,
    last_reviewed_by: Option<&str>,
    sort_field: Option<&str>,
    sort_direction: Option<&str>,
    takendown: Option<bool>,
    appealed: Option<bool>,
    limit: Option<i64>,
    tags: Option<&[&str]>,
    exclude_tags: Option<&[&str]>,
    cursor: Option<&str>,
  ) -> Result<ComAtprotoAdminQuerymoderationstatuses> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/com.atproto.admin.queryModerationStatuses",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut _q = Vec::new();

    if subject.is_some() {
      _q.push(("subject", subject.unwrap_or_default()));
    };

    if comment.is_some() {
      _q.push(("comment", comment.unwrap_or_default()));
    }

    let reported_after_value = serde_json::to_string(&reported_after)?;

    if reported_after.is_some() {
      _q.push(("reported_after", reported_after_value.as_str()));
    }

    let reported_before_value = serde_json::to_string(&reported_before)?;

    if reported_before.is_some() {
      _q.push(("reported_before", reported_before_value.as_str()));
    }

    let reviewed_after_value = serde_json::to_string(&reviewed_after)?;

    if reviewed_after.is_some() {
      _q.push(("reviewed_after", reviewed_after_value.as_str()));
    }

    let reviewed_before_value = serde_json::to_string(&reviewed_before)?;

    if reviewed_before.is_some() {
      _q.push(("reviewed_before", reviewed_before_value.as_str()));
    }

    let include_muted_value = serde_json::to_string(&include_muted)?;

    if include_muted.is_some() {
      _q.push(("include_muted", include_muted_value.as_str()));
    };

    if review_state.is_some() {
      _q.push(("review_state", review_state.unwrap_or_default()));
    }

    let ignore_subjects_value = serde_json::to_string(&ignore_subjects)?;

    if ignore_subjects.is_some() {
      _q.push(("ignore_subjects", ignore_subjects_value.as_str()));
    };

    if last_reviewed_by.is_some() {
      _q.push(("last_reviewed_by", last_reviewed_by.unwrap_or_default()));
    };

    if sort_field.is_some() {
      _q.push(("sort_field", sort_field.unwrap_or_default()));
    };

    if sort_direction.is_some() {
      _q.push(("sort_direction", sort_direction.unwrap_or_default()));
    }

    let takendown_value = serde_json::to_string(&takendown)?;

    if takendown.is_some() {
      _q.push(("takendown", takendown_value.as_str()));
    }

    let appealed_value = serde_json::to_string(&appealed)?;

    if appealed.is_some() {
      _q.push(("appealed", appealed_value.as_str()));
    }

    let limit_value = serde_json::to_string(&limit)?;

    if limit.is_some() {
      _q.push(("limit", limit_value.as_str()));
    }

    let tags_value = serde_json::to_string(&tags)?;

    if tags.is_some() {
      _q.push(("tags", tags_value.as_str()));
    }

    let exclude_tags_value = serde_json::to_string(&exclude_tags)?;

    if exclude_tags.is_some() {
      _q.push(("exclude_tags", exclude_tags_value.as_str()));
    };

    if cursor.is_some() {
      _q.push(("cursor", cursor.unwrap_or_default()));
    }

    Ok(req.query_pairs(_q).call()?.into_json()?)
  }

  /// Find repositories based on a search term.

  pub fn com_atproto_admin_searchrepos(
    &self,
    term: Option<&str>,
    q: Option<&str>,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<ComAtprotoAdminSearchrepos> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/com.atproto.admin.searchRepos",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut _q = Vec::new();

    if term.is_some() {
      _q.push(("term", term.unwrap_or_default()));
    };

    if q.is_some() {
      _q.push(("q", q.unwrap_or_default()));
    }

    let limit_value = serde_json::to_string(&limit)?;

    if limit.is_some() {
      _q.push(("limit", limit_value.as_str()));
    };

    if cursor.is_some() {
      _q.push(("cursor", cursor.unwrap_or_default()));
    }

    Ok(req.query_pairs(_q).call()?.into_json()?)
  }

  /// Describe the credentials that should be included in the DID doc of an account that is migrating to this service.

  pub fn com_atproto_identity_getrecommendeddidcredentials(
    &self,
  ) -> Result<ComAtprotoIdentityGetrecommendeddidcredentials> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/com.atproto.identity.getRecommendedDidCredentials",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    Ok(req.call()?.into_json()?)
  }

  /// Resolves a handle (domain name) to a DID.

  pub fn com_atproto_identity_resolvehandle(
    &self,
    handle: &str,
  ) -> Result<ComAtprotoIdentityResolvehandle> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/com.atproto.identity.resolveHandle",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut _q = Vec::new();

    _q.push(("handle", handle));

    Ok(req.query_pairs(_q).call()?.into_json()?)
  }

  /// Find labels relevant to the provided AT-URI patterns. Public endpoint for moderation services, though may return different or additional results with auth.

  pub fn com_atproto_label_querylabels(
    &self,
    uri_patterns: &[&str],
    sources: Option<&[&str]>,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<ComAtprotoLabelQuerylabels> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/com.atproto.label.queryLabels",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut _q = Vec::new();

    let mut uri_patterns_value = uri_patterns
      .iter()
      .map(|i| ("uriPatterns", *i))
      .collect::<Vec<_>>();

    _q.append(&mut uri_patterns_value);

    let sources_value = serde_json::to_string(&sources)?;

    if sources.is_some() {
      _q.push(("sources", sources_value.as_str()));
    }

    let limit_value = serde_json::to_string(&limit)?;

    if limit.is_some() {
      _q.push(("limit", limit_value.as_str()));
    };

    if cursor.is_some() {
      _q.push(("cursor", cursor.unwrap_or_default()));
    }

    Ok(req.query_pairs(_q).call()?.into_json()?)
  }

  /// Get information about an account and repository, including the list of collections. Does not require auth.

  pub fn com_atproto_repo_describerepo(&self, repo: &str) -> Result<ComAtprotoRepoDescriberepo> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/com.atproto.repo.describeRepo",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut _q = Vec::new();

    _q.push(("repo", repo));

    Ok(req.query_pairs(_q).call()?.into_json()?)
  }

  /// Get a single record from a repository. Does not require auth.

  pub fn com_atproto_repo_getrecord(
    &self,
    repo: &str,
    collection: &str,
    rkey: &str,
    cid: Option<&CidString>,
  ) -> Result<ComAtprotoRepoGetrecord> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/com.atproto.repo.getRecord",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut _q = Vec::new();

    _q.push(("repo", repo));

    _q.push(("collection", collection));

    _q.push(("rkey", rkey));

    let cid_value = serde_json::to_string(&cid)?;

    if cid.is_some() {
      _q.push(("cid", cid_value.as_str()));
    }

    Ok(req.query_pairs(_q).call()?.into_json()?)
  }

  /// Returns a list of missing blobs for the requesting account. Intended to be used in the account migration flow.

  pub fn com_atproto_repo_listmissingblobs(
    &self,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<ComAtprotoRepoListmissingblobs> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/com.atproto.repo.listMissingBlobs",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut _q = Vec::new();

    let limit_value = serde_json::to_string(&limit)?;

    if limit.is_some() {
      _q.push(("limit", limit_value.as_str()));
    };

    if cursor.is_some() {
      _q.push(("cursor", cursor.unwrap_or_default()));
    }

    Ok(req.query_pairs(_q).call()?.into_json()?)
  }

  /// List a range of records in a repository, matching a specific collection. Does not require auth.

  pub fn com_atproto_repo_listrecords(
    &self,
    repo: &str,
    collection: &str,
    limit: Option<i64>,
    cursor: Option<&str>,
    rkey_start: Option<&str>,
    rkey_end: Option<&str>,
    reverse: Option<bool>,
  ) -> Result<ComAtprotoRepoListrecords> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/com.atproto.repo.listRecords",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut _q = Vec::new();

    _q.push(("repo", repo));

    _q.push(("collection", collection));

    let limit_value = serde_json::to_string(&limit)?;

    if limit.is_some() {
      _q.push(("limit", limit_value.as_str()));
    };

    if cursor.is_some() {
      _q.push(("cursor", cursor.unwrap_or_default()));
    };

    if rkey_start.is_some() {
      _q.push(("rkey_start", rkey_start.unwrap_or_default()));
    };

    if rkey_end.is_some() {
      _q.push(("rkey_end", rkey_end.unwrap_or_default()));
    }

    let reverse_value = serde_json::to_string(&reverse)?;

    if reverse.is_some() {
      _q.push(("reverse", reverse_value.as_str()));
    }

    Ok(req.query_pairs(_q).call()?.into_json()?)
  }

  /// Returns the status of an account, especially as pertaining to import or recovery. Can be called many times over the course of an account migration. Requires auth and can only be called pertaining to oneself.

  pub fn com_atproto_server_checkaccountstatus(
    &self,
  ) -> Result<ComAtprotoServerCheckaccountstatus> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/com.atproto.server.checkAccountStatus",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    Ok(req.call()?.into_json()?)
  }

  /// Describes the server&#39;s account creation requirements and capabilities. Implemented by PDS.

  pub fn com_atproto_server_describeserver(&self) -> Result<ComAtprotoServerDescribeserver> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/com.atproto.server.describeServer",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    Ok(req.call()?.into_json()?)
  }

  /// Get all invite codes for the current account. Requires auth.

  pub fn com_atproto_server_getaccountinvitecodes(
    &self,
    include_used: Option<bool>,
    create_available: Option<bool>,
  ) -> Result<ComAtprotoServerGetaccountinvitecodes> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/com.atproto.server.getAccountInviteCodes",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut _q = Vec::new();

    let include_used_value = serde_json::to_string(&include_used)?;

    if include_used.is_some() {
      _q.push(("include_used", include_used_value.as_str()));
    }

    let create_available_value = serde_json::to_string(&create_available)?;

    if create_available.is_some() {
      _q.push(("create_available", create_available_value.as_str()));
    }

    Ok(req.query_pairs(_q).call()?.into_json()?)
  }

  /// Get a signed token on behalf of the requesting DID for the requested service.

  pub fn com_atproto_server_getserviceauth(
    &self,
    aud: &str,
  ) -> Result<ComAtprotoServerGetserviceauth> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/com.atproto.server.getServiceAuth",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut _q = Vec::new();

    _q.push(("aud", aud));

    Ok(req.query_pairs(_q).call()?.into_json()?)
  }

  /// Get information about the current auth session. Requires auth.

  pub fn com_atproto_server_getsession(&self) -> Result<ComAtprotoServerGetsession> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/com.atproto.server.getSession",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    Ok(req.call()?.into_json()?)
  }

  /// List all App Passwords.

  pub fn com_atproto_server_listapppasswords(&self) -> Result<ComAtprotoServerListapppasswords> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/com.atproto.server.listAppPasswords",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    Ok(req.call()?.into_json()?)
  }

  /// Get a blob associated with a given account. Returns the full blob as originally uploaded. Does not require auth; implemented by PDS.

  pub fn com_atproto_sync_getblob(&self, did: &str, cid: &CidString) -> Result<Vec<u8>> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/com.atproto.sync.getBlob",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut _q = Vec::new();

    _q.push(("did", did));

    let cid_value = serde_json::to_string(&cid)?;

    _q.push(("cid", cid_value.as_str()));

    let mut ret = Vec::new();
    req
      .query_pairs(_q)
      .call()?
      .into_reader()
      .read_to_end(&mut ret)?;
    Ok(ret)
  }

  /// Get data blocks from a given repo, by CID. For example, intermediate MST nodes, or records. Does not require auth; implemented by PDS.

  pub fn com_atproto_sync_getblocks(&self, did: &str, cids: &[&CidString]) -> Result<Blocks> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/com.atproto.sync.getBlocks",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut _q = Vec::new();

    _q.push(("did", did));

    let cids_value = serde_json::to_string(&cids)?;

    _q.push(("cids", cids_value.as_str()));

    let mut ret = Vec::new();
    req
      .query_pairs(_q)
      .call()?
      .into_reader()
      .read_to_end(&mut ret)?;

    Ok(Blocks::from(ret.as_slice()))
  }

  /// DEPRECATED - please use com.atproto.sync.getRepo instead

  pub fn com_atproto_sync_getcheckout(&self, did: &str) -> Result<Blocks> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/com.atproto.sync.getCheckout",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut _q = Vec::new();

    _q.push(("did", did));

    let mut ret = Vec::new();
    req
      .query_pairs(_q)
      .call()?
      .into_reader()
      .read_to_end(&mut ret)?;

    Ok(Blocks::from(ret.as_slice()))
  }

  /// DEPRECATED - please use com.atproto.sync.getLatestCommit instead

  pub fn com_atproto_sync_gethead(&self, did: &str) -> Result<ComAtprotoSyncGethead> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/com.atproto.sync.getHead",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut _q = Vec::new();

    _q.push(("did", did));

    Ok(req.query_pairs(_q).call()?.into_json()?)
  }

  /// Get the current commit CID &amp; revision of the specified repo. Does not require auth.

  pub fn com_atproto_sync_getlatestcommit(
    &self,
    did: &str,
  ) -> Result<ComAtprotoSyncGetlatestcommit> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/com.atproto.sync.getLatestCommit",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut _q = Vec::new();

    _q.push(("did", did));

    Ok(req.query_pairs(_q).call()?.into_json()?)
  }

  /// Get data blocks needed to prove the existence or non-existence of record in the current version of repo. Does not require auth.

  pub fn com_atproto_sync_getrecord(
    &self,
    did: &str,
    collection: &str,
    rkey: &str,
    commit: Option<&CidString>,
  ) -> Result<Blocks> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/com.atproto.sync.getRecord",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut _q = Vec::new();

    _q.push(("did", did));

    _q.push(("collection", collection));

    _q.push(("rkey", rkey));

    let commit_value = serde_json::to_string(&commit)?;

    if commit.is_some() {
      _q.push(("commit", commit_value.as_str()));
    }

    let mut ret = Vec::new();
    req
      .query_pairs(_q)
      .call()?
      .into_reader()
      .read_to_end(&mut ret)?;

    Ok(Blocks::from(ret.as_slice()))
  }

  /// Download a repository export as CAR file. Optionally only a &#39;diff&#39; since a previous revision. Does not require auth; implemented by PDS.

  pub fn com_atproto_sync_getrepo(&self, did: &str, since: Option<&str>) -> Result<Blocks> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/com.atproto.sync.getRepo",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut _q = Vec::new();

    _q.push(("did", did));

    if since.is_some() {
      _q.push(("since", since.unwrap_or_default()));
    }

    let mut ret = Vec::new();
    req
      .query_pairs(_q)
      .call()?
      .into_reader()
      .read_to_end(&mut ret)?;

    Ok(Blocks::from(ret.as_slice()))
  }

  /// List blob CIDso for an account, since some repo revision. Does not require auth; implemented by PDS.

  pub fn com_atproto_sync_listblobs(
    &self,
    did: &str,
    since: Option<&str>,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<ComAtprotoSyncListblobs> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/com.atproto.sync.listBlobs",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut _q = Vec::new();

    _q.push(("did", did));

    if since.is_some() {
      _q.push(("since", since.unwrap_or_default()));
    }

    let limit_value = serde_json::to_string(&limit)?;

    if limit.is_some() {
      _q.push(("limit", limit_value.as_str()));
    };

    if cursor.is_some() {
      _q.push(("cursor", cursor.unwrap_or_default()));
    }

    Ok(req.query_pairs(_q).call()?.into_json()?)
  }

  /// Enumerates all the DID, rev, and commit CID for all repos hosted by this service. Does not require auth; implemented by PDS and Relay.

  pub fn com_atproto_sync_listrepos(
    &self,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<ComAtprotoSyncListrepos> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/com.atproto.sync.listRepos",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut _q = Vec::new();

    let limit_value = serde_json::to_string(&limit)?;

    if limit.is_some() {
      _q.push(("limit", limit_value.as_str()));
    };

    if cursor.is_some() {
      _q.push(("cursor", cursor.unwrap_or_default()));
    }

    Ok(req.query_pairs(_q).call()?.into_json()?)
  }

  /// Check accounts location in signup queue.

  pub fn com_atproto_temp_checksignupqueue(&self) -> Result<ComAtprotoTempChecksignupqueue> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/com.atproto.temp.checkSignupQueue",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    Ok(req.call()?.into_json()?)
  }

  /// Fetch all labels from a labeler created after a certain date. DEPRECATED: use queryLabels or subscribeLabels instead

  pub fn com_atproto_temp_fetchlabels(
    &self,
    since: Option<i64>,
    limit: Option<i64>,
  ) -> Result<ComAtprotoTempFetchlabels> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/com.atproto.temp.fetchLabels",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut _q = Vec::new();

    let since_value = serde_json::to_string(&since)?;

    if since.is_some() {
      _q.push(("since", since_value.as_str()));
    }

    let limit_value = serde_json::to_string(&limit)?;

    if limit.is_some() {
      _q.push(("limit", limit_value.as_str()));
    }

    Ok(req.query_pairs(_q).call()?.into_json()?)
  }

  /// Set the private preferences attached to the account.

  pub fn app_bsky_actor_putpreferences(
    &self,
    preferences: &AppBskyActorDefsPreferences,
  ) -> Result<ureq::Response> {
    let mut req = self.agent.post(&format!(
      "https://{}/xrpc/app.bsky.actor.putPreferences",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut input = serde_json::Map::new();

    input.insert(String::from("preferences"), json!(preferences));

    Ok(req.send_json(json!(input))?)
  }

  /// Creates a mute relationship for the specified account. Mutes are private in Bluesky. Requires auth.

  pub fn app_bsky_graph_muteactor(&self, actor: &str) -> Result<ureq::Response> {
    let mut req = self.agent.post(&format!(
      "https://{}/xrpc/app.bsky.graph.muteActor",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut input = serde_json::Map::new();

    input.insert(String::from("actor"), json!(actor));

    Ok(req.send_json(json!(input))?)
  }

  /// Creates a mute relationship for the specified list of accounts. Mutes are private in Bluesky. Requires auth.

  pub fn app_bsky_graph_muteactorlist(&self, list: &str) -> Result<ureq::Response> {
    let mut req = self.agent.post(&format!(
      "https://{}/xrpc/app.bsky.graph.muteActorList",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut input = serde_json::Map::new();

    input.insert(String::from("list"), json!(list));

    Ok(req.send_json(json!(input))?)
  }

  /// Unmutes the specified account. Requires auth.

  pub fn app_bsky_graph_unmuteactor(&self, actor: &str) -> Result<ureq::Response> {
    let mut req = self.agent.post(&format!(
      "https://{}/xrpc/app.bsky.graph.unmuteActor",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut input = serde_json::Map::new();

    input.insert(String::from("actor"), json!(actor));

    Ok(req.send_json(json!(input))?)
  }

  /// Unmutes the specified list of accounts. Requires auth.

  pub fn app_bsky_graph_unmuteactorlist(&self, list: &str) -> Result<ureq::Response> {
    let mut req = self.agent.post(&format!(
      "https://{}/xrpc/app.bsky.graph.unmuteActorList",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut input = serde_json::Map::new();

    input.insert(String::from("list"), json!(list));

    Ok(req.send_json(json!(input))?)
  }

  /// Register to receive push notifications, via a specified service, for the requesting account. Requires auth.

  pub fn app_bsky_notification_registerpush(
    &self,
    service_did: &str,
    token: &str,
    platform: &str,
    app_id: &str,
  ) -> Result<ureq::Response> {
    let mut req = self.agent.post(&format!(
      "https://{}/xrpc/app.bsky.notification.registerPush",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut input = serde_json::Map::new();

    input.insert(String::from("service_did"), json!(service_did));

    input.insert(String::from("token"), json!(token));

    input.insert(String::from("platform"), json!(platform));

    input.insert(String::from("app_id"), json!(app_id));

    Ok(req.send_json(json!(input))?)
  }

  /// Notify server that the requesting account has seen notifications. Requires auth.

  pub fn app_bsky_notification_updateseen(
    &self,
    seen_at: &DateTime<Utc>,
  ) -> Result<ureq::Response> {
    let mut req = self.agent.post(&format!(
      "https://{}/xrpc/app.bsky.notification.updateSeen",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut input = serde_json::Map::new();

    input.insert(String::from("seen_at"), json!(seen_at));

    Ok(req.send_json(json!(input))?)
  }

  /// Administrative action to create a new, re-usable communication (email for now) template.

  pub fn com_atproto_admin_createcommunicationtemplate(
    &self,
    name: &str,
    content_markdown: &str,
    subject: &str,
    created_by: Option<&str>,
  ) -> Result<ComAtprotoAdminDefsCommunicationtemplateview> {
    let mut req = self.agent.post(&format!(
      "https://{}/xrpc/com.atproto.admin.createCommunicationTemplate",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut input = serde_json::Map::new();

    input.insert(String::from("name"), json!(name));

    input.insert(String::from("content_markdown"), json!(content_markdown));

    input.insert(String::from("subject"), json!(subject));

    if let Some(v) = &created_by {
      input.insert(String::from("created_by"), json!(v));
    }

    Ok(req.send_json(json!(input))?.into_json()?)
  }

  /// Delete a user account as an administrator.

  pub fn com_atproto_admin_deleteaccount(&self, did: &str) -> Result<ureq::Response> {
    let mut req = self.agent.post(&format!(
      "https://{}/xrpc/com.atproto.admin.deleteAccount",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut input = serde_json::Map::new();

    input.insert(String::from("did"), json!(did));

    Ok(req.send_json(json!(input))?)
  }

  /// Delete a communication template.

  pub fn com_atproto_admin_deletecommunicationtemplate(&self, id: &str) -> Result<ureq::Response> {
    let mut req = self.agent.post(&format!(
      "https://{}/xrpc/com.atproto.admin.deleteCommunicationTemplate",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut input = serde_json::Map::new();

    input.insert(String::from("id"), json!(id));

    Ok(req.send_json(json!(input))?)
  }

  /// Disable an account from receiving new invite codes, but does not invalidate existing codes.

  pub fn com_atproto_admin_disableaccountinvites(
    &self,
    account: &str,
    note: Option<&str>,
  ) -> Result<ureq::Response> {
    let mut req = self.agent.post(&format!(
      "https://{}/xrpc/com.atproto.admin.disableAccountInvites",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut input = serde_json::Map::new();

    input.insert(String::from("account"), json!(account));

    if let Some(v) = &note {
      input.insert(String::from("note"), json!(v));
    }

    Ok(req.send_json(json!(input))?)
  }

  /// Disable some set of codes and/or all codes associated with a set of users.

  pub fn com_atproto_admin_disableinvitecodes(
    &self,
    codes: Option<&[&str]>,
    accounts: Option<&[&str]>,
  ) -> Result<ureq::Response> {
    let mut req = self.agent.post(&format!(
      "https://{}/xrpc/com.atproto.admin.disableInviteCodes",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut input = serde_json::Map::new();

    if let Some(v) = &codes {
      input.insert(String::from("codes"), json!(v));
    }

    if let Some(v) = &accounts {
      input.insert(String::from("accounts"), json!(v));
    }

    Ok(req.send_json(json!(input))?)
  }

  /// Take a moderation action on an actor.

  pub fn com_atproto_admin_emitmoderationevent(
    &self,
    event: ComAtprotoAdminEmitmoderationeventMainInputEvent,
    subject: ComAtprotoAdminEmitmoderationeventMainInputSubject,
    created_by: &str,
    subject_blob_cids: Option<&[&CidString]>,
  ) -> Result<ComAtprotoAdminDefsModeventview> {
    let mut req = self.agent.post(&format!(
      "https://{}/xrpc/com.atproto.admin.emitModerationEvent",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut input = serde_json::Map::new();

    input.insert(String::from("event"), json!(event));

    input.insert(String::from("subject"), json!(subject));

    input.insert(String::from("created_by"), json!(created_by));

    if let Some(v) = &subject_blob_cids {
      input.insert(String::from("subject_blob_cids"), json!(v));
    }

    Ok(req.send_json(json!(input))?.into_json()?)
  }

  /// Re-enable an account&#39;s ability to receive invite codes.

  pub fn com_atproto_admin_enableaccountinvites(
    &self,
    account: &str,
    note: Option<&str>,
  ) -> Result<ureq::Response> {
    let mut req = self.agent.post(&format!(
      "https://{}/xrpc/com.atproto.admin.enableAccountInvites",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut input = serde_json::Map::new();

    input.insert(String::from("account"), json!(account));

    if let Some(v) = &note {
      input.insert(String::from("note"), json!(v));
    }

    Ok(req.send_json(json!(input))?)
  }

  /// Send email to a user&#39;s account email address.

  pub fn com_atproto_admin_sendemail(
    &self,
    recipient_did: &str,
    content: &str,
    sender_did: &str,
    subject: Option<&str>,
    comment: Option<&str>,
  ) -> Result<ComAtprotoAdminSendemail> {
    let mut req = self.agent.post(&format!(
      "https://{}/xrpc/com.atproto.admin.sendEmail",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut input = serde_json::Map::new();

    input.insert(String::from("recipient_did"), json!(recipient_did));

    input.insert(String::from("content"), json!(content));

    input.insert(String::from("sender_did"), json!(sender_did));

    if let Some(v) = &subject {
      input.insert(String::from("subject"), json!(v));
    }

    if let Some(v) = &comment {
      input.insert(String::from("comment"), json!(v));
    }

    Ok(req.send_json(json!(input))?.into_json()?)
  }

  /// Administrative action to update an account&#39;s email.

  pub fn com_atproto_admin_updateaccountemail(
    &self,
    account: &str,
    email: &str,
  ) -> Result<ureq::Response> {
    let mut req = self.agent.post(&format!(
      "https://{}/xrpc/com.atproto.admin.updateAccountEmail",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut input = serde_json::Map::new();

    input.insert(String::from("account"), json!(account));

    input.insert(String::from("email"), json!(email));

    Ok(req.send_json(json!(input))?)
  }

  /// Administrative action to update an account&#39;s handle.

  pub fn com_atproto_admin_updateaccounthandle(
    &self,
    did: &str,
    handle: &str,
  ) -> Result<ureq::Response> {
    let mut req = self.agent.post(&format!(
      "https://{}/xrpc/com.atproto.admin.updateAccountHandle",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut input = serde_json::Map::new();

    input.insert(String::from("did"), json!(did));

    input.insert(String::from("handle"), json!(handle));

    Ok(req.send_json(json!(input))?)
  }

  /// Administrative action to update an existing communication template. Allows passing partial fields to patch specific fields only.

  pub fn com_atproto_admin_updatecommunicationtemplate(
    &self,
    id: &str,
    name: Option<&str>,
    content_markdown: Option<&str>,
    subject: Option<&str>,
    updated_by: Option<&str>,
    disabled: Option<bool>,
  ) -> Result<ComAtprotoAdminDefsCommunicationtemplateview> {
    let mut req = self.agent.post(&format!(
      "https://{}/xrpc/com.atproto.admin.updateCommunicationTemplate",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut input = serde_json::Map::new();

    input.insert(String::from("id"), json!(id));

    if let Some(v) = &name {
      input.insert(String::from("name"), json!(v));
    }

    if let Some(v) = &content_markdown {
      input.insert(String::from("content_markdown"), json!(v));
    }

    if let Some(v) = &subject {
      input.insert(String::from("subject"), json!(v));
    }

    if let Some(v) = &updated_by {
      input.insert(String::from("updated_by"), json!(v));
    }

    if let Some(v) = &disabled {
      input.insert(String::from("disabled"), json!(v));
    }

    Ok(req.send_json(json!(input))?.into_json()?)
  }

  /// Update the service-specific admin status of a subject (account, record, or blob).

  pub fn com_atproto_admin_updatesubjectstatus(
    &self,
    subject: ComAtprotoAdminUpdatesubjectstatusMainInputSubject,
    takedown: Option<&ComAtprotoAdminDefsStatusattr>,
  ) -> Result<ComAtprotoAdminUpdatesubjectstatus> {
    let mut req = self.agent.post(&format!(
      "https://{}/xrpc/com.atproto.admin.updateSubjectStatus",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut input = serde_json::Map::new();

    input.insert(String::from("subject"), json!(subject));

    if let Some(v) = &takedown {
      input.insert(String::from("takedown"), json!(v));
    }

    Ok(req.send_json(json!(input))?.into_json()?)
  }

  /// Request an email with a code to in order to request a signed PLC operation. Requires Auth.

  pub fn com_atproto_identity_requestplcoperationsignature(&self) -> Result<ureq::Response> {
    let mut req = self.agent.post(&format!(
      "https://{}/xrpc/com.atproto.identity.requestPlcOperationSignature",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    Ok(req.call()?)
  }

  /// Signs a PLC operation to update some value(s) in the requesting DID&#39;s document.

  pub fn com_atproto_identity_signplcoperation(
    &self,
    token: Option<&str>,
    rotation_keys: Option<&[&str]>,
    also_known_as: Option<&[&str]>,
    verification_methods: Option<&Record>,
    services: Option<&Record>,
  ) -> Result<ComAtprotoIdentitySignplcoperation> {
    let mut req = self.agent.post(&format!(
      "https://{}/xrpc/com.atproto.identity.signPlcOperation",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut input = serde_json::Map::new();

    if let Some(v) = &token {
      input.insert(String::from("token"), json!(v));
    }

    if let Some(v) = &rotation_keys {
      input.insert(String::from("rotation_keys"), json!(v));
    }

    if let Some(v) = &also_known_as {
      input.insert(String::from("also_known_as"), json!(v));
    }

    if let Some(v) = &verification_methods {
      input.insert(String::from("verification_methods"), json!(v));
    }

    if let Some(v) = &services {
      input.insert(String::from("services"), json!(v));
    }

    Ok(req.send_json(json!(input))?.into_json()?)
  }

  /// Validates a PLC operation to ensure that it doesn&#39;t violate a service&#39;s constraints or get the identity into a bad state, then submits it to the PLC registry

  pub fn com_atproto_identity_submitplcoperation(
    &self,
    operation: &Record,
  ) -> Result<ureq::Response> {
    let mut req = self.agent.post(&format!(
      "https://{}/xrpc/com.atproto.identity.submitPlcOperation",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut input = serde_json::Map::new();

    input.insert(String::from("operation"), json!(operation));

    Ok(req.send_json(json!(input))?)
  }

  /// Updates the current account&#39;s handle. Verifies handle validity, and updates did:plc document if necessary. Implemented by PDS, and requires auth.

  pub fn com_atproto_identity_updatehandle(&self, handle: &str) -> Result<ureq::Response> {
    let mut req = self.agent.post(&format!(
      "https://{}/xrpc/com.atproto.identity.updateHandle",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut input = serde_json::Map::new();

    input.insert(String::from("handle"), json!(handle));

    Ok(req.send_json(json!(input))?)
  }

  /// Submit a moderation report regarding an atproto account or record. Implemented by moderation services (with PDS proxying), and requires auth.

  pub fn com_atproto_moderation_createreport(
    &self,
    reason_type: &ComAtprotoModerationDefsReasontype,
    subject: ComAtprotoModerationCreatereportMainInputSubject,
    reason: Option<&str>,
  ) -> Result<ComAtprotoModerationCreatereport> {
    let mut req = self.agent.post(&format!(
      "https://{}/xrpc/com.atproto.moderation.createReport",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut input = serde_json::Map::new();

    input.insert(String::from("reason_type"), json!(reason_type));

    input.insert(String::from("subject"), json!(subject));

    if let Some(v) = &reason {
      input.insert(String::from("reason"), json!(v));
    }

    Ok(req.send_json(json!(input))?.into_json()?)
  }

  /// Apply a batch transaction of repository creates, updates, and deletes. Requires auth, implemented by PDS.

  pub fn com_atproto_repo_applywrites(
    &self,
    repo: &str,
    writes: &[&ComAtprotoRepoApplywritesMainInputWritesItem],
    validate: Option<bool>,
    swap_commit: Option<&CidString>,
  ) -> Result<ureq::Response> {
    let mut req = self.agent.post(&format!(
      "https://{}/xrpc/com.atproto.repo.applyWrites",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut input = serde_json::Map::new();

    input.insert(String::from("repo"), json!(repo));

    input.insert(String::from("writes"), json!(writes));

    if let Some(v) = &validate {
      input.insert(String::from("validate"), json!(v));
    }

    if let Some(v) = &swap_commit {
      input.insert(String::from("swap_commit"), json!(v));
    }

    Ok(req.send_json(json!(input))?)
  }

  /// Create a single new repository record. Requires auth, implemented by PDS.

  pub fn com_atproto_repo_createrecord(
    &self,
    repo: &str,
    collection: &str,
    record: &Record,
    rkey: Option<&str>,
    validate: Option<bool>,
    swap_commit: Option<&CidString>,
  ) -> Result<ComAtprotoRepoCreaterecord> {
    let mut req = self.agent.post(&format!(
      "https://{}/xrpc/com.atproto.repo.createRecord",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut input = serde_json::Map::new();

    input.insert(String::from("repo"), json!(repo));

    input.insert(String::from("collection"), json!(collection));

    input.insert(String::from("record"), json!(record));

    if let Some(v) = &rkey {
      input.insert(String::from("rkey"), json!(v));
    }

    if let Some(v) = &validate {
      input.insert(String::from("validate"), json!(v));
    }

    if let Some(v) = &swap_commit {
      input.insert(String::from("swap_commit"), json!(v));
    }

    Ok(req.send_json(json!(input))?.into_json()?)
  }

  /// Delete a repository record, or ensure it doesn&#39;t exist. Requires auth, implemented by PDS.

  pub fn com_atproto_repo_deleterecord(
    &self,
    repo: &str,
    collection: &str,
    rkey: &str,
    swap_record: Option<&CidString>,
    swap_commit: Option<&CidString>,
  ) -> Result<ureq::Response> {
    let mut req = self.agent.post(&format!(
      "https://{}/xrpc/com.atproto.repo.deleteRecord",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut input = serde_json::Map::new();

    input.insert(String::from("repo"), json!(repo));

    input.insert(String::from("collection"), json!(collection));

    input.insert(String::from("rkey"), json!(rkey));

    if let Some(v) = &swap_record {
      input.insert(String::from("swap_record"), json!(v));
    }

    if let Some(v) = &swap_commit {
      input.insert(String::from("swap_commit"), json!(v));
    }

    Ok(req.send_json(json!(input))?)
  }

  /// Import a repo in the form of a CAR file. Requires Content-Length HTTP header to be set.

  pub fn com_atproto_repo_importrepo(&self) -> Result<ureq::Response> {
    let mut req = self.agent.post(&format!(
      "https://{}/xrpc/com.atproto.repo.importRepo",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    Ok(req.call()?)
  }

  /// Write a repository record, creating or updating it as needed. Requires auth, implemented by PDS.

  pub fn com_atproto_repo_putrecord(
    &self,
    repo: &str,
    collection: &str,
    rkey: &str,
    record: &Record,
    validate: Option<bool>,
    swap_record: Option<&CidString>,
    swap_commit: Option<&CidString>,
  ) -> Result<ComAtprotoRepoPutrecord> {
    let mut req = self.agent.post(&format!(
      "https://{}/xrpc/com.atproto.repo.putRecord",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut input = serde_json::Map::new();

    input.insert(String::from("repo"), json!(repo));

    input.insert(String::from("collection"), json!(collection));

    input.insert(String::from("rkey"), json!(rkey));

    input.insert(String::from("record"), json!(record));

    if let Some(v) = &validate {
      input.insert(String::from("validate"), json!(v));
    }

    if let Some(v) = &swap_record {
      input.insert(String::from("swap_record"), json!(v));
    }

    if let Some(v) = &swap_commit {
      input.insert(String::from("swap_commit"), json!(v));
    }

    Ok(req.send_json(json!(input))?.into_json()?)
  }

  /// Upload a new blob, to be referenced from a repository record. The blob will be deleted if it is not referenced within a time window (eg, minutes). Blob restrictions (mimetype, size, etc) are enforced when the reference is created. Requires auth, implemented by PDS.

  pub fn com_atproto_repo_uploadblob(
    &self,
    bytes: &[u8],
    content_type: &str,
  ) -> Result<ComAtprotoRepoUploadblob> {
    let mut req = self.agent.post(&format!(
      "https://{}/xrpc/com.atproto.repo.uploadBlob",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    req = req.set("Content-Type", content_type);

    Ok(req.send_bytes(bytes)?.into_json()?)
  }

  /// Activates a currently deactivated account. Used to finalize account migration after the account&#39;s repo is imported and identity is setup.

  pub fn com_atproto_server_activateaccount(&self) -> Result<ureq::Response> {
    let mut req = self.agent.post(&format!(
      "https://{}/xrpc/com.atproto.server.activateAccount",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    Ok(req.call()?)
  }

  /// Confirm an email using a token from com.atproto.server.requestEmailConfirmation.

  pub fn com_atproto_server_confirmemail(
    &self,
    email: &str,
    token: &str,
  ) -> Result<ureq::Response> {
    let mut req = self.agent.post(&format!(
      "https://{}/xrpc/com.atproto.server.confirmEmail",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut input = serde_json::Map::new();

    input.insert(String::from("email"), json!(email));

    input.insert(String::from("token"), json!(token));

    Ok(req.send_json(json!(input))?)
  }

  /// Create an account. Implemented by PDS.

  pub fn com_atproto_server_createaccount(
    &self,
    handle: &str,
    email: Option<&str>,
    did: Option<&str>,
    invite_code: Option<&str>,
    verification_code: Option<&str>,
    verification_phone: Option<&str>,
    password: Option<&str>,
    recovery_key: Option<&str>,
    plc_op: Option<&Record>,
  ) -> Result<ComAtprotoServerCreateaccount> {
    let mut req = self.agent.post(&format!(
      "https://{}/xrpc/com.atproto.server.createAccount",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut input = serde_json::Map::new();

    input.insert(String::from("handle"), json!(handle));

    if let Some(v) = &email {
      input.insert(String::from("email"), json!(v));
    }

    if let Some(v) = &did {
      input.insert(String::from("did"), json!(v));
    }

    if let Some(v) = &invite_code {
      input.insert(String::from("invite_code"), json!(v));
    }

    if let Some(v) = &verification_code {
      input.insert(String::from("verification_code"), json!(v));
    }

    if let Some(v) = &verification_phone {
      input.insert(String::from("verification_phone"), json!(v));
    }

    if let Some(v) = &password {
      input.insert(String::from("password"), json!(v));
    }

    if let Some(v) = &recovery_key {
      input.insert(String::from("recovery_key"), json!(v));
    }

    if let Some(v) = &plc_op {
      input.insert(String::from("plc_op"), json!(v));
    }

    Ok(req.send_json(json!(input))?.into_json()?)
  }

  /// Create an App Password.

  pub fn com_atproto_server_createapppassword(
    &self,
    name: &str,
  ) -> Result<ComAtprotoServerCreateapppasswordApppassword> {
    let mut req = self.agent.post(&format!(
      "https://{}/xrpc/com.atproto.server.createAppPassword",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut input = serde_json::Map::new();

    input.insert(String::from("name"), json!(name));

    Ok(req.send_json(json!(input))?.into_json()?)
  }

  /// Create an invite code.

  pub fn com_atproto_server_createinvitecode(
    &self,
    use_count: i64,
    for_account: Option<&str>,
  ) -> Result<ComAtprotoServerCreateinvitecode> {
    let mut req = self.agent.post(&format!(
      "https://{}/xrpc/com.atproto.server.createInviteCode",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut input = serde_json::Map::new();

    input.insert(String::from("use_count"), json!(use_count));

    if let Some(v) = &for_account {
      input.insert(String::from("for_account"), json!(v));
    }

    Ok(req.send_json(json!(input))?.into_json()?)
  }

  /// Create invite codes.

  pub fn com_atproto_server_createinvitecodes(
    &self,
    code_count: i64,
    use_count: i64,
    for_accounts: Option<&[&str]>,
  ) -> Result<ComAtprotoServerCreateinvitecodes> {
    let mut req = self.agent.post(&format!(
      "https://{}/xrpc/com.atproto.server.createInviteCodes",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut input = serde_json::Map::new();

    input.insert(String::from("code_count"), json!(code_count));

    input.insert(String::from("use_count"), json!(use_count));

    if let Some(v) = &for_accounts {
      input.insert(String::from("for_accounts"), json!(v));
    }

    Ok(req.send_json(json!(input))?.into_json()?)
  }

  /// Create an authentication session.

  pub fn com_atproto_server_createsession(
    &self,
    identifier: &str,
    password: &str,
  ) -> Result<ComAtprotoServerCreatesession> {
    let mut req = self.agent.post(&format!(
      "https://{}/xrpc/com.atproto.server.createSession",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut input = serde_json::Map::new();

    input.insert(String::from("identifier"), json!(identifier));

    input.insert(String::from("password"), json!(password));

    Ok(req.send_json(json!(input))?.into_json()?)
  }

  /// Deactivates a currently active account. Stops serving of repo, and future writes to repo until reactivated. Used to finalize account migration with the old host after the account has been activated on the new host.

  pub fn com_atproto_server_deactivateaccount(
    &self,
    delete_after: Option<&DateTime<Utc>>,
  ) -> Result<ureq::Response> {
    let mut req = self.agent.post(&format!(
      "https://{}/xrpc/com.atproto.server.deactivateAccount",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut input = serde_json::Map::new();

    if let Some(v) = &delete_after {
      input.insert(String::from("delete_after"), json!(v));
    }

    Ok(req.send_json(json!(input))?)
  }

  /// Delete an actor&#39;s account with a token and password. Can only be called after requesting a deletion token. Requires auth.

  pub fn com_atproto_server_deleteaccount(
    &self,
    did: &str,
    password: &str,
    token: &str,
  ) -> Result<ureq::Response> {
    let mut req = self.agent.post(&format!(
      "https://{}/xrpc/com.atproto.server.deleteAccount",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut input = serde_json::Map::new();

    input.insert(String::from("did"), json!(did));

    input.insert(String::from("password"), json!(password));

    input.insert(String::from("token"), json!(token));

    Ok(req.send_json(json!(input))?)
  }

  /// Delete the current session. Requires auth.

  pub fn com_atproto_server_deletesession(&self) -> Result<ureq::Response> {
    let mut req = self.agent.post(&format!(
      "https://{}/xrpc/com.atproto.server.deleteSession",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    Ok(req.call()?)
  }

  /// Refresh an authentication session. Requires auth using the &#39;refreshJwt&#39; (not the &#39;accessJwt&#39;).

  pub fn com_atproto_server_refreshsession(&self) -> Result<ComAtprotoServerRefreshsession> {
    let mut req = self.agent.post(&format!(
      "https://{}/xrpc/com.atproto.server.refreshSession",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    Ok(req.call()?.into_json()?)
  }

  /// Initiate a user account deletion via email.

  pub fn com_atproto_server_requestaccountdelete(&self) -> Result<ureq::Response> {
    let mut req = self.agent.post(&format!(
      "https://{}/xrpc/com.atproto.server.requestAccountDelete",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    Ok(req.call()?)
  }

  /// Request an email with a code to confirm ownership of email.

  pub fn com_atproto_server_requestemailconfirmation(&self) -> Result<ureq::Response> {
    let mut req = self.agent.post(&format!(
      "https://{}/xrpc/com.atproto.server.requestEmailConfirmation",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    Ok(req.call()?)
  }

  /// Request a token in order to update email.

  pub fn com_atproto_server_requestemailupdate(
    &self,
  ) -> Result<ComAtprotoServerRequestemailupdate> {
    let mut req = self.agent.post(&format!(
      "https://{}/xrpc/com.atproto.server.requestEmailUpdate",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    Ok(req.call()?.into_json()?)
  }

  /// Initiate a user account password reset via email.

  pub fn com_atproto_server_requestpasswordreset(&self, email: &str) -> Result<ureq::Response> {
    let mut req = self.agent.post(&format!(
      "https://{}/xrpc/com.atproto.server.requestPasswordReset",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut input = serde_json::Map::new();

    input.insert(String::from("email"), json!(email));

    Ok(req.send_json(json!(input))?)
  }

  /// Reserve a repo signing key, for use with account creation. Necessary so that a DID PLC update operation can be constructed during an account migraiton. Public and does not require auth; implemented by PDS. NOTE: this endpoint may change when full account migration is implemented.

  pub fn com_atproto_server_reservesigningkey(
    &self,
    did: Option<&str>,
  ) -> Result<ComAtprotoServerReservesigningkey> {
    let mut req = self.agent.post(&format!(
      "https://{}/xrpc/com.atproto.server.reserveSigningKey",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut input = serde_json::Map::new();

    if let Some(v) = &did {
      input.insert(String::from("did"), json!(v));
    }

    Ok(req.send_json(json!(input))?.into_json()?)
  }

  /// Reset a user account password using a token.

  pub fn com_atproto_server_resetpassword(
    &self,
    token: &str,
    password: &str,
  ) -> Result<ureq::Response> {
    let mut req = self.agent.post(&format!(
      "https://{}/xrpc/com.atproto.server.resetPassword",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut input = serde_json::Map::new();

    input.insert(String::from("token"), json!(token));

    input.insert(String::from("password"), json!(password));

    Ok(req.send_json(json!(input))?)
  }

  /// Revoke an App Password by name.

  pub fn com_atproto_server_revokeapppassword(&self, name: &str) -> Result<ureq::Response> {
    let mut req = self.agent.post(&format!(
      "https://{}/xrpc/com.atproto.server.revokeAppPassword",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut input = serde_json::Map::new();

    input.insert(String::from("name"), json!(name));

    Ok(req.send_json(json!(input))?)
  }

  /// Update an account&#39;s email.

  pub fn com_atproto_server_updateemail(
    &self,
    email: &str,
    token: Option<&str>,
  ) -> Result<ureq::Response> {
    let mut req = self.agent.post(&format!(
      "https://{}/xrpc/com.atproto.server.updateEmail",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut input = serde_json::Map::new();

    input.insert(String::from("email"), json!(email));

    if let Some(v) = &token {
      input.insert(String::from("token"), json!(v));
    }

    Ok(req.send_json(json!(input))?)
  }

  /// Notify a crawling service of a recent update, and that crawling should resume. Intended use is after a gap between repo stream events caused the crawling service to disconnect. Does not require auth; implemented by Relay.

  pub fn com_atproto_sync_notifyofupdate(&self, hostname: &str) -> Result<ureq::Response> {
    let mut req = self.agent.post(&format!(
      "https://{}/xrpc/com.atproto.sync.notifyOfUpdate",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut input = serde_json::Map::new();

    input.insert(String::from("hostname"), json!(hostname));

    Ok(req.send_json(json!(input))?)
  }

  /// Request a service to persistently crawl hosted repos. Expected use is new PDS instances declaring their existence to Relays. Does not require auth.

  pub fn com_atproto_sync_requestcrawl(&self, hostname: &str) -> Result<ureq::Response> {
    let mut req = self.agent.post(&format!(
      "https://{}/xrpc/com.atproto.sync.requestCrawl",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut input = serde_json::Map::new();

    input.insert(String::from("hostname"), json!(hostname));

    Ok(req.send_json(json!(input))?)
  }

  /// Request a verification code to be sent to the supplied phone number

  pub fn com_atproto_temp_requestphoneverification(
    &self,
    phone_number: &str,
  ) -> Result<ureq::Response> {
    let mut req = self.agent.post(&format!(
      "https://{}/xrpc/com.atproto.temp.requestPhoneVerification",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut input = serde_json::Map::new();

    input.insert(String::from("phone_number"), json!(phone_number));

    Ok(req.send_json(json!(input))?)
  }

  /// Subscribe to stream of labels (and negations). Public endpoint implemented by mod services. Uses same sequencing scheme as repo event stream.
  pub fn com_atproto_label_subscribelabels(
    &self,
    cursor: Option<i64>,
  ) -> Result<WebSocket<MaybeTlsStream<TcpStream>>> {
    let mut url = Url::parse(&format!(
      "wss://{}/xrpc/com.atproto.label.subscribeLabels",
      self.bgs_host
    ))?;

    let mut query = Vec::new();

    let cursor_value = serde_json::to_string(&cursor)?;

    if cursor.is_some() {
      query.push(("cursor", cursor_value.as_str()));
    }

    url.query_pairs_mut().extend_pairs(query);

    Ok(tungstenite::connect(&url)?.0)
  }

  /// Repository event stream, aka Firehose endpoint. Outputs repo commits with diff data, and identity update events, for all repositories on the current server. See the atproto specifications for details around stream sequencing, repo versioning, CAR diff format, and more. Public and does not require auth; implemented by PDS and Relay.
  pub fn com_atproto_sync_subscriberepos(
    &self,
    cursor: Option<i64>,
  ) -> Result<WebSocket<MaybeTlsStream<TcpStream>>> {
    let mut url = Url::parse(&format!(
      "wss://{}/xrpc/com.atproto.sync.subscribeRepos",
      self.bgs_host
    ))?;

    let mut query = Vec::new();

    let cursor_value = serde_json::to_string(&cursor)?;

    if cursor.is_some() {
      query.push(("cursor", cursor_value.as_str()));
    }

    url.query_pairs_mut().extend_pairs(query);

    Ok(tungstenite::connect(&url)?.0)
  }
}
