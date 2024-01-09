// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium -f backupvolumes.yml --schema=derived --docs -b
// kopium version: 0.16.2

use kube_derive::CustomResource;
#[cfg(feature = "schemars")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
#[cfg(feature = "builder")]
use typed_builder::TypedBuilder;

/// BackupVolumeSpec defines the desired state of the Longhorn backup volume
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(not(feature = "schemars"), kube(schema = "disabled"))]
#[kube(
    group = "longhorn.io",
    version = "v1beta2",
    kind = "BackupVolume",
    plural = "backupvolumes"
)]
#[kube(namespaced)]
#[kube(status = "BackupVolumeStatus")]
pub struct BackupVolumeSpec {
    /// The time to request run sync the remote backup volume.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "syncRequestedAt"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub sync_requested_at: Option<String>,
}

/// BackupVolumeStatus defines the observed state of the Longhorn backup volume
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct BackupVolumeStatus {
    /// the backing image checksum.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "backingImageChecksum"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub backing_image_checksum: Option<String>,
    /// The backing image name.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "backingImageName"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub backing_image_name: Option<String>,
    /// The backup volume creation time.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "createdAt")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub created_at: Option<String>,
    /// The backup volume block count.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "dataStored"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub data_stored: Option<String>,
    /// The backup volume labels.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub labels: Option<BTreeMap<String, String>>,
    /// The latest volume backup time.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "lastBackupAt"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub last_backup_at: Option<String>,
    /// The latest volume backup name.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "lastBackupName"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub last_backup_name: Option<String>,
    /// The backup volume config last modification time.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "lastModificationTime"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub last_modification_time: Option<String>,
    /// The last time that the backup volume was synced into the cluster.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "lastSyncedAt"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub last_synced_at: Option<String>,
    /// The error messages when call longhorn engine on list or inspect backup volumes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub messages: Option<BTreeMap<String, String>>,
    /// The node ID on which the controller is responsible to reconcile this backup volume CR.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ownerID")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub owner_id: Option<String>,
    /// The backup volume size.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub size: Option<String>,
    /// the storage class name of pv/pvc binding with the volume.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "storageClassName"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub storage_class_name: Option<String>,
}