use super::{ExitCode, Signal, WaitState};

/// A Unix-like wait status.
///
/// On Unix-like systems, processes can terminate with a combination of exit codes and signals;
/// this struct encapsulates that information and can separate the exit code from the signal that
/// caused the termination, or whether the process was stopped or continued.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(transparent)
)]
pub struct WaitStatus(i32);

impl WaitStatus {
    /// Creates a new `UnixWaitStatus` from the underlying `i32` status code.
    #[must_use]
    pub const fn from_raw(status: i32) -> Self {
        Self(status)
    }

    /// Returns the underlying `i32` status code.
    #[must_use]
    pub const fn to_raw(&self) -> i32 {
        self.0
    }

    /// Returns the state represented by this wait status.
    #[must_use]
    pub const fn state(&self) -> WaitState {
        WaitState::from_raw(self.0)
    }

    /// Returns `true` if the process terminated, regardless of exiting normally or by a signal.
    #[must_use]
    pub const fn is_terminated(&self) -> bool {
        matches!(
            self.state(),
            WaitState::Exited { .. } | WaitState::Signaled { .. }
        )
    }

    /// Returns `true` if the process terminated normally (i.e., exited without a signal).
    #[must_use]
    pub const fn is_exited(&self) -> bool {
        matches!(self.state(), WaitState::Exited { .. })
    }

    /// Returns `true` if the process was terminated by a signal.
    #[must_use]
    pub const fn is_signaled(&self) -> bool {
        matches!(self.state(), WaitState::Signaled { .. })
    }

    /// Returns the exit code if the process terminated normally, or `None` otherwise.
    #[must_use]
    pub const fn exit_code(&self) -> Option<ExitCode> {
        match self.state() {
            WaitState::Exited { exit_code: code } => Some(code),
            _ => None,
        }
    }

    /// Returns the signal that resulted in this wait status, or `None` if the process exited.
    #[must_use]
    pub const fn signal(&self) -> Option<Signal> {
        match self.state() {
            WaitState::Signaled { signal, .. } => Some(signal),
            _ => None,
        }
    }
}

#[cfg(all(unix, feature = "std"))]
impl From<std::process::ExitStatus> for WaitStatus {
    fn from(status: std::process::ExitStatus) -> Self {
        if let Some(code) = status.code() {
            WaitStatus::from_raw(code)
        } else {
            use std::os::unix::process::ExitStatusExt;
            WaitStatus::from_raw(status.into_raw())
        }
    }
}

#[cfg(all(unix, feature = "std"))]
impl From<WaitStatus> for std::process::ExitStatus {
    fn from(status: WaitStatus) -> Self {
        use std::os::unix::process::ExitStatusExt;
        std::process::ExitStatus::from_raw(status.to_raw())
    }
}
