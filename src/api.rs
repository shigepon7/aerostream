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
use serde_json::json;
use serde_with::skip_serializing_none;
use tungstenite::{stream::MaybeTlsStream, WebSocket};
use ureq::{Agent, AgentBuilder, Proxy};
use url::Url;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyGraphDefsListpurpose(String);

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoAdminDefsActiontype(String);

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoModerationDefsReasontype(String);

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyActorDefsPreferences(Vec<AppBskyActorDefsPreferencesItem>);

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyActorDefsProfileview {
  pub did: String,
  pub handle: String,
  pub display_name: Option<String>,
  pub description: Option<String>,
  pub avatar: Option<String>,
  pub indexed_at: Option<DateTime<Utc>>,
  pub viewer: Option<AppBskyActorDefsViewerstate>,
  pub labels: Option<Vec<ComAtprotoLabelDefsLabel>>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyActorDefsViewerstate {
  pub muted: Option<bool>,
  pub muted_by_list: Option<AppBskyGraphDefsListviewbasic>,
  pub blocked_by: Option<bool>,
  pub blocking: Option<String>,
  pub following: Option<String>,
  pub followed_by: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyActorDefsContentlabelpref {
  pub label: String,
  pub visibility: String,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyActorDefsSavedfeedspref {
  pub pinned: Vec<String>,
  pub saved: Vec<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyActorDefsProfileviewbasic {
  pub did: String,
  pub handle: String,
  pub display_name: Option<String>,
  pub avatar: Option<String>,
  pub viewer: Option<AppBskyActorDefsViewerstate>,
  pub labels: Option<Vec<ComAtprotoLabelDefsLabel>>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyActorDefsProfileviewdetailed {
  pub did: String,
  pub handle: String,
  pub display_name: Option<String>,
  pub description: Option<String>,
  pub avatar: Option<String>,
  pub banner: Option<String>,
  pub followers_count: Option<i64>,
  pub follows_count: Option<i64>,
  pub posts_count: Option<i64>,
  pub indexed_at: Option<DateTime<Utc>>,
  pub viewer: Option<AppBskyActorDefsViewerstate>,
  pub labels: Option<Vec<ComAtprotoLabelDefsLabel>>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyActorDefsAdultcontentpref {
  pub enabled: bool,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyEmbedExternalViewexternal {
  pub uri: String,
  pub title: String,
  pub description: String,
  pub thumb: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyEmbedExternalView {
  pub external: AppBskyEmbedExternalViewexternal,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyEmbedExternal {
  pub external: AppBskyEmbedExternalExternal,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyEmbedExternalExternal {
  pub uri: String,
  pub title: String,
  pub description: String,
  pub thumb: Option<Blob>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyEmbedImagesViewimage {
  pub thumb: String,
  pub fullsize: String,
  pub alt: String,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyEmbedImagesView {
  pub images: Vec<AppBskyEmbedImagesViewimage>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyEmbedImages {
  pub images: Vec<AppBskyEmbedImagesImage>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyEmbedImagesImage {
  pub image: Blob,
  pub alt: String,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyEmbedRecord {
  pub record: ComAtprotoRepoStrongref,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyEmbedRecordView {
  pub record: AppBskyEmbedRecordViewRecord,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyEmbedRecordViewnotfound {
  pub uri: String,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyEmbedRecordViewblocked {
  pub uri: String,
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
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyEmbedRecordwithmedia {
  pub record: AppBskyEmbedRecord,
  pub media: AppBskyEmbedRecordwithmediaMainMedia,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyEmbedRecordwithmediaView {
  pub record: AppBskyEmbedRecordView,
  pub media: AppBskyEmbedRecordwithmediaViewMedia,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyFeedDefsViewerstate {
  pub repost: Option<String>,
  pub like: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyFeedDefsSkeletonreasonrepost {
  pub repost: String,
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
  pub description_facets: Option<Vec<AppBskyRichtextFacet>>,
  pub avatar: Option<String>,
  pub like_count: Option<i64>,
  pub viewer: Option<AppBskyFeedDefsGeneratorviewerstate>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyFeedDefsFeedviewpost {
  pub post: AppBskyFeedDefsPostview,
  pub reply: Option<AppBskyFeedDefsReplyref>,
  pub reason: Option<AppBskyFeedDefsFeedviewpostReason>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyFeedDefsGeneratorviewerstate {
  pub like: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyFeedDefsNotfoundpost {
  pub uri: String,
  #[serde(rename = "notFound")]
  pub not_found: bool,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyFeedDefsSkeletonfeedpost {
  pub post: String,
  pub reason: Option<AppBskyFeedDefsSkeletonfeedpostReason>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyFeedDefsReasonrepost {
  pub by: AppBskyActorDefsProfileviewbasic,
  #[serde(rename = "indexedAt")]
  pub indexed_at: DateTime<Utc>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyFeedDefsThreadviewpost {
  pub post: AppBskyFeedDefsPostview,
  pub parent: Option<AppBskyFeedDefsThreadviewpostParent>,
  pub replies: Option<Vec<AppBskyFeedDefsThreadviewpostRepliesItem>>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyFeedDefsBlockedpost {
  pub uri: String,
  pub blocked: bool,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyFeedDefsReplyref {
  pub root: AppBskyFeedDefsReplyrefRoot,
  pub parent: AppBskyFeedDefsReplyrefParent,
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
  pub reply_count: Option<i64>,
  pub repost_count: Option<i64>,
  pub like_count: Option<i64>,
  pub viewer: Option<AppBskyFeedDefsViewerstate>,
  pub labels: Option<Vec<ComAtprotoLabelDefsLabel>>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyFeedDescribefeedgeneratorFeed {
  pub uri: String,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyFeedDescribefeedgeneratorLinks {
  pub privacy_policy: Option<String>,
  pub terms_of_service: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyFeedGetlikesLike {
  #[serde(rename = "indexedAt")]
  pub indexed_at: DateTime<Utc>,
  #[serde(rename = "createdAt")]
  pub created_at: DateTime<Utc>,
  pub actor: AppBskyActorDefsProfileview,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyFeedPostReplyref {
  pub root: ComAtprotoRepoStrongref,
  pub parent: ComAtprotoRepoStrongref,
}

/// Deprecated: use facets instead.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyFeedPostEntity {
  pub index: AppBskyFeedPostTextslice,
  #[serde(rename = "type")]
  pub value_type: String,
  pub value: String,
}

/// Deprecated. Use app.bsky.richtext instead -- A text segment. Start is inclusive, end is exclusive. Indices are for utf16-encoded strings.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyFeedPostTextslice {
  pub start: i64,
  pub end: i64,
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
  pub indexed_at: Option<DateTime<Utc>>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyGraphDefsListitemview {
  pub subject: AppBskyActorDefsProfileview,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyGraphDefsListviewerstate {
  pub muted: Option<bool>,
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
  pub description_facets: Option<Vec<AppBskyRichtextFacet>>,
  pub avatar: Option<String>,
  pub viewer: Option<AppBskyGraphDefsListviewerstate>,
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
  pub reason_subject: Option<String>,
  pub labels: Option<Vec<ComAtprotoLabelDefsLabel>>,
}

/// A facet feature for actor mentions.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyRichtextFacetMention {
  pub did: String,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyRichtextFacet {
  pub index: AppBskyRichtextFacetByteslice,
  pub features: Vec<AppBskyRichtextFacetMainFeaturesItem>,
}

/// A text segment. Start is inclusive, end is exclusive. Indices are for utf8-encoded strings.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyRichtextFacetByteslice {
  #[serde(rename = "byteStart")]
  pub byte_start: i64,
  #[serde(rename = "byteEnd")]
  pub byte_end: i64,
}

/// A facet feature for links.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyRichtextFacetLink {
  pub uri: String,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoAdminDefsReporef {
  pub did: String,
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
  pub resolved_by_actions: Vec<ComAtprotoAdminDefsActionview>,
  pub reason: Option<String>,
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
  pub invited_by: Option<ComAtprotoServerDefsInvitecode>,
  pub invites: Option<Vec<ComAtprotoServerDefsInvitecode>>,
  pub invites_disabled: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoAdminDefsImagedetails {
  pub width: i64,
  pub height: i64,
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
  pub reason: Option<String>,
  pub subject_repo_handle: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoAdminDefsActionviewdetail {
  pub id: i64,
  pub action: ComAtprotoAdminDefsActiontype,
  pub subject: ComAtprotoAdminDefsActionviewdetailSubject,
  #[serde(rename = "subjectBlobs")]
  pub subject_blobs: Vec<ComAtprotoAdminDefsBlobview>,
  pub reason: String,
  #[serde(rename = "createdBy")]
  pub created_by: String,
  #[serde(rename = "createdAt")]
  pub created_at: DateTime<Utc>,
  #[serde(rename = "resolvedReports")]
  pub resolved_reports: Vec<ComAtprotoAdminDefsReportview>,
  pub create_label_vals: Option<Vec<String>>,
  pub negate_label_vals: Option<Vec<String>>,
  pub reversal: Option<ComAtprotoAdminDefsActionreversal>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoAdminDefsVideodetails {
  pub width: i64,
  pub height: i64,
  pub length: i64,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoAdminDefsRepoviewnotfound {
  pub did: String,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoAdminDefsRecordviewnotfound {
  pub uri: String,
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
  pub invited_by: Option<ComAtprotoServerDefsInvitecode>,
  pub invites_disabled: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoAdminDefsActionviewcurrent {
  pub id: i64,
  pub action: ComAtprotoAdminDefsActiontype,
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
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoAdminDefsModeration {
  pub current_action: Option<ComAtprotoAdminDefsActionviewcurrent>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoAdminDefsModerationdetail {
  pub actions: Vec<ComAtprotoAdminDefsActionview>,
  pub reports: Vec<ComAtprotoAdminDefsReportview>,
  pub current_action: Option<ComAtprotoAdminDefsActionviewcurrent>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoAdminDefsActionreversal {
  pub reason: String,
  #[serde(rename = "createdBy")]
  pub created_by: String,
  #[serde(rename = "createdAt")]
  pub created_at: DateTime<Utc>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoAdminDefsActionview {
  pub id: i64,
  pub action: ComAtprotoAdminDefsActiontype,
  pub subject: ComAtprotoAdminDefsActionviewSubject,
  #[serde(rename = "subjectBlobCids")]
  pub subject_blob_cids: Vec<String>,
  pub reason: String,
  #[serde(rename = "createdBy")]
  pub created_by: String,
  #[serde(rename = "createdAt")]
  pub created_at: DateTime<Utc>,
  #[serde(rename = "resolvedReportIds")]
  pub resolved_report_ids: Vec<i64>,
  pub create_label_vals: Option<Vec<String>>,
  pub negate_label_vals: Option<Vec<String>>,
  pub reversal: Option<ComAtprotoAdminDefsActionreversal>,
}

/// Metadata tag on an atproto resource (eg, repo or record)
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoLabelDefsLabel {
  pub src: String,
  pub uri: String,
  pub val: String,
  pub cts: DateTime<Utc>,
  pub cid: Option<CidString>,
  pub neg: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoLabelSubscribelabelsLabels {
  pub seq: i64,
  pub labels: Vec<ComAtprotoLabelDefsLabel>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoLabelSubscribelabelsInfo {
  pub name: String,
  pub message: Option<String>,
}

/// Create a new record.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoRepoApplywritesCreate {
  pub collection: String,
  pub value: Record,
  pub rkey: Option<String>,
}

/// Update an existing record.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoRepoApplywritesUpdate {
  pub collection: String,
  pub rkey: String,
  pub value: Record,
}

/// Delete an existing record.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoRepoApplywritesDelete {
  pub collection: String,
  pub rkey: String,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoRepoListrecordsRecord {
  pub uri: String,
  pub cid: CidString,
  pub value: Record,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoRepoStrongref {
  pub uri: String,
  pub cid: CidString,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoServerCreateapppasswordApppassword {
  pub name: String,
  pub password: String,
  #[serde(rename = "createdAt")]
  pub created_at: DateTime<Utc>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoServerCreateinvitecodesAccountcodes {
  pub account: String,
  pub codes: Vec<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoServerDefsInvitecodeuse {
  #[serde(rename = "usedBy")]
  pub used_by: String,
  #[serde(rename = "usedAt")]
  pub used_at: DateTime<Utc>,
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
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoServerDescribeserverLinks {
  pub privacy_policy: Option<String>,
  pub terms_of_service: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoServerListapppasswordsApppassword {
  pub name: String,
  #[serde(rename = "createdAt")]
  pub created_at: DateTime<Utc>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoSyncListreposRepo {
  pub did: String,
  pub head: CidString,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoSyncSubscribereposMigrate {
  pub seq: i64,
  pub did: String,
  #[serde(rename = "migrateTo")]
  pub migrate_to: String,
  pub time: DateTime<Utc>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoSyncSubscribereposTombstone {
  pub seq: i64,
  pub did: String,
  pub time: DateTime<Utc>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoSyncSubscribereposCommit {
  pub seq: i64,
  pub rebase: bool,
  #[serde(rename = "tooBig")]
  pub too_big: bool,
  pub repo: String,
  pub commit: String,
  pub prev: String,
  pub blocks: Vec<u8>,
  pub ops: Vec<ComAtprotoSyncSubscribereposRepoop>,
  pub blobs: Vec<String>,
  pub time: DateTime<Utc>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoSyncSubscribereposHandle {
  pub seq: i64,
  pub did: String,
  pub handle: String,
  pub time: DateTime<Utc>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoSyncSubscribereposInfo {
  pub name: String,
  pub message: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoSyncSubscribereposRepoop {
  pub action: String,
  pub path: String,
  pub cid: String,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyActorProfile {
  pub display_name: Option<String>,
  pub description: Option<String>,
  pub avatar: Option<Blob>,
  pub banner: Option<Blob>,
}

/// A declaration of the existence of a feed generator
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyFeedGenerator {
  pub did: String,
  #[serde(rename = "displayName")]
  pub display_name: String,
  #[serde(rename = "createdAt")]
  pub created_at: DateTime<Utc>,
  pub description: Option<String>,
  pub description_facets: Option<Vec<AppBskyRichtextFacet>>,
  pub avatar: Option<Blob>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyFeedLike {
  pub subject: ComAtprotoRepoStrongref,
  #[serde(rename = "createdAt")]
  pub created_at: DateTime<Utc>,
}

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
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyFeedRepost {
  pub subject: ComAtprotoRepoStrongref,
  #[serde(rename = "createdAt")]
  pub created_at: DateTime<Utc>,
}

/// A block.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyGraphBlock {
  pub subject: String,
  #[serde(rename = "createdAt")]
  pub created_at: DateTime<Utc>,
}

/// A social follow.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyGraphFollow {
  pub subject: String,
  #[serde(rename = "createdAt")]
  pub created_at: DateTime<Utc>,
}

/// A declaration of a list of actors.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyGraphList {
  pub purpose: AppBskyGraphDefsListpurpose,
  pub name: String,
  #[serde(rename = "createdAt")]
  pub created_at: DateTime<Utc>,
  pub description: Option<String>,
  pub description_facets: Option<Vec<AppBskyRichtextFacet>>,
  pub avatar: Option<Blob>,
}

/// An item under a declared list of actors
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

  #[serde(rename = "app.bsky.graph.block")]
  AppBskyGraphBlock(AppBskyGraphBlock),

  #[serde(rename = "app.bsky.graph.follow")]
  AppBskyGraphFollow(AppBskyGraphFollow),

  #[serde(rename = "app.bsky.graph.list")]
  AppBskyGraphList(AppBskyGraphList),

  #[serde(rename = "app.bsky.graph.listitem")]
  AppBskyGraphListitem(AppBskyGraphListitem),
}

impl Default for Record {
  fn default() -> Self {
    Self::AppBskyFeedPost(AppBskyFeedPost::default())
  }
}

fn ipld_to_string(ipld: &Ipld) -> String {
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
    Ipld::String(s) => format!("\"{}\"", s),
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

  pub fn as_app_bsky_graph_listitem(&self) -> Option<&AppBskyGraphListitem> {
    match self {
      Self::AppBskyGraphListitem(v) => Some(v),
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
  version: i64,
  prev: Option<Link>,
  data: Link,
  sig: Vec<u8>,
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
  pub service: Vec<HashMap<String, String>>,
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
  pub blob_type: String,
  #[serde(rename = "ref")]
  pub blob_ref: Link,
  #[serde(rename = "mimeType")]
  pub mime_type: String,
  pub size: i64,
}

impl Default for Blob {
  fn default() -> Self {
    Self {
      blob_type: String::from("blob"),
      blob_ref: Link::default(),
      mime_type: String::new(),
      size: 0,
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyActorGetpreferences {
  pub preferences: AppBskyActorDefsPreferences,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyActorGetprofiles {
  pub profiles: Vec<AppBskyActorDefsProfileviewdetailed>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyActorGetsuggestions {
  pub actors: Vec<AppBskyActorDefsProfileview>,
  pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyActorSearchactors {
  pub actors: Vec<AppBskyActorDefsProfileview>,
  pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyActorSearchactorstypeahead {
  pub actors: Vec<AppBskyActorDefsProfileviewbasic>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyFeedDescribefeedgenerator {
  pub did: String,
  pub feeds: Vec<AppBskyFeedDescribefeedgeneratorFeed>,
  pub links: Option<AppBskyFeedDescribefeedgeneratorLinks>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyFeedGetactorfeeds {
  pub feeds: Vec<AppBskyFeedDefsGeneratorview>,
  pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyFeedGetauthorfeed {
  pub feed: Vec<AppBskyFeedDefsFeedviewpost>,
  pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyFeedGetfeed {
  pub feed: Vec<AppBskyFeedDefsFeedviewpost>,
  pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyFeedGetfeedgenerator {
  pub view: AppBskyFeedDefsGeneratorview,
  #[serde(rename = "isOnline")]
  pub is_online: bool,
  #[serde(rename = "isValid")]
  pub is_valid: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyFeedGetfeedgenerators {
  pub feeds: Vec<AppBskyFeedDefsGeneratorview>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyFeedGetfeedskeleton {
  pub feed: Vec<AppBskyFeedDefsSkeletonfeedpost>,
  pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyFeedGetlikes {
  pub uri: String,
  pub likes: Vec<AppBskyFeedGetlikesLike>,
  pub cid: Option<CidString>,
  pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyFeedGetpostthread {
  pub thread: AppBskyFeedGetpostthreadMainOutputThread,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyFeedGetposts {
  pub posts: Vec<AppBskyFeedDefsPostview>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyFeedGetrepostedby {
  pub uri: String,
  #[serde(rename = "repostedBy")]
  pub reposted_by: Vec<AppBskyActorDefsProfileview>,
  pub cid: Option<CidString>,
  pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyFeedGettimeline {
  pub feed: Vec<AppBskyFeedDefsFeedviewpost>,
  pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyGraphGetblocks {
  pub blocks: Vec<AppBskyActorDefsProfileview>,
  pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyGraphGetfollowers {
  pub subject: AppBskyActorDefsProfileview,
  pub followers: Vec<AppBskyActorDefsProfileview>,
  pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyGraphGetfollows {
  pub subject: AppBskyActorDefsProfileview,
  pub follows: Vec<AppBskyActorDefsProfileview>,
  pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyGraphGetlist {
  pub list: AppBskyGraphDefsListview,
  pub items: Vec<AppBskyGraphDefsListitemview>,
  pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyGraphGetlistmutes {
  pub lists: Vec<AppBskyGraphDefsListview>,
  pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyGraphGetlists {
  pub lists: Vec<AppBskyGraphDefsListview>,
  pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyGraphGetmutes {
  pub mutes: Vec<AppBskyActorDefsProfileview>,
  pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyNotificationGetunreadcount {
  pub count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyNotificationListnotifications {
  pub notifications: Vec<AppBskyNotificationListnotificationsNotification>,
  pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyUnspeccedGetpopular {
  pub feed: Vec<AppBskyFeedDefsFeedviewpost>,
  pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyUnspeccedGetpopularfeedgenerators {
  pub feeds: Vec<AppBskyFeedDefsGeneratorview>,
  pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyUnspeccedGettimelineskeleton {
  pub feed: Vec<AppBskyFeedDefsSkeletonfeedpost>,
  pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoAdminGetinvitecodes {
  pub codes: Vec<ComAtprotoServerDefsInvitecode>,
  pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoAdminGetmoderationactions {
  pub actions: Vec<ComAtprotoAdminDefsActionview>,
  pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoAdminGetmoderationreports {
  pub reports: Vec<ComAtprotoAdminDefsReportview>,
  pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoAdminSearchrepos {
  pub repos: Vec<ComAtprotoAdminDefsRepoview>,
  pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoIdentityResolvehandle {
  pub did: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoLabelQuerylabels {
  pub labels: Vec<ComAtprotoLabelDefsLabel>,
  pub cursor: Option<String>,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoRepoGetrecord {
  pub uri: String,
  pub value: Record,
  pub cid: Option<CidString>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoRepoListrecords {
  pub records: Vec<ComAtprotoRepoListrecordsRecord>,
  pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoServerDescribeserver {
  #[serde(rename = "availableUserDomains")]
  pub available_user_domains: Vec<String>,
  pub invite_code_required: Option<bool>,
  pub links: Option<ComAtprotoServerDescribeserverLinks>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoServerGetaccountinvitecodes {
  pub codes: Vec<ComAtprotoServerDefsInvitecode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoServerGetsession {
  pub handle: String,
  pub did: String,
  pub email: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoServerListapppasswords {
  pub passwords: Vec<ComAtprotoServerListapppasswordsApppassword>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoSyncGetcommitpath {
  pub commits: Vec<CidString>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoSyncGethead {
  pub root: CidString,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoSyncListblobs {
  pub cids: Vec<CidString>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoSyncListrepos {
  pub repos: Vec<ComAtprotoSyncListreposRepo>,
  pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoAdminSendemail {
  pub sent: bool,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoRepoCreaterecord {
  pub uri: String,
  pub cid: CidString,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoRepoPutrecord {
  pub uri: String,
  pub cid: CidString,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoRepoUploadblob {
  pub blob: Blob,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoServerCreateaccount {
  #[serde(rename = "accessJwt")]
  pub access_jwt: String,
  #[serde(rename = "refreshJwt")]
  pub refresh_jwt: String,
  pub handle: String,
  pub did: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoServerCreateinvitecode {
  pub code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoServerCreateinvitecodes {
  pub codes: Vec<ComAtprotoServerCreateinvitecodesAccountcodes>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoServerCreatesession {
  #[serde(rename = "accessJwt")]
  pub access_jwt: String,
  #[serde(rename = "refreshJwt")]
  pub refresh_jwt: String,
  pub handle: String,
  pub did: String,
  pub email: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoServerRefreshsession {
  #[serde(rename = "accessJwt")]
  pub access_jwt: String,
  #[serde(rename = "refreshJwt")]
  pub refresh_jwt: String,
  pub handle: String,
  pub did: String,
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
}

impl Default for AppBskyActorDefsPreferencesItem {
  fn default() -> Self {
    Self::AppBskyActorDefsAdultcontentpref(Box::new(AppBskyActorDefsAdultcontentpref::default()))
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
}

impl Default for AppBskyEmbedRecordwithmediaViewMedia {
  fn default() -> Self {
    Self::AppBskyEmbedImagesView(Box::new(AppBskyEmbedImagesView::default()))
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "$type")]
pub enum AppBskyFeedDefsFeedviewpostReason {
  #[serde(rename = "app.bsky.feed.defs#reasonRepost")]
  AppBskyFeedDefsReasonrepost(Box<AppBskyFeedDefsReasonrepost>),
}

impl Default for AppBskyFeedDefsFeedviewpostReason {
  fn default() -> Self {
    Self::AppBskyFeedDefsReasonrepost(Box::new(AppBskyFeedDefsReasonrepost::default()))
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "$type")]
pub enum AppBskyFeedDefsSkeletonfeedpostReason {
  #[serde(rename = "app.bsky.feed.defs#skeletonReasonRepost")]
  AppBskyFeedDefsSkeletonreasonrepost(Box<AppBskyFeedDefsSkeletonreasonrepost>),
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
pub enum AppBskyFeedDefsThreadviewpostParent {
  #[serde(rename = "app.bsky.feed.defs#threadViewPost")]
  AppBskyFeedDefsThreadviewpost(Box<AppBskyFeedDefsThreadviewpost>),
  #[serde(rename = "app.bsky.feed.defs#notFoundPost")]
  AppBskyFeedDefsNotfoundpost(Box<AppBskyFeedDefsNotfoundpost>),
  #[serde(rename = "app.bsky.feed.defs#blockedPost")]
  AppBskyFeedDefsBlockedpost(Box<AppBskyFeedDefsBlockedpost>),
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
}

impl Default for AppBskyFeedDefsThreadviewpostRepliesItem {
  fn default() -> Self {
    Self::AppBskyFeedDefsThreadviewpost(Box::new(AppBskyFeedDefsThreadviewpost::default()))
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
}

impl Default for AppBskyFeedDefsReplyrefParent {
  fn default() -> Self {
    Self::AppBskyFeedDefsPostview(Box::new(AppBskyFeedDefsPostview::default()))
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
}

impl Default for AppBskyFeedDefsPostviewEmbed {
  fn default() -> Self {
    Self::AppBskyEmbedImagesView(Box::new(AppBskyEmbedImagesView::default()))
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
}

impl Default for AppBskyFeedPostMainEmbed {
  fn default() -> Self {
    Self::AppBskyEmbedImages(Box::new(AppBskyEmbedImages::default()))
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "$type")]
pub enum AppBskyRichtextFacetMainFeaturesItem {
  #[serde(rename = "app.bsky.richtext.facet#mention")]
  AppBskyRichtextFacetMention(Box<AppBskyRichtextFacetMention>),
  #[serde(rename = "app.bsky.richtext.facet#link")]
  AppBskyRichtextFacetLink(Box<AppBskyRichtextFacetLink>),
}

impl Default for AppBskyRichtextFacetMainFeaturesItem {
  fn default() -> Self {
    Self::AppBskyRichtextFacetMention(Box::new(AppBskyRichtextFacetMention::default()))
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
}

impl Default for ComAtprotoAdminDefsBlobviewDetails {
  fn default() -> Self {
    Self::ComAtprotoAdminDefsImagedetails(Box::new(ComAtprotoAdminDefsImagedetails::default()))
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "$type")]
pub enum ComAtprotoAdminDefsReportviewSubject {
  #[serde(rename = "com.atproto.admin.defs#repoRef")]
  ComAtprotoAdminDefsReporef(Box<ComAtprotoAdminDefsReporef>),
  #[serde(rename = "com.atproto.repo.strongRef")]
  ComAtprotoRepoStrongref(Box<ComAtprotoRepoStrongref>),
}

impl Default for ComAtprotoAdminDefsReportviewSubject {
  fn default() -> Self {
    Self::ComAtprotoAdminDefsReporef(Box::new(ComAtprotoAdminDefsReporef::default()))
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "$type")]
pub enum ComAtprotoAdminDefsActionviewdetailSubject {
  #[serde(rename = "com.atproto.admin.defs#repoView")]
  ComAtprotoAdminDefsRepoview(Box<ComAtprotoAdminDefsRepoview>),
  #[serde(rename = "com.atproto.admin.defs#repoViewNotFound")]
  ComAtprotoAdminDefsRepoviewnotfound(Box<ComAtprotoAdminDefsRepoviewnotfound>),
  #[serde(rename = "com.atproto.admin.defs#recordView")]
  ComAtprotoAdminDefsRecordview(Box<ComAtprotoAdminDefsRecordview>),
  #[serde(rename = "com.atproto.admin.defs#recordViewNotFound")]
  ComAtprotoAdminDefsRecordviewnotfound(Box<ComAtprotoAdminDefsRecordviewnotfound>),
}

impl Default for ComAtprotoAdminDefsActionviewdetailSubject {
  fn default() -> Self {
    Self::ComAtprotoAdminDefsRepoview(Box::new(ComAtprotoAdminDefsRepoview::default()))
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "$type")]
pub enum ComAtprotoAdminDefsActionviewSubject {
  #[serde(rename = "com.atproto.admin.defs#repoRef")]
  ComAtprotoAdminDefsReporef(Box<ComAtprotoAdminDefsReporef>),
  #[serde(rename = "com.atproto.repo.strongRef")]
  ComAtprotoRepoStrongref(Box<ComAtprotoRepoStrongref>),
}

impl Default for ComAtprotoAdminDefsActionviewSubject {
  fn default() -> Self {
    Self::ComAtprotoAdminDefsReporef(Box::new(ComAtprotoAdminDefsReporef::default()))
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "$type")]
pub enum ComAtprotoAdminTakemoderationactionMainInputSubject {
  #[serde(rename = "com.atproto.admin.defs#repoRef")]
  ComAtprotoAdminDefsReporef(Box<ComAtprotoAdminDefsReporef>),
  #[serde(rename = "com.atproto.repo.strongRef")]
  ComAtprotoRepoStrongref(Box<ComAtprotoRepoStrongref>),
}

impl Default for ComAtprotoAdminTakemoderationactionMainInputSubject {
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
  #[serde(rename = "com.atproto.sync.subscribeRepos#handle")]
  ComAtprotoSyncSubscribereposHandle(Box<ComAtprotoSyncSubscribereposHandle>),
  #[serde(rename = "com.atproto.sync.subscribeRepos#migrate")]
  ComAtprotoSyncSubscribereposMigrate(Box<ComAtprotoSyncSubscribereposMigrate>),
  #[serde(rename = "com.atproto.sync.subscribeRepos#tombstone")]
  ComAtprotoSyncSubscribereposTombstone(Box<ComAtprotoSyncSubscribereposTombstone>),
  #[serde(rename = "com.atproto.sync.subscribeRepos#info")]
  ComAtprotoSyncSubscribereposInfo(Box<ComAtprotoSyncSubscribereposInfo>),
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
  proxy: Option<String>,
  jwt: Option<String>,
  agent: Agent,
}

impl Client {
  pub fn new<T1: ToString, T2: ToString>(host: T1, proxy: Option<T2>) -> Self {
    Self {
      host: host.to_string(),
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

  pub fn get_host(&self) -> String {
    self.host.clone()
  }

  pub fn get_proxy(&self) -> Option<String> {
    self.proxy.clone()
  }

  /// Get private preferences attached to the account.

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

    let mut q = Vec::new();

    q.push(("actor", actor));

    Ok(req.query_pairs(q).call()?.into_json()?)
  }

  pub fn app_bsky_actor_getprofiles(&self, actors: &[&str]) -> Result<AppBskyActorGetprofiles> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/app.bsky.actor.getProfiles",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut q = Vec::new();

    let actors_value = actors.join(",");

    q.push(("actors", actors_value.as_str()));

    Ok(req.query_pairs(q).call()?.into_json()?)
  }

  /// Get a list of actors suggested for following. Used in discovery UIs.

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

    let mut q = Vec::new();

    let limit_value = serde_json::to_string(&limit)?;

    if limit.is_some() {
      q.push(("limit", limit_value.as_str()));
    };

    if cursor.is_some() {
      q.push(("cursor", cursor.unwrap_or_default()));
    }

    Ok(req.query_pairs(q).call()?.into_json()?)
  }

  /// Find actors matching search criteria.

  pub fn app_bsky_actor_searchactors(
    &self,
    term: Option<&str>,
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

    let mut q = Vec::new();

    if term.is_some() {
      q.push(("term", term.unwrap_or_default()));
    }

    let limit_value = serde_json::to_string(&limit)?;

    if limit.is_some() {
      q.push(("limit", limit_value.as_str()));
    };

    if cursor.is_some() {
      q.push(("cursor", cursor.unwrap_or_default()));
    }

    Ok(req.query_pairs(q).call()?.into_json()?)
  }

  /// Find actor suggestions for a search term.

  pub fn app_bsky_actor_searchactorstypeahead(
    &self,
    term: Option<&str>,
    limit: Option<i64>,
  ) -> Result<AppBskyActorSearchactorstypeahead> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/app.bsky.actor.searchActorsTypeahead",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut q = Vec::new();

    if term.is_some() {
      q.push(("term", term.unwrap_or_default()));
    }

    let limit_value = serde_json::to_string(&limit)?;

    if limit.is_some() {
      q.push(("limit", limit_value.as_str()));
    }

    Ok(req.query_pairs(q).call()?.into_json()?)
  }

  /// Returns information about a given feed generator including TOS &amp; offered feed URIs

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

  /// Retrieve a list of feeds created by a given actor

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

    let mut q = Vec::new();

    q.push(("actor", actor));

    let limit_value = serde_json::to_string(&limit)?;

    if limit.is_some() {
      q.push(("limit", limit_value.as_str()));
    };

    if cursor.is_some() {
      q.push(("cursor", cursor.unwrap_or_default()));
    }

    Ok(req.query_pairs(q).call()?.into_json()?)
  }

  /// A view of an actor&#39;s feed.

  pub fn app_bsky_feed_getauthorfeed(
    &self,
    actor: &str,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<AppBskyFeedGetauthorfeed> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/app.bsky.feed.getAuthorFeed",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut q = Vec::new();

    q.push(("actor", actor));

    let limit_value = serde_json::to_string(&limit)?;

    if limit.is_some() {
      q.push(("limit", limit_value.as_str()));
    };

    if cursor.is_some() {
      q.push(("cursor", cursor.unwrap_or_default()));
    }

    Ok(req.query_pairs(q).call()?.into_json()?)
  }

  /// Compose and hydrate a feed from a user&#39;s selected feed generator

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

    let mut q = Vec::new();

    q.push(("feed", feed));

    let limit_value = serde_json::to_string(&limit)?;

    if limit.is_some() {
      q.push(("limit", limit_value.as_str()));
    };

    if cursor.is_some() {
      q.push(("cursor", cursor.unwrap_or_default()));
    }

    Ok(req.query_pairs(q).call()?.into_json()?)
  }

  /// Get information about a specific feed offered by a feed generator, such as its online status

  pub fn app_bsky_feed_getfeedgenerator(&self, feed: &str) -> Result<AppBskyFeedGetfeedgenerator> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/app.bsky.feed.getFeedGenerator",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut q = Vec::new();

    q.push(("feed", feed));

    Ok(req.query_pairs(q).call()?.into_json()?)
  }

  /// Get information about a list of feed generators

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

    let mut q = Vec::new();

    let feeds_value = feeds.join(",");

    q.push(("feeds", feeds_value.as_str()));

    Ok(req.query_pairs(q).call()?.into_json()?)
  }

  /// A skeleton of a feed provided by a feed generator

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

    let mut q = Vec::new();

    q.push(("feed", feed));

    let limit_value = serde_json::to_string(&limit)?;

    if limit.is_some() {
      q.push(("limit", limit_value.as_str()));
    };

    if cursor.is_some() {
      q.push(("cursor", cursor.unwrap_or_default()));
    }

    Ok(req.query_pairs(q).call()?.into_json()?)
  }

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

    let mut q = Vec::new();

    q.push(("uri", uri));

    let cid_value = serde_json::to_string(&cid)?;

    if cid.is_some() {
      q.push(("cid", cid_value.as_str()));
    }

    let limit_value = serde_json::to_string(&limit)?;

    if limit.is_some() {
      q.push(("limit", limit_value.as_str()));
    };

    if cursor.is_some() {
      q.push(("cursor", cursor.unwrap_or_default()));
    }

    Ok(req.query_pairs(q).call()?.into_json()?)
  }

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

    let mut q = Vec::new();

    q.push(("uri", uri));

    let depth_value = serde_json::to_string(&depth)?;

    if depth.is_some() {
      q.push(("depth", depth_value.as_str()));
    }

    let parent_height_value = serde_json::to_string(&parent_height)?;

    if parent_height.is_some() {
      q.push(("parent_height", parent_height_value.as_str()));
    }

    Ok(req.query_pairs(q).call()?.into_json()?)
  }

  /// A view of an actor&#39;s feed.

  pub fn app_bsky_feed_getposts(&self, uris: &[&str]) -> Result<AppBskyFeedGetposts> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/app.bsky.feed.getPosts",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut q = Vec::new();

    let uris_value = uris.join(",");

    q.push(("uris", uris_value.as_str()));

    Ok(req.query_pairs(q).call()?.into_json()?)
  }

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

    let mut q = Vec::new();

    q.push(("uri", uri));

    let cid_value = serde_json::to_string(&cid)?;

    if cid.is_some() {
      q.push(("cid", cid_value.as_str()));
    }

    let limit_value = serde_json::to_string(&limit)?;

    if limit.is_some() {
      q.push(("limit", limit_value.as_str()));
    };

    if cursor.is_some() {
      q.push(("cursor", cursor.unwrap_or_default()));
    }

    Ok(req.query_pairs(q).call()?.into_json()?)
  }

  /// A view of the user&#39;s home timeline.

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

    let mut q = Vec::new();

    if algorithm.is_some() {
      q.push(("algorithm", algorithm.unwrap_or_default()));
    }

    let limit_value = serde_json::to_string(&limit)?;

    if limit.is_some() {
      q.push(("limit", limit_value.as_str()));
    };

    if cursor.is_some() {
      q.push(("cursor", cursor.unwrap_or_default()));
    }

    Ok(req.query_pairs(q).call()?.into_json()?)
  }

  /// Who is the requester&#39;s account blocking?

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

    let mut q = Vec::new();

    let limit_value = serde_json::to_string(&limit)?;

    if limit.is_some() {
      q.push(("limit", limit_value.as_str()));
    };

    if cursor.is_some() {
      q.push(("cursor", cursor.unwrap_or_default()));
    }

    Ok(req.query_pairs(q).call()?.into_json()?)
  }

  /// Who is following an actor?

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

    let mut q = Vec::new();

    q.push(("actor", actor));

    let limit_value = serde_json::to_string(&limit)?;

    if limit.is_some() {
      q.push(("limit", limit_value.as_str()));
    };

    if cursor.is_some() {
      q.push(("cursor", cursor.unwrap_or_default()));
    }

    Ok(req.query_pairs(q).call()?.into_json()?)
  }

  /// Who is an actor following?

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

    let mut q = Vec::new();

    q.push(("actor", actor));

    let limit_value = serde_json::to_string(&limit)?;

    if limit.is_some() {
      q.push(("limit", limit_value.as_str()));
    };

    if cursor.is_some() {
      q.push(("cursor", cursor.unwrap_or_default()));
    }

    Ok(req.query_pairs(q).call()?.into_json()?)
  }

  /// Fetch a list of actors

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

    let mut q = Vec::new();

    q.push(("list", list));

    let limit_value = serde_json::to_string(&limit)?;

    if limit.is_some() {
      q.push(("limit", limit_value.as_str()));
    };

    if cursor.is_some() {
      q.push(("cursor", cursor.unwrap_or_default()));
    }

    Ok(req.query_pairs(q).call()?.into_json()?)
  }

  /// Which lists is the requester&#39;s account muting?

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

    let mut q = Vec::new();

    let limit_value = serde_json::to_string(&limit)?;

    if limit.is_some() {
      q.push(("limit", limit_value.as_str()));
    };

    if cursor.is_some() {
      q.push(("cursor", cursor.unwrap_or_default()));
    }

    Ok(req.query_pairs(q).call()?.into_json()?)
  }

  /// Fetch a list of lists that belong to an actor

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

    let mut q = Vec::new();

    q.push(("actor", actor));

    let limit_value = serde_json::to_string(&limit)?;

    if limit.is_some() {
      q.push(("limit", limit_value.as_str()));
    };

    if cursor.is_some() {
      q.push(("cursor", cursor.unwrap_or_default()));
    }

    Ok(req.query_pairs(q).call()?.into_json()?)
  }

  /// Who does the viewer mute?

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

    let mut q = Vec::new();

    let limit_value = serde_json::to_string(&limit)?;

    if limit.is_some() {
      q.push(("limit", limit_value.as_str()));
    };

    if cursor.is_some() {
      q.push(("cursor", cursor.unwrap_or_default()));
    }

    Ok(req.query_pairs(q).call()?.into_json()?)
  }

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

    let mut q = Vec::new();

    let seen_at_value = serde_json::to_string(&seen_at)?;

    if seen_at.is_some() {
      q.push(("seen_at", seen_at_value.as_str()));
    }

    Ok(req.query_pairs(q).call()?.into_json()?)
  }

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

    let mut q = Vec::new();

    let limit_value = serde_json::to_string(&limit)?;

    if limit.is_some() {
      q.push(("limit", limit_value.as_str()));
    };

    if cursor.is_some() {
      q.push(("cursor", cursor.unwrap_or_default()));
    }

    let seen_at_value = serde_json::to_string(&seen_at)?;

    if seen_at.is_some() {
      q.push(("seen_at", seen_at_value.as_str()));
    }

    Ok(req.query_pairs(q).call()?.into_json()?)
  }

  /// An unspecced view of globally popular items

  pub fn app_bsky_unspecced_getpopular(
    &self,
    include_nsfw: Option<bool>,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<AppBskyUnspeccedGetpopular> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/app.bsky.unspecced.getPopular",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut q = Vec::new();

    let include_nsfw_value = serde_json::to_string(&include_nsfw)?;

    if include_nsfw.is_some() {
      q.push(("include_nsfw", include_nsfw_value.as_str()));
    }

    let limit_value = serde_json::to_string(&limit)?;

    if limit.is_some() {
      q.push(("limit", limit_value.as_str()));
    };

    if cursor.is_some() {
      q.push(("cursor", cursor.unwrap_or_default()));
    }

    Ok(req.query_pairs(q).call()?.into_json()?)
  }

  /// An unspecced view of globally popular feed generators

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

    let mut q = Vec::new();

    let limit_value = serde_json::to_string(&limit)?;

    if limit.is_some() {
      q.push(("limit", limit_value.as_str()));
    };

    if cursor.is_some() {
      q.push(("cursor", cursor.unwrap_or_default()));
    };

    if query.is_some() {
      q.push(("query", query.unwrap_or_default()));
    }

    Ok(req.query_pairs(q).call()?.into_json()?)
  }

  /// A skeleton of a timeline - UNSPECCED &amp; WILL GO AWAY SOON

  pub fn app_bsky_unspecced_gettimelineskeleton(
    &self,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<AppBskyUnspeccedGettimelineskeleton> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/app.bsky.unspecced.getTimelineSkeleton",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut q = Vec::new();

    let limit_value = serde_json::to_string(&limit)?;

    if limit.is_some() {
      q.push(("limit", limit_value.as_str()));
    };

    if cursor.is_some() {
      q.push(("cursor", cursor.unwrap_or_default()));
    }

    Ok(req.query_pairs(q).call()?.into_json()?)
  }

  /// Admin view of invite codes

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

    let mut q = Vec::new();

    if sort.is_some() {
      q.push(("sort", sort.unwrap_or_default()));
    }

    let limit_value = serde_json::to_string(&limit)?;

    if limit.is_some() {
      q.push(("limit", limit_value.as_str()));
    };

    if cursor.is_some() {
      q.push(("cursor", cursor.unwrap_or_default()));
    }

    Ok(req.query_pairs(q).call()?.into_json()?)
  }

  /// View details about a moderation action.

  pub fn com_atproto_admin_getmoderationaction(
    &self,
    id: i64,
  ) -> Result<ComAtprotoAdminDefsActionviewdetail> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/com.atproto.admin.getModerationAction",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut q = Vec::new();

    let id_value = serde_json::to_string(&id)?;

    q.push(("id", id_value.as_str()));

    Ok(req.query_pairs(q).call()?.into_json()?)
  }

  /// List moderation actions related to a subject.

  pub fn com_atproto_admin_getmoderationactions(
    &self,
    subject: Option<&str>,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<ComAtprotoAdminGetmoderationactions> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/com.atproto.admin.getModerationActions",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut q = Vec::new();

    if subject.is_some() {
      q.push(("subject", subject.unwrap_or_default()));
    }

    let limit_value = serde_json::to_string(&limit)?;

    if limit.is_some() {
      q.push(("limit", limit_value.as_str()));
    };

    if cursor.is_some() {
      q.push(("cursor", cursor.unwrap_or_default()));
    }

    Ok(req.query_pairs(q).call()?.into_json()?)
  }

  /// View details about a moderation report.

  pub fn com_atproto_admin_getmoderationreport(
    &self,
    id: i64,
  ) -> Result<ComAtprotoAdminDefsReportviewdetail> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/com.atproto.admin.getModerationReport",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut q = Vec::new();

    let id_value = serde_json::to_string(&id)?;

    q.push(("id", id_value.as_str()));

    Ok(req.query_pairs(q).call()?.into_json()?)
  }

  /// List moderation reports related to a subject.

  pub fn com_atproto_admin_getmoderationreports(
    &self,
    subject: Option<&str>,
    ignore_subjects: Option<&[&str]>,
    actioned_by: Option<&str>,
    reporters: Option<&[&str]>,
    resolved: Option<bool>,
    action_type: Option<&str>,
    limit: Option<i64>,
    cursor: Option<&str>,
    reverse: Option<bool>,
  ) -> Result<ComAtprotoAdminGetmoderationreports> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/com.atproto.admin.getModerationReports",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut q = Vec::new();

    if subject.is_some() {
      q.push(("subject", subject.unwrap_or_default()));
    }

    let ignore_subjects_value = serde_json::to_string(&ignore_subjects)?;

    if ignore_subjects.is_some() {
      q.push(("ignore_subjects", ignore_subjects_value.as_str()));
    };

    if actioned_by.is_some() {
      q.push(("actioned_by", actioned_by.unwrap_or_default()));
    }

    let reporters_value = serde_json::to_string(&reporters)?;

    if reporters.is_some() {
      q.push(("reporters", reporters_value.as_str()));
    }

    let resolved_value = serde_json::to_string(&resolved)?;

    if resolved.is_some() {
      q.push(("resolved", resolved_value.as_str()));
    };

    if action_type.is_some() {
      q.push(("action_type", action_type.unwrap_or_default()));
    }

    let limit_value = serde_json::to_string(&limit)?;

    if limit.is_some() {
      q.push(("limit", limit_value.as_str()));
    };

    if cursor.is_some() {
      q.push(("cursor", cursor.unwrap_or_default()));
    }

    let reverse_value = serde_json::to_string(&reverse)?;

    if reverse.is_some() {
      q.push(("reverse", reverse_value.as_str()));
    }

    Ok(req.query_pairs(q).call()?.into_json()?)
  }

  /// View details about a record.

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

    let mut q = Vec::new();

    q.push(("uri", uri));

    let cid_value = serde_json::to_string(&cid)?;

    if cid.is_some() {
      q.push(("cid", cid_value.as_str()));
    }

    Ok(req.query_pairs(q).call()?.into_json()?)
  }

  /// View details about a repository.

  pub fn com_atproto_admin_getrepo(&self, did: &str) -> Result<ComAtprotoAdminDefsRepoviewdetail> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/com.atproto.admin.getRepo",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut q = Vec::new();

    q.push(("did", did));

    Ok(req.query_pairs(q).call()?.into_json()?)
  }

  /// Find repositories based on a search term.

  pub fn com_atproto_admin_searchrepos(
    &self,
    term: Option<&str>,
    invited_by: Option<&str>,
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

    let mut q = Vec::new();

    if term.is_some() {
      q.push(("term", term.unwrap_or_default()));
    };

    if invited_by.is_some() {
      q.push(("invited_by", invited_by.unwrap_or_default()));
    }

    let limit_value = serde_json::to_string(&limit)?;

    if limit.is_some() {
      q.push(("limit", limit_value.as_str()));
    };

    if cursor.is_some() {
      q.push(("cursor", cursor.unwrap_or_default()));
    }

    Ok(req.query_pairs(q).call()?.into_json()?)
  }

  /// Provides the DID of a repo.

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

    let mut q = Vec::new();

    q.push(("handle", handle));

    Ok(req.query_pairs(q).call()?.into_json()?)
  }

  /// Find labels relevant to the provided URI patterns.

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

    let mut q = Vec::new();

    let uri_patterns_value = uri_patterns.join(",");

    q.push(("uri_patterns", uri_patterns_value.as_str()));

    let sources_value = serde_json::to_string(&sources)?;

    if sources.is_some() {
      q.push(("sources", sources_value.as_str()));
    }

    let limit_value = serde_json::to_string(&limit)?;

    if limit.is_some() {
      q.push(("limit", limit_value.as_str()));
    };

    if cursor.is_some() {
      q.push(("cursor", cursor.unwrap_or_default()));
    }

    Ok(req.query_pairs(q).call()?.into_json()?)
  }

  /// Get information about the repo, including the list of collections.

  pub fn com_atproto_repo_describerepo(&self, repo: &str) -> Result<ComAtprotoRepoDescriberepo> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/com.atproto.repo.describeRepo",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut q = Vec::new();

    q.push(("repo", repo));

    Ok(req.query_pairs(q).call()?.into_json()?)
  }

  /// Get a record.

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

    let mut q = Vec::new();

    q.push(("repo", repo));

    q.push(("collection", collection));

    q.push(("rkey", rkey));

    let cid_value = serde_json::to_string(&cid)?;

    if cid.is_some() {
      q.push(("cid", cid_value.as_str()));
    }

    Ok(req.query_pairs(q).call()?.into_json()?)
  }

  /// List a range of records in a collection.

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

    let mut q = Vec::new();

    q.push(("repo", repo));

    q.push(("collection", collection));

    let limit_value = serde_json::to_string(&limit)?;

    if limit.is_some() {
      q.push(("limit", limit_value.as_str()));
    };

    if cursor.is_some() {
      q.push(("cursor", cursor.unwrap_or_default()));
    };

    if rkey_start.is_some() {
      q.push(("rkey_start", rkey_start.unwrap_or_default()));
    };

    if rkey_end.is_some() {
      q.push(("rkey_end", rkey_end.unwrap_or_default()));
    }

    let reverse_value = serde_json::to_string(&reverse)?;

    if reverse.is_some() {
      q.push(("reverse", reverse_value.as_str()));
    }

    Ok(req.query_pairs(q).call()?.into_json()?)
  }

  /// Get a document describing the service&#39;s accounts configuration.

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

  /// Get all invite codes for a given account

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

    let mut q = Vec::new();

    let include_used_value = serde_json::to_string(&include_used)?;

    if include_used.is_some() {
      q.push(("include_used", include_used_value.as_str()));
    }

    let create_available_value = serde_json::to_string(&create_available)?;

    if create_available.is_some() {
      q.push(("create_available", create_available_value.as_str()));
    }

    Ok(req.query_pairs(q).call()?.into_json()?)
  }

  /// Get information about the current session.

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

  /// List all app-specific passwords.

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

  /// Get a blob associated with a given repo.

  pub fn com_atproto_sync_getblob(&self, did: &str, cid: &CidString) -> Result<Vec<u8>> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/com.atproto.sync.getBlob",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut q = Vec::new();

    q.push(("did", did));

    let cid_value = serde_json::to_string(&cid)?;

    q.push(("cid", cid_value.as_str()));

    let mut ret = Vec::new();
    req
      .query_pairs(q)
      .call()?
      .into_reader()
      .read_to_end(&mut ret)?;
    Ok(ret)
  }

  /// Gets blocks from a given repo.

  pub fn com_atproto_sync_getblocks(&self, did: &str, cids: &[&CidString]) -> Result<Blocks> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/com.atproto.sync.getBlocks",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut q = Vec::new();

    q.push(("did", did));

    let cids_value = serde_json::to_string(&cids)?;

    q.push(("cids", cids_value.as_str()));

    let mut ret = Vec::new();
    req
      .query_pairs(q)
      .call()?
      .into_reader()
      .read_to_end(&mut ret)?;

    Ok(Blocks::from(ret.as_slice()))
  }

  /// Gets the repo state.

  pub fn com_atproto_sync_getcheckout(
    &self,
    did: &str,
    commit: Option<&CidString>,
  ) -> Result<Blocks> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/com.atproto.sync.getCheckout",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut q = Vec::new();

    q.push(("did", did));

    let commit_value = serde_json::to_string(&commit)?;

    if commit.is_some() {
      q.push(("commit", commit_value.as_str()));
    }

    let mut ret = Vec::new();
    req
      .query_pairs(q)
      .call()?
      .into_reader()
      .read_to_end(&mut ret)?;

    Ok(Blocks::from(ret.as_slice()))
  }

  /// Gets the path of repo commits

  pub fn com_atproto_sync_getcommitpath(
    &self,
    did: &str,
    latest: Option<&CidString>,
    earliest: Option<&CidString>,
  ) -> Result<ComAtprotoSyncGetcommitpath> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/com.atproto.sync.getCommitPath",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut q = Vec::new();

    q.push(("did", did));

    let latest_value = serde_json::to_string(&latest)?;

    if latest.is_some() {
      q.push(("latest", latest_value.as_str()));
    }

    let earliest_value = serde_json::to_string(&earliest)?;

    if earliest.is_some() {
      q.push(("earliest", earliest_value.as_str()));
    }

    Ok(req.query_pairs(q).call()?.into_json()?)
  }

  /// Gets the current HEAD CID of a repo.

  pub fn com_atproto_sync_gethead(&self, did: &str) -> Result<ComAtprotoSyncGethead> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/com.atproto.sync.getHead",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut q = Vec::new();

    q.push(("did", did));

    Ok(req.query_pairs(q).call()?.into_json()?)
  }

  /// Gets blocks needed for existence or non-existence of record.

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

    let mut q = Vec::new();

    q.push(("did", did));

    q.push(("collection", collection));

    q.push(("rkey", rkey));

    let commit_value = serde_json::to_string(&commit)?;

    if commit.is_some() {
      q.push(("commit", commit_value.as_str()));
    }

    let mut ret = Vec::new();
    req
      .query_pairs(q)
      .call()?
      .into_reader()
      .read_to_end(&mut ret)?;

    Ok(Blocks::from(ret.as_slice()))
  }

  /// Gets the repo state.

  pub fn com_atproto_sync_getrepo(
    &self,
    did: &str,
    earliest: Option<&CidString>,
    latest: Option<&CidString>,
  ) -> Result<Blocks> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/com.atproto.sync.getRepo",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut q = Vec::new();

    q.push(("did", did));

    let earliest_value = serde_json::to_string(&earliest)?;

    if earliest.is_some() {
      q.push(("earliest", earliest_value.as_str()));
    }

    let latest_value = serde_json::to_string(&latest)?;

    if latest.is_some() {
      q.push(("latest", latest_value.as_str()));
    }

    let mut ret = Vec::new();
    req
      .query_pairs(q)
      .call()?
      .into_reader()
      .read_to_end(&mut ret)?;

    Ok(Blocks::from(ret.as_slice()))
  }

  /// List blob cids for some range of commits

  pub fn com_atproto_sync_listblobs(
    &self,
    did: &str,
    latest: Option<&CidString>,
    earliest: Option<&CidString>,
  ) -> Result<ComAtprotoSyncListblobs> {
    let mut req = self.agent.get(&format!(
      "https://{}/xrpc/com.atproto.sync.listBlobs",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut q = Vec::new();

    q.push(("did", did));

    let latest_value = serde_json::to_string(&latest)?;

    if latest.is_some() {
      q.push(("latest", latest_value.as_str()));
    }

    let earliest_value = serde_json::to_string(&earliest)?;

    if earliest.is_some() {
      q.push(("earliest", earliest_value.as_str()));
    }

    Ok(req.query_pairs(q).call()?.into_json()?)
  }

  /// List dids and root cids of hosted repos

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

    let mut q = Vec::new();

    let limit_value = serde_json::to_string(&limit)?;

    if limit.is_some() {
      q.push(("limit", limit_value.as_str()));
    };

    if cursor.is_some() {
      q.push(("cursor", cursor.unwrap_or_default()));
    }

    Ok(req.query_pairs(q).call()?.into_json()?)
  }

  /// Sets the private preferences attached to the account.

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

  /// Mute an actor by did or handle.

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

  /// Mute a list of actors.

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

  /// Unmute an actor by did or handle.

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

  /// Unmute a list of actors.

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

  /// Notify server that the user has seen notifications.

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

  /// Disable an account from receiving new invite codes, but does not invalidate existing codes

  pub fn com_atproto_admin_disableaccountinvites(&self, account: &str) -> Result<ureq::Response> {
    let mut req = self.agent.post(&format!(
      "https://{}/xrpc/com.atproto.admin.disableAccountInvites",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut input = serde_json::Map::new();

    input.insert(String::from("account"), json!(account));

    Ok(req.send_json(json!(input))?)
  }

  /// Disable some set of codes and/or all codes associated with a set of users

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

  /// Re-enable an accounts ability to receive invite codes

  pub fn com_atproto_admin_enableaccountinvites(&self, account: &str) -> Result<ureq::Response> {
    let mut req = self.agent.post(&format!(
      "https://{}/xrpc/com.atproto.admin.enableAccountInvites",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut input = serde_json::Map::new();

    input.insert(String::from("account"), json!(account));

    Ok(req.send_json(json!(input))?)
  }

  /// Administrative action to rebase an account&#39;s repo

  pub fn com_atproto_admin_rebaserepo(
    &self,
    repo: &str,
    swap_commit: Option<&CidString>,
  ) -> Result<ureq::Response> {
    let mut req = self.agent.post(&format!(
      "https://{}/xrpc/com.atproto.admin.rebaseRepo",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut input = serde_json::Map::new();

    input.insert(String::from("repo"), json!(repo));

    if let Some(v) = &swap_commit {
      input.insert(String::from("swap_commit"), json!(v));
    }

    Ok(req.send_json(json!(input))?)
  }

  /// Resolve moderation reports by an action.

  pub fn com_atproto_admin_resolvemoderationreports(
    &self,
    action_id: i64,
    report_ids: &[i64],
    created_by: &str,
  ) -> Result<ComAtprotoAdminDefsActionview> {
    let mut req = self.agent.post(&format!(
      "https://{}/xrpc/com.atproto.admin.resolveModerationReports",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut input = serde_json::Map::new();

    input.insert(String::from("action_id"), json!(action_id));

    input.insert(String::from("report_ids"), json!(report_ids));

    input.insert(String::from("created_by"), json!(created_by));

    Ok(req.send_json(json!(input))?.into_json()?)
  }

  /// Reverse a moderation action.

  pub fn com_atproto_admin_reversemoderationaction(
    &self,
    id: i64,
    reason: &str,
    created_by: &str,
  ) -> Result<ComAtprotoAdminDefsActionview> {
    let mut req = self.agent.post(&format!(
      "https://{}/xrpc/com.atproto.admin.reverseModerationAction",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut input = serde_json::Map::new();

    input.insert(String::from("id"), json!(id));

    input.insert(String::from("reason"), json!(reason));

    input.insert(String::from("created_by"), json!(created_by));

    Ok(req.send_json(json!(input))?.into_json()?)
  }

  /// Send email to a user&#39;s primary email address

  pub fn com_atproto_admin_sendemail(
    &self,
    recipient_did: &str,
    content: &str,
    subject: Option<&str>,
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

    if let Some(v) = &subject {
      input.insert(String::from("subject"), json!(v));
    }

    Ok(req.send_json(json!(input))?.into_json()?)
  }

  /// Take a moderation action on a repo.

  pub fn com_atproto_admin_takemoderationaction(
    &self,
    action: &str,
    subject: ComAtprotoAdminTakemoderationactionMainInputSubject,
    reason: &str,
    created_by: &str,
    subject_blob_cids: Option<&[&CidString]>,
    create_label_vals: Option<&[&str]>,
    negate_label_vals: Option<&[&str]>,
  ) -> Result<ComAtprotoAdminDefsActionview> {
    let mut req = self.agent.post(&format!(
      "https://{}/xrpc/com.atproto.admin.takeModerationAction",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut input = serde_json::Map::new();

    input.insert(String::from("action"), json!(action));

    input.insert(String::from("subject"), json!(subject));

    input.insert(String::from("reason"), json!(reason));

    input.insert(String::from("created_by"), json!(created_by));

    if let Some(v) = &subject_blob_cids {
      input.insert(String::from("subject_blob_cids"), json!(v));
    }

    if let Some(v) = &create_label_vals {
      input.insert(String::from("create_label_vals"), json!(v));
    }

    if let Some(v) = &negate_label_vals {
      input.insert(String::from("negate_label_vals"), json!(v));
    }

    Ok(req.send_json(json!(input))?.into_json()?)
  }

  /// Administrative action to update an account&#39;s email

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

  /// Administrative action to update an account&#39;s handle

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

  /// Updates the handle of the account

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

  /// Report a repo or a record.

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

  /// Apply a batch transaction of creates, updates, and deletes.

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

  /// Create a new record.

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

  /// Delete a record, or ensure it doesn&#39;t exist.

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

  /// Write a record, creating or updating it as needed.

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

  /// Simple rebase of repo that deletes history

  pub fn com_atproto_repo_rebaserepo(
    &self,
    repo: &str,
    swap_commit: Option<&CidString>,
  ) -> Result<ureq::Response> {
    let mut req = self.agent.post(&format!(
      "https://{}/xrpc/com.atproto.repo.rebaseRepo",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut input = serde_json::Map::new();

    input.insert(String::from("repo"), json!(repo));

    if let Some(v) = &swap_commit {
      input.insert(String::from("swap_commit"), json!(v));
    }

    Ok(req.send_json(json!(input))?)
  }

  /// Upload a new blob to be added to repo in a later request.

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

  /// Create an account.

  pub fn com_atproto_server_createaccount(
    &self,
    email: &str,
    handle: &str,
    password: &str,
    did: Option<&str>,
    invite_code: Option<&str>,
    recovery_key: Option<&str>,
  ) -> Result<ComAtprotoServerCreateaccount> {
    let mut req = self.agent.post(&format!(
      "https://{}/xrpc/com.atproto.server.createAccount",
      self.host
    ));
    if let Some(jwt) = &self.jwt {
      req = req.set("Authorization", &format!("Bearer {}", jwt));
    }

    let mut input = serde_json::Map::new();

    input.insert(String::from("email"), json!(email));

    input.insert(String::from("handle"), json!(handle));

    input.insert(String::from("password"), json!(password));

    if let Some(v) = &did {
      input.insert(String::from("did"), json!(v));
    }

    if let Some(v) = &invite_code {
      input.insert(String::from("invite_code"), json!(v));
    }

    if let Some(v) = &recovery_key {
      input.insert(String::from("recovery_key"), json!(v));
    }

    Ok(req.send_json(json!(input))?.into_json()?)
  }

  /// Create an app-specific password.

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

  /// Create an invite code.

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

  /// Delete a user account with a token and password.

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

  /// Delete the current session.

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

  /// Refresh an authentication session.

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

  /// Revoke an app-specific password by name.

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

  /// Notify a crawling service of a recent update. Often when a long break between updates causes the connection with the crawling service to break.

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

  /// Request a service to persistently crawl hosted repos.

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

  /// Subscribe to label updates
  pub fn com_atproto_label_subscribelabels(
    &self,
    cursor: Option<i64>,
  ) -> Result<WebSocket<MaybeTlsStream<TcpStream>>> {
    let mut url = Url::parse(&format!(
      "wss://{}/xrpc/com.atproto.label.subscribeLabels",
      self.host
    ))?;

    let mut query = Vec::new();

    let cursor_value = serde_json::to_string(&cursor)?;

    if cursor.is_some() {
      query.push(("cursor", cursor_value.as_str()));
    }

    url.query_pairs_mut().extend_pairs(query);

    Ok(tungstenite::connect(&url)?.0)
  }

  /// Subscribe to repo updates
  pub fn com_atproto_sync_subscriberepos(
    &self,
    cursor: Option<i64>,
  ) -> Result<WebSocket<MaybeTlsStream<TcpStream>>> {
    let mut url = Url::parse(&format!(
      "wss://{}/xrpc/com.atproto.sync.subscribeRepos",
      self.host
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
