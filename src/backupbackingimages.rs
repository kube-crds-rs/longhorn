// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium -f backupbackingimages.yml --schema=derived --docs -b --derive=Default --derive=PartialEq --smart-derive-elision
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

/// BackupBackingImageSpec defines the desired state of the Longhorn backing image backup
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(not(feature = "schemars"), kube(schema = "disabled"))]
#[kube(
    group = "longhorn.io",
    version = "v1beta2",
    kind = "BackupBackingImage",
    plural = "backupbackingimages"
)]
#[kube(namespaced)]
#[kube(status = "BackupBackingImageStatus")]
#[kube(derive = "Default")]
#[kube(derive = "PartialEq")]
pub struct BackupBackingImageSpec {
    /// The labels of backing image backup.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub labels: Option<BTreeMap<String, String>>,
    /// The time to request run sync the remote backing image backup.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "syncRequestedAt"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub sync_requested_at: Option<String>,
    /// Is this CR created by user through API or UI.
    /// Required
    #[serde(rename = "userCreated")]
    pub user_created: bool,
}

/// BackupBackingImageStatus defines the observed state of the Longhorn backing image backup
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct BackupBackingImageStatus {
    /// The backing image name.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "backingImage"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub backing_image: Option<String>,
    /// The backing image backup upload finished time.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "backupCreatedAt"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub backup_created_at: Option<String>,
    /// The checksum of the backing image.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub checksum: Option<String>,
    /// Compression method
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "compressionMethod"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub compression_method: Option<String>,
    /// The error message when taking the backing image backup.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub error: Option<String>,
    /// The labels of backing image backup.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub labels: Option<BTreeMap<String, String>>,
    /// The last time that the backing image backup was synced with the remote backup target.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "lastSyncedAt"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub last_synced_at: Option<String>,
    /// The address of the backing image manager that runs backing image backup.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "managerAddress"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub manager_address: Option<String>,
    /// The error messages when listing or inspecting backing image backup.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub messages: Option<BTreeMap<String, String>>,
    /// The node ID on which the controller is responsible to reconcile this CR.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ownerID")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub owner_id: Option<String>,
    /// The backing image backup progress.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub progress: Option<i64>,
    /// The backing image size.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub size: Option<i64>,
    /// The backing image backup creation state.
    /// Can be "", "InProgress", "Completed", "Error", "Unknown".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub state: Option<String>,
    /// The backing image backup URL.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub url: Option<String>,
}
