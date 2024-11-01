//! Utility functions and types used throughout the application.
//!
//! This module contains various utility to interact with the shell, and the file system.

use crate::error::Result;
use crate::fetcher::platform::Platform;
use serde::{Deserialize, Deserializer};
use tokio::task::JoinHandle;

pub mod executor;
pub mod file_system;

/// Converts a vector of string slices to a vector of owned strings.
pub fn to_owned(vec: Vec<&str>) -> Vec<String> {
    vec.into_iter().map(|s| s.to_owned()).collect()
}

/// Find the name of the executable for the given platform.
pub fn find_executable(name: &str) -> String {
    let platform = Platform::detect();

    match platform {
        Platform::Windows => format!("{}.exe", name),
        _ => name.to_owned(),
    }
}

/// Awaits two futures and returns a tuple of their results.
/// If either future returns an error, the error is propagated.
///
/// # Arguments
///
/// * `first` - The first future to await.
/// * `second` - The second future to await.
#[cfg_attr(feature = "tracing", instrument(level = "debug"))]
pub async fn await_two<T>(
    first: JoinHandle<Result<T>>,
    second: JoinHandle<Result<T>>,
) -> Result<(T, T)> {
    let (first_result, second_result) = tokio::try_join!(first, second)?;

    let first = first_result?;
    let second = second_result?;

    Ok((first, second))
}

/// Awaits all futures and returns a vector of their results.
/// If any future returns an error, the error is propagated.
///
/// # Arguments
///
/// * `handles` - The futures to await.
#[cfg_attr(feature = "tracing", instrument(level = "debug"))]
pub async fn await_all<T, I>(handles: I) -> Result<Vec<T>>
where
    I: IntoIterator<Item = JoinHandle<Result<T>>>,
    T: Send + 'static,
{
    let results = futures_util::future::try_join_all(handles).await?;

    results.into_iter().collect()
}

/// Fix issue with 'none' string in JSON.
pub fn json_none<'de, D>(deserializer: D) -> std::result::Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let string: Option<String> = Option::deserialize(deserializer)?;

    match string.as_deref() {
        Some("none") => Ok(None),
        _ => Ok(string),
    }
}

/// A macro to mimic the ternary operator in Rust.
#[macro_export]
macro_rules! ternary {
    ($condition:expr, $true:expr, $false:expr) => {
        if $condition {
            $true
        } else {
            $false
        }
    };
}
