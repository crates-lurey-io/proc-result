use core::fmt::Display;

use crate::raw::RawExitCode;

/// A Unix-like exit code.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(transparent)
)]
pub struct ExitCode(u8);

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

    /// The command line arguments were invalid or used incorrectly.
    ///
    /// Corresponds to exit code `2`.
    ///
    /// Often used by shell builtins (e.g., `bash`'s `exit 2` for `builtin_status_bad_usage`).
    pub const INVALID_ARGS: Self = Self(2);

    /// The command was used incorrectly.
    ///
    /// Corresponds to exit code `64` (`EX_USAGE` from `sysexits.h`).
    pub const USAGE: Self = Self(64);

    /// The program received a malformed or invalid input.
    ///
    /// Corresponds to exit code `65` (`EX_DATAERR` from `sysexits.h`).
    pub const DATA_ERROR: Self = Self(65);

    /// An input file (not a system file) did not exist or was not readable.
    ///
    /// Corresponds to exit code `66` (`EX_NOINPUT` from `sysexits.h`).
    pub const NO_INPUT: Self = Self(66);

    /// The user specified did not exist.
    ///
    /// Corresponds to exit code `67` (`EX_NOUSER` from `sysexits.h`).
    pub const NO_USER: Self = Self(67);

    /// The host specified did not exist.
    ///
    /// Corresponds to exit code `68` (`EX_NOHOST` from `sysexits.h`).
    pub const NO_HOST: Self = Self(68);

    /// A server is unavailable.
    ///
    /// Corresponds to exit code `69` (`EX_UNAVAILABLE` from `sysexits.h`).
    ///
    /// Often used when something does not work as expected.
    pub const UNAVAILABLE: Self = Self(69);

    /// An internal software error occurred.
    ///
    /// Corresponds to exit code `70` (`EX_SOFTWARE` from `sysexits.h`).
    ///
    /// Should be limited to non-operating system software errors.
    pub const SOFTWARE: Self = Self(70);

    /// An operating system error occurred.
    ///
    /// Corresponds to exit code `71` (`EX_OSERR` from `sysexits.h`).
    ///
    /// Intended to be used for errors such as "cannot fork" or "cannot create pipe".
    pub const OS_ERROR: Self = Self(71);

    /// A system file did not exist, cannot be opened, or has an incorrect format.
    ///
    /// Corresponds to exit code `72` (`EX_OSFILE` from `sysexits.h`).
    pub const OS_FILE: Self = Self(72);

    /// A (user specified) output file cannot be created.
    ///
    /// Corresponds to exit code `73` (`EX_CANTCREAT` from `sysexits.h`).
    pub const CANT_CREATE: Self = Self(73);

    /// An error occurred while reading or writing to a file.
    ///
    /// Corresponds to exit code `74` (`EX_IOERR` from `sysexits.h`).
    pub const IO_ERROR: Self = Self(74);

    /// A temporary failure occurred.
    ///
    /// Corresponds to exit code `75` (`EX_TEMPFAIL` from `sysexits.h`).
    ///
    /// The request may be retried later.
    pub const TEMP_FAIL: Self = Self(75);

    /// A remote system returned something that was "not possible" during a protocol exchange.
    ///
    /// Corresponds to exit code `76` (`EX_PROTOCOL` from `sysexits.h`).
    pub const PROTOCOL: Self = Self(76);

    /// The user specified did not have sufficient permissions to perform the operation.
    ///
    /// Corresponds to exit code `77` (`EX_NOPERM` from `sysexits.h`).
    ///
    /// Not intended for file system problems, (use [`ExitCode::NO_INPUT`] or
    /// [`ExitCode::CANT_CREATE`]) but for higher-level permissions.
    pub const NO_PERM: Self = Self(77);

    /// Something was found in an unconfigured or misconfigured state.
    ///
    /// Corresponds to exit code `78` (`EX_CONFIG` from `sysexits.h`).
    pub const CONFIG: Self = Self(78);

    /// The command was found but could not be executed (e.g. permission denied).
    ///
    /// Corresponds to exit code `126`.
    ///
    /// Often returned by shells when a command is found but not executable.
    pub const COMMAND_CANNOT_EXECUTE: Self = Self(126);

    /// The command or program could not be found.
    ///
    /// Corresponds to exit code `127`.
    ///
    /// Often returned by shells when a command is not found in `PATH`.
    pub const COMMAND_NOT_FOUND: Self = Self(127);

    /// Creates a new `UnixExitCode` from the underlying `u8` code.
    #[must_use]
    pub const fn from_raw(code: u8) -> Self {
        Self(code)
    }

    /// Returns the underlying `u8` code.
    #[must_use]
    pub const fn to_raw(&self) -> u8 {
        self.0
    }

    /// Returns `true` if the exit code represents a successful termination.
    #[must_use]
    pub const fn is_success(&self) -> bool {
        self.0 == Self::SUCCESS.to_raw()
    }

    /// Returns `true` if the exit code represents a failure termination.
    #[must_use]
    pub const fn is_failure(&self) -> bool {
        !self.is_success()
    }
}

impl Display for ExitCode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

impl RawExitCode for ExitCode {
    type Code = u8;

    fn from_raw(code: Self::Code) -> Self {
        ExitCode::from_raw(code)
    }

    fn to_raw(&self) -> Self::Code {
        self.to_raw()
    }
}

impl From<u8> for ExitCode {
    fn from(code: u8) -> Self {
        ExitCode::from_raw(code)
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
    fn test_from_u8() {
        let code: ExitCode = 1.into();
        assert_eq!(code.to_raw(), 1);
    }
}

#[cfg(all(test, feature = "serde"))]
mod serde_tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_serde() {
        let code = ExitCode::GENERAL_ERROR;
        let serialized = serde_json::to_string(&code).unwrap();
        let deserialized: ExitCode = serde_json::from_str(&serialized).unwrap();
        assert_eq!(code, deserialized);
    }
}
