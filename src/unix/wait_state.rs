use super::{ExitCode, Signal};

/// Conditions that interpret a Unix `int status` returned by `waitpid` or similar functions.
///
/// Provides methods to check the status of a process, such as whether it exited normally, was
/// terminated by a signal, stopped, or continued, and to retrieve the exit code or signal number
/// associated with the process's termination or stopping, without dependence on external crates
/// such as `libc`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WaitState {
    /// Indicates that the process exited normally with a specific exit code.
    Exited {
        /// The exit code of the process.
        exit_code: ExitCode,
    },

    /// Indicates that the process was terminated by a signal, with an optional core dump.
    Signaled {
        /// The signal that caused the termination.
        signal: Signal,

        /// Whether a core dump occurred.
        core_dump: bool,
    },

    /// Indicates a wait status code that is not recognized or supported.
    Unsupported(i32),
}

impl WaitState {
    /// Creates a new `UnixWaitIf` from the underlying `i32` status code.
    #[must_use]
    pub const fn from_raw(status: i32) -> Self {
        if Self::is_w_exited(status) {
            Self::Exited {
                exit_code: ExitCode::from_raw(Self::w_exit_status(status)),
            }
        } else if Self::is_w_signaled(status) {
            Self::Signaled {
                signal: Signal::from_raw(Self::w_term_sig(status)),
                core_dump: Self::is_w_coredump(status),
            }
        } else {
            Self::Unsupported(status)
        }
    }

    /// Returns a `i32` status code that represents the current wait status code.
    ///
    /// # Note
    ///
    /// It is not guaranteed that the result of this function will be reflexive, meaning that
    /// `UnixWaitIf::from_raw(status).to_raw() == status` may not always hold true; this function
    /// is provided as a convenience to be able to programatically create a wait status code where
    /// certain conditions are met, i.e. for testing or simulation purposes.
    #[must_use]
    pub const fn to_raw(&self) -> i32 {
        match self {
            Self::Exited { exit_code } => (exit_code.to_raw() as i32) << 8,
            Self::Signaled { signal, core_dump } => {
                (signal.to_raw() as i32) | if *core_dump { 0x80 } else { 0 }
            }
            Self::Unsupported(code) => *code,
        }
    }

    /// A copy of the Unix `WIFEXITED(status)` macro.
    #[allow(non_snake_case)]
    #[inline]
    #[must_use]
    const fn WSTOPSIG(status: i32) -> i32 {
        (status >> 8) & 0xFF
    }

    /// Represents the stopped status bit.
    const _WSTOPPED: i32 = 0x7F;

    /// A copy of the Unix `_WSTATUS(status)` macro.
    #[allow(non_snake_case)]
    #[inline]
    #[must_use]
    const fn _WSTATUS(status: i32) -> i32 {
        status & 0xFF
    }

    /// A copy of the Unix `WIFSIGNALED(status)` macro.
    #[allow(non_snake_case)]
    #[inline]
    #[must_use]
    const fn WIFSIGNALED(status: i32) -> bool {
        Self::_WSTATUS(status) != Self::_WSTOPPED && Self::_WSTATUS(status) != 0
    }

    /// A copy of the Unix `WTERMSIG(status)` macro.
    #[allow(non_snake_case)]
    #[inline]
    #[must_use]
    const fn WTERMSIG(status: i32) -> i32 {
        status & 0x7F
    }

    /// A copy of the Unix `WIFEXITED(status)` macro.
    #[allow(non_snake_case, clippy::verbose_bit_mask)]
    #[inline]
    #[must_use]
    const fn WIFEXITED(status: i32) -> bool {
        (status & 0o177) == 0
    }

    /// A copy of the Unix `WEXITSTATUS(status)` macro.
    #[allow(non_snake_case)]
    #[inline]
    #[must_use]
    const fn WEXITSTATUS(status: i32) -> i32 {
        (status >> 8) & 0xFF
    }

    /// A copy of the Unix `WCOREDUMP(status)` macro.
    #[allow(non_snake_case)]
    #[inline]
    #[must_use]
    const fn WCOREDUMP(status: i32) -> bool {
        (status & 0o200) != 0
    }

    /// Returns `true` if the status indicates that the process exited successfully.
    ///
    /// Equivalent to the Unix `WIFEXITED(status)` macro.
    #[must_use]
    pub const fn is_w_exited(status: i32) -> bool {
        Self::WIFEXITED(status)
    }

    /// Returns the exit code of the process.
    ///
    /// Equivalent to the Unix `WEXITSTATUS(status)` macro.
    ///
    /// # Panics
    ///
    /// If [`is_w_exited`](Self::is_w_exited) returns `false`, this function will panic.
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    #[must_use]
    pub const fn w_exit_status(status: i32) -> u8 {
        Self::WEXITSTATUS(status) as u8
    }

    /// Returns `true` if the status indicates that the process was terminated by a signal.
    ///
    /// Equivalent to the Unix `WIFSIGNALED(status)` macro.
    #[must_use]
    pub const fn is_w_signaled(status: i32) -> bool {
        Self::WIFSIGNALED(status)
    }

    /// Returns the signal number that caused the process to terminate.
    ///
    /// Equivalent to the Unix `WTERMSIG(status)` macro.
    ///
    /// # Panics
    ///
    /// If [`is_w_signaled`](Self::is_w_signaled) returns `false`, this function will panic.
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    #[must_use]
    pub const fn w_term_sig(status: i32) -> u8 {
        Self::WTERMSIG(status) as u8
    }

    /// Returns the signal number that caused the process to stop.
    ///
    /// Equivalent to the Unix `WSTOPSIG(status)` macro.
    ///
    /// # Panics
    ///
    /// If [`is_w_stopped`](Self::is_w_stopped) returns `false`, this function will panic.
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    #[must_use]
    pub const fn w_stop_sig(status: i32) -> u8 {
        Self::WSTOPSIG(status) as u8
    }

    /// Returns `true` if the status indicates a core dump occurred.
    ///
    /// Equivalent to the Unix `WCOREDUMP(status)` macro.
    ///
    /// # Note
    ///
    /// Not universally supported across all Unix-like systems; `WCOREDUMP` is a Linux/BSD
    /// extension, and is checked only if `WIFSIGNALED(status)` is true.
    #[must_use]
    pub const fn is_w_coredump(status: i32) -> bool {
        Self::WCOREDUMP(status)
    }
}

impl From<i32> for WaitState {
    fn from(status: i32) -> Self {
        Self::from_raw(status)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_raw_exited_0() {
        let status = WaitState::from_raw(0x0000_0000);
        assert_eq!(
            status,
            WaitState::Exited {
                exit_code: ExitCode::from_raw(0),
            }
        );
    }

    #[test]
    fn test_to_raw_exited_0() {
        let status = WaitState::Exited {
            exit_code: ExitCode::from_raw(0),
        };
        assert_eq!(status.to_raw(), 0x0000_0000);
    }

    #[test]
    fn test_from_raw_exited_1() {
        let status = WaitState::from_raw(0x0000_0100);
        assert_eq!(
            status,
            WaitState::Exited {
                exit_code: ExitCode::from_raw(1),
            }
        );
    }

    #[test]
    fn test_to_raw_exited_1() {
        let status = WaitState::Exited {
            exit_code: ExitCode::from_raw(1),
        };
        assert_eq!(status.to_raw(), 0x0000_0100);
    }

    #[test]
    fn test_from_raw_signaled() {
        let status = WaitState::from_raw(0x0000_0001);
        assert_eq!(
            status,
            WaitState::Signaled {
                signal: Signal::from_raw(1),
                core_dump: false,
            }
        );
    }

    #[test]
    fn test_to_raw_signaled() {
        let status = WaitState::Signaled {
            signal: Signal::from_raw(1),
            core_dump: false,
        };
        assert_eq!(status.to_raw(), 0x0000_0001);
    }

    #[test]
    fn test_from_raw_signaled_with_coredump() {
        let status = WaitState::from_raw(0x0000_0081);
        assert_eq!(
            status,
            WaitState::Signaled {
                signal: Signal::from_raw(1),
                core_dump: true,
            }
        );
    }

    #[test]
    fn test_to_raw_signaled_with_coredump() {
        let status = WaitState::Signaled {
            signal: Signal::from_raw(1),
            core_dump: true,
        };
        assert_eq!(status.to_raw(), 0x0000_0081);
    }
}

// Tests that compare the behavior of the `UnixWaitIf` struct with the libc macros.
#[cfg(all(test, unix))]
mod libc_verification_tests {
    use super::*;
    use libc::{WCOREDUMP, WEXITSTATUS, WIFEXITED, WIFSIGNALED, WSTOPSIG, WTERMSIG};

    #[test]
    fn test_wifexited_true() {
        assert!(WIFEXITED(0x0000_0000));
        assert!(WaitState::is_w_exited(0x0000_0000));
    }

    #[test]
    fn test_wifexited_false() {
        assert!(!WIFEXITED(0x0000_0001));
        assert!(!WaitState::is_w_exited(0x0000_0001));
    }

    #[test]
    fn test_wexitstatus_success() {
        assert_eq!(WEXITSTATUS(0x0000_0000), 0);
        assert_eq!(WaitState::w_exit_status(0x0000_0000), 0);
    }

    #[test]
    fn test_wexitstatus_failure() {
        assert_eq!(WEXITSTATUS(0x0000_0100), 1);
        assert_eq!(WaitState::w_exit_status(0x0000_0100), 1);
    }

    #[test]
    fn test_wifsignaled_true() {
        assert!(WIFSIGNALED(0x0000_0001));
        assert!(WaitState::is_w_signaled(0x0000_0001));
    }

    #[test]
    fn test_wifsignaled_false() {
        assert!(!WIFSIGNALED(0x0000_0000));
        assert!(!WaitState::is_w_signaled(0x0000_0000));
    }

    #[test]
    fn test_wtermsig() {
        assert_eq!(WTERMSIG(0x0000_0001), 1);
        assert_eq!(WaitState::w_term_sig(0x0000_0001), 1);
    }

    #[test]
    fn test_wstopsig() {
        assert_eq!(WaitState::w_stop_sig(0x0000_007F), 0);
        assert_eq!(WSTOPSIG(0x0000_007F), 0);
    }

    #[test]
    fn test_wcoredump_true() {
        assert!(WCOREDUMP(0x0000_0081));
        assert!(WaitState::is_w_coredump(0x0000_0081));
    }

    #[test]
    fn test_wcoredump_false() {
        assert!(!WCOREDUMP(0x0000_0001));
        assert!(!WaitState::is_w_coredump(0x0000_0001));
    }
}
