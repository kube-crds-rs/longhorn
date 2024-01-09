// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium -f instancemanagers.yml --schema=derived --docs -b
// kopium version: 0.16.2

use kube_derive::CustomResource;
#[cfg(feature = "schemars")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
#[cfg(feature = "builder")]
use typed_builder::TypedBuilder;

/// InstanceManagerSpec defines the desired state of the Longhorn instance manager
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(not(feature = "schemars"), kube(schema = "disabled"))]
#[kube(
    group = "longhorn.io",
    version = "v1beta2",
    kind = "InstanceManager",
    plural = "instancemanagers"
)]
#[kube(namespaced)]
#[kube(status = "InstanceManagerStatus")]
pub struct InstanceManagerSpec {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "dataEngine"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub data_engine: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub image: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeID")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub node_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub r#type: Option<InstanceManagerType>,
}

/// InstanceManagerSpec defines the desired state of the Longhorn instance manager
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub enum InstanceManagerType {
    #[serde(rename = "aio")]
    Aio,
    #[serde(rename = "engine")]
    Engine,
    #[serde(rename = "replica")]
    Replica,
}

/// InstanceManagerStatus defines the observed state of the Longhorn instance manager
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct InstanceManagerStatus {
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
        rename = "currentState"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub current_state: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "instanceEngines"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub instance_engines: Option<BTreeMap<String, InstanceManagerStatusInstanceEngines>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "instanceReplicas"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub instance_replicas: Option<BTreeMap<String, InstanceManagerStatusInstanceReplicas>>,
    /// Deprecated: Replaced by InstanceEngines and InstanceReplicas
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub instances: Option<BTreeMap<String, InstanceManagerStatusInstances>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub ip: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ownerID")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub owner_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "proxyApiMinVersion"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub proxy_api_min_version: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "proxyApiVersion"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub proxy_api_version: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct InstanceManagerStatusInstanceEngines {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub spec: Option<InstanceManagerStatusInstanceEnginesSpec>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub status: Option<InstanceManagerStatusInstanceEnginesStatus>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct InstanceManagerStatusInstanceEnginesSpec {
    /// Deprecated: Replaced by field `dataEngine`.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "backendStoreDriver"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub backend_store_driver: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "dataEngine"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub data_engine: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct InstanceManagerStatusInstanceEnginesStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub conditions: Option<BTreeMap<String, bool>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub endpoint: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "errorMsg")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub error_msg: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub listen: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "portEnd")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub port_end: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "portStart")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub port_start: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "resourceVersion"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub resource_version: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub r#type: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct InstanceManagerStatusInstanceReplicas {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub spec: Option<InstanceManagerStatusInstanceReplicasSpec>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub status: Option<InstanceManagerStatusInstanceReplicasStatus>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct InstanceManagerStatusInstanceReplicasSpec {
    /// Deprecated: Replaced by field `dataEngine`.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "backendStoreDriver"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub backend_store_driver: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "dataEngine"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub data_engine: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct InstanceManagerStatusInstanceReplicasStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub conditions: Option<BTreeMap<String, bool>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub endpoint: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "errorMsg")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub error_msg: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub listen: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "portEnd")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub port_end: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "portStart")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub port_start: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "resourceVersion"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub resource_version: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub r#type: Option<String>,
}

/// Deprecated: Replaced by InstanceEngines and InstanceReplicas
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct InstanceManagerStatusInstances {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub spec: Option<InstanceManagerStatusInstancesSpec>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub status: Option<InstanceManagerStatusInstancesStatus>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct InstanceManagerStatusInstancesSpec {
    /// Deprecated: Replaced by field `dataEngine`.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "backendStoreDriver"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub backend_store_driver: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "dataEngine"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub data_engine: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct InstanceManagerStatusInstancesStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub conditions: Option<BTreeMap<String, bool>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub endpoint: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "errorMsg")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub error_msg: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub listen: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "portEnd")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub port_end: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "portStart")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub port_start: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "resourceVersion"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub resource_version: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub r#type: Option<String>,
}