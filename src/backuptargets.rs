// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium -f backuptargets.yml --schema=derived --docs -b
// kopium version: 0.16.2

use kube_derive::CustomResource;
#[cfg(feature = "schemars")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
#[cfg(feature = "builder")]
use typed_builder::TypedBuilder;

/// BackupTargetSpec defines the desired state of the Longhorn backup target
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(not(feature = "schemars"), kube(schema = "disabled"))]
#[kube(
    group = "longhorn.io",
    version = "v1beta2",
    kind = "BackupTarget",
    plural = "backuptargets"
)]
#[kube(namespaced)]
#[kube(status = "BackupTargetStatus")]
pub struct BackupTargetSpec {
    /// The backup target URL.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "backupTargetURL"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub backup_target_url: Option<String>,
    /// The backup target credential secret.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "credentialSecret"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub credential_secret: Option<String>,
    /// The interval that the cluster needs to run sync with the backup target.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "pollInterval"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub poll_interval: Option<String>,
    /// The time to request run sync the remote backup target.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "syncRequestedAt"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub sync_requested_at: Option<String>,
}

/// BackupTargetStatus defines the observed state of the Longhorn backup target
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct BackupTargetStatus {
    /// Available indicates if the remote backup target is available or not.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub available: Option<bool>,
    /// Records the reason on why the backup target is unavailable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub conditions: Option<Vec<BackupTargetStatusConditions>>,
    /// The last time that the controller synced with the remote backup target.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "lastSyncedAt"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub last_synced_at: Option<String>,
    /// The node ID on which the controller is responsible to reconcile this backup target CR.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ownerID")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub owner_id: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct BackupTargetStatusConditions {
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
