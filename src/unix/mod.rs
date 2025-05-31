//! Unix-specific process signals and exit codes.
//!
//! This module is cross-platform, but on Unix systems, it provides conversions to/from
//! [`std::process::ExitStatus`].

mod exit_code;
pub use exit_code::ExitCode;

mod signal;
pub use signal::Signal;

mod wait_state;
pub use wait_state::WaitState;

mod wait_status;
pub use wait_status::WaitStatus;
