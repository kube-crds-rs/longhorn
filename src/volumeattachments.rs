// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium -f volumeattachments.yml --schema=derived --docs -b --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
    pub use kube_derive::CustomResource;
    #[cfg(feature = "schemars")]
    pub use schemars::JsonSchema;
    pub use serde::{Deserialize, Serialize};
    pub use std::collections::BTreeMap;
    #[cfg(feature = "builder")]
    pub use typed_builder::TypedBuilder;
}
use self::prelude::*;

/// VolumeAttachmentSpec defines the desired state of Longhorn VolumeAttachment
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(not(feature = "schemars"), kube(schema = "disabled"))]
#[kube(
    group = "longhorn.io",
    version = "v1beta2",
    kind = "VolumeAttachment",
    plural = "volumeattachments"
)]
#[kube(namespaced)]
#[kube(status = "VolumeAttachmentStatus")]
#[kube(derive = "Default")]
#[kube(derive = "PartialEq")]
pub struct VolumeAttachmentSpec {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "attachmentTickets"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub attachment_tickets: Option<BTreeMap<String, VolumeAttachmentAttachmentTickets>>,
    /// The name of Longhorn volume of this VolumeAttachment
    pub volume: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct VolumeAttachmentAttachmentTickets {
    /// A sequence number representing a specific generation of the desired state.
    /// Populated by the system. Read-only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub generation: Option<i64>,
    /// The unique ID of this attachment. Used to differentiate different attachments of the same volume.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub id: Option<String>,
    /// The node that this attachment is requesting
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeID")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub node_id: Option<String>,
    /// Optional additional parameter for this attachment
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub parameters: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub r#type: Option<String>,
}

/// VolumeAttachmentStatus defines the observed state of Longhorn VolumeAttachment
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct VolumeAttachmentStatus {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "attachmentTicketStatuses"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub attachment_ticket_statuses:
        Option<BTreeMap<String, VolumeAttachmentStatusAttachmentTicketStatuses>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct VolumeAttachmentStatusAttachmentTicketStatuses {
    /// Record any error when trying to fulfill this attachment
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub conditions: Option<Vec<Condition>>,
    /// A sequence number representing a specific generation of the desired state.
    /// Populated by the system. Read-only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub generation: Option<i64>,
    /// The unique ID of this attachment. Used to differentiate different attachments of the same volume.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub id: Option<String>,
    /// Indicate whether this attachment ticket has been satisfied
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub satisfied: Option<bool>,
}
