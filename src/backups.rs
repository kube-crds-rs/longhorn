// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium -f backups.yml --schema=derived --docs -b
// kopium version: 0.16.2

use kube_derive::CustomResource;
#[cfg(feature = "schemars")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
#[cfg(feature = "builder")]
use typed_builder::TypedBuilder;

/// BackupSpec defines the desired state of the Longhorn backup
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(not(feature = "schemars"), kube(schema = "disabled"))]
#[kube(
    group = "longhorn.io",
    version = "v1beta2",
    kind = "Backup",
    plural = "backups"
)]
#[kube(namespaced)]
#[kube(status = "BackupStatus")]
pub struct BackupSpec {
    /// The labels of snapshot backup.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub labels: Option<BTreeMap<String, String>>,
    /// The snapshot name.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "snapshotName"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub snapshot_name: Option<String>,
    /// The time to request run sync the remote backup.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "syncRequestedAt"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub sync_requested_at: Option<String>,
}

/// BackupStatus defines the observed state of the Longhorn backup
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct BackupStatus {
    /// The snapshot backup upload finished time.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "backupCreatedAt"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub backup_created_at: Option<String>,
    /// Compression method
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "compressionMethod"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub compression_method: Option<String>,
    /// The error message when taking the snapshot backup.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub error: Option<String>,
    /// The labels of snapshot backup.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub labels: Option<BTreeMap<String, String>>,
    /// The last time that the backup was synced with the remote backup target.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "lastSyncedAt"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub last_synced_at: Option<String>,
    /// The error messages when calling longhorn engine on listing or inspecting backups.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub messages: Option<BTreeMap<String, String>>,
    /// The node ID on which the controller is responsible to reconcile this backup CR.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ownerID")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub owner_id: Option<String>,
    /// The snapshot backup progress.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub progress: Option<i64>,
    /// The address of the replica that runs snapshot backup.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "replicaAddress"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub replica_address: Option<String>,
    /// The snapshot size.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub size: Option<String>,
    /// The snapshot creation time.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "snapshotCreatedAt"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub snapshot_created_at: Option<String>,
    /// The snapshot name.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "snapshotName"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub snapshot_name: Option<String>,
    /// The backup creation state. Can be "", "InProgress", "Completed", "Error", "Unknown".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub state: Option<String>,
    /// The snapshot backup URL.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub url: Option<String>,
    /// The volume's backing image name.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "volumeBackingImageName"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub volume_backing_image_name: Option<String>,
    /// The volume creation time.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "volumeCreated"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub volume_created: Option<String>,
    /// The volume name.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "volumeName"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub volume_name: Option<String>,
    /// The volume size.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "volumeSize"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub volume_size: Option<String>,
}
