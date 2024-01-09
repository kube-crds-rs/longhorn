// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium -f supportbundles.yml --schema=derived --docs -b
// kopium version: 0.16.2

use kube_derive::CustomResource;
#[cfg(feature = "schemars")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
#[cfg(feature = "builder")]
use typed_builder::TypedBuilder;

/// SupportBundleSpec defines the desired state of the Longhorn SupportBundle
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(not(feature = "schemars"), kube(schema = "disabled"))]
#[kube(
    group = "longhorn.io",
    version = "v1beta2",
    kind = "SupportBundle",
    plural = "supportbundles"
)]
#[kube(namespaced)]
#[kube(status = "SupportBundleStatus")]
pub struct SupportBundleSpec {
    /// A brief description of the issue
    pub description: String,
    /// The issue URL
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "issueURL")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub issue_url: Option<String>,
    /// The preferred responsible controller node ID.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeID")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub node_id: Option<String>,
}

/// SupportBundleStatus defines the observed state of the Longhorn SupportBundle
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct SupportBundleStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub conditions: Option<Vec<SupportBundleStatusConditions>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub filename: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub filesize: Option<i64>,
    /// The support bundle manager image
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub image: Option<String>,
    /// The support bundle manager IP
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "managerIP")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub manager_ip: Option<String>,
    /// The current responsible controller node ID
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ownerID")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub owner_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub progress: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub state: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct SupportBundleStatusConditions {
    /// Last time we probed the condition.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "lastProbeTime"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub last_probe_time: Option<String>,
    /// Last time the condition transitioned from one status to another.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "lastTransitionTime"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub last_transition_time: Option<String>,
    /// Human-readable message indicating details about last transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub message: Option<String>,
    /// Unique, one-word, CamelCase reason for the condition's last transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub reason: Option<String>,
    /// Status is the status of the condition. Can be True, False, Unknown.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub status: Option<String>,
    /// Type is the type of the condition.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub r#type: Option<String>,
}