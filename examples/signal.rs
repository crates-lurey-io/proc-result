use proc_result::ProcResult;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut child = std::process::Command::new("examples/hang_forever.sh").spawn()?;
    let pid = child.id();

    // Spawn a thread, and in 1s, use the PID to kill the process (not the child handle).
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_secs(1));
        println!("Using SIGTERM to kill process {pid}");
        let _ = std::process::Command::new("kill")
            .arg("-9")
            .arg(pid.to_string())
            .status();
    });

    println!("Waiting for process to end");
    let result: ProcResult = child.wait()?.into();
    result.ok()?;

    // Will never happen.
    println!("Done!");
    Ok(())
}
