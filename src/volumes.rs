// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium -f volumes.yml --schema=derived --docs -b --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
    pub use kube_derive::CustomResource;
    #[cfg(feature = "schemars")]
    pub use schemars::JsonSchema;
    pub use serde::{Deserialize, Serialize};
    #[cfg(feature = "builder")]
    pub use typed_builder::TypedBuilder;
}
use self::prelude::*;

/// VolumeSpec defines the desired state of the Longhorn volume
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(not(feature = "schemars"), kube(schema = "disabled"))]
#[kube(
    group = "longhorn.io",
    version = "v1beta2",
    kind = "Volume",
    plural = "volumes"
)]
#[kube(namespaced)]
#[kube(status = "VolumeStatus")]
#[kube(derive = "Default")]
#[kube(derive = "PartialEq")]
pub struct VolumeSpec {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "Standby")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub standby: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "accessMode"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub access_mode: Option<VolumeAccessMode>,
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
        rename = "backingImage"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub backing_image: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "backupCompressionMethod"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub backup_compression_method: Option<VolumeBackupCompressionMethod>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "dataEngine"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub data_engine: Option<VolumeDataEngine>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "dataLocality"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub data_locality: Option<VolumeDataLocality>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "dataSource"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub data_source: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "disableFrontend"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub disable_frontend: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "diskSelector"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub disk_selector: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub encrypted: Option<bool>,
    /// Deprecated: Replaced by field `image`.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "engineImage"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub engine_image: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "fromBackup"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub from_backup: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub frontend: Option<VolumeFrontend>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub image: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "lastAttachedBy"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub last_attached_by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub migratable: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "migrationNodeID"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub migration_node_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeID")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub node_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "nodeSelector"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub node_selector: Option<Vec<String>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "numberOfReplicas"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub number_of_replicas: Option<i64>,
    /// OfflineReplicaRebuilding is used to determine if the offline replica rebuilding feature is enabled or not
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "offlineReplicaRebuilding"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub offline_replica_rebuilding: Option<VolumeOfflineReplicaRebuilding>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "replicaAutoBalance"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub replica_auto_balance: Option<VolumeReplicaAutoBalance>,
    /// Replica disk soft anti affinity of the volume. Set enabled to allow replicas to be scheduled in the same disk.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "replicaDiskSoftAntiAffinity"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub replica_disk_soft_anti_affinity: Option<VolumeReplicaDiskSoftAntiAffinity>,
    /// Replica soft anti affinity of the volume. Set enabled to allow replicas to be scheduled on the same node.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "replicaSoftAntiAffinity"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub replica_soft_anti_affinity: Option<VolumeReplicaSoftAntiAffinity>,
    /// Replica zone soft anti affinity of the volume. Set enabled to allow replicas to be scheduled in the same zone.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "replicaZoneSoftAntiAffinity"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub replica_zone_soft_anti_affinity: Option<VolumeReplicaZoneSoftAntiAffinity>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "restoreVolumeRecurringJob"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub restore_volume_recurring_job: Option<VolumeRestoreVolumeRecurringJob>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "revisionCounterDisabled"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub revision_counter_disabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub size: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "snapshotDataIntegrity"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub snapshot_data_integrity: Option<VolumeSnapshotDataIntegrity>,
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
        rename = "staleReplicaTimeout"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub stale_replica_timeout: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "unmapMarkSnapChainRemoved"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub unmap_mark_snap_chain_removed: Option<VolumeUnmapMarkSnapChainRemoved>,
}

/// VolumeSpec defines the desired state of the Longhorn volume
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub enum VolumeAccessMode {
    #[serde(rename = "rwo")]
    Rwo,
    #[serde(rename = "rwx")]
    Rwx,
}

/// VolumeSpec defines the desired state of the Longhorn volume
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub enum VolumeBackupCompressionMethod {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "lz4")]
    Lz4,
    #[serde(rename = "gzip")]
    Gzip,
}

/// VolumeSpec defines the desired state of the Longhorn volume
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub enum VolumeDataEngine {
    #[serde(rename = "v1")]
    V1,
    #[serde(rename = "v2")]
    V2,
}

/// VolumeSpec defines the desired state of the Longhorn volume
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub enum VolumeDataLocality {
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "best-effort")]
    BestEffort,
    #[serde(rename = "strict-local")]
    StrictLocal,
}

/// VolumeSpec defines the desired state of the Longhorn volume
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub enum VolumeFrontend {
    #[serde(rename = "blockdev")]
    Blockdev,
    #[serde(rename = "iscsi")]
    Iscsi,
    #[serde(rename = "nvmf")]
    Nvmf,
    #[serde(rename = "")]
    KopiumEmpty,
}

/// VolumeSpec defines the desired state of the Longhorn volume
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub enum VolumeOfflineReplicaRebuilding {
    #[serde(rename = "ignored")]
    Ignored,
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "enabled")]
    Enabled,
}

/// VolumeSpec defines the desired state of the Longhorn volume
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub enum VolumeReplicaAutoBalance {
    #[serde(rename = "ignored")]
    Ignored,
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "least-effort")]
    LeastEffort,
    #[serde(rename = "best-effort")]
    BestEffort,
}

/// VolumeSpec defines the desired state of the Longhorn volume
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub enum VolumeReplicaDiskSoftAntiAffinity {
    #[serde(rename = "ignored")]
    Ignored,
    #[serde(rename = "enabled")]
    Enabled,
    #[serde(rename = "disabled")]
    Disabled,
}

/// VolumeSpec defines the desired state of the Longhorn volume
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub enum VolumeReplicaSoftAntiAffinity {
    #[serde(rename = "ignored")]
    Ignored,
    #[serde(rename = "enabled")]
    Enabled,
    #[serde(rename = "disabled")]
    Disabled,
}

/// VolumeSpec defines the desired state of the Longhorn volume
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub enum VolumeReplicaZoneSoftAntiAffinity {
    #[serde(rename = "ignored")]
    Ignored,
    #[serde(rename = "enabled")]
    Enabled,
    #[serde(rename = "disabled")]
    Disabled,
}

/// VolumeSpec defines the desired state of the Longhorn volume
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub enum VolumeRestoreVolumeRecurringJob {
    #[serde(rename = "ignored")]
    Ignored,
    #[serde(rename = "enabled")]
    Enabled,
    #[serde(rename = "disabled")]
    Disabled,
}

/// VolumeSpec defines the desired state of the Longhorn volume
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub enum VolumeSnapshotDataIntegrity {
    #[serde(rename = "ignored")]
    Ignored,
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "enabled")]
    Enabled,
    #[serde(rename = "fast-check")]
    FastCheck,
}

/// VolumeSpec defines the desired state of the Longhorn volume
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub enum VolumeUnmapMarkSnapChainRemoved {
    #[serde(rename = "ignored")]
    Ignored,
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "enabled")]
    Enabled,
}

/// VolumeStatus defines the observed state of the Longhorn volume
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct VolumeStatus {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "actualSize"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub actual_size: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "cloneStatus"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub clone_status: Option<VolumeStatusCloneStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub conditions: Option<Vec<Condition>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "currentImage"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub current_image: Option<String>,
    /// the node that this volume is currently migrating to
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "currentMigrationNodeID"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub current_migration_node_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "currentNodeID"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub current_node_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "expansionRequired"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub expansion_required: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "frontendDisabled"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub frontend_disabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "isStandby")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub is_standby: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "kubernetesStatus"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub kubernetes_status: Option<VolumeStatusKubernetesStatus>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "lastBackup"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub last_backup: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "lastBackupAt"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub last_backup_at: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "lastDegradedAt"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub last_degraded_at: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "offlineReplicaRebuildingRequired"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub offline_replica_rebuilding_required: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ownerID")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub owner_id: Option<String>,
    /// Deprecated.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "pendingNodeID"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub pending_node_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "remountRequestedAt"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub remount_requested_at: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "restoreInitiated"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub restore_initiated: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "restoreRequired"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub restore_required: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub robustness: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "shareEndpoint"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub share_endpoint: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "shareState"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub share_state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub state: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct VolumeStatusCloneStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub snapshot: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "sourceVolume"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub source_volume: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub state: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct VolumeStatusKubernetesStatus {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "lastPVCRefAt"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub last_pvc_ref_at: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "lastPodRefAt"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub last_pod_ref_at: Option<String>,
    /// determine if PVC/Namespace is history or not
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub namespace: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pvName")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub pv_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pvStatus")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub pv_status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pvcName")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub pvc_name: Option<String>,
    /// determine if Pod/Workload is history or not
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "workloadsStatus"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub workloads_status: Option<Vec<VolumeStatusKubernetesStatusWorkloadsStatus>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct VolumeStatusKubernetesStatusWorkloadsStatus {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "podName")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub pod_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "podStatus")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub pod_status: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "workloadName"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub workload_name: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "workloadType"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub workload_type: Option<String>,
}
