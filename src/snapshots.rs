// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium -f snapshots.yml --schema=derived --docs -b --derive=Default --derive=PartialEq --smart-derive-elision
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

/// SnapshotSpec defines the desired state of Longhorn Snapshot
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(not(feature = "schemars"), kube(schema = "disabled"))]
#[kube(
    group = "longhorn.io",
    version = "v1beta2",
    kind = "Snapshot",
    plural = "snapshots"
)]
#[kube(namespaced)]
#[kube(status = "SnapshotStatus")]
#[kube(derive = "Default")]
#[kube(derive = "PartialEq")]
pub struct SnapshotSpec {
    /// require creating a new snapshot
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "createSnapshot"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub create_snapshot: Option<bool>,
    /// The labels of snapshot
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub labels: Option<BTreeMap<String, String>>,
    /// the volume that this snapshot belongs to.
    /// This field is immutable after creation.
    /// Required
    pub volume: String,
}

/// SnapshotStatus defines the observed state of Longhorn Snapshot
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct SnapshotStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub checksum: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub children: Option<BTreeMap<String, bool>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "creationTime"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub creation_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub error: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub labels: Option<BTreeMap<String, String>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "markRemoved"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub mark_removed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ownerID")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub owner_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub parent: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "readyToUse"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub ready_to_use: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "restoreSize"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub restore_size: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub size: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "userCreated"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub user_created: Option<bool>,
}
