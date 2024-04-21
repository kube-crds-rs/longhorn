// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium -f backingimages.yml --schema=derived --docs -b
// kopium version: 0.18.0

use kube_derive::CustomResource;
#[cfg(feature = "schemars")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
#[cfg(feature = "builder")]
use typed_builder::TypedBuilder;

/// BackingImageSpec defines the desired state of the Longhorn backing image
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(not(feature = "schemars"), kube(schema = "disabled"))]
#[kube(
    group = "longhorn.io",
    version = "v1beta2",
    kind = "BackingImage",
    plural = "backingimages"
)]
#[kube(namespaced)]
#[kube(status = "BackingImageStatus")]
pub struct BackingImageSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub checksum: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub disks: Option<BTreeMap<String, String>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "sourceParameters"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub source_parameters: Option<BTreeMap<String, String>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "sourceType"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub source_type: Option<BackingImageSourceType>,
}

/// BackingImageSpec defines the desired state of the Longhorn backing image
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub enum BackingImageSourceType {
    #[serde(rename = "download")]
    Download,
    #[serde(rename = "upload")]
    Upload,
    #[serde(rename = "export-from-volume")]
    ExportFromVolume,
    #[serde(rename = "restore")]
    Restore,
}

/// BackingImageStatus defines the observed state of the Longhorn backing image status
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct BackingImageStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub checksum: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "diskFileStatusMap"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub disk_file_status_map: Option<BTreeMap<String, BackingImageStatusDiskFileStatusMap>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "diskLastRefAtMap"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub disk_last_ref_at_map: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ownerID")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub owner_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub size: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub uuid: Option<String>,
    /// Virtual size of image, which may be larger than physical size. Will be zero until known (e.g. while a backing image is uploading)
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "virtualSize"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub virtual_size: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct BackingImageStatusDiskFileStatusMap {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "lastStateTransitionTime"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub last_state_transition_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub progress: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub state: Option<String>,
}
