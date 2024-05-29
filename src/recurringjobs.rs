// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium -f recurringjobs.yml --schema=derived --docs -b --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube_derive::CustomResource;
    #[cfg(feature = "schemars")]
    pub use schemars::JsonSchema;
    pub use serde::{Deserialize, Serialize};
    pub use std::collections::BTreeMap;
    #[cfg(feature = "builder")]
    pub use typed_builder::TypedBuilder;
}
use self::prelude::*;

/// RecurringJobSpec defines the desired state of the Longhorn recurring job
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(not(feature = "schemars"), kube(schema = "disabled"))]
#[kube(
    group = "longhorn.io",
    version = "v1beta2",
    kind = "RecurringJob",
    plural = "recurringjobs"
)]
#[kube(namespaced)]
#[kube(status = "RecurringJobStatus")]
#[kube(derive = "Default")]
#[kube(derive = "PartialEq")]
pub struct RecurringJobSpec {
    /// The concurrency of taking the snapshot/backup.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub concurrency: Option<i64>,
    /// The cron setting.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub cron: Option<String>,
    /// The recurring job group.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub groups: Option<Vec<String>>,
    /// The label of the snapshot/backup.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub labels: Option<BTreeMap<String, String>>,
    /// The recurring job name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub name: Option<String>,
    /// The retain count of the snapshot/backup.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub retain: Option<i64>,
    /// The recurring job task. Can be "snapshot", "snapshot-force-create", "snapshot-cleanup", "snapshot-delete", "backup", "backup-force-create" or "filesystem-trim"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub task: Option<RecurringJobTask>,
}

/// RecurringJobSpec defines the desired state of the Longhorn recurring job
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub enum RecurringJobTask {
    #[serde(rename = "snapshot")]
    Snapshot,
    #[serde(rename = "snapshot-force-create")]
    SnapshotForceCreate,
    #[serde(rename = "snapshot-cleanup")]
    SnapshotCleanup,
    #[serde(rename = "snapshot-delete")]
    SnapshotDelete,
    #[serde(rename = "backup")]
    Backup,
    #[serde(rename = "backup-force-create")]
    BackupForceCreate,
    #[serde(rename = "filesystem-trim")]
    FilesystemTrim,
}

/// RecurringJobStatus defines the observed state of the Longhorn recurring job
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct RecurringJobStatus {
    /// The owner ID which is responsible to reconcile this recurring job CR.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ownerID")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub owner_id: Option<String>,
}
