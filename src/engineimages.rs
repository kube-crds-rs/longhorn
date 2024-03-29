// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium -f engineimages.yml --schema=derived --docs -b
// kopium version: 0.16.2

use kube_derive::CustomResource;
#[cfg(feature = "schemars")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
#[cfg(feature = "builder")]
use typed_builder::TypedBuilder;

/// EngineImageSpec defines the desired state of the Longhorn engine image
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(not(feature = "schemars"), kube(schema = "disabled"))]
#[kube(
    group = "longhorn.io",
    version = "v1beta2",
    kind = "EngineImage",
    plural = "engineimages"
)]
#[kube(namespaced)]
#[kube(status = "EngineImageStatus")]
pub struct EngineImageSpec {
    pub image: String,
}

/// EngineImageStatus defines the observed state of the Longhorn engine image
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct EngineImageStatus {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "buildDate")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub build_date: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "cliAPIMinVersion"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub cli_api_min_version: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "cliAPIVersion"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub cli_api_version: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub conditions: Option<Vec<EngineImageStatusConditions>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "controllerAPIMinVersion"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub controller_api_min_version: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "controllerAPIVersion"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub controller_api_version: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "dataFormatMinVersion"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub data_format_min_version: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "dataFormatVersion"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub data_format_version: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "gitCommit")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub git_commit: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub incompatible: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "noRefSince"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub no_ref_since: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "nodeDeploymentMap"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub node_deployment_map: Option<BTreeMap<String, bool>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ownerID")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub owner_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "refCount")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub ref_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub version: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct EngineImageStatusConditions {
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
