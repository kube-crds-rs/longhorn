//! Kubernetes CRDs for Longhorn v1.6.0
//!
//! This library provides automatically generated types for the [Longhorn v1.6.0 CRD definitions]. It is
//! intended to be used with the [Kube-rs] library.
//!
//! [Longhorn v1.6.0 CRD definitions]: https://raw.githubusercontent.com/longhorn/longhorn/master/deploy/longhorn.yaml
//! [Kube-rs]: https://kube.rs/

pub mod backingimagedatasources;
pub use backingimagedatasources::*;
pub mod backingimagemanagers;
pub use backingimagemanagers::*;
pub mod backingimages;
pub use backingimages::*;
pub mod backupbackingimages;
pub use backupbackingimages::*;
pub mod backups;
pub use backups::*;
pub mod backuptargets;
pub use backuptargets::*;
pub mod backupvolumes;
pub use backupvolumes::*;
pub mod engineimages;
pub use engineimages::*;
pub mod engines;
pub use engines::*;
pub mod instancemanagers;
pub use instancemanagers::*;
pub mod nodes;
pub use nodes::*;
pub mod orphans;
pub use orphans::*;
pub mod recurringjobs;
pub use recurringjobs::*;
pub mod replicas;
pub use replicas::*;
pub mod settings;
pub use settings::*;
pub mod sharemanagers;
pub use sharemanagers::*;
pub mod snapshots;
pub use snapshots::*;
pub mod supportbundles;
pub use supportbundles::*;
pub mod systembackups;
pub use systembackups::*;
pub mod systemrestores;
pub use systemrestores::*;
pub mod volumes;
pub use volumes::*;
pub mod volumeattachments;
pub use volumeattachments::*;
