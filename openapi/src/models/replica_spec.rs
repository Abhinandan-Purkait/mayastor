#![allow(
    clippy::too_many_arguments,
    clippy::new_without_default,
    non_camel_case_types,
    unused_imports
)]
/*
 * Mayastor RESTful API
 *
 * The version of the OpenAPI document: v0
 *
 * Generated by: https://github.com/openebs/openapi-generator
 */

use crate::apis::IntoVec;

/// ReplicaSpec : User specification of a replica.

/// User specification of a replica.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ReplicaSpec {
    /// Managed by our control plane
    #[serde(rename = "managed")]
    pub managed: bool,
    #[serde(rename = "operation", skip_serializing_if = "Option::is_none")]
    pub operation: Option<crate::models::ReplicaSpecOperation>,
    #[serde(rename = "owners")]
    pub owners: crate::models::ReplicaSpecOwners,
    /// The pool that the replica should live on.
    #[serde(rename = "pool")]
    pub pool: String,
    #[serde(rename = "share")]
    pub share: crate::models::Protocol,
    /// The size that the replica should be.
    #[serde(rename = "size")]
    pub size: i64,
    #[serde(rename = "state")]
    pub state: crate::models::SpecState,
    /// Thin provisioning.
    #[serde(rename = "thin")]
    pub thin: bool,
    /// uuid of the replica
    #[serde(rename = "uuid")]
    pub uuid: uuid::Uuid,
}

impl ReplicaSpec {
    /// ReplicaSpec using only the required fields
    pub fn new(
        managed: impl Into<bool>,
        owners: impl Into<crate::models::ReplicaSpecOwners>,
        pool: impl Into<String>,
        share: impl Into<crate::models::Protocol>,
        size: impl Into<i64>,
        state: impl Into<crate::models::SpecState>,
        thin: impl Into<bool>,
        uuid: impl Into<uuid::Uuid>,
    ) -> ReplicaSpec {
        ReplicaSpec {
            managed: managed.into(),
            operation: None,
            owners: owners.into(),
            pool: pool.into(),
            share: share.into(),
            size: size.into(),
            state: state.into(),
            thin: thin.into(),
            uuid: uuid.into(),
        }
    }
    /// ReplicaSpec using all fields
    pub fn new_all(
        managed: impl Into<bool>,
        operation: impl Into<Option<crate::models::ReplicaSpecOperation>>,
        owners: impl Into<crate::models::ReplicaSpecOwners>,
        pool: impl Into<String>,
        share: impl Into<crate::models::Protocol>,
        size: impl Into<i64>,
        state: impl Into<crate::models::SpecState>,
        thin: impl Into<bool>,
        uuid: impl Into<uuid::Uuid>,
    ) -> ReplicaSpec {
        ReplicaSpec {
            managed: managed.into(),
            operation: operation.into(),
            owners: owners.into(),
            pool: pool.into(),
            share: share.into(),
            size: size.into(),
            state: state.into(),
            thin: thin.into(),
            uuid: uuid.into(),
        }
    }
}
