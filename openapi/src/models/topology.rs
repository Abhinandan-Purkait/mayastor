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

/// Topology : topology to choose a replacement replica for self healing  (overrides the initial creation topology)

/// topology to choose a replacement replica for self healing  (overrides the initial creation topology)
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Topology {
    /// volume topology, explicitly selected
    #[serde(rename = "explicit", skip_serializing_if = "Option::is_none")]
    pub explicit: Option<crate::models::ExplicitTopology>,
    /// volume topology definition through labels
    #[serde(rename = "labelled", skip_serializing_if = "Option::is_none")]
    pub labelled: Option<crate::models::LabelledTopology>,
}

impl Topology {
    /// Topology using only the required fields
    pub fn new() -> Topology {
        Topology {
            explicit: None,
            labelled: None,
        }
    }
    /// Topology using all fields
    pub fn new_all(
        explicit: impl Into<Option<crate::models::ExplicitTopology>>,
        labelled: impl Into<Option<crate::models::LabelledTopology>>,
    ) -> Topology {
        Topology {
            explicit: explicit.into(),
            labelled: labelled.into(),
        }
    }
}
