// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium -f backingimagemanagers.yml --schema=derived --docs -b
// kopium version: 0.18.0

use kube_derive::CustomResource;
#[cfg(feature = "schemars")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
#[cfg(feature = "builder")]
use typed_builder::TypedBuilder;

/// BackingImageManagerSpec defines the desired state of the Longhorn backing image manager
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(not(feature = "schemars"), kube(schema = "disabled"))]
#[kube(
    group = "longhorn.io",
    version = "v1beta2",
    kind = "BackingImageManager",
    plural = "backingimagemanagers"
)]
#[kube(namespaced)]
#[kube(status = "BackingImageManagerStatus")]
pub struct BackingImageManagerSpec {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "backingImages"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub backing_images: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "diskPath")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub disk_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "diskUUID")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub disk_uuid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub image: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeID")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub node_id: Option<String>,
}

/// BackingImageManagerStatus defines the observed state of the Longhorn backing image manager
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct BackingImageManagerStatus {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "apiMinVersion"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub api_min_version: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "apiVersion"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub api_version: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "backingImageFileMap"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub backing_image_file_map:
        Option<BTreeMap<String, BackingImageManagerStatusBackingImageFileMap>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "currentState"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub current_state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub ip: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ownerID")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub owner_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "storageIP")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub storage_ip: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct BackingImageManagerStatusBackingImageFileMap {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "currentChecksum"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub current_checksum: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub progress: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "senderManagerAddress"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub sender_manager_address: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "sendingReference"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub sending_reference: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub size: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub uuid: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "virtualSize"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub virtual_size: Option<i64>,
}
