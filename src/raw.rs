//! Raw exit code and status representations.
//!
//! The `raw` module provides a platform-agnostic representation of exit codes [`RawExitCode`], as
//! well as a concrete implementation for Unix and Windows platforms. While useful, the `raw` module
//! is not necessary for most applications.

use core::fmt::Display;
use std::fmt::Debug;

use num_traits::{PrimInt, Zero};

/// A trait that represents a raw platform-specific exit code.
pub trait RawExitCode: Clone + Copy + Debug + PartialEq + Eq {
    /// Underlying code type.
    type Code: PrimInt + Display;

    /// Returns whether the exit status indicates success.
    fn is_success(&self) -> bool {
        self.to_raw().is_zero()
    }

    /// Returns whether the exit status indicates failure.
    fn is_failure(&self) -> bool {
        !self.is_success()
    }

    /// Create a [`RawExitCode`] from the underlying code.
    fn from_raw(code: Self::Code) -> Self;

    /// Returns the underlying code.
    fn to_raw(&self) -> Self::Code;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unix::ExitCode;

    #[test]
    fn test_into_raw_exit_code() {
        fn arg_raw_exit_code<T: RawExitCode>(code: impl Into<T>) -> T {
            code.into()
        }

        let code: ExitCode = arg_raw_exit_code(0);
        assert!(code.is_success());
        assert_eq!(code.to_raw(), 0);

        let code: ExitCode = arg_raw_exit_code(1);
        assert!(code.is_failure());
        assert_eq!(code.to_raw(), 1);
    }
}
