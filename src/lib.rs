//! A tiny cross-platforrm library containing exit status and code types.
//!
//! Unlike `std::process`, this crate does not require the standard library[^1], nor
//! `libc`, and can create and interpret exit codes of non-current platforms. For
//! example, on Windows, it can read and interpret exit codes that may have been
//! recorded from a Linux process, or vice versa.
//!
//! [^1]: The `std` feature is enabled by default, but can be disabled.
#![cfg_attr(not(feature = "std"), no_std)]

use core::fmt::Display;

use raw::RawExitCode;

pub mod raw;
pub mod unix;
pub mod windows;

// Import README.md so that doc tests run on it.
#[allow(dead_code)]
mod doc_tests {
    #[doc = include_str!("../README.md")]
    struct Readme;
}

/// An exit code or exit state returned by a program.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ProcResult {
    /// An unclassified exit status on a Unix platform.
    Unix(unix::WaitStatus),

    /// An unclassified exit status on a Windows platform.
    Windows(windows::ExitCode),
}

impl ProcResult {
    /// Returns a result that is `Ok` if the exit code or status indicates a success.
    ///
    /// # Errors
    ///
    /// Returns `Self` if not [`ProcResult::is_success`].
    pub fn ok(&self) -> Result<(), Self> {
        if self.is_success() {
            Ok(())
        } else {
            Err(*self)
        }
    }

    /// Returns whether the process terminated successfully.
    #[must_use]
    pub fn is_success(&self) -> bool {
        match self {
            ProcResult::Unix(status) => status.exit_code().is_some_and(|code| code.is_success()),
            ProcResult::Windows(code) => code.is_success(),
        }
    }

    /// Returns whether the process did not terminate successfully.
    #[must_use]
    pub fn is_failure(&self) -> bool {
        !self.is_success()
    }
}

#[cfg(feature = "std")]
impl Display for ProcResult {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Unix(status) => write!(f, "Unix exit status: {}", status.to_raw()),
            Self::Windows(code) => write!(f, "Windows exit code: {}", code.to_raw()),
        }
    }
}

impl core::error::Error for ProcResult {}

#[cfg(feature = "std")]
impl From<std::process::ExitStatus> for ProcResult {
    #[allow(unreachable_code)]
    fn from(status: std::process::ExitStatus) -> Self {
        #[cfg(unix)]
        {
            let err: unix::WaitStatus = status.into();
            return Self::Unix(err);
        }
        #[cfg(windows)]
        {
            let err: windows::ExitCode = status.into();
            return Self::Windows(err);
        }
        panic!("Cannot convert exit status to error on this platform");
    }
}
