use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Bean information
///
/// Description of a bean as returned by Spring Boot Actuator
#[derive(Serialize, Deserialize, Debug)]
pub struct BeanInfo {
    /// Aliases defined for this bean
    pub aliases: Vec<String>,
    /// Scope
    pub scope: String,
    /// Resource in which the bean was defined
    pub resource: Option<String>,
    /// Fully qualified type
    #[serde(rename = "type")]
    pub kind: String,
    /// Names of any dependencies
    pub dependencies: Vec<String>,
}

/// Bean Context
///
/// Application contexts's information returned by the Spring Boot Actuator.
///
/// This represents an Application Context in the `/actuator/beans/` endpoint
#[derive(Debug, Deserialize, Serialize)]
pub struct BeanContext {
    /// Parent of this context (if any)
    #[serde(rename = "parentId")]
    pub parent_id: Option<String>,
    /// Information about beans in this context
    beans: HashMap<String, BeanInfo>,
}

/// Response of `/actuator.beans` call.
///
/// This provide information about the applications beans
#[derive(Deserialize, Serialize, Debug)]
pub struct BeanResponse {
    /// Per context information
    pub contexts: HashMap<String, BeanContext>,
}
