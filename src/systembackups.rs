// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium -f systembackups.yml --schema=derived --docs -b
// kopium version: 0.16.2

use kube_derive::CustomResource;
#[cfg(feature = "schemars")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
#[cfg(feature = "builder")]
use typed_builder::TypedBuilder;

/// SystemBackupSpec defines the desired state of the Longhorn SystemBackup
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(not(feature = "schemars"), kube(schema = "disabled"))]
#[kube(
    group = "longhorn.io",
    version = "v1beta2",
    kind = "SystemBackup",
    plural = "systembackups"
)]
#[kube(namespaced)]
#[kube(status = "SystemBackupStatus")]
pub struct SystemBackupSpec {
    /// The create volume backup policy Can be "if-not-present", "always" or "disabled"
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "volumeBackupPolicy"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub volume_backup_policy: Option<String>,
}

/// SystemBackupStatus defines the observed state of the Longhorn SystemBackup
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct SystemBackupStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub conditions: Option<Vec<SystemBackupStatusConditions>>,
    /// The system backup creation time.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "createdAt")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub created_at: Option<String>,
    /// The saved Longhorn manager git commit.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "gitCommit")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub git_commit: Option<String>,
    /// The last time that the system backup was synced into the cluster.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "lastSyncedAt"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub last_synced_at: Option<String>,
    /// The saved manager image.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "managerImage"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub manager_image: Option<String>,
    /// The node ID of the responsible controller to reconcile this SystemBackup.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ownerID")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub owner_id: Option<String>,
    /// The system backup state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub state: Option<String>,
    /// The saved Longhorn version.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub version: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct SystemBackupStatusConditions {
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
