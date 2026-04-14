//! Service data model
//!
//! This module defines the [`Service`] struct which represents
//! a runit service with its metadata and state.

/// Represents a runit service with its current state
///
/// # Fields
///
/// * `name` - Service name (e.g., "NetworkManager")
/// * `status` - Current status ("run", "down", "finish", "unknown")
/// * `pid` - Process ID if running
/// * `enabled` - Whether the service is enabled (symlinked)
/// * `path` - Path to the service directory
#[derive(Debug, Clone)]
pub struct Service {
    /// Service name
    pub name: String,
    /// Current status of the service
    pub status: String,
    /// Process ID if service is running
    pub pid: Option<String>,
    /// Whether the service is enabled
    pub enabled: bool,
    /// Path to the service configuration directory
    pub path: String,
}

impl Service {
    /// Creates a new Service with default values
    ///
    /// # Arguments
    ///
    /// * `name` - The service name
    /// * `path` - Path to the service directory
    pub fn new(name: String, path: String) -> Self {
        Self {
            name,
            status: "down".to_string(),
            pid: None,
            enabled: false,
            path,
        }
    }
}
