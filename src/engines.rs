// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium -f engines.yml --schema=derived --docs -b
// kopium version: 0.16.2

use kube_derive::CustomResource;
#[cfg(feature = "schemars")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
#[cfg(feature = "builder")]
use typed_builder::TypedBuilder;

/// EngineSpec defines the desired state of the Longhorn engine
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(not(feature = "schemars"), kube(schema = "disabled"))]
#[kube(
    group = "longhorn.io",
    version = "v1beta2",
    kind = "Engine",
    plural = "engines"
)]
#[kube(namespaced)]
#[kube(status = "EngineStatus")]
pub struct EngineSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub active: Option<bool>,
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
        rename = "backupVolume"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub backup_volume: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "dataEngine"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub data_engine: Option<EngineDataEngine>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "desireState"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub desire_state: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "disableFrontend"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub disable_frontend: Option<bool>,
    /// Deprecated: Replaced by field `image`.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "engineImage"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub engine_image: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub frontend: Option<EngineFrontend>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub image: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "logRequested"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub log_requested: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeID")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub node_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "replicaAddressMap"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub replica_address_map: Option<BTreeMap<String, String>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "requestedBackupRestore"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub requested_backup_restore: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "requestedDataSource"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub requested_data_source: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "revisionCounterDisabled"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub revision_counter_disabled: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "salvageRequested"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub salvage_requested: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "snapshotMaxCount"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub snapshot_max_count: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "snapshotMaxSize"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub snapshot_max_size: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "unmapMarkSnapChainRemovedEnabled"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub unmap_mark_snap_chain_removed_enabled: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "upgradedReplicaAddressMap"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub upgraded_replica_address_map: Option<BTreeMap<String, String>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "volumeName"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub volume_name: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "volumeSize"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub volume_size: Option<String>,
}

/// EngineSpec defines the desired state of the Longhorn engine
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub enum EngineDataEngine {
    #[serde(rename = "v1")]
    V1,
    #[serde(rename = "v2")]
    V2,
}

/// EngineSpec defines the desired state of the Longhorn engine
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub enum EngineFrontend {
    #[serde(rename = "blockdev")]
    Blockdev,
    #[serde(rename = "iscsi")]
    Iscsi,
    #[serde(rename = "nvmf")]
    Nvmf,
    #[serde(rename = "")]
    KopiumEmpty,
}

/// EngineStatus defines the observed state of the Longhorn engine
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct EngineStatus {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "backupStatus"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub backup_status: Option<BTreeMap<String, EngineStatusBackupStatus>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "cloneStatus"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub clone_status: Option<BTreeMap<String, EngineStatusCloneStatus>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub conditions: Option<Vec<EngineStatusConditions>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "currentImage"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub current_image: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "currentReplicaAddressMap"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub current_replica_address_map: Option<BTreeMap<String, String>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "currentSize"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub current_size: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "currentState"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub current_state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub endpoint: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "instanceManagerName"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub instance_manager_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub ip: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "isExpanding"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub is_expanding: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "lastExpansionError"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub last_expansion_error: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "lastExpansionFailedAt"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub last_expansion_failed_at: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "lastRestoredBackup"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub last_restored_backup: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "logFetched"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub log_fetched: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ownerID")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub owner_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub port: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "purgeStatus"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub purge_status: Option<BTreeMap<String, EngineStatusPurgeStatus>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "rebuildStatus"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub rebuild_status: Option<BTreeMap<String, EngineStatusRebuildStatus>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "replicaModeMap"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub replica_mode_map: Option<BTreeMap<String, String>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "restoreStatus"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub restore_status: Option<BTreeMap<String, EngineStatusRestoreStatus>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "salvageExecuted"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub salvage_executed: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "snapshotMaxCount"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub snapshot_max_count: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "snapshotMaxSize"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub snapshot_max_size: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub snapshots: Option<BTreeMap<String, EngineStatusSnapshots>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "snapshotsError"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub snapshots_error: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub started: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "storageIP")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub storage_ip: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "unmapMarkSnapChainRemovedEnabled"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub unmap_mark_snap_chain_removed_enabled: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct EngineStatusBackupStatus {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "backupURL")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub backup_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub error: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub progress: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "replicaAddress"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub replica_address: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "snapshotName"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub snapshot_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub state: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct EngineStatusCloneStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub error: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "fromReplicaAddress"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub from_replica_address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "isCloning")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub is_cloning: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub progress: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "snapshotName"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub snapshot_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub state: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct EngineStatusConditions {
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct EngineStatusPurgeStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub error: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "isPurging")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub is_purging: Option<bool>,
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
pub struct EngineStatusRebuildStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub error: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "fromReplicaAddress"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub from_replica_address: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "isRebuilding"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub is_rebuilding: Option<bool>,
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
pub struct EngineStatusRestoreStatus {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "backupURL")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub backup_url: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "currentRestoringBackup"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub current_restoring_backup: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub error: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub filename: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "isRestoring"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub is_restoring: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "lastRestored"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub last_restored: Option<String>,
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
pub struct EngineStatusSnapshots {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub children: Option<BTreeMap<String, bool>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub created: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub labels: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub parent: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub removed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub size: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub usercreated: Option<bool>,
}
