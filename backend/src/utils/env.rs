//! Environment variable utility functions
//!
//! This module contains helper functions for working with environment variables
//! to reduce code duplication in configuration loading.

use std::time::Duration;

/// Helper function to get environment variable with fallback names
///
/// This function attempts to retrieve an environment variable using multiple
/// possible names, returning the first one found.
pub fn get_env_var_with_fallback(primary: &str, fallback: &str) -> Option<String> {
    std::env::var(primary)
        .or_else(|_| std::env::var(fallback))
        .ok()
}

/// Parse environment variable as unsigned integer with fallback names and default
///
/// Attempts to parse an environment variable as u32 using primary and fallback names,
/// returning the provided default value if parsing fails or variable is not found.
pub fn parse_env_u32_with_fallback(primary: &str, fallback: &str, default: u32) -> u32 {
    get_env_var_with_fallback(primary, fallback)
        .and_then(|s| s.parse().ok())
        .unwrap_or(default)
}

/// Parse environment variable as Duration (seconds) with fallback names and default
///
/// Attempts to parse an environment variable as Duration (in seconds) using primary
/// and fallback names, returning the provided default value if parsing fails.
pub fn parse_env_duration_with_fallback(
    primary: &str,
    fallback: &str,
    default: Duration
) -> Duration {
    get_env_var_with_fallback(primary, fallback)
        .and_then(|s| s.parse::<u64>().ok())
        .map(Duration::from_secs)
        .unwrap_or(default)
}

/// Parse environment variable as Optional Duration (seconds) with fallback names
///
/// Attempts to parse an environment variable as Optional Duration (in seconds)
/// using primary and fallback names, returning None if parsing fails or not found.
pub fn parse_env_optional_duration_with_fallback(
    primary: &str,
    fallback: &str
) -> Option<Duration> {
    get_env_var_with_fallback(primary, fallback)
        .and_then(|s| s.parse::<u64>().ok())
        .map(Duration::from_secs)
}