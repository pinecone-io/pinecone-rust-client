use std::time::Duration;

/// Defines the wait policy for index creation.
#[derive(Clone, Debug, PartialEq)]
pub enum WaitPolicy {
    /// Wait for the index to become ready, up to the specified duration.
    WaitFor(Duration),

    /// Do not wait for the index to become ready -- return immediately.
    NoWait,
}

impl Default for WaitPolicy {
    fn default() -> Self {
        WaitPolicy::WaitFor(Duration::from_secs(300))
    }
}
