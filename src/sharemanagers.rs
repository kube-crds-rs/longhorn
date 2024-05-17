// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium -f sharemanagers.yml --schema=derived --docs -b --derive=Default --derive=PartialEq
// kopium version: 0.19.0

#[allow(unused_imports)]
mod prelude {
    pub use kube_derive::CustomResource;
    #[cfg(feature = "schemars")]
    pub use schemars::JsonSchema;
    pub use serde::{Deserialize, Serialize};
    #[cfg(feature = "builder")]
    pub use typed_builder::TypedBuilder;
}
use self::prelude::*;

/// ShareManagerSpec defines the desired state of the Longhorn share manager
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(not(feature = "schemars"), kube(schema = "disabled"))]
#[kube(
    group = "longhorn.io",
    version = "v1beta2",
    kind = "ShareManager",
    plural = "sharemanagers"
)]
#[kube(namespaced)]
#[kube(status = "ShareManagerStatus")]
#[kube(derive = "Default")]
#[kube(derive = "PartialEq")]
pub struct ShareManagerSpec {
    /// Share manager image used for creating a share manager pod
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub image: Option<String>,
}

/// ShareManagerStatus defines the observed state of the Longhorn share manager
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct ShareManagerStatus {
    /// NFS endpoint that can access the mounted filesystem of the volume
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub endpoint: Option<String>,
    /// The node ID on which the controller is responsible to reconcile this share manager resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ownerID")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub owner_id: Option<String>,
    /// The state of the share manager resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub state: Option<String>,
}
