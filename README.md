# proc-result

A tiny cross-platform library containing exit status and code types.

Unlike `std::process`, this crate does not require the standard library[^1], nor
`libc`, and can create and interpret exit codes of non-current platforms. For
example, on Windows, it can read and interpret exit codes that may have been
recorded from a Linux process, or vice versa.

[^1]: The `std` feature is enabled by default, but can be disabled.

## Usage

Most users of the crate will use the `ToProcResult` trait, which converts the
result a run of a subprocess to a `ProcResult`, either a successful or an error
explaining what exit code or (on Unix platforms) the signal the subprocess was
prematurely terminated with, and is constructed from a
`std::process::ExitStatus`:

```rust
use proc_result::ToProcResult;
use std::error::Error;
use std::process::Command;

fn main() -> Result<(), Box<dyn Error>> {
    let result = Command::new("ls").status()?.to_proc_result()?;
    Ok(())
}
```

Advanced users, or users writing tests or interpreting exit codes from other
platforms may import and use the platform-specific exit code types directly,
from the `unix` or `windows` modules, respectively. For example, to create a
Unix exit code from a raw integer:

```rust
use proc_result::unix::ExitCode;

let code = ExitCode::from_raw(1);
if code.is_success() {
    println!("Command succeeded!");
} else {
    eprintln!("Command failed with exit code: {}", code);
}
```

## Features

Name    | Default | Description
------- | ------- | -----------
`serde` | `false` | Enables serialization support for most types using `serde`.
`std`   | `true`  | Enables compatibility with `std::process::ExitStatus`.
