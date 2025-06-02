//! Windows-specific exit codes and handling.
//!
//! This module is cross-platform, but on Windows, it provides conversions to/from
//! [`std::process::ExitStatus`].

use crate::raw::RawExitCode;
use core::fmt::Display;

/// A Windows-specific exit code.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(transparent)
)]
pub struct ExitCode(u32);

impl ExitCode {
    /// The program terminated successfully.
    ///
    /// Corresponds to exit code `0`.
    ///
    /// This is the universal success code.
    pub const SUCCESS: Self = Self(0);

    /// The program terminated with a general, unspecified error.
    ///
    /// Corresponds to exit code `1`.
    ///
    /// This is a common "catch-all" for general failures.
    pub const GENERAL_ERROR: Self = Self(1);

    /// The system cannot find the file specified.
    ///
    /// Corresponds to exit code `2` (`ERROR_FILE_NOT_FOUND`).
    pub const FILE_NOT_FOUND: Self = Self(2);

    /// The system cannot find the path specified.
    ///
    /// Corresponds to exit code `3` (`ERROR_PATH_NOT_FOUND`).
    pub const PATH_NOT_FOUND: Self = Self(3);

    /// Access is denied.
    ///
    /// Corresponds to exit code `5` (`ERROR_ACCESS_DENIED`).
    pub const ACCESS_DENIED: Self = Self(5);

    /// Not enough storage is available to process this command.
    ///
    /// Corresponds to exit code `8` (`ERROR_NOT_ENOUGH_MEMORY`).
    pub const NOT_ENOUGH_MEMORY: Self = Self(8);

    /// The parameter is incorrect.
    ///
    /// Corresponds to exit code `87` (`ERROR_INVALID_PARAMETER`).
    pub const INVALID_PARAMETER: Self = Self(87);

    /// The pipe has been ended.
    ///
    /// Corresponds to exit code `109` (`ERROR_BROKEN_PIPE`).
    pub const BROKEN_PIPE: Self = Self(109);

    /// The program is not recognized as a command, operable program, or batch file.
    ///
    /// Corresponds to exit code `9009`.
    ///
    /// This is a common code returned by `cmd.exe` when a command cannot be found or executed.
    pub const COMMAND_NOT_RECOGNIZED: Self = Self(9009);

    /// The program terminated as a result of a CTRL+C or Ctrl+Break signal.
    ///
    /// Cooresponds to exit code `0xC000_013A`.
    pub const TERMINATED_BY_CTRL_C: Self = Self(0xC000_013A);

    /// The program was terminate due to an access violation.
    ///
    /// Corresponds to exit code `0xC000_0005`.
    pub const ACCESS_VIOLATION: Self = Self(0xC000_0005);

    /// The program terminated due to a stack oveflow.
    ///
    /// Corresponds to exit code `0xC000_00FD`.
    pub const STACK_OVERFLOW: Self = Self(0xC000_00FD);
}

impl RawExitCode for ExitCode {
    type Code = u32;

    fn from_raw(code: Self::Code) -> Self {
        Self(code)
    }

    fn to_raw(&self) -> Self::Code {
        self.0
    }
}

impl From<u32> for ExitCode {
    fn from(code: u32) -> Self {
        ExitCode::from_raw(code)
    }
}

impl Display for ExitCode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(all(windows, feature = "std"))]
impl From<std::process::ExitStatus> for ExitCode {
    fn from(status: std::process::ExitStatus) -> ExitCode {
        ExitCode::from_raw(
            status
                .code()
                .expect("cannot fail on Windows")
                .try_into()
                .unwrap(),
        )
    }
}

#[cfg(all(windows, feature = "std"))]
impl From<ExitCode> for std::process::ExitStatus {
    fn from(code: ExitCode) -> Self {
        use std::os::windows::process::ExitStatusExt;
        std::process::ExitStatus::from_raw(code.to_raw())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_raw() {
        assert_eq!(ExitCode::from_raw(0).to_raw(), 0);
    }

    #[test]
    fn test_is_success() {
        assert!(ExitCode::SUCCESS.is_success());
    }

    #[test]
    fn test_is_failure() {
        assert!(ExitCode::GENERAL_ERROR.is_failure());
    }

    #[test]
    fn test_from_u32() {
        let code: ExitCode = 1.into();
        assert_eq!(code.to_raw(), 1);
    }

    #[test]
    #[cfg(all(feature = "std", windows))]
    fn test_from_exit_status() {
        use std::os::windows::process::ExitStatusExt;
        use std::process::ExitStatus;

        // Simulate a successful exit status
        let success_status = ExitStatus::from_raw(0);
        let success_code: ExitCode = (&success_status).into();
        assert!(success_code.is_success());
        assert_eq!(success_code.to_raw(), 0);

        // Simulate a failure exit status
        let failure_status = ExitStatus::from_raw(1);
        let failure_code: ExitCode = (&failure_status).into();
        assert!(failure_code.is_failure());
        assert_eq!(failure_code.to_raw(), 1);
    }
}

#[cfg(all(test, windows, feature = "serde"))]
mod serde_tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_serde() {
        let code = ExitCode::SUCCESS;
        let serialized = serde_json::to_string(&code).unwrap();
        let deserialized: ExitCode = serde_json::from_str(&serialized).unwrap();
        assert_eq!(code, deserialized);
    }
}
