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
pub enum Error {
    /// An unclassified exit status on a Unix platform.
    Unix(unix::WaitStatus),

    /// An unclassified exit status on a Windows platform.
    Windows(windows::ExitCode),
}

#[cfg(feature = "std")]
impl Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::Unix(status) => write!(f, "Unix exit status: {}", status.to_raw()),
            Error::Windows(code) => write!(f, "Windows exit code: {}", code.to_raw()),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

/// A possible result of running and waiting for a process to terminate.
#[cfg(feature = "std")]
pub type ProcResult<T> = std::result::Result<T, Error>;

/// A trait that converts a result of running a process into a [`ProcResult`].
#[cfg(feature = "std")]
pub trait ToProcResult<T> {
    /// Converts the current result of running a process to a [`ProcResult`].
    ///
    /// # Errors
    ///
    /// If the result was a non-zero exit code, returns [`Error`].
    fn to_proc_result(&self) -> ProcResult<T>;
}

#[cfg(feature = "std")]
impl ToProcResult<()> for std::process::ExitStatus {
    #[allow(unreachable_code)]
    fn to_proc_result(&self) -> ProcResult<()> {
        if self.success() {
            Ok(())
        } else {
            #[cfg(unix)]
            {
                let err: unix::WaitStatus = self.into();
                return Err(Error::Unix(err));
            }
            #[cfg(windows)]
            {
                let err: windows::ExitCode = self.into();
                return Err(Error::Windows(err));
            }
            panic!("Cannot convert exit status to error on this platform");
        }
    }
}
